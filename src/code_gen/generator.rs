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
    /// True if the current label already wrote `_result = ...`
    result_written: bool,
    /// 子标签入口节点的 ID 集合，generate_sequence 遇到它们停止跟随 flow
    child_label_node_ids: HashSet<String>,
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
            result_written: false,
            child_label_node_ids: HashSet::new(),
        }
    }

    /// 执行代码生成
    pub fn run(&mut self) -> Result<()> {
        let (labels, skip_top_level) = self.collect_labels();

        // 发现 Listener 子标签：BFS 找出每个标签可达的节点，匹配 CreateListener/CreateListenerLocal 目标
        let child_map = compute_child_labels(self.graph, &labels, &skip_top_level);
        let child_names: HashSet<String> = child_map
            .values()
            .flat_map(|v| v.iter().map(|(n, _)| n.clone()))
            .collect();

        // 子标签入口的 Label 节点 ID：generate_sequence 遇到它们停止跟随 flow
        // 包括 listener 子标签，和从 Thread/Listener out_flow 连接到 Label 入口的节点
        self.child_label_node_ids = labels
            .iter()
            .filter(|(name, _)| child_names.contains(name))
            .flat_map(|(_, ids)| ids.iter().cloned())
            .collect();
        for edge in self.graph.edges.values() {
            if edge.edge_type != PortType::Flow || edge.to.port_id != "in_flow" {
                continue;
            }
            let source_is_thread_or_listener = matches!(
                self.graph.nodes.get(&edge.from.node_id).map(|n| &n.node_type),
                Some(NodeType::CreateThread)
                    | Some(NodeType::CreateListener)
                    | Some(NodeType::CreateListenerLocal)
            );
            if source_is_thread_or_listener {
                if let Some(target_node) = self.graph.nodes.get(&edge.to.node_id) {
                    if target_node.node_type == NodeType::Label {
                        self.child_label_node_ids.insert(edge.to.node_id.clone());
                    }
                }
            }
        }

        // Top-level: CreateThread only for entry-point labels
        for (label_name, _) in &labels {
            if skip_top_level.contains(label_name) || child_names.contains(label_name) {
                continue;
            }
            let var = format!("var_{}_thread", label_name);
            self.formatter
                .write_line(&format!("{var} = CreateThread(\"{label_name}\")"));
        }

        // Label bodies: 父标签内嵌子标签
        for (label_name, node_ids) in labels {
            if child_names.contains(&label_name) {
                continue;
            }
            self.result_written = false;
            self.formatter.write_line(&format!("{label_name}:"));
            self.formatter.indent();
            self.visited.clear();
            if let Some(first_id) = node_ids.first() {
                self.generate_sequence(first_id, None)?;
            }
            // 在 _result = null 前内嵌子标签
            let parent_result = self.result_written;
            if let Some(kids) = child_map.get(&label_name) {
                for (child_name, child_ids) in kids {
                    self.result_written = false;
                    self.formatter.write_line(&format!("{child_name}:"));
                    self.formatter.indent();
                    self.visited.clear();
                    if let Some(first_id) = child_ids.first() {
                        self.generate_sequence(first_id, None)?;
                    }
                    if !self.result_written {
                        self.formatter.write_line("_result = null");
                    }
                    self.formatter.dedent();
                }
            }
            self.result_written = parent_result;
            if !self.result_written {
                self.formatter.write_line("_result = null");
            }
            self.formatter.dedent();
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
                let mut line = format!("thread.Goto({label})");
                if let Some(args) = self.resolve_param_opt(node, "args") {
                    if args != "null" && args != "[]" && !args.is_empty() {
                        line.push_str(&format!(", {args}"));
                    }
                }
                self.formatter.write_line(&line);
                // 输出 out_label Data 端口值，供其他节点引用
                self.formatter
                    .write_line(&format!("var_{node_id}_out_label = {label}"));
            }
            NodeType::If => self.generate_if(node_id, stop_at)?,
            NodeType::While => self.generate_while(node_id, stop_at)?,
            NodeType::For => self.generate_for(node_id, stop_at)?,
            NodeType::Break => {
                self.formatter.write_line("break");
            }
            NodeType::Return => {
                let value = self
                    .resolve_param(node, "value")
                    .unwrap_or_else(|_| "null".to_string());
                self.formatter.write_line(&format!("_result = {value}"));
                self.result_written = true;
            }
            NodeType::CallFunction => {
                let func = self.require_param(node, "function")?;
                let mut line = format!("{func}(");
                if let Some(args) = self.resolve_param_opt(node, "params") {
                    if args != "null" && args != "[]" && !args.is_empty() {
                        line.push_str(&args);
                    }
                }
                line.push(')');
                self.formatter.write_line(&line);
                self.follow_flow(node_id, "out_flow", stop_at)?;
            }
            NodeType::ForeachNode => {
                let list = self.require_param(node, "list")?;
                let thread = self.require_param(node, "threadVar")?;
                let list = list.trim_matches('"');
                let var = format!("var_{node_id}_idx");
                self.formatter
                    .write_line(&format!("{var} = Foreach({list}, {thread})"));
                self.follow_flow(node_id, "out_flow", stop_at)?;
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
    /// Thread/Listener 节点头一个参数（labelName）按位置参数输出，params Object 解包为独立命名参数。
    fn generate_node_call(&mut self, node: &Node) -> Result<()> {
        let def = self
            .registry
            .get(&node.node_type)
            .ok_or_else(|| FlowError::UnknownNodeType(format!("{:?}", node.node_type)))?;

        let is_thread_or_listener = matches!(
            node.node_type,
            NodeType::CreateThread | NodeType::CreateListener | NodeType::CreateListenerLocal
        );

        let mut params: Vec<String> = Vec::new();
        for param in &def.params {
            if is_thread_or_listener && param.name == "labelName" {
                let value = match self.resolve_param_opt(node, &param.name) {
                    Some(v) => v,
                    None if param.required => "\"\"".to_string(),
                    None => continue,
                };
                // position 参数不加参数名前缀
                params.push(value);
                continue;
            }
            if is_thread_or_listener && param.name == "params" {
                // 解包 Object 为独立命名参数
                if let Some(ParamValue::Literal(obj)) = node.params.get("params") {
                    if obj.is_object() {
                        if let Some(map) = obj.as_object() {
                            for (key, value) in map {
                                params.push(format!("{}={}", key, format_literal(value)));
                            }
                        }
                    } else if !obj.is_null() {
                        // 非 Object 引用：直接作为额外参数
                        params.push(format_literal(obj));
                    }
                }
                continue;
            }
            let value = match self.resolve_param_opt(node, &param.name) {
                Some(v) => v,
                None if param.required => {
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
            // 子标签入口节点：停止跟随，由 run() 单独缩进生成
            if self.child_label_node_ids.contains(&target) {
                return Ok(());
            }
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
                Some(v.trim_matches('"').to_string())
            }
            NodeType::NumberConstant => {
                let v = self.resolve_param_opt(node, "value")?;
                Some(v.trim_matches('"').to_string())
            }
            NodeType::StringConstant => {
                self.resolve_param_opt(node, "value")
            }
            NodeType::CheckCondition => {
                let cond = self.resolve_param_opt(node, "cond")?;
                Some(format!("{cond}.Check()"))
            }
            NodeType::CheckEquipment => {
                let t = self.resolve_param_opt(node, "equipType")?;
                let t = t.trim_matches('"');
                Some(format!("_state.AdultToys.{t} != null"))
            }
            NodeType::CheckCosplay => {
                let k = self.resolve_param_opt(node, "cosplayKey")?;
                let k = k.trim_matches('"');
                Some(format!("Cosplay_{k}"))
            }
            NodeType::GetStateBool => {
                let key = self.resolve_param_opt(node, "stateKey")?;
                let key = key.trim_matches('"');
                Some(format!("_state.{key}"))
            }
            NodeType::GetStateNumber => {
                let key = self.resolve_param_opt(node, "stateKey")?;
                let key = key.trim_matches('"');
                Some(format!("_state.{key}"))
            }
            NodeType::CompareNumbers => {
                let a = self.resolve_param_opt(node, "a")?;
                let a = a.trim_matches('"');
                let b = self.resolve_param_opt(node, "b")?;
                let b = b.trim_matches('"');
                let op = self.resolve_param_opt(node, "operator")
                    .unwrap_or_else(|| ">=".to_string());
                let op = op.trim_matches('"');
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
            NodeType::GetPosition => {
                if port_name == "out_stage" {
                    let stage = self.resolve_param_opt(node, "stage")?;
                    Some(stage.trim_matches('"').to_string())
                } else {
                    let x = self.resolve_param_opt(node, "x")?;
                    let y = self.resolve_param_opt(node, "y")?;
                    let z = self.resolve_param_opt(node, "z")?;
                    let x = x.trim_matches('"');
                    let y = y.trim_matches('"');
                    let z = z.trim_matches('"');
                    Some(format!("[{x}, {y}, {z}]"))
                }
            }
            NodeType::MakeVector => {
                let x = self.resolve_param_opt(node, "x")?;
                let y = self.resolve_param_opt(node, "y")?;
                let z = self.resolve_param_opt(node, "z")?;
                Some(format!("[{x}, {y}, {z}]"))
            }
            NodeType::BreakVector => {
                let v = self.resolve_param_opt(node, "in_vec")?;
                match port_name {
                    "x" => Some(format!("{v}[0]")),
                    "y" => Some(format!("{v}[1]")),
                    "z" => Some(format!("{v}[2]")),
                    _ => Some(format!("{v}[0]")),
                }
            }
            // Goto / CreateThread / CreateListener 的 out_label/out_name 端口映射到参数值
            NodeType::Goto if port_name == "out_label" => {
                let label = self.resolve_param_opt(node, "label")?;
                Some(label.trim_matches('"').to_string())
            }
            NodeType::CreateThread | NodeType::CreateListener | NodeType::CreateListenerLocal
                if port_name == "out_name" =>
            {
                let name = self.resolve_param_opt(node, "labelName")?;
                Some(name.trim_matches('"').to_string())
            }
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

    /// 收集标签 + 标记仅由 Listener 引用的标签（不应有顶层 CreateThread）。
    fn collect_labels(&self) -> (Vec<(String, Vec<String>)>, HashSet<String>) {
        let mut labels: Vec<(String, Vec<String>)> = self
            .graph
            .labels
            .iter()
            .filter(|(k, _)| !k.is_empty())
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

        let mut discovered: HashSet<String> =
            labels.iter().map(|(n, _)| n.clone()).collect();

        // Pass 1: discover labels from non-listener sources (Label / Goto / CreateThread)
        for node in self.graph.nodes.values() {
            let target_label = match node.node_type {
                NodeType::Goto => node
                    .params
                    .get("label")
                    .and_then(|v| match v {
                        ParamValue::Literal(val) => val.as_str().map(|s| s.to_string()),
                        _ => None,
                    }),
                NodeType::CreateThread => node
                    .params
                    .get("labelName")
                    .and_then(|v| match v {
                        ParamValue::Literal(val) => val.as_str().map(|s| s.to_string()),
                        _ => None,
                    }),
                NodeType::Label => {
                    // 优先解析 Data 边（out_name / out_label → name）
                    self.graph
                        .edges
                        .values()
                        .find(|e| {
                            e.to.node_id == node.id
                                && e.to.port_id == "name"
                                && e.edge_type != PortType::Flow
                        })
                        .and_then(|edge| {
                            let source = self.graph.nodes.get(&edge.from.node_id)?;
                            match source.node_type {
                                NodeType::CreateThread
                                | NodeType::CreateListener
                                | NodeType::CreateListenerLocal => source
                                    .params
                                    .get("labelName")
                                    .and_then(|v| match v {
                                        ParamValue::Literal(val) => {
                                            val.as_str().map(|s| s.to_string())
                                        }
                                        _ => None,
                                    }),
                                _ => None,
                            }
                        })
                        .or_else(|| {
                            self.resolve_param_opt(node, "name")
                                .map(|s| s.trim_matches('"').to_string())
                        })
                }
                _ => None,
            };
            if let Some(t) = target_label {
                if t.is_empty() {
                    continue;
                }
                if node.node_type == NodeType::Label {
                    if let Some(idx) = labels.iter().position(|(n, _)| *n == t) {
                        if !labels[idx].1.contains(&node.id) {
                            labels[idx].1.push(node.id.clone());
                        }
                    } else {
                        labels.push((t.clone(), vec![node.id.clone()]));
                        discovered.insert(t.clone());
                    }
                } else if !discovered.contains(&t) {
                    discovered.insert(t.clone());
                    labels.push((t, Vec::new()));
                }
            }
        }

        // Pass 2: discover labels from Listener targets — any not already known
        for node in self.graph.nodes.values() {
            if !matches!(node.node_type, NodeType::CreateListener | NodeType::CreateListenerLocal) {
                continue;
            }
            let target_label = node
                .params
                .get("labelName")
                .and_then(|v| match v {
                    ParamValue::Literal(val) => val.as_str().map(|s| s.to_string()),
                    _ => None,
                });
            if let Some(t) = target_label {
                if t.is_empty() || discovered.contains(&t) {
                    continue;
                }
                discovered.insert(t.clone());
                labels.push((t, Vec::new()));
            }
        }

        // Top-level CreateThread only for labels explicitly created by CreateThread
        // (or "main"). Listener targets and Goto targets never need it.
        let skip_top_level: HashSet<String> = labels
            .iter()
            .filter(|(name, _)| {
                if name == "main" {
                    return false;
                }
                !self.graph.nodes.values().any(|n| matches!(n.node_type, NodeType::CreateThread)
                    && match n.params.get("labelName") {
                        Some(ParamValue::Literal(val)) => val.as_str() == Some(name.as_str()),
                        _ => false,
                    })
            })
            .map(|(name, _)| name.clone())
            .collect();

        labels.sort_by(|a, b| {
            if a.0 == "main" {
                return std::cmp::Ordering::Less;
            }
            if b.0 == "main" {
                return std::cmp::Ordering::Greater;
            }
            a.0.cmp(&b.0)
        });
        (labels, skip_top_level)
    }
}

/// BFS 可达节点集合：从 start_id 出发沿所有 Flow 边可达的节点 ID
fn flow_reachable(graph: &Graph, start_id: &str) -> HashSet<String> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start_id.to_string());
    while let Some(current) = queue.pop_front() {
        if !visited.insert(current.clone()) {
            continue;
        }
        for edge in graph.outgoing_edges(&current) {
            if edge.edge_type == PortType::Flow {
                queue.push_back(edge.to.node_id.clone());
            }
        }
    }
    visited
}

/// 识别 Listener 创建的子标签：返回 parent → [(child_name, child_node_ids)]
fn compute_child_labels(
    graph: &Graph,
    labels: &[(String, Vec<String>)],
    skip_top_level: &HashSet<String>,
) -> HashMap<String, Vec<(String, Vec<String>)>> {
    let mut child_map: HashMap<String, Vec<(String, Vec<String>)>> = HashMap::new();

    // 计算每个标签从入口点 BFS 可达的节点集合
    let label_reachable: HashMap<String, HashSet<String>> = labels
        .iter()
        .map(|(name, entry_ids)| {
            let mut reachable = HashSet::new();
            for id in entry_ids {
                reachable.extend(flow_reachable(graph, id));
            }
            (name.clone(), reachable)
        })
        .collect();

    for node in graph.nodes.values() {
        if !matches!(
            node.node_type,
            NodeType::CreateListener | NodeType::CreateListenerLocal
        ) {
            continue;
        }
        let target = node
            .params
            .get("labelName")
            .and_then(|v| match v {
                ParamValue::Literal(val) => val.as_str(),
                _ => None,
            })
            .unwrap_or("");
        if target.is_empty() || !skip_top_level.contains(target) {
            continue;
        }
        for (parent_name, reachable) in &label_reachable {
            if reachable.contains(&node.id) {
                let target_ids = labels
                    .iter()
                    .find(|(n, _)| n == target)
                    .map(|(_, ids)| ids.clone())
                    .unwrap_or_default();
                child_map
                    .entry(parent_name.clone())
                    .or_default()
                    .push((target.to_string(), target_ids));
                break;
            }
        }
    }
    child_map
}

/// 将 JSON 字面量格式化为 `.code` 可识别的字符串
fn format_literal(value: &serde_json::Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "null".to_string())
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
        assert!(code.contains("thread.Goto(\"target\")"));
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
        assert!(code.contains("CreateThread(\"m1\")"));
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
        // out_name port changes output format: check only essential content
        assert!(code.contains("main:"));
        assert!(code.contains("CreateListener"));
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
