use std::collections::{HashMap, HashSet, VecDeque};

use super::{graph::Graph, types::PortType};
use crate::error::{FlowError, Result};

/// 图验证器，负责检查图的结构合法性
///
/// 验证器不修改图，只返回 `Ok(())` 或错误列表。
/// `validate` 保留快速失败语义；`collect_errors` 返回当前所有错误。
pub struct GraphValidator;

impl GraphValidator {
    /// 执行全部验证，返回第一个错误（快速失败）。
    pub fn validate(graph: &Graph) -> Result<()> {
        Self::check_unique_ids(graph)?;
        Self::check_edge_endpoints(graph)?;
        Self::check_type_compatibility(graph)?;
        Self::check_single_input_per_port(graph)?;
        Self::check_no_cycles(graph)?;
        Self::check_required_params(graph)?;
        Ok(())
    }

    /// 收集所有验证错误，包含阻塞错误和非阻塞警告。
    pub fn collect_errors(graph: &Graph) -> Vec<FlowError> {
        let mut errors = Vec::new();

        if let Err(e) = Self::check_unique_ids(graph) {
            errors.push(e);
        }
        if let Err(e) = Self::check_edge_endpoints(graph) {
            errors.push(e);
        }
        if let Err(e) = Self::check_type_compatibility(graph) {
            errors.push(e);
        }
        if let Err(e) = Self::check_single_input_per_port(graph) {
            errors.push(e);
        }
        if let Err(e) = Self::check_no_cycles(graph) {
            errors.push(e);
        }
        if let Err(e) = Self::check_required_params(graph) {
            errors.push(e);
        }

        // 非阻塞警告：不阻止代码生成，仅提示。
        errors.extend(Self::warn_multiple_starts(graph));
        errors.extend(Self::warn_unreachable_nodes(graph));
        errors.extend(Self::warn_diamond_reachable(graph));

        errors
    }

    /// 检查节点 ID 唯一性
    ///
    /// 理论上 `HashMap` 的键已经唯一，此检查用于捕获 JSON 中重复的 ID
    fn check_unique_ids(graph: &Graph) -> Result<()> {
        let mut ids = HashSet::new();
        for id in graph.nodes.keys() {
            if !ids.insert(id) {
                return Err(FlowError::Validation(format!("Duplicate node id: {}", id)));
            }
        }
        Ok(())
    }

    /// 检查每条边的端点是否都指向存在的节点
    fn check_edge_endpoints(graph: &Graph) -> Result<()> {
        for edge in graph.edges.values() {
            if !graph.nodes.contains_key(&edge.from.node_id) {
                return Err(FlowError::NodeNotFound(edge.from.node_id.clone()));
            }
            if !graph.nodes.contains_key(&edge.to.node_id) {
                return Err(FlowError::NodeNotFound(edge.to.node_id.clone()));
            }
        }
        Ok(())
    }

    /// 检查边的端口类型是否兼容
    fn check_type_compatibility(graph: &Graph) -> Result<()> {
        for edge in graph.edges.values() {
            let from_node = graph
                .nodes
                .get(&edge.from.node_id)
                .ok_or_else(|| FlowError::NodeNotFound(edge.from.node_id.clone()))?;
            let to_node = graph
                .nodes
                .get(&edge.to.node_id)
                .ok_or_else(|| FlowError::NodeNotFound(edge.to.node_id.clone()))?;

            let from_port = from_node
                .get_port(&edge.from.port_id, false)
                .ok_or_else(|| {
                    FlowError::ConnectionError(format!(
                        "Output port {} not found on node {}",
                        edge.from.port_id, edge.from.node_id
                    ))
                })?;
            let to_port = to_node.get_port(&edge.to.port_id, true).ok_or_else(|| {
                FlowError::ConnectionError(format!(
                    "Input port {} not found on node {}",
                    edge.to.port_id, edge.to.node_id
                ))
            })?;

            if !from_port.port_type.is_compatible_with(&to_port.port_type) {
                return Err(FlowError::TypeMismatch {
                    expected: format!("{:?}", to_port.port_type),
                    actual: format!("{:?}", from_port.port_type),
                });
            }
        }
        Ok(())
    }

