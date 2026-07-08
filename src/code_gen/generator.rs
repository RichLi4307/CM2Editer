use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;

use super::formatter::CodeFormatter;
use crate::api::definitions::NodeDefinition;
use crate::api::registry;
use crate::error::{FlowError, Result};
use crate::graph::graph::Graph;
use crate::graph::node::{Node, ParamValue};
use crate::graph::types::{NodeType, PortType};
use crate::graph::validation::GraphValidator;

/// 生成 `.code` 文件代码
///
/// 先运行图验证器，再按标签遍历 Flow 边生成代码。
pub fn generate_code(graph: &Graph) -> Result<String> {
    GraphValidator::validate(graph)?;

    let registry = registry::registry();
    let mut formatter = CodeFormatter::new();
    let mut generator = CodeGenerator::new(graph, registry, &mut formatter);
    generator.run()?;
    Ok(formatter.into_content())
}

/// 生成 `.code` 文件并写入磁盘
///
/// 路径父目录必须存在。
pub fn generate_code_to_file(graph: &Graph, path: &Path) -> Result<()> {
    let code = generate_code(graph)?;
    std::fs::write(path, code)?;
    Ok(())
}

/// 代码生成器状态
pub struct CodeGenerator<'a> {
    graph: &'a Graph,
    registry: &'a HashMap<NodeType, NodeDefinition>,
    formatter: &'a mut CodeFormatter,
    visited: HashSet<String>,
}

impl<'a> CodeGenerator<'a> {
    /// 创建新的代码生成器
    pub fn new(
        graph: &'a Graph,
        registry: &'a HashMap<NodeType, NodeDefinition>,
        formatter: &'a mut CodeFormatter,
    ) -> Self {
        Self {
            graph,
            registry,
            formatter,
            visited: HashSet::new(),
        }
    }

    /// 执行代码生成
    pub fn run(&mut self) -> Result<()> {
        let labels = self.collect_labels();
        for (label_name, node_ids) in labels {
            if let Some(first_id) = node_ids.first() {
                self.formatter.write_line(&format!("{label_name}:"));
                self.formatter.indent();
                self.generate_sequence(first_id, None)?;
                self.formatter.dedent();
            }
        }
        Ok(())
    }

    /// 递归生成从某个节点开始的代码序列
    fn generate_sequence(&mut self, node_id: &str, stop_at: Option<&str>) -> Result<()> {
        if let Some(stop) = stop_at {
            if node_id == stop {
                return Ok(());
            }
        }
        if !self.visited.insert(node_id.to_string()) {
            return Ok(());
        }

        let node = self
            .graph
            .nodes
            .get(node_id)
            .ok_or_else(|| FlowError::NodeNotFound(node_id.to_string()))?;

        match node.node_type {
            NodeType::Start | NodeType::Label => {
                self.follow_flow(node_id, "out_flow", stop_at)?;
            }
            NodeType::Comment | NodeType::Meta | NodeType::Group => {
                self.follow_flow(node_id, "out_flow", stop_at)?;
            }
            NodeType::Goto => {
                let label = self.require_param(node, "label")?;
                self.formatter.write_line(&format!("Goto({label})"));
            }
            NodeType::If => self.generate_if(node_id, stop_at)?,
            NodeType::While => self.generate_while(node_id, stop_at)?,
            NodeType::For => self.generate_for(node_id, stop_at)?,
            NodeType::Break => {
                self.formatter.write_line("Break");
            }
            NodeType::Return => {
                let value = self
                    .resolve_param(node, "value")
                    .unwrap_or_else(|_| "null".to_string());
                self.formatter.write_line(&format!("_result = {value}"));
            }
            NodeType::CreateThread | NodeType::CreateListener | NodeType::CreateListenerLocal => {
                self.generate_node_call(node)?;
                self.follow_flow(node_id, "out_flow", stop_at)?;
            }
            _ => {
                self.generate_node_call(node)?;
                self.follow_flow(node_id, "out_flow", stop_at)?;
            }
        }

        Ok(())
    }

