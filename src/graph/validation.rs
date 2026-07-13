use std::collections::{HashMap, HashSet, VecDeque};

use super::container::{ContainerGraph, LabelContainer, ThreadContainer};
use super::types::PortType;
use crate::error::{FlowError, Result};

/// 图验证器（新架构）
///
/// 验证器不修改图，只返回 `Ok(())` 或错误列表。
/// `validate` 保留快速失败语义；`collect_errors` 返回当前所有错误。
pub struct GraphValidator;

impl GraphValidator {
    /// 执行全部验证，返回第一个错误（快速失败）。
    pub fn validate(graph: &ContainerGraph) -> Result<()> {
        Self::check_unique_ids(graph)?;
        for thread in &graph.threads {
            Self::check_label_names_unique(thread)?;
            for label in &thread.labels {
                Self::check_label(label)?;
            }
            for listener in &thread.listeners {
                Self::check_label(&listener.inner)?;
            }
        }
        Ok(())
    }

    /// 收集所有验证错误。
    pub fn collect_errors(graph: &ContainerGraph) -> Vec<FlowError> {
        let mut errors = Vec::new();
        if let Err(e) = Self::check_unique_ids(graph) {
            errors.push(e);
        }
        for thread in &graph.threads {
            if let Err(e) = Self::check_label_names_unique(thread) {
                errors.push(e);
            }
            for label in &thread.labels {
                if let Err(e) = Self::check_label(label) {
                    errors.push(e);
                }
            }
            for listener in &thread.listeners {
                if let Err(e) = Self::check_label(&listener.inner) {
                    errors.push(e);
                }
            }
        }
        errors
    }

    /// 检查所有容器内节点 ID 全局唯一
    fn check_unique_ids(graph: &ContainerGraph) -> Result<()> {
        let mut ids = HashSet::new();
        for thread in &graph.threads {
            for label in &thread.labels {
                for id in label.nodes.keys() {
                    if !ids.insert(id.clone()) {
                        return Err(FlowError::Validation(format!("Duplicate node id: {}", id)));
                    }
                }
            }
            for listener in &thread.listeners {
                for id in listener.inner.nodes.keys() {
                    if !ids.insert(id.clone()) {
                        return Err(FlowError::Validation(format!("Duplicate node id: {}", id)));
                    }
                }
            }
        }
        Ok(())
    }

    /// 检查同一线程内标签名唯一
    fn check_label_names_unique(thread: &ThreadContainer) -> Result<()> {
        let mut names = HashSet::new();
        for label in &thread.labels {
            if !names.insert(label.name.clone()) {
                return Err(FlowError::Validation(format!(
                    "Duplicate label name '{}' in thread '{}'",
                    label.name, thread.name
                )));
            }
        }
        for listener in &thread.listeners {
            if !names.insert(listener.name().to_string()) {
                return Err(FlowError::Validation(format!(
                    "Duplicate listener name '{}' in thread '{}'",
                    listener.name(),
                    thread.name
                )));
            }
        }
        Ok(())
    }

    /// 检查单个标签容器内部
    fn check_label(label: &LabelContainer) -> Result<()> {
        Self::check_edge_endpoints(label)?;
        Self::check_type_compatibility(label)?;
        Self::check_single_input_per_port(label)?;
        Self::check_no_cycles(label)?;
        Ok(())
    }

    /// 检查每条边的端点是否都指向容器内存在的节点
    fn check_edge_endpoints(label: &LabelContainer) -> Result<()> {
        for edge in label.edges.values() {
            if !label.nodes.contains_key(&edge.from.node_id) {
                return Err(FlowError::NodeNotFound(edge.from.node_id.clone()));
            }
            if !label.nodes.contains_key(&edge.to.node_id) {
                return Err(FlowError::NodeNotFound(edge.to.node_id.clone()));
            }
        }
        Ok(())
    }