    /// 检查数据输入端口是否只有一条入边
    ///
    /// Flow 输入端口允许有多条入边，Data 输入端口只能有一条
    fn check_single_input_per_port(graph: &Graph) -> Result<()> {
        let mut port_connections: HashSet<(String, String)> = HashSet::new();

        for edge in graph.edges.values() {
            if edge.edge_type != PortType::Flow {
                let key = (edge.to.node_id.clone(), edge.to.port_id.clone());
                if !port_connections.insert(key) {
                    return Err(FlowError::ConnectionError(format!(
                        "Input port {} on node {} has multiple connections",
                        edge.to.port_id, edge.to.node_id
                    )));
                }
            }
        }
        Ok(())
    }

    /// 检查 Flow 边是否构成有向无环图（DAG）
    ///
    /// 使用 Kahn 算法进行拓扑排序，如果访问节点数少于总节点数，则存在环。
    /// 若存在环，返回环上涉及的节点 ID 列表。
    fn check_no_cycles(graph: &Graph) -> Result<()> {
        // 构建邻接表和入度表（仅考虑 Flow 边）
        let mut adj: HashMap<String, Vec<String>> = HashMap::new();
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut parent: HashMap<String, String> = HashMap::new();

        for node_id in graph.nodes.keys() {
            adj.insert(node_id.clone(), Vec::new());
            in_degree.insert(node_id.clone(), 0);
        }

        for edge in graph.edges.values() {
            if edge.edge_type == PortType::Flow {
                if let Some(neighbors) = adj.get_mut(&edge.from.node_id) {
                    neighbors.push(edge.to.node_id.clone());
                }
                if let Some(deg) = in_degree.get_mut(&edge.to.node_id) {
                    *deg += 1;
                }
            }
        }

        // Kahn 算法
        let mut queue: VecDeque<String> = in_degree
            .iter()
            .filter(|(_, deg)| **deg == 0)
            .map(|(id, _)| id.clone())
            .collect();

        let mut visited = 0;
        while let Some(node_id) = queue.pop_front() {
            visited += 1;
            if let Some(neighbors) = adj.get(&node_id) {
                for neighbor in neighbors {
                    parent.insert(neighbor.clone(), node_id.clone());
                    if let Some(deg) = in_degree.get_mut(neighbor) {
                        *deg -= 1;
                        if *deg == 0 {
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
        }

        if visited == graph.nodes.len() {
            return Ok(());
        }

        // 找到环：从任意未访问节点回溯 parent 链
        let unvisited = graph
            .nodes
            .keys()
            .find(|id| !queue.is_empty() || in_degree.get(*id).copied().unwrap_or(0) > 0)
            .cloned()
            .unwrap_or_default();

        let cycle = reconstruct_cycle(&parent, &adj, &unvisited);
        Err(FlowError::CycleDetected(cycle))
    }

    /// 检查必填参数
    ///
    /// 当前实现仅检查是否存在 `ParamValue::Null` 的参数值。
    /// 接入 `api::definitions` 后可进一步检查每个节点类型的必填字段。
    fn check_required_params(graph: &Graph) -> Result<()> {
        for node in graph.nodes.values() {
            for (name, value) in &node.params {
                if matches!(value, super::node::ParamValue::Null) {
                    return Err(FlowError::Validation(format!(
                        "Node {} has null value for required parameter '{}'",
                        node.id, name
                    )));
                }
            }
        }
        Ok(())
    }

    /// 警告：存在多个 Start 节点（非阻塞，但建议只保留一个）。
    fn warn_multiple_starts(graph: &Graph) -> Vec<FlowError> {
        use crate::graph::types::NodeType;
        let starts: Vec<&String> = graph
            .nodes
            .iter()
            .filter(|(_, n)| n.node_type == NodeType::Start)
            .map(|(id, _)| id)
            .collect();
        if starts.len() > 1 {
            vec![FlowError::Warning(format!(
                "存在 {} 个 Start 节点（通常只需要一个）：{}",
                starts.len(),
                starts.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")
            ))]
        } else {
            Vec::new()
        }
    }

    /// 警告：存在无法从任何 Start 节点或子标签入口沿 Flow 边到达的节点。
    fn warn_unreachable_nodes(graph: &Graph) -> Vec<FlowError> {
        use crate::graph::types::NodeType;
        let mut reachable = HashSet::new();
        let mut queue: VecDeque<String> = VecDeque::new();

        // 源 1：所有 Start 节点
        for (id, _) in graph.nodes.iter().filter(|(_, n)| n.node_type == NodeType::Start) {
            if reachable.insert(id.clone()) {
                queue.push_back(id.clone());
            }
        }

        // 源 2：每个非 main 标签的首节点（子标签入口）
        for (name, ids) in &graph.labels {
            if name.starts_with("main") {
                continue;
            }
            if let Some(first) = ids.first() {
                if reachable.insert(first.clone()) {
                    queue.push_back(first.clone());
                }
            }
        }

        // 源 3：Label 节点自身（name 参数匹配标签名但未注册到 labels 列表的）
        for node in graph.nodes.values() {
            if node.node_type == NodeType::Label {
                if let Some(label_name) = node.params.get("name").and_then(|v| match v {
                    crate::graph::node::ParamValue::Literal(val) => val.as_str().map(|s| s.to_string()),
                    _ => None,
                }) {
                    if !label_name.starts_with("main")
                        && graph.labels.contains_key(&label_name)
                        && reachable.insert(node.id.clone())
                    {
                        queue.push_back(node.id.clone());
                    }
                }
            }
        }

        // BFS 沿 Flow 边
        while let Some(current) = queue.pop_front() {
            for edge in graph.edges.values() {
                if edge.edge_type == PortType::Flow
                    && edge.from.node_id == current
                    && reachable.insert(edge.to.node_id.clone())
                {
                    queue.push_back(edge.to.node_id.clone());
                }
            }
        }

        let unreachable: Vec<String> = graph
            .nodes
            .iter()
            .filter(|(id, n)| {
                if !reachable.contains(*id) {
                    // Data-only 节点（零 Flow 端口）不需要接入，跳过
                    let has_flow = n.inputs.iter().any(|p| p.port_type == PortType::Flow)
                        || n.outputs.iter().any(|p| p.port_type == PortType::Flow);
                    has_flow
                } else {
                    false
                }
            })
            .map(|(id, _)| id.clone())
            .collect();

        if unreachable.is_empty() {
            Vec::new()
        } else {
            vec![FlowError::Warning(format!(
                "{} 个节点无法从 Start 或标签入口到达（缺少 Flow 连接）：{}",
                unreachable.len(),
                unreachable.join(", ")
            ))]
        }
    }

    /// 警告：存在从 Start 出发通过多条路径可达的节点（菱形依赖），
    /// 可能导致代码生成后的控制流路径被重复执行。
    fn warn_diamond_reachable(graph: &Graph) -> Vec<FlowError> {
        use crate::graph::types::NodeType;
        for (start_id, _) in graph.nodes.iter().filter(|(_, n)| n.node_type == NodeType::Start) {
            let mut branch_of: HashMap<String, String> = HashMap::new();
            let mut queue: VecDeque<(String, String)> = VecDeque::new();

            for edge in graph.edges.values() {
                if edge.edge_type == PortType::Flow && edge.from.node_id == *start_id {
                    branch_of.insert(edge.to.node_id.clone(), edge.to.node_id.clone());
                    queue.push_back((edge.to.node_id.clone(), edge.to.node_id.clone()));
                }
            }

            while let Some((current, branch_root)) = queue.pop_front() {
                for edge in graph.edges.values() {
                    if edge.edge_type == PortType::Flow && edge.from.node_id == current {
                        if let Some(existing) = branch_of.get(&edge.to.node_id) {
                            if *existing != branch_root {
                                return vec![FlowError::Warning(format!(
                                    "节点 {} 从 Start ({}) 被多条路径到达（分支 {} 和 {}），菱形可能重复执行",
                                    edge.to.node_id, start_id, existing, branch_root
                                ))];
                            }
                        } else {
                            branch_of.insert(edge.to.node_id.clone(), branch_root.clone());
                            queue.push_back((edge.to.node_id.clone(), branch_root.clone()));
                        }
                    }
                }
            }
        }
        Vec::new()
    }
}

/// 从未访问节点出发重建环路径。
fn reconstruct_cycle(
    parent: &HashMap<String, String>,
    adj: &HashMap<String, Vec<String>>,
    start: &str,
) -> Vec<String> {
    let mut path: Vec<String> = vec![start.to_string()];
    let mut visited_in_path: HashSet<String> = HashSet::new();
    visited_in_path.insert(start.to_string());

    // 沿父链回溯，直到遇到已在路径中的节点
    let mut current = parent.get(start).cloned();
    while let Some(node) = current {
        if visited_in_path.contains(&node) {
            // 截断到环起点
            if let Some(start_idx) = path.iter().position(|id| id == &node) {
                path = path.split_off(start_idx);
            }
            path.push(node);
            return path;
        }
        path.push(node.clone());
        visited_in_path.insert(node.clone());
        current = parent.get(&node).cloned();
    }

    // 父链无环，则尝试从 start 沿邻接表前进
    current = Some(start.to_string());
    while let Some(node) = current {
        if let Some(neighbors) = adj.get(&node) {
            if let Some(next) = neighbors.first() {
                if visited_in_path.contains(next) {
                    if let Some(start_idx) = path.iter().position(|id| id == next) {
                        path = path.split_off(start_idx);
                    }
                    path.push(next.clone());
                    return path;
                }
                path.push(next.clone());
                visited_in_path.insert(next.clone());
                current = Some(next.clone());
                continue;
            }
        }
        break;
    }

    path
}

#[cfg(test)]
mod tests {
    use super::super::edge::{Edge, EdgeEndpoint};
    use super::super::node::{Node, ParamValue, Port, Vec2};
    use super::super::types::{NodeType, PortType};
    use super::*;

    fn make_flow_node(id: &str) -> Node {
        Node {
            id: id.to_string(),
            node_type: NodeType::Log,
            position: Vec2::ZERO,
            size: Vec2::new(180.0, 120.0),
            collapsed: false,
            params: std::collections::HashMap::new(),
            inputs: vec![Port::new("in_flow", PortType::Flow, "执行")],
            outputs: vec![Port::new("out_flow", PortType::Flow, "下一步")],
            category: String::new(),
        }
    }

    fn make_data_node(id: &str) -> Node {
        Node {
            id: id.to_string(),
            node_type: NodeType::Random,
            position: Vec2::ZERO,
            size: Vec2::new(180.0, 120.0),
            collapsed: false,
            params: std::collections::HashMap::new(),
            inputs: vec![Port::new("in_flow", PortType::Flow, "执行")],
            outputs: vec![
                Port::new("out_flow", PortType::Flow, "下一步"),
                Port::new("out_value", PortType::Number, "值"),
            ],
            category: String::new(),
        }
    }

    fn add_flow_edge(graph: &mut Graph, from: &str, to: &str) {
        let edge = Edge::new(
            EdgeEndpoint::new(from, "out_flow"),
            EdgeEndpoint::new(to, "in_flow"),
            PortType::Flow,
        );
        graph.add_edge(edge).unwrap();
    }

    #[test]
    fn test_valid_graph_passes() {
        let mut graph = Graph::default();
        let n1 = make_flow_node("node_1");
        let n2 = make_flow_node("node_2");
        graph.add_node(n1);
        graph.add_node(n2);
        add_flow_edge(&mut graph, "node_1", "node_2");

        assert!(GraphValidator::validate(&graph).is_ok());
    }

    #[test]
    fn test_cycle_detected() {
        let mut graph = Graph::default();
        let n1 = make_flow_node("node_1");
        let n2 = make_flow_node("node_2");
        let n3 = make_flow_node("node_3");
        graph.add_node(n1);
        graph.add_node(n2);
        graph.add_node(n3);

        add_flow_edge(&mut graph, "node_1", "node_2");
        add_flow_edge(&mut graph, "node_2", "node_3");
        add_flow_edge(&mut graph, "node_3", "node_1");

        let errors = GraphValidator::collect_errors(&graph);
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, FlowError::CycleDetected(_)))
        );
    }

    #[test]
    fn test_type_mismatch() {
        let mut graph = Graph::default();
        let n1 = make_data_node("node_1");
        let n2 = Node {
            id: "node_2".to_string(),
            node_type: NodeType::Log,
            position: Vec2::ZERO,
            size: Vec2::new(180.0, 120.0),
            collapsed: false,
            params: std::collections::HashMap::new(),
            inputs: vec![
                Port::new("in_flow", PortType::Flow, "执行"),
                Port::new("in_msg", PortType::String, "消息"),
            ],
            outputs: vec![Port::new("out_flow", PortType::Flow, "下一步")],
            category: String::new(),
        };
        graph.add_node(n1);
        graph.add_node(n2);

        let edge = Edge::new(
            EdgeEndpoint::new("node_1", "out_value"),
            EdgeEndpoint::new("node_2", "in_msg"),
            PortType::Number,
        );
        graph.add_edge(edge).unwrap();

        assert!(matches!(
            GraphValidator::validate(&graph),
            Err(FlowError::TypeMismatch { .. })
        ));
    }

    #[test]
    fn test_multiple_input_edges_to_data_port() {
        let mut graph = Graph::default();
        let n1 = make_data_node("node_1");
        let n2 = make_data_node("node_2");
        let n3 = Node {
            id: "node_3".to_string(),
            node_type: NodeType::AddCurrentRP,
            position: Vec2::ZERO,
            size: Vec2::new(180.0, 120.0),
            collapsed: false,
            params: std::collections::HashMap::new(),
            inputs: vec![
                Port::new("in_flow", PortType::Flow, "执行"),
                Port::new("in_value", PortType::Number, "值"),
            ],
            outputs: vec![Port::new("out_flow", PortType::Flow, "下一步")],
            category: String::new(),
        };
        graph.add_node(n1);
        graph.add_node(n2);
        graph.add_node(n3);

        let edge1 = Edge::new(
            EdgeEndpoint::new("node_1", "out_value"),
            EdgeEndpoint::new("node_3", "in_value"),
            PortType::Number,
        );
        let edge2 = Edge::new(
            EdgeEndpoint::new("node_2", "out_value"),
            EdgeEndpoint::new("node_3", "in_value"),
            PortType::Number,
        );
        graph.add_edge(edge1).unwrap();
        graph.add_edge(edge2).unwrap();

        assert!(matches!(
            GraphValidator::validate(&graph),
            Err(FlowError::ConnectionError(_))
        ));
    }

    #[test]
    fn test_multiple_input_edges_to_flow_port_allowed() {
        let mut graph = Graph::default();
        let n1 = make_flow_node("node_1");
        let n2 = make_flow_node("node_2");
        let n3 = make_flow_node("node_3");
        graph.add_node(n1);
        graph.add_node(n2);
        graph.add_node(n3);

        add_flow_edge(&mut graph, "node_1", "node_3");
        add_flow_edge(&mut graph, "node_2", "node_3");

        assert!(GraphValidator::validate(&graph).is_ok());
    }

    #[test]
    fn test_missing_edge_endpoint() {
        let mut graph = Graph::default();
        let n1 = make_flow_node("node_1");
        graph.add_node(n1);

        let edge = Edge::new(
            EdgeEndpoint::new("node_1", "out_flow"),
            EdgeEndpoint::new("node_2", "in_flow"),
            PortType::Flow,
        );
        // 绕过 add_edge 的端点检查，直接插入边，以验证验证器能否发现缺失节点
        graph.edges.insert(edge.id.clone(), edge);

        assert!(matches!(
            GraphValidator::validate(&graph),
            Err(FlowError::NodeNotFound(_))
        ));
    }

    #[test]
    fn test_null_param_fails() {
        let mut graph = Graph::default();
        let mut n1 = make_flow_node("node_1");
        n1.set_param("output", ParamValue::Null);
        graph.add_node(n1);

        assert!(matches!(
            GraphValidator::validate(&graph),
            Err(FlowError::Validation(_))
        ));
    }
}