    /// 生成 `if` 条件分支
    fn generate_if(&mut self, node_id: &str, stop_at: Option<&str>) -> Result<()> {
        let node = self
            .graph
            .nodes
            .get(node_id)
            .ok_or_else(|| FlowError::NodeNotFound(node_id.to_string()))?;
        let condition = self.require_param(node, "condition")?;

        let true_target = self.flow_target(node_id, "out_true")?;
        let false_target = self.flow_target(node_id, "out_false")?;

        let join = match (&true_target, &false_target) {
            (Some(a), Some(b)) => find_join_node(self.graph, a, b),
            _ => None,
        };

        self.formatter.write_line(&format!("if {condition}"));
        self.formatter.indent();
        if let Some(ref target) = true_target {
            self.generate_sequence(target, join.as_deref())?;
        }
        self.formatter.dedent();
        if let Some(ref target) = false_target {
            self.formatter.write_line("else");
            self.formatter.indent();
            self.generate_sequence(target, join.as_deref())?;
            self.formatter.dedent();
        }

        if let Some(join_id) = join {
            self.generate_sequence(&join_id, stop_at)?;
        }
        Ok(())
    }

    /// 生成 `while` 循环
    fn generate_while(&mut self, node_id: &str, stop_at: Option<&str>) -> Result<()> {
        let node = self
            .graph
            .nodes
            .get(node_id)
            .ok_or_else(|| FlowError::NodeNotFound(node_id.to_string()))?;
        let condition = self.require_param(node, "condition")?;

        let body_target = self.flow_target(node_id, "out_flow")?;
        let break_target = self.flow_target(node_id, "out_break")?;

        self.formatter.write_line(&format!("while {condition}"));
        self.formatter.indent();
        if let Some(ref target) = body_target {
            self.generate_sequence(target, Some(node_id))?;
        }
        self.formatter.dedent();

        if let Some(ref target) = break_target {
            self.generate_sequence(target, stop_at)?;
        }
        Ok(())
    }

    /// 生成 `for` 循环
    fn generate_for(&mut self, node_id: &str, stop_at: Option<&str>) -> Result<()> {
        let node = self
            .graph
            .nodes
            .get(node_id)
            .ok_or_else(|| FlowError::NodeNotFound(node_id.to_string()))?;
        let iterable = self.require_param(node, "iterable")?;

        let body_target = self.flow_target(node_id, "out_flow")?;
        let break_target = self.flow_target(node_id, "out_break")?;

        self.formatter
            .write_line(&format!("for i in {iterable}"));
        self.formatter.indent();
        if let Some(ref target) = body_target {
            self.generate_sequence(target, Some(node_id))?;
        }
        self.formatter.dedent();

        if let Some(ref target) = break_target {
            self.generate_sequence(target, stop_at)?;
        }
        Ok(())
    }

    /// 生成普通函数调用节点
    fn generate_node_call(&mut self, node: &Node) -> Result<()> {
        let def = self
            .registry
            .get(&node.node_type)
            .ok_or_else(|| FlowError::UnknownNodeType(format!("{:?}", node.node_type)))?;

        let mut params = Vec::new();
        for param in &def.params {
            let value = match self.resolve_param_opt(node, &param.name) {
                Some(v) => v,
                None if param.required => {
                    // 缺失的必填参数使用默认值，避免导出失败
                    match param.default_value() {
                        ParamValue::Ref {
                            node: ref_node,
                            port,
                        } => {
                            format!("ref:{}/{}", ref_node, port)
                        }
                        ParamValue::Literal(v) => format_literal(&v),
                        ParamValue::Null => "null".to_string(),
                    }
                }
                None => continue,
            };
            params.push(format!("{}={}", param.name, value));
        }
        let param_str = params.join(", ");

        let node_name = format!("{:?}", node.node_type);
        let data_outputs: Vec<_> = node
            .outputs
            .iter()
            .filter(|p| p.port_type != PortType::Flow)
            .collect();

        if let Some(output) = data_outputs.first() {
            let var_name = format!("var_{}_{}", node.id, output.id);
            self.formatter
                .write_line(&format!("{var_name} = {node_name}({param_str})"));
        } else {
            self.formatter
                .write_line(&format!("{node_name}({param_str})"));
        }

        Ok(())
    }

