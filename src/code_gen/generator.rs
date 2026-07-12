use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;

use super::formatter::CodeFormatter;
use crate::api::definitions::NodeDefinition;
use crate::api::registry;
use crate::error::{FlowError, Result};
use crate::graph::container::{ContainerGraph, LabelContainer, ThreadContainer};
use crate::graph::node::{Node, ParamValue};
use crate::graph::types::{NodeType, PortType};
use crate::graph::validation::GraphValidator;

/// 生成 `.code` 文件代码
///
/// 先运行图验证器，再按线程/标签遍历 Flow 边生成代码。
pub fn generate_code(graph: &ContainerGraph) -> Result<String> {
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
pub fn generate_code_to_file(graph: &ContainerGraph, path: &Path) -> Result<()> {
    let code = generate_code(graph)?;
    std::fs::write(path, code)?;
    Ok(())
}

/// 代码生成器状态
pub struct CodeGenerator<'a> {
    graph: &'a ContainerGraph,
    registry: &'a HashMap<NodeType, NodeDefinition>,
    formatter: &'a mut CodeFormatter,
    visited: HashSet<String>,
    /// True if the current label already wrote `_result = ...`
    result_written: bool,
}

impl<'a> CodeGenerator<'a> {
    /// 创建新的代码生成器
    pub fn new(
        graph: &'a ContainerGraph,
        registry: &'a HashMap<NodeType, NodeDefinition>,
        formatter: &'a mut CodeFormatter,
    ) -> Self {
        Self {
            graph,
            registry,
            formatter,
            visited: HashSet::new(),
            result_written: false,
        }
    }

    /// 执行代码生成
    pub fn run(&mut self) -> Result<()> {
        for thread in &self.graph.threads {
            self.generate_thread(thread)?;
        }
        Ok(())
    }

    /// 生成单个线程的代码
    fn generate_thread(&mut self, thread: &ThreadContainer) -> Result<()> {
        // 顶层：auto_start 线程创建
        if thread.auto_start {
            if let Some(first_label) = thread.labels.first() {
                self.formatter.write_line(&format!(
                    "{} = CreateThread(\"{}\")",
                    thread.variable_name, first_label.name
                ));
            }
        }

        // 监听器创建
        for listener in &thread.listeners {
            let func = match listener.kind {
                crate::graph::container::ListenerKind::Listener => "CreateListener",
                crate::graph::container::ListenerKind::LocalListener => "CreateListenerLocal",
            };
            self.formatter.write_line(&format!(
                "{} = {}(\"{}\")",
                listener.variable_name, func, listener.inner.name
            ));
        }

        // 标签体
        for label in &thread.labels {
            self.generate_label(label)?;
        }

        Ok(())
    }

    /// 生成单个标签体的代码
    fn generate_label(&mut self, label: &LabelContainer) -> Result<()> {
        self.result_written = false;
        self.formatter.write_line(&format!("{}:", label.name));
        self.formatter.indent();
        self.formatter.write_line("thread = _this");
        self.visited.clear();

        // 找到入口节点：没有入 Flow 边的节点
        let entry = self.find_entry_node(label);
        if let Some(entry_id) = entry {
            self.generate_sequence(label, &entry_id, None)?;
        }

        if !self.result_written {
            self.formatter.write_line("_result = null");
        }
        self.formatter.dedent();
        Ok(())
    }

    /// 找到标签的入口节点
    fn find_entry_node(&self, label: &LabelContainer) -> Option<String> {
        // 优先返回有 out_flow 但没有 in_flow 的节点
        for (id, node) in &label.nodes {
            if node.outputs.iter().any(|p| p.port_type == PortType::Flow)
                && !self.has_incoming_flow(label, id)
            {
                return Some(id.clone());
            }
        }
        // 兜底：返回第一个有 out_flow 的节点
        label
            .nodes
            .values()
            .find(|n| n.outputs.iter().any(|p| p.port_type == PortType::Flow))
            .map(|n| n.id.clone())
    }

    fn has_incoming_flow(&self, label: &LabelContainer, node_id: &str) -> bool {
        label.edges.values().any(|e| {
            e.edge_type == PortType::Flow && e.to.node_id == node_id && e.to.port_id == "in_flow"
        })
    }