    /// 检查边的端口类型是否兼容
    fn check_type_compatibility(label: &LabelContainer) -> Result<()> {
        for edge in label.edges.values() {
            let from_node = label
                .nodes
                .get(&edge.from.node_id)
                .ok_or_else(|| FlowError::NodeNotFound(edge.from.node_id.clone()))?;
            let to_node = label
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
    fn check_single_input_per_port(label: &LabelContainer) -> Result<()> {
        let mut port_inputs: HashSet<(String, String)> = HashSet::new();
        for edge in label.edges.values() {
            if edge.edge_type == PortType::Flow {
                continue;
            }
            let key = (edge.to.node_id.clone(), edge.to.port_id.clone());
            if !port_inputs.insert(key) {
                return Err(FlowError::ConnectionError(format!(
                    "Multiple input edges to data port {} on node {}",
                    edge.to.port_id, edge.to.node_id
                )));
            }
        }
        Ok(())
    }

    /// 检查 Flow 图是否有环
    fn check_no_cycles(label: &LabelContainer) -> Result<()> {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut nodes_with_flow_out: HashSet<String> = HashSet::new();
        for (id, node) in &label.nodes {
            if node.outputs.iter().any(|p| p.port_type == PortType::Flow) {
                nodes_with_flow_out.insert(id.clone());
            }
        }
        for edge in label.edges.values() {
            if edge.edge_type == PortType::Flow {
                *in_degree.entry(edge.to.node_id.clone()).or_insert(0) += 1;
            }
        }

        let mut queue: VecDeque<String> = VecDeque::new();
        for id in &nodes_with_flow_out {
            if in_degree.get(id).copied().unwrap_or(0) == 0 {
                queue.push_back(id.clone());
            }
        }

        let mut visited = 0usize;
        while let Some(id) = queue.pop_front() {
            visited += 1;
            for edge in label.edges.values() {
                if edge.edge_type == PortType::Flow && edge.from.node_id == id {
                    let next = &edge.to.node_id;
                    let count = in_degree.get_mut(next).ok_or_else(|| {
                        FlowError::Validation(format!(
                            "Flow edge target node {next} not found in in-degree map"
                        ))
                    })?;
                    *count -= 1;
                    if *count == 0 {
                        queue.push_back(next.clone());
                    }
                }
            }
        }

        let flow_nodes = nodes_with_flow_out.len();
        if visited < flow_nodes {
            return Err(FlowError::Validation(
                "Flow graph contains a cycle".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::container::ContainerGraph;
    use crate::graph::edge::{Edge, EdgeEndpoint};
    use crate::graph::node::{Node, Port, Vec2};
    use crate::graph::types::{NodeType, PortType};
    use std::collections::HashMap;

    fn make_node(id: &str, node_type: NodeType) -> Node {
        Node {
            id: id.to_string(),
            node_type,
            position: Vec2::default(),
            size: Vec2::new(180.0, 120.0),
            collapsed: false,
            params: HashMap::new(),
            inputs: vec![Port::new("in_flow", PortType::Flow, "执行")],
            outputs: vec![Port::new("out_flow", PortType::Flow, "下一步")],
            category: "Control".to_string(),
        }
    }

    fn connect_flow(label: &mut LabelContainer, from: &str, to: &str) {
        let edge = Edge::new(
            EdgeEndpoint::new(from, "out_flow"),
            EdgeEndpoint::new(to, "in_flow"),
            PortType::Flow,
        );
        label.edges.insert(edge.id.clone(), edge);
    }

    #[test]
    fn test_valid_graph_passes() -> Result<()> {
        let mut graph = ContainerGraph::default_main();
        graph.threads[0].labels[0]
            .nodes
            .insert("a".to_string(), make_node("a", NodeType::Log));
        GraphValidator::validate(&graph)?;
        Ok(())
    }

    #[test]
    fn test_cycle_detected() {
        let mut graph = ContainerGraph::default_main();
        graph.threads[0].labels[0]
            .nodes
            .insert("a".to_string(), make_node("a", NodeType::Log));
        graph.threads[0].labels[0]
            .nodes
            .insert("b".to_string(), make_node("b", NodeType::Log));
        connect_flow(&mut graph.threads[0].labels[0], "a", "b");
        connect_flow(&mut graph.threads[0].labels[0], "b", "a");
        assert!(GraphValidator::validate(&graph).is_err());
    }

    #[test]
    fn test_missing_edge_endpoint() {
        let mut graph = ContainerGraph::default_main();
        let edge = Edge::new(
            EdgeEndpoint::new("missing", "out_flow"),
            EdgeEndpoint::new("also_missing", "in_flow"),
            PortType::Flow,
        );
        graph.threads[0].labels[0].edges.insert(edge.id.clone(), edge);
        assert!(GraphValidator::validate(&graph).is_err());
    }

    #[test]
    fn test_multiple_input_edges_to_data_port() {
        let mut graph = ContainerGraph::default_main();
        let mut a = make_node("a", NodeType::NumberConstant);
        a.outputs = vec![Port::new("out_value", PortType::Number, "值")];
        graph.threads[0].labels[0].nodes.insert("a".to_string(), a);

        let mut b = make_node("b", NodeType::NumberConstant);
        b.outputs = vec![Port::new("out_value", PortType::Number, "值")];
        graph.threads[0].labels[0].nodes.insert("b".to_string(), b);

        let mut c = make_node("c", NodeType::Log);
        c.inputs.push(Port::new("input", PortType::Number, "输入"));
        graph.threads[0].labels[0].nodes.insert("c".to_string(), c);

        let e1 = Edge::new(
            EdgeEndpoint::new("a", "out_value"),
            EdgeEndpoint::new("c", "input"),
            PortType::Number,
        );
        let e2 = Edge::new(
            EdgeEndpoint::new("b", "out_value"),
            EdgeEndpoint::new("c", "input"),
            PortType::Number,
        );
        graph.threads[0].labels[0].edges.insert(e1.id.clone(), e1);
        graph.threads[0].labels[0].edges.insert(e2.id.clone(), e2);
        assert!(GraphValidator::validate(&graph).is_err());
    }
}