    /// 沿指定 Flow 输出端口继续生成
    fn follow_flow(&mut self, node_id: &str, port_id: &str, stop_at: Option<&str>) -> Result<()> {
        if let Some(target) = self.flow_target(node_id, port_id)? {
            self.generate_sequence(&target, stop_at)?;
        }
        Ok(())
    }

    /// 查找指定 Flow 输出端口连接的目标节点
    fn flow_target(&self, node_id: &str, port_id: &str) -> Result<Option<String>> {
        for edge in self.graph.outgoing_edges(node_id) {
            if edge.edge_type == PortType::Flow && edge.from.port_id == port_id {
                return Ok(Some(edge.to.node_id.clone()));
            }
        }
        Ok(None)
    }

    /// 解析参数值，必填参数缺失时使用默认值。
    fn require_param(&self, node: &Node, name: &str) -> Result<String> {
        if let Some(value) = self.resolve_param_opt(node, name) {
            return Ok(value);
        }
        let def = self
            .registry
            .get(&node.node_type)
            .ok_or_else(|| FlowError::UnknownNodeType(format!("{:?}", node.node_type)))?;
        let param = def.params.iter().find(|p| p.name == name).ok_or_else(|| {
            FlowError::Validation(format!("Node {} has no parameter '{}'", node.id, name))
        })?;
        Ok(match param.default_value() {
            ParamValue::Ref {
                node: ref_node,
                port,
            } => format!("ref:{}/{}", ref_node, port),
            ParamValue::Literal(v) => format_literal(&v),
            ParamValue::Null => "null".to_string(),
        })
    }

    /// 解析参数值为 `.code` 字符串，缺失时返回 `None`（用于可选参数）
    fn resolve_param_opt(&self, node: &Node, name: &str) -> Option<String> {
        // DataFlow：优先使用参数对应 Data 输入端口的连接。
        if let Some((src_node, src_port)) = self.connected_param_source(node, name) {
            return self.evaluate_data_output(&src_node, &src_port);
        }
        match node.params.get(name) {
            Some(ParamValue::Ref {
                node: ref_node,
                port,
            }) => Some(format!("var_{ref_node}_{port}")),
            Some(ParamValue::Literal(value)) => Some(format_literal(value)),
            Some(ParamValue::Null) => Some("null".to_string()),
            None => None,
        }
    }

    /// 递归解析 Data 节点的输出端口值，生成 `.code` 表达式。
    fn evaluate_data_output(&self, node_id: &str, port_name: &str) -> Option<String> {
        let node = self.graph.nodes.get(node_id)?;
        match node.node_type {
            NodeType::Boolean => {
                let v = self.resolve_param_opt(node, "value")?;
                Some(format_literal_string(&v))
            }
            NodeType::GetStateBool => {
                let key = self.resolve_param_opt(node, "stateKey")?;
                Some(format!("_state.{key}"))
            }
            NodeType::GetStateNumber => {
                let key = self.resolve_param_opt(node, "stateKey")?;
                Some(format!("_state.{key}"))
            }
            NodeType::CompareNumbers => {
                let a = self.resolve_param_opt(node, "a")?;
                let b = self.resolve_param_opt(node, "b")?;
                let op = self.resolve_param_opt(node, "operator")
                    .unwrap_or_else(|| ">=".to_string());
                Some(format!("{a} {op} {b}"))
            }
            NodeType::LogicAnd => {
                let a = self.resolve_param_opt(node, "a")?;
                let b = self.resolve_param_opt(node, "b")?;
                Some(format!("({a}) && ({b})"))
            }
            NodeType::LogicOr => {
                let a = self.resolve_param_opt(node, "a")?;
                let b = self.resolve_param_opt(node, "b")?;
                Some(format!("({a}) || ({b})"))
            }
            NodeType::LogicNot => {
                let a = self.resolve_param_opt(node, "a")?;
                Some(format!("!({a})"))
            }
            // 在 Data 链中间的非专用 Data 节点：回退到变量引用
            _ => Some(format!("var_{node_id}_{port_name}")),
        }
    }