    /// 递归生成从某个节点开始的代码序列
    fn generate_sequence(
        &mut self,
        label: &LabelContainer,
        node_id: &str,
        stop_at: Option<&str>,
    ) -> Result<()> {
        if let Some(stop) = stop_at {
            if node_id == stop {
                return Ok(());
            }
        }
        if !self.visited.insert(node_id.to_string()) {
            return Ok(());
        }

        let node = label
            .nodes
            .get(node_id)
            .ok_or_else(|| FlowError::NodeNotFound(node_id.to_string()))?;

        match node.node_type {
            NodeType::Comment | NodeType::Meta | NodeType::Group => {
                self.follow_flow(label, node_id, "out_flow", stop_at)?;
            }
            NodeType::Goto => {
                let label_param = self.require_param(label, node, "label")?;
                let mut line = format!("thread.Goto({label_param})");
                if let Some(args) = self.resolve_param_opt(label, node, "args") {
                    if args != "null" && args != "[]" && !args.is_empty() {
                        line.push_str(&format!(", {args}"));
                    }
                }
                self.formatter.write_line(&line);
                self.formatter
                    .write_line(&format!("var_{node_id}_out_label = {label_param}"));
            }
            NodeType::If => self.generate_if(label, node_id, stop_at)?,
            NodeType::While => self.generate_while(label, node_id, stop_at)?,
            NodeType::For => self.generate_for(label, node_id, stop_at)?,
            NodeType::Break => {
                self.formatter.write_line("break");
            }
            NodeType::Return => {
                let value = self
                    .resolve_param(label, node, "value")
                    .unwrap_or_else(|_| "null".to_string());
                self.formatter.write_line(&format!("_result = {value}"));
                self.result_written = true;
            }
            NodeType::CallFunction => {
                let func = self.require_param(label, node, "function")?;
                let func = func.trim_matches('"');
                let mut line = format!("{func}(");
                if let Some(args) = self.resolve_param_opt(label, node, "params") {
                    if args != "null" && args != "[]" && !args.is_empty() {
                        line.push_str(&args);
                    }
                }
                line.push(')');
                self.formatter.write_line(&line);
                self.follow_flow(label, node_id, "out_flow", stop_at)?;
            }
            NodeType::ForeachNode => {
                let list = self.require_param(label, node, "list")?;
                let thread = self.require_param(label, node, "threadVar")?;
                let list = list.trim_matches('"');
                let var = format!("var_{node_id}_idx");
                self.formatter
                    .write_line(&format!("{var} = Foreach({list}, {thread})"));
                self.follow_flow(label, node_id, "out_flow", stop_at)?;
            }
            NodeType::CreateThread | NodeType::CreateListener | NodeType::CreateListenerLocal => {
                self.generate_node_call(label, node)?;
                self.follow_flow(label, node_id, "out_flow", stop_at)?;
            }
            NodeType::DestroyListener => {
                self.formatter.write_line("listener = null");
                self.follow_flow(label, node_id, "out_flow", stop_at)?;
            }
            NodeType::WaitForThread => {
                let thread = self.require_param(label, node, "thread")?;
                self.formatter
                    .write_line(&format!("{thread}.WaitForFinish()"));
                self.follow_flow(label, node_id, "out_flow", stop_at)?;
            }
            _ => {
                self.generate_node_call(label, node)?;
                self.follow_flow(label, node_id, "out_flow", stop_at)?;
            }
        }

        Ok(())
    }

    /// 生成 `if` 条件分支
    fn generate_if(
        &mut self,
        label: &LabelContainer,
        node_id: &str,
        stop_at: Option<&str>,
    ) -> Result<()> {
        let node = label
            .nodes
            .get(node_id)
            .ok_or_else(|| FlowError::NodeNotFound(node_id.to_string()))?;
        let condition = self.require_param(label, node, "condition")?;

        let true_target = self.flow_target(label, node_id, "out_true")?;
        let false_target = self.flow_target(label, node_id, "out_false")?;

        let join = match (&true_target, &false_target) {
            (Some(a), Some(b)) => find_join_node(label, a, b),
            _ => None,
        };

        self.formatter.write_line(&format!("if {condition}"));
        self.formatter.indent();
        if let Some(ref target) = true_target {
            self.generate_sequence(label, target, join.as_deref())?;
        }
        self.formatter.dedent();
        if let Some(ref target) = false_target {
            self.formatter.write_line("else");
            self.formatter.indent();
            self.generate_sequence(label, target, join.as_deref())?;
            self.formatter.dedent();
        }

        if let Some(join_id) = join {
            self.generate_sequence(label, &join_id, stop_at)?;
        }
        Ok(())
    }