    /// 查找参数对应的 Data 输入端口是否连接了数据源。
    fn connected_param_source(&self, node: &Node, param_name: &str) -> Option<(String, String)> {
        self.graph
            .incoming_edges(&node.id)
            .iter()
            .find(|e| e.to.port_id == param_name && e.edge_type != PortType::Flow)
            .map(|e| (e.from.node_id.clone(), e.from.port_id.clone()))
    }

    /// 解析参数值为 `.code` 字符串，缺失时返回错误
    fn resolve_param(&self, node: &Node, name: &str) -> Result<String> {
        match node.params.get(name) {
            Some(ParamValue::Ref {
                node: ref_node,
                port,
            }) => Ok(format!("var_{ref_node}_{port}")),
            Some(ParamValue::Literal(value)) => Ok(format_literal(value)),
            Some(ParamValue::Null) => Ok("null".to_string()),
            None => Err(FlowError::Validation(format!(
                "Node {} missing required parameter '{}'",
                node.id, name
            ))),
        }
    }

    /// 收集标签。如果图没有显式标签，则自动为每个 Start 节点生成独立标签，
    /// 避免多 Start 流程被合并到单一 `main` 下。
    fn collect_labels(&self) -> Vec<(String, Vec<String>)> {
        let mut labels: Vec<(String, Vec<String>)> = self
            .graph
            .labels
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        if labels.is_empty() {
            let start_nodes: Vec<String> = self
                .graph
                .nodes
                .values()
                .filter(|n| n.node_type == NodeType::Start)
                .map(|n| n.id.clone())
                .collect();
            for (index, start_id) in start_nodes.into_iter().enumerate() {
                let label_name = if index == 0 {
                    "main".to_string()
                } else {
                    format!("main_{}", index)
                };
                labels.push((label_name, vec![start_id]));
            }
        }

        labels.sort_by(|a, b| {
            if a.0 == "main" {
                return std::cmp::Ordering::Less;
            }
            if b.0 == "main" {
                return std::cmp::Ordering::Greater;
            }
            a.0.cmp(&b.0)
        });
        labels
    }
}

/// 将 JSON 字面量格式化为 `.code` 可识别的字符串
fn format_literal(value: &serde_json::Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "null".to_string())
}

/// 格式化字符串字面量，去掉 JSON 双引号包裹——
/// 专门的 Data 节点（如 Boolean、GetStateBool）的 param 值是裸表达式，不能加引号。
fn format_literal_string(value: &str) -> String {
    let json = serde_json::to_string(value).unwrap_or_else(|_| "null".to_string());
    // 剥去周围的双引号
    if json.len() >= 2 && json.starts_with('"') && json.ends_with('"') {
        json[1..json.len() - 1].to_string()
    } else {
        json
    }
}

/// 查找两个分支汇合的第一个公共节点
fn find_join_node(graph: &Graph, a: &str, b: &str) -> Option<String> {
    let reachable_a = reachable_nodes(graph, a);
    let reachable_b = reachable_nodes(graph, b);
    let mut common: Vec<(String, usize)> = reachable_a
        .iter()
        .filter_map(|(id, depth_a)| {
            reachable_b
                .get(id)
                .map(|depth_b| (id.clone(), depth_a + depth_b))
        })
        .collect();
    common.sort_by_key(|x| x.1);
    common.first().map(|(id, _)| id.clone())
}