    /// 生成 `while` 循环
    fn generate_while(
        &mut self,
        label: &LabelContainer,
        node_id: &str,
        stop_at: Option<&str>,
    ) -> Result<()> {
        let node = label
            .nodes
            .get(node_id)
            .ok_or_else(|| FlowError::NodeNotFound(node_id.to_string()))?;
        let condition = self.require_param(label, node, "condition")?;

        let body_target = self.flow_target(label, node_id, "out_flow")?;
        let break_target = self.flow_target(label, node_id, "out_break")?;

        self.formatter.write_line(&format!("while {condition}"));
        self.formatter.indent();
        if let Some(ref target) = body_target {
            self.generate_sequence(label, target, Some(node_id))?;
        }
        self.formatter.dedent();

        if let Some(ref target) = break_target {
            self.generate_sequence(label, target, stop_at)?;
        }
        Ok(())
    }

    /// 生成 `for` 循环
    fn generate_for(
        &mut self,
        label: &LabelContainer,
        node_id: &str,
        stop_at: Option<&str>,
    ) -> Result<()> {
        let node = label
            .nodes
            .get(node_id)
            .ok_or_else(|| FlowError::NodeNotFound(node_id.to_string()))?;
        let iterable = self.require_param(label, node, "iterable")?;

        let body_target = self.flow_target(label, node_id, "out_flow")?;
        let break_target = self.flow_target(label, node_id, "out_break")?;

        self.formatter
            .write_line(&format!("for i in {iterable}"));
        self.formatter.indent();
        if let Some(ref target) = body_target {
            self.generate_sequence(label, target, Some(node_id))?;
        }
        self.formatter.dedent();

        if let Some(ref target) = break_target {
            self.generate_sequence(label, target, stop_at)?;
        }
        Ok(())
    }

    /// 生成普通函数调用节点
    fn generate_node_call(&mut self, label: &LabelContainer, node: &Node) -> Result<()> {
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
                let value = match self.resolve_param_opt(label, node, &param.name) {
                    Some(v) => v,
                    None if param.required => "\"\"".to_string(),
                    None => continue,
                };
                params.push(value);
                continue;
            }
            if is_thread_or_listener && param.name == "params" {
                if let Some(ParamValue::Literal(obj)) = node.params.get("params") {
                    if obj.is_object() {
                        if let Some(map) = obj.as_object() {
                            for (key, value) in map {
                                params.push(format!("{}={}", key, format_literal(value)));
                            }
                        }
                    } else if !obj.is_null() {
                        params.push(format_literal(obj));
                    }
                }
                continue;
            }
            let value = match self.resolve_param_opt(label, node, &param.name) {
                Some(v) => v,
                None if param.required => match param.default_value() {
                    ParamValue::Ref {
                        node: ref_node,
                        port,
                    } => format!("ref:{}/{}", ref_node, port),
                    ParamValue::Literal(v) => format_literal(&v),
                    ParamValue::Null => "null".to_string(),
                },
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
    fn follow_flow(
        &mut self,
        label: &LabelContainer,
        node_id: &str,
        port_id: &str,
        stop_at: Option<&str>,
    ) -> Result<()> {
        if let Some(target) = self.flow_target(label, node_id, port_id)? {
            self.generate_sequence(label, &target, stop_at)?;
        }
        Ok(())
    }

    /// 查找指定 Flow 输出端口连接的目标节点
    fn flow_target(
        &self,
        label: &LabelContainer,
        node_id: &str,
        port_id: &str,
    ) -> Result<Option<String>> {
        for edge in label.edges.values() {
            if edge.edge_type == PortType::Flow
                && edge.from.node_id == node_id
                && edge.from.port_id == port_id
            {
                return Ok(Some(edge.to.node_id.clone()));
            }
        }
        Ok(None)
    }

    /// 解析参数值，必填参数缺失时使用默认值。
    fn require_param(
        &self,
        label: &LabelContainer,
        node: &Node,
        name: &str,
    ) -> Result<String> {
        if let Some(value) = self.resolve_param_opt(label, node, name) {
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
    fn resolve_param_opt(
        &self,
        label: &LabelContainer,
        node: &Node,
        name: &str,
    ) -> Option<String> {
        if let Some((src_node, src_port)) = self.connected_param_source(label, node, name) {
            return self.evaluate_data_output(label, &src_node, &src_port);
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

    /// 解析参数值为 `.code` 字符串，缺失时返回错误
    fn resolve_param(
        &self,
        _label: &LabelContainer,
        node: &Node,
        name: &str,
    ) -> Result<String> {
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

    /// 递归解析 Data 节点的输出端口值，生成 `.code` 表达式。
    fn evaluate_data_output(
        &self,
        label: &LabelContainer,
        node_id: &str,
        port_name: &str,
    ) -> Option<String> {
        let node = label.nodes.get(node_id)?;
        match node.node_type {
            NodeType::Boolean => {
                let v = self.resolve_param_opt(label, node, "value")?;
                Some(v.trim_matches('"').to_string())
            }
            NodeType::NumberConstant => {
                let v = self.resolve_param_opt(label, node, "value")?;
                Some(v.trim_matches('"').to_string())
            }
            NodeType::StringConstant => self.resolve_param_opt(label, node, "value"),
            NodeType::CheckCondition => {
                let cond = self.resolve_param_opt(label, node, "cond")?;
                Some(format!("{cond}.Check()"))
            }
            NodeType::CheckEquipment => {
                let t = self.resolve_param_opt(label, node, "equipType")?;
                let t = t.trim_matches('"');
                Some(format!("_state.AdultToys.{t} != null"))
            }
            NodeType::CheckCosplay => {
                let k = self.resolve_param_opt(label, node, "cosplayKey")?;
                let k = k.trim_matches('"');
                Some(format!("Cosplay_{k}"))
            }
            NodeType::GetStateBool => {
                let key = self.resolve_param_opt(label, node, "stateKey")?;
                let key = key.trim_matches('"');
                Some(format!("_state.{key}"))
            }
            NodeType::GetStateNumber => {
                let key = self.resolve_param_opt(label, node, "stateKey")?;
                let key = key.trim_matches('"');
                Some(format!("_state.{key}"))
            }
            NodeType::CompareNumbers => {
                let a = self.resolve_param_opt(label, node, "a")?;
                let a = a.trim_matches('"');
                let b = self.resolve_param_opt(label, node, "b")?;
                let b = b.trim_matches('"');
                let op = self
                    .resolve_param_opt(label, node, "operator")
                    .unwrap_or_else(|| ">=".to_string());
                let op = op.trim_matches('"');
                Some(format!("{a} {op} {b}"))
            }
            NodeType::LogicAnd => {
                let a = self.resolve_param_opt(label, node, "a")?;
                let b = self.resolve_param_opt(label, node, "b")?;
                Some(format!("({a}) && ({b})"))
            }
            NodeType::LogicOr => {
                let a = self.resolve_param_opt(label, node, "a")?;
                let b = self.resolve_param_opt(label, node, "b")?;
                Some(format!("({a}) || ({b})"))
            }
            NodeType::LogicNot => {
                let a = self.resolve_param_opt(label, node, "a")?;
                Some(format!("!({a})"))
            }
            NodeType::GetPosition => {
                if port_name == "out_stage" {
                    let stage = self.resolve_param_opt(label, node, "stage")?;
                    Some(stage.trim_matches('"').to_string())
                } else {
                    let x = self.resolve_param_opt(label, node, "x")?;
                    let y = self.resolve_param_opt(label, node, "y")?;
                    let z = self.resolve_param_opt(label, node, "z")?;
                    Some(format!("[{x}, {y}, {z}]"))
                }
            }
            NodeType::MakeVector => {
                let x = self.resolve_param_opt(label, node, "x")?;
                let y = self.resolve_param_opt(label, node, "y")?;
                let z = self.resolve_param_opt(label, node, "z")?;
                Some(format!("[{x}, {y}, {z}]"))
            }
            NodeType::BreakVector => {
                let v = self.resolve_param_opt(label, node, "in_vec")?;
                match port_name {
                    "x" => Some(format!("{v}[0]")),
                    "y" => Some(format!("{v}[1]")),
                    "z" => Some(format!("{v}[2]")),
                    _ => Some(format!("{v}[0]")),
                }
            }
            NodeType::GetCurrentThread => Some("_this".to_string()),
            NodeType::GetSave => Some("_save".to_string()),
            NodeType::GetTime => Some("_time".to_string()),
            NodeType::GetTimeDiff => Some("_timediff".to_string()),
            NodeType::GetSettings => Some("_settings".to_string()),
            NodeType::GetMod => Some("_mod".to_string()),
            NodeType::GetMods => Some("_mods".to_string()),
            NodeType::Range => {
                let start = self.resolve_param_opt(label, node, "start")?;
                let stop = self.resolve_param_opt(label, node, "stop")?;
                let step = self.resolve_param_opt(label, node, "step");
                match step {
                    Some(s) if s != "null" && !s.is_empty() => {
                        Some(format!("Range({start}, {stop}, {s})"))
                    }
                    _ => Some(format!("Range({start}, {stop})")),
                }
            }
            NodeType::Goto if port_name == "out_label" => {
                let label = self.resolve_param_opt(label, node, "label")?;
                Some(label.trim_matches('"').to_string())
            }
            NodeType::CreateThread | NodeType::CreateListener | NodeType::CreateListenerLocal
                if port_name == "out_name" =>
            {
                let name = self.resolve_param_opt(label, node, "labelName")?;
                Some(name.trim_matches('"').to_string())
            }
            _ => Some(format!("var_{node_id}_{port_name}")),
        }
    }

    /// 查找参数对应的 Data 输入端口是否连接了数据源。
    fn connected_param_source(
        &self,
        label: &LabelContainer,
        node: &Node,
        param_name: &str,
    ) -> Option<(String, String)> {
        label
            .edges
            .values()
            .find(|e| {
                e.to.node_id == node.id && e.to.port_id == param_name && e.edge_type != PortType::Flow
            })
            .map(|e| (e.from.node_id.clone(), e.from.port_id.clone()))
    }
}

/// 格式化 JSON 字面量为 `.code` 字符串
fn format_literal(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => format!("\"{}\"", s),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Array(arr) => {
            let parts: Vec<String> = arr.iter().map(format_literal).collect();
            format!("[{}]", parts.join(", "))
        }
        serde_json::Value::Object(obj) => {
            let parts: Vec<String> = obj
                .iter()
                .map(|(k, v)| format!("{}={}", k, format_literal(v)))
                .collect();
            format!("{{{}}}", parts.join(", "))
        }
    }
}

/// 寻找两个分支的汇合节点，用于 if 生成
fn find_join_node(label: &LabelContainer, a: &str, b: &str) -> Option<String> {
    let mut queue_a: VecDeque<String> = VecDeque::new();
    let mut queue_b: VecDeque<String> = VecDeque::new();
    let mut visited_a: HashSet<String> = HashSet::new();
    let mut visited_b: HashSet<String> = HashSet::new();

    queue_a.push_back(a.to_string());
    queue_b.push_back(b.to_string());
    visited_a.insert(a.to_string());
    visited_b.insert(b.to_string());

    while !queue_a.is_empty() || !queue_b.is_empty() {
        if let Some(id) = queue_a.pop_front() {
            if visited_b.contains(&id) {
                return Some(id);
            }
            for edge in label.edges.values() {
                if edge.edge_type == PortType::Flow && edge.from.node_id == id {
                    let next = edge.to.node_id.clone();
                    if visited_a.insert(next.clone()) {
                        queue_a.push_back(next);
                    }
                }
            }
        }
        if let Some(id) = queue_b.pop_front() {
            if visited_a.contains(&id) {
                return Some(id);
            }
            for edge in label.edges.values() {
                if edge.edge_type == PortType::Flow && edge.from.node_id == id {
                    let next = edge.to.node_id.clone();
                    if visited_b.insert(next.clone()) {
                        queue_b.push_back(next);
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
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

    fn make_graph() -> ContainerGraph {
        ContainerGraph::default_main()
    }

    #[test]
    fn test_generate_empty_label() -> Result<()> {
        let graph = make_graph();
        let code = generate_code(&graph)?;
        assert!(code.contains("main:"));
        assert!(code.contains("thread = _this"));
        assert!(code.contains("_result = null"));
        Ok(())
    }

    #[test]
    fn test_generate_log() -> Result<()> {
        let mut graph = make_graph();
        let mut node = make_node("log1", NodeType::Log);
        node.set_param("output", ParamValue::Literal(serde_json::json!("hello")));
        graph.threads[0].labels[0].nodes.insert("log1".to_string(), node);
        // log1 没有入 Flow 边，会作为入口节点
        let code = generate_code(&graph)?;
        assert!(code.contains("Log(output=\"hello\")"));
        Ok(())
    }
}