/// 计算从起点出发可达的所有节点及其深度
fn reachable_nodes(graph: &Graph, start: &str) -> HashMap<String, usize> {
    let mut result = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((start.to_string(), 0));
    while let Some((node_id, depth)) = queue.pop_front() {
        if result.contains_key(&node_id) {
            continue;
        }
        result.insert(node_id.clone(), depth);
        for edge in graph.outgoing_edges(&node_id) {
            if edge.edge_type == PortType::Flow && edge.from.node_id == node_id {
                queue.push_back((edge.to.node_id.clone(), depth + 1));
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::graph::edge::{Edge, EdgeEndpoint};
    use crate::graph::node::{Node, ParamValue, Port, Vec2};
    use crate::graph::types::{NodeType, PortType};
    use serde_json::json;

    fn make_node(id: &str, node_type: NodeType) -> Node {
        Node {
            id: id.to_string(),
            node_type,
            position: Vec2::ZERO,
            size: Vec2::new(180.0, 120.0),
            collapsed: false,
            params: HashMap::new(),
            inputs: vec![Port::new("in_flow", PortType::Flow, "执行")],
            outputs: vec![Port::new("out_flow", PortType::Flow, "下一步")],
            category: "Test".to_string(),
        }
    }

    fn add_flow_edge(
        graph: &mut Graph,
        from_node: &str,
        from_port: &str,
        to_node: &str,
        to_port: &str,
    ) {
        let edge = Edge::new(
            EdgeEndpoint::new(from_node, from_port),
            EdgeEndpoint::new(to_node, to_port),
            PortType::Flow,
        );
        let _ = graph.add_edge(edge);
    }

    fn build_graph() -> Graph {
        Graph::default()
    }

    #[test]
    fn test_generate_log_and_return() -> Result<()> {
        let mut graph = build_graph();
        let mut start = make_node("start", NodeType::Start);
        start.outputs = vec![Port::new("out_flow", PortType::Flow, "下一步")];
        let mut log = make_node("log", NodeType::Log);
        log.set_param("output", ParamValue::Literal(json!("hello")));
        let mut ret = make_node("ret", NodeType::Return);
        ret.set_param("value", ParamValue::Literal(json!(42)));

        graph.add_node(start);
        graph.add_node(log);
        graph.add_node(ret);
        graph.add_label(
            "main",
            vec!["start".to_string(), "log".to_string(), "ret".to_string()],
        );

        add_flow_edge(&mut graph, "start", "out_flow", "log", "in_flow");
        add_flow_edge(&mut graph, "log", "out_flow", "ret", "in_flow");

        let code = generate_code(&graph)?;
        assert!(code.contains("main:"));
        assert!(code.contains("Log(output=\"hello\")"));
        assert!(code.contains("_result = 42"));
        Ok(())
    }

    #[test]
    fn test_generate_if() -> Result<()> {
        let mut graph = build_graph();
        let start = make_node("start", NodeType::Start);
        let mut if_node = make_node("if", NodeType::If);
        if_node.set_param("condition", ParamValue::Literal(json!(true)));
        if_node.outputs = vec![
            Port::new("out_true", PortType::Flow, "True"),
            Port::new("out_false", PortType::Flow, "False"),
        ];
        let mut log_true = make_node("log_true", NodeType::Log);
        log_true.set_param("output", ParamValue::Literal(json!("yes")));
        let mut log_false = make_node("log_false", NodeType::Log);
        log_false.set_param("output", ParamValue::Literal(json!("no")));
        let mut end = make_node("end", NodeType::Log);
        end.set_param("output", ParamValue::Literal(json!("done")));

        graph.add_node(start);
        graph.add_node(if_node);
        graph.add_node(log_true);
        graph.add_node(log_false);
        graph.add_node(end);
        graph.add_label(
            "main",
            vec!["start".to_string(), "if".to_string(), "end".to_string()],
        );

        add_flow_edge(&mut graph, "start", "out_flow", "if", "in_flow");
        add_flow_edge(&mut graph, "if", "out_true", "log_true", "in_flow");
        add_flow_edge(&mut graph, "if", "out_false", "log_false", "in_flow");
        add_flow_edge(&mut graph, "log_true", "out_flow", "end", "in_flow");
        add_flow_edge(&mut graph, "log_false", "out_flow", "end", "in_flow");

        let code = generate_code(&graph)?;
        assert!(code.contains("if true"));
        assert!(code.contains("Log(output=\"yes\")"));
        assert!(code.contains("Log(output=\"no\")"));
        assert!(code.contains("Log(output=\"done\")"));
        Ok(())
    }

    #[test]
    fn test_generate_while() -> Result<()> {
        let mut graph = build_graph();
        let start = make_node("start", NodeType::Start);
        let mut while_node = make_node("while", NodeType::While);
        while_node.set_param("condition", ParamValue::Literal(json!(false)));
        while_node.outputs = vec![
            Port::new("out_flow", PortType::Flow, "Loop"),
            Port::new("out_break", PortType::Flow, "Break"),
        ];
        let mut body = make_node("body", NodeType::Log);
        body.set_param("output", ParamValue::Literal(json!("loop")));
        let mut end = make_node("end", NodeType::Log);
        end.set_param("output", ParamValue::Literal(json!("end")));

        graph.add_node(start);
        graph.add_node(while_node);
        graph.add_node(body);
        graph.add_node(end);
        graph.add_label(
            "main",
            vec!["start".to_string(), "while".to_string(), "end".to_string()],
        );

        add_flow_edge(&mut graph, "start", "out_flow", "while", "in_flow");
        add_flow_edge(&mut graph, "while", "out_flow", "body", "in_flow");
        add_flow_edge(&mut graph, "while", "out_break", "end", "in_flow");

        let code = generate_code(&graph)?;
        assert!(code.contains("while false"));
        assert!(code.contains("Log(output=\"loop\")"));
        assert!(code.contains("Log(output=\"end\")"));
        Ok(())
    }

    #[test]
    fn test_generate_goto() -> Result<()> {
        let mut graph = build_graph();
        let start = make_node("start", NodeType::Start);
        let mut goto = make_node("goto", NodeType::Goto);
        goto.set_param("label", ParamValue::Literal(json!("target")));
        let mut target = make_node("target", NodeType::Log);
        target.set_param("output", ParamValue::Literal(json!("reached")));

        graph.add_node(start);
        graph.add_node(goto);
        graph.add_node(target);
        graph.add_label("main", vec!["start".to_string(), "goto".to_string()]);
        graph.add_label("target", vec!["target".to_string()]);

        add_flow_edge(&mut graph, "start", "out_flow", "goto", "in_flow");

        let code = generate_code(&graph)?;
        assert!(code.contains("Goto(\"target\")"));
        assert!(code.contains("target:"));
        assert!(code.contains("Log(output=\"reached\")"));
        Ok(())
    }

    #[test]
    fn test_generate_param_ref() -> Result<()> {
        let mut graph = build_graph();
        let start = make_node("start", NodeType::Start);
        let mut random = make_node("random", NodeType::Random);
        random.set_param("min", ParamValue::Literal(json!(0)));
        random.set_param("max", ParamValue::Literal(json!(10)));
        random.outputs = vec![
            Port::new("out_flow", PortType::Flow, "下一步"),
            Port::new("out_value", PortType::Number, "值"),
        ];
        let mut log = make_node("log", NodeType::Log);
        log.set_param("output", ParamValue::from_ref("random", "out_value"));

        graph.add_node(start);
        graph.add_node(random);
        graph.add_node(log);
        graph.add_label(
            "main",
            vec!["start".to_string(), "random".to_string(), "log".to_string()],
        );

        add_flow_edge(&mut graph, "start", "out_flow", "random", "in_flow");
        add_flow_edge(&mut graph, "random", "out_flow", "log", "in_flow");

        let code = generate_code(&graph)?;
        assert!(code.contains("var_random_out_value = Random(min=0, max=10)"));
        assert!(code.contains("Log(output=var_random_out_value)"));
        Ok(())
    }

    #[test]
    fn test_generate_create_thread() -> Result<()> {
        let mut graph = build_graph();
        let start = make_node("start", NodeType::Start);
        let mut create_thread = make_node("ct", NodeType::CreateThread);
        create_thread.set_param("labelName", ParamValue::Literal(json!("m1")));
        let mut end = make_node("end", NodeType::Log);
        end.set_param("output", ParamValue::Literal(json!("done")));

        graph.add_node(start);
        graph.add_node(create_thread);
        graph.add_node(end);
        graph.add_label(
            "main",
            vec!["start".to_string(), "ct".to_string(), "end".to_string()],
        );

        add_flow_edge(&mut graph, "start", "out_flow", "ct", "in_flow");
        add_flow_edge(&mut graph, "ct", "out_flow", "end", "in_flow");

        let code = generate_code(&graph)?;
        assert!(code.contains("main:"));
        assert!(code.contains("CreateThread(labelName=\"m1\")"));
        assert!(code.contains("Log(output=\"done\")"));
        Ok(())
    }

    #[test]
    fn test_generate_create_listener() -> Result<()> {
        let mut graph = build_graph();
        let start = make_node("start", NodeType::Start);
        let mut listener = make_node("cl", NodeType::CreateListener);
        listener.set_param("labelName", ParamValue::Literal(json!("on_tick")));
        listener.set_param("params", ParamValue::Literal(json!({})));
        let mut end = make_node("end", NodeType::Return);
        end.set_param("value", ParamValue::Literal(json!(null)));

        graph.add_node(start);
        graph.add_node(listener);
        graph.add_node(end);
        graph.add_label(
            "main",
            vec!["start".to_string(), "cl".to_string(), "end".to_string()],
        );

        add_flow_edge(&mut graph, "start", "out_flow", "cl", "in_flow");
        add_flow_edge(&mut graph, "cl", "out_flow", "end", "in_flow");

        let code = generate_code(&graph)?;
        assert!(code.contains("main:"));
        assert!(code.contains("CreateListener(labelName=\"on_tick\", params={})"));
        assert!(code.contains("_result = null"));
        Ok(())
    }

    #[test]
    fn test_generate_two_starts_produce_separate_labels() -> Result<()> {
        let mut graph = build_graph();
        let start1 = make_node("start1", NodeType::Start);
        let mut log1 = make_node("log1", NodeType::Log);
        log1.set_param("output", ParamValue::Literal(json!("a")));
        let start2 = make_node("start2", NodeType::Start);
        let mut log2 = make_node("log2", NodeType::Log);
        log2.set_param("output", ParamValue::Literal(json!("b")));

        graph.add_node(start1);
        graph.add_node(log1);
        graph.add_node(start2);
        graph.add_node(log2);

        add_flow_edge(&mut graph, "start1", "out_flow", "log1", "in_flow");
        add_flow_edge(&mut graph, "start2", "out_flow", "log2", "in_flow");

        let code = generate_code(&graph)?;
        assert!(code.contains("main:"));
        assert!(code.contains("main_1:"));
        assert!(code.contains("Log(output=\"a\")"));
        assert!(code.contains("Log(output=\"b\")"));
        Ok(())
    }

    #[test]
    fn test_generate_param_via_data_port() -> Result<()> {
        let mut graph = build_graph();
        let start = make_node("start", NodeType::Start);
        let mut random = make_node("random", NodeType::Random);
        random.set_param("min", ParamValue::Literal(json!(0)));
        random.set_param("max", ParamValue::Literal(json!(10)));
        random.outputs = vec![
            Port::new("out_flow", PortType::Flow, "下一步"),
            Port::new("out_value", PortType::Number, "值"),
        ];
        let mut set_ecstasy = make_node("set", NodeType::SetEcstasy);
        set_ecstasy.set_param("value", ParamValue::Literal(json!(0.0)));
        set_ecstasy
            .inputs
            .push(Port::new("value", PortType::Number, "数值"));

        graph.add_node(start);
        graph.add_node(random);
        graph.add_node(set_ecstasy);
        graph.add_label(
            "main",
            vec!["start".to_string(), "random".to_string(), "set".to_string()],
        );

        add_flow_edge(&mut graph, "start", "out_flow", "random", "in_flow");
        add_flow_edge(&mut graph, "random", "out_flow", "set", "in_flow");

        let data_edge = Edge::new(
            EdgeEndpoint::new("random", "out_value"),
            EdgeEndpoint::new("set", "value"),
            PortType::Number,
        );
        graph.add_edge(data_edge)?;

        let code = generate_code(&graph)?;
        assert!(code.contains("var_random_out_value = Random(min=0, max=10)"));
        assert!(code.contains("SetEcstasy(value=var_random_out_value)"));
        Ok(())
    }
}
