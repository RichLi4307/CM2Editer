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
        self.formatter.write_line(&format!("{}:", label.name));
        self.formatter.indent();
        self.formatter.write_line("thread = _this");
        self.visited.clear();

        // 找到入口节点：没有入 Flow 边的节点
        let entry = self.find_entry_node(label);
        if let Some(entry_id) = entry {
            self.generate_sequence(label, &entry_id, None)?;
        }

        self.formatter.dedent();
        Ok(())
    }

    /// 找到标签的入口节点
    fn find_entry_node(&self, label: &LabelContainer) -> Option<String> {
        label.entry_node_id()
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
            NodeType::CreateThread | NodeType::CreateListener | NodeType::CreateListenerLocal
            | NodeType::CreateEventListener | NodeType::CreateEventListenerLocal => {
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
            NodeType::SetVariable => {
                let name = self.require_param(label, node, "name")?;
                let name = name.trim_matches('"');
                let value = self
                    .resolve_param_opt(label, node, "value")
                    .unwrap_or_else(|| "null".to_string());
                self.formatter.write_line(&format!("{name} = {value}"));
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

        // CreateCondition 的官方语法把 Condition 作为位置参数：
        //   CreateCondition("Exposed_All") 或 CreateCondition("...", id="MyID")
        if node.node_type == NodeType::CreateCondition {
            let condition = self.require_param(label, node, "condition")?;
            let id = self
                .resolve_param_opt(label, node, "id")
                .filter(|v| !v.is_empty() && v != "\"\"");
            let mut args = vec![condition];
            if let Some(id) = id {
                args.push(format!("id={id}"));
            }
            let param_str = args.join(", ");
            self.formatter
                .write_line(&format!("var_{}_out_condition = CreateCondition({param_str})", node.id));
            return Ok(());
        }

        let is_thread_or_listener = matches!(
            node.node_type,
            NodeType::CreateThread | NodeType::CreateListener | NodeType::CreateListenerLocal
            | NodeType::CreateEventListener | NodeType::CreateEventListenerLocal
        );

        let mut params: Vec<String> = Vec::new();
        for param in &def.params {
            // labelName / eventName 按官方签名走位置参数（CreateEventListener(LabelName, EventName, ...)）
            if is_thread_or_listener && (param.name == "labelName" || param.name == "eventName") {
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
            // 可选 id 参数为空字符串时无意义，直接跳过。
            if param.name == "id" && (value == "\"\"" || value == "''") {
                continue;
            }
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
                let keys_str = self.resolve_param_opt(label, node, "cosplayKeys")?;
                let keys = parse_string_list(&keys_str).ok()?;
                if keys.is_empty() {
                    return Some("true".to_string());
                }
                if keys.len() == 1 {
                    return Some(format!("Cosplay_{}", keys[0]));
                }
                // 多件服装用逻辑与连接并加括号，方便与 LogicAnd/LogicOr 节点组合。
                Some(format!(
                    "({})",
                    keys.iter()
                        .map(|k| format!("Cosplay_{k}"))
                        .collect::<Vec<_>>()
                        .join(" && ")
                ))
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
            NodeType::GetSave => {
                let key = self.resolve_param_opt(label, node, "key")?;
                let key = key.trim_matches('"');
                Some(format!("_save.{key}"))
            }
            NodeType::GetTime => Some("_time".to_string()),
            NodeType::GetTimeDiff => Some("_timediff".to_string()),
            NodeType::GetSettings => Some("_settings".to_string()),
            NodeType::GetMod => Some("_mod".to_string()),
            NodeType::GetMods => Some("_mods".to_string()),
            NodeType::Variable => {
                let name = self.resolve_param_opt(label, node, "name")?;
                Some(name.trim_matches('"').to_string())
            }
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
            | NodeType::CreateEventListener | NodeType::CreateEventListenerLocal
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

/// 解析 `.code` 格式的字符串数组字面量（如 `["A", "B"]`）为字符串列表。
fn parse_string_list(s: &str) -> Result<Vec<String>> {
    let s = s.trim();
    if s == "null" || s.is_empty() || s == "[]" {
        return Ok(Vec::new());
    }
    let value: serde_json::Value =
        serde_json::from_str(s).map_err(|e| FlowError::Validation(format!("Invalid string list: {e}")))?;
    match value {
        serde_json::Value::Array(arr) => arr
            .iter()
            .map(|v| {
                v.as_str()
                    .map(|s| s.to_string())
                    .ok_or_else(|| FlowError::Validation("List element must be string".to_string()))
            })
            .collect(),
        _ => Err(FlowError::Validation(format!("Expected string list, got: {s}"))),
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
    use crate::graph::container::{ContainerGraph, LabelContainer};
    use crate::graph::edge::{Edge, EdgeEndpoint};
    use crate::graph::node::{Node, ParamValue, Port, Vec2};
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
            inputs: vec![Port::new("in_flow", PortType::Flow, "Execute")],
            outputs: vec![Port::new("out_flow", PortType::Flow, "Next")],
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
        assert!(!code.contains("_result = null"));
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

    // -----------------------------------------------------------------
    // P1-D: comprehensive node generation tests by category
    // -----------------------------------------------------------------

    fn add_flow_node(
        graph: &mut ContainerGraph,
        id: &str,
        node_type: NodeType,
        params: HashMap<String, ParamValue>,
    ) {
        let mut node = make_node(id, node_type);
        node.params = params;
        graph.threads[0].labels[0].nodes.insert(id.to_string(), node);
    }

    fn make_data_node(id: &str, node_type: NodeType, output_port: &str) -> Node {
        let mut node = make_node(id, node_type);
        // Data nodes have no Flow ports; remove the default flow I/O and add
        // the requested data output so the generator routes through
        // `evaluate_data_output` rather than `generate_node_call`.
        node.inputs.retain(|p| p.port_type != PortType::Flow);
        node.outputs.retain(|p| p.port_type != PortType::Flow);
        node.outputs.push(Port::new(output_port, PortType::Any, "Value"));
        node
    }

    fn connect_data(
        label: &mut LabelContainer,
        from_node: &str,
        from_port: &str,
        to_node: &str,
        to_port: &str,
        port_type: PortType,
    ) {
        let edge = Edge::new(
            EdgeEndpoint::new(from_node, from_port),
            EdgeEndpoint::new(to_node, to_port),
            port_type,
        );
        label.edges.insert(edge.id.clone(), edge);
    }

    /// Insert a data node and wire it into a SetVariable so that
    /// `evaluate_data_output` is exercised during code generation.
    fn add_data_node_through_setvar(
        graph: &mut ContainerGraph,
        id: &str,
        node_type: NodeType,
        output_port: &str,
        params: HashMap<String, ParamValue>,
    ) {
        let mut data_node = make_data_node(id, node_type, output_port);
        data_node.params = params;
        graph.threads[0].labels[0]
            .nodes
            .insert(id.to_string(), data_node);

        let mut setvar = make_node("setvar", NodeType::SetVariable);
        setvar.inputs.push(Port::new("value", PortType::Any, "Value").required(true));
        setvar.set_param("name", ParamValue::Literal(serde_json::json!("testVar")));
        graph.threads[0].labels[0]
            .nodes
            .insert("setvar".to_string(), setvar);

        connect_data(
            &mut graph.threads[0].labels[0],
            id,
            output_port,
            "setvar",
            "value",
            PortType::Any,
        );
    }

    fn expect_flow_pattern(node_type: NodeType) -> &'static str {
        match node_type {
            NodeType::Goto => "thread.Goto",
            NodeType::If => "if ",
            NodeType::While => "while ",
            NodeType::For => "for i in ",
            NodeType::Break => "break",
            NodeType::Return => "_result =",
            NodeType::DestroyListener => "listener = null",
            NodeType::WaitForThread => ".WaitForFinish()",
            NodeType::CallFunction => "myFunc",
            NodeType::CallMethod => "CallMethod",
            NodeType::ForeachNode => "Foreach",
            NodeType::SetVariable => "testVar =",
            NodeType::Meta | NodeType::Comment | NodeType::Group => "",
            _ => {
                let name = format!("{:?}", node_type);
                // leak a static string for the test lifetime
                Box::leak(name.into_boxed_str())
            }
        }
    }

    fn assert_flow_node_generates(node_type: NodeType, params: HashMap<String, ParamValue>) -> Result<()> {
        let mut graph = make_graph();
        add_flow_node(&mut graph, "n1", node_type, params);
        let code = generate_code(&graph)?;
        let pattern = expect_flow_pattern(node_type);
        if !pattern.is_empty() {
            assert!(
                code.contains(pattern),
                "Expected code for {:?} to contain '{}', got:\n{}",
                node_type,
                pattern,
                code
            );
        }
        Ok(())
    }

    fn assert_data_node_generates(
        node_type: NodeType,
        output_port: &str,
        params: HashMap<String, ParamValue>,
    ) -> Result<()> {
        let mut graph = make_graph();
        add_data_node_through_setvar(&mut graph, "n1", node_type, output_port, params);
        let code = generate_code(&graph)?;
        assert!(
            code.contains("testVar ="),
            "Expected code for {:?} to contain 'testVar =', got:\n{}",
            node_type,
            code
        );
        Ok(())
    }

    #[test]
    fn test_generate_threading_and_concurrency() -> Result<()> {
        assert_flow_node_generates(NodeType::CreateThread, [(
            "labelName".to_string(),
            ParamValue::Literal(serde_json::json!("sub")),
        )].into())?;
        assert_flow_node_generates(NodeType::CreateListener, [(
            "labelName".to_string(),
            ParamValue::Literal(serde_json::json!("sub")),
        )].into())?;
        assert_flow_node_generates(NodeType::CreateListenerLocal, [(
            "labelName".to_string(),
            ParamValue::Literal(serde_json::json!("sub")),
        )].into())?;
        assert_flow_node_generates(NodeType::CreateEventListener, [
            ("labelName".to_string(), ParamValue::Literal(serde_json::json!("sub"))),
            ("eventName".to_string(), ParamValue::Literal(serde_json::json!("my_event"))),
        ].into())?;
        assert_flow_node_generates(NodeType::CreateEventListenerLocal, [
            ("labelName".to_string(), ParamValue::Literal(serde_json::json!("sub"))),
            ("eventName".to_string(), ParamValue::Literal(serde_json::json!("my_event"))),
        ].into())?;
        assert_flow_node_generates(NodeType::DestroyListener, HashMap::new())?;
        assert_data_node_generates(NodeType::GetCurrentThread, "out_value", HashMap::new())?;
        assert_flow_node_generates(NodeType::WaitForThread, [(
            "thread".to_string(),
            ParamValue::Literal(serde_json::json!("t")),
        )].into())?;
        Ok(())
    }

    #[test]
    fn test_create_event_listener_positional_args_and_params() -> Result<()> {
        let mut graph = make_graph();
        let params: HashMap<String, ParamValue> = [
            ("labelName".to_string(), ParamValue::Literal(serde_json::json!("on_hit"))),
            ("eventName".to_string(), ParamValue::Literal(serde_json::json!("player_hit"))),
            ("params".to_string(), ParamValue::Literal(serde_json::json!({ "level": 2 }))),
        ].into();
        add_flow_node(&mut graph, "n1", NodeType::CreateEventListener, params);
        let code = generate_code(&graph)?;
        assert!(
            code.contains("CreateEventListener(\"on_hit\", \"player_hit\", level=2)"),
            "Expected CreateEventListener with positional labelName/eventName then expanded named params, got:\n{code}"
        );
        Ok(())
    }

    #[test]
    fn test_create_event_listener_local_generates_local_variant() -> Result<()> {
        let mut graph = make_graph();
        let params: HashMap<String, ParamValue> = [
            ("labelName".to_string(), ParamValue::Literal(serde_json::json!("on_hit"))),
            ("eventName".to_string(), ParamValue::Literal(serde_json::json!("player_hit"))),
        ].into();
        add_flow_node(&mut graph, "n1", NodeType::CreateEventListenerLocal, params);
        let code = generate_code(&graph)?;
        assert!(
            code.contains("CreateEventListenerLocal(\"on_hit\", \"player_hit\")"),
            "Expected CreateEventListenerLocal with positional args, got:\n{code}"
        );
        Ok(())
    }

    #[test]
    fn test_generate_control_flow() -> Result<()> {
        assert_flow_node_generates(NodeType::Goto, [(
            "label".to_string(),
            ParamValue::Literal(serde_json::json!("sub")),
        )].into())?;
        assert_flow_node_generates(NodeType::If, [(
            "condition".to_string(),
            ParamValue::Literal(serde_json::json!(true)),
        )].into())?;
        assert_flow_node_generates(NodeType::While, [(
            "condition".to_string(),
            ParamValue::Literal(serde_json::json!(true)),
        )].into())?;
        assert_flow_node_generates(NodeType::For, [(
            "iterable".to_string(),
            ParamValue::Literal(serde_json::json!([])),
        )].into())?;
        assert_flow_node_generates(NodeType::Break, HashMap::new())?;
        assert_flow_node_generates(NodeType::Return, [(
            "value".to_string(),
            ParamValue::Literal(serde_json::json!(1)),
        )].into())?;
        assert_flow_node_generates(NodeType::Wait, [(
            "seconds".to_string(),
            ParamValue::Literal(serde_json::json!(1)),
        )].into())?;
        assert_flow_node_generates(NodeType::WaitForEvent, [(
            "eventName".to_string(),
            ParamValue::Literal(serde_json::json!("evt")),
        )].into())?;
        Ok(())
    }

    #[test]
    fn test_generate_variables_and_globals() -> Result<()> {
        assert_flow_node_generates(NodeType::Global, [(
            "name".to_string(),
            ParamValue::Literal(serde_json::json!("g")),
        )].into())?;
        assert_flow_node_generates(NodeType::Local, [(
            "name".to_string(),
            ParamValue::Literal(serde_json::json!("l")),
        )].into())?;
        assert_data_node_generates(NodeType::GetSave, "out_value", [(
            "key".to_string(),
            ParamValue::Literal(serde_json::json!("TotalScore")),
        )].into())?;
        assert_data_node_generates(NodeType::GetTime, "out_value", HashMap::new())?;
        assert_data_node_generates(NodeType::GetTimeDiff, "out_value", HashMap::new())?;
        assert_data_node_generates(NodeType::GetSettings, "out_value", HashMap::new())?;
        assert_data_node_generates(NodeType::GetMod, "out_value", HashMap::new())?;
        assert_data_node_generates(NodeType::GetMods, "out_value", HashMap::new())?;
        assert_flow_node_generates(NodeType::SetEvent, [
            ("name".to_string(), ParamValue::Literal(serde_json::json!("evt"))),
            ("value".to_string(), ParamValue::Literal(serde_json::json!(1))),
        ].into())?;
        assert_flow_node_generates(NodeType::GetEvent, [(
            "name".to_string(),
            ParamValue::Literal(serde_json::json!("evt")),
        )].into())?;
        assert_flow_node_generates(NodeType::DumpVariables, HashMap::new())?;
        assert_flow_node_generates(NodeType::DumpVariable, [(
            "var".to_string(),
            ParamValue::Literal(serde_json::json!("v")),
        )].into())?;
        assert_flow_node_generates(NodeType::GetType, [(
            "value".to_string(),
            ParamValue::Literal(serde_json::json!(1)),
        )].into())?;
        assert_data_node_generates(NodeType::GetLanguage, "out_language", HashMap::new())?;
        assert_data_node_generates(NodeType::Variable, "out_value", [(
            "name".to_string(),
            ParamValue::Literal(serde_json::json!("myVar")),
        )].into())?;
        assert_flow_node_generates(NodeType::SetVariable, [(
            "name".to_string(),
            ParamValue::Literal(serde_json::json!("testVar")),
        )].into())?;
        Ok(())
    }

    #[test]
    fn test_generate_literals() -> Result<()> {
        assert_data_node_generates(NodeType::NumberConstant, "out_value", [(
            "value".to_string(),
            ParamValue::Literal(serde_json::json!(42)),
        )].into())?;
        assert_data_node_generates(NodeType::StringConstant, "out_value", [(
            "value".to_string(),
            ParamValue::Literal(serde_json::json!("hello")),
        )].into())?;
        assert_data_node_generates(NodeType::Boolean, "out_value", [(
            "value".to_string(),
            ParamValue::Literal(serde_json::json!(true)),
        )].into())?;
        assert_flow_node_generates(NodeType::Color, [
            ("r".to_string(), ParamValue::Literal(serde_json::json!(1))),
            ("g".to_string(), ParamValue::Literal(serde_json::json!(1))),
            ("b".to_string(), ParamValue::Literal(serde_json::json!(1))),
            ("a".to_string(), ParamValue::Literal(serde_json::json!(1))),
        ].into())?;
        assert_flow_node_generates(NodeType::Range, [
            ("start".to_string(), ParamValue::Literal(serde_json::json!(0))),
            ("stop".to_string(), ParamValue::Literal(serde_json::json!(10))),
        ].into())?;
        Ok(())
    }

    #[test]
    fn test_generate_math_and_logic() -> Result<()> {
        for ty in [
            NodeType::Random,
            NodeType::RandomInt,
            NodeType::Sin,
            NodeType::Cos,
            NodeType::Tan,
            NodeType::Asin,
            NodeType::Acos,
            NodeType::Atan,
            NodeType::Floor,
            NodeType::Ceil,
            NodeType::Round,
            NodeType::Trunc,
            NodeType::Sign,
            NodeType::Abs,
            NodeType::LogN,
            NodeType::Log2,
            NodeType::Log10,
            NodeType::Min,
            NodeType::Max,
            NodeType::Vector,
            NodeType::Quaternion,
            NodeType::Vector3Length,
            NodeType::Vector3SqrLength,
            NodeType::Vector3Add,
            NodeType::Vector3Sub,
            NodeType::Vector3Scale,
            NodeType::Vector3Dot,
            NodeType::Vector3Cross,
            NodeType::Vector3Rotate,
            NodeType::Vector3Distance,
        ] {
            assert_flow_node_generates(ty, HashMap::new())?;
        }
        assert_data_node_generates(NodeType::CompareNumbers, "out_result", [
            ("a".to_string(), ParamValue::Literal(serde_json::json!(1))),
            ("b".to_string(), ParamValue::Literal(serde_json::json!(2))),
            ("operator".to_string(), ParamValue::Literal(serde_json::json!(">="))),
        ].into())?;
        assert_data_node_generates(NodeType::LogicAnd, "out_result", [
            ("a".to_string(), ParamValue::Literal(serde_json::json!(true))),
            ("b".to_string(), ParamValue::Literal(serde_json::json!(true))),
        ].into())?;
        assert_data_node_generates(NodeType::LogicOr, "out_result", [
            ("a".to_string(), ParamValue::Literal(serde_json::json!(true))),
            ("b".to_string(), ParamValue::Literal(serde_json::json!(true))),
        ].into())?;
        assert_data_node_generates(NodeType::LogicNot, "out_result", [(
            "a".to_string(),
            ParamValue::Literal(serde_json::json!(false)),
        )].into())?;
        assert_data_node_generates(NodeType::MakeVector, "out_vec", [
            ("x".to_string(), ParamValue::Literal(serde_json::json!(1))),
            ("y".to_string(), ParamValue::Literal(serde_json::json!(2))),
            ("z".to_string(), ParamValue::Literal(serde_json::json!(3))),
        ].into())?;
        assert_data_node_generates(NodeType::BreakVector, "x", [(
            "in_vec".to_string(),
            ParamValue::Literal(serde_json::json!([1, 2, 3])),
        )].into())?;
        Ok(())
    }

    #[test]
    fn test_generate_conditions_and_queries() -> Result<()> {
        assert_data_node_generates(NodeType::CheckCondition, "out_result", [(
            "cond".to_string(),
            ParamValue::Literal(serde_json::json!("cond")),
        )].into())?;
        assert_data_node_generates(NodeType::CheckEquipment, "out_value", [(
            "equipType".to_string(),
            ParamValue::Literal(serde_json::json!("Vibrator")),
        )].into())?;
        assert_data_node_generates(NodeType::CheckCosplay, "out_value", [(
            "cosplayKeys".to_string(),
            ParamValue::Literal(serde_json::json!(["nurse"])),
        )].into())?;
        assert_data_node_generates(NodeType::CheckCosplay, "out_value", [(
            "cosplayKeys".to_string(),
            ParamValue::Literal(serde_json::json!(["Maid", "Bunny"])),
        )].into())?;
        assert_data_node_generates(NodeType::GetStateBool, "out_value", [(
            "stateKey".to_string(),
            ParamValue::Literal(serde_json::json!("Futanari")),
        )].into())?;
        assert_data_node_generates(NodeType::GetStateNumber, "out_value", [(
            "stateKey".to_string(),
            ParamValue::Literal(serde_json::json!("Ecstasy")),
        )].into())?;
        assert_flow_node_generates(NodeType::CreateCondition, [(
            "condition".to_string(),
            ParamValue::Literal(serde_json::json!("Always")),
        )].into())?;
        assert_flow_node_generates(NodeType::CreateItemCondition, [(
            "itemtype".to_string(),
            ParamValue::Literal(serde_json::json!("Key")),
        )].into())?;
        Ok(())
    }

    #[test]
    fn test_generate_check_cosplay_multiple_keys() -> Result<()> {
        let mut graph = make_graph();
        add_data_node_through_setvar(
            &mut graph,
            "n1",
            NodeType::CheckCosplay,
            "out_value",
            [(
                "cosplayKeys".to_string(),
                ParamValue::Literal(serde_json::json!(["Maid", "Bunny"])),
            )]
            .into(),
        );
        let code = generate_code(&graph)?;
        assert!(
            code.contains("(Cosplay_Maid && Cosplay_Bunny)"),
            "Expected CheckCosplay with multiple keys to generate parenthesized AND expression, got:\n{}",
            code
        );
        Ok(())
    }

    #[test]
    fn test_create_condition_uses_positional_argument_and_skips_empty_id() -> Result<()> {
        let mut graph = make_graph();
        add_flow_node(
            &mut graph,
            "n1",
            NodeType::CreateCondition,
            [(
                "condition".to_string(),
                ParamValue::Literal(serde_json::json!("Exposed_All")),
            ),
            (
                "id".to_string(),
                ParamValue::Literal(serde_json::json!("")),
            )]
            .into(),
        );
        let code = generate_code(&graph)?;
        assert!(
            code.contains("var_n1_out_condition = CreateCondition(\"Exposed_All\")"),
            "Expected CreateCondition with positional argument and no empty id, got:\n{}",
            code
        );
        assert!(
            !code.contains("condition=") && !code.contains("id=\"\""),
            "CreateCondition should not emit 'condition=' or empty id, got:\n{}",
            code
        );
        Ok(())
    }

    #[test]
    fn test_create_condition_with_id_emits_named_id() -> Result<()> {
        let mut graph = make_graph();
        add_flow_node(
            &mut graph,
            "n1",
            NodeType::CreateCondition,
            [(
                "condition".to_string(),
                ParamValue::Literal(serde_json::json!("Exposed_All")),
            ),
            (
                "id".to_string(),
                ParamValue::Literal(serde_json::json!("my_id")),
            )]
            .into(),
        );
        let code = generate_code(&graph)?;
        assert!(
            code.contains("var_n1_out_condition = CreateCondition(\"Exposed_All\", id=\"my_id\")"),
            "Expected CreateCondition with named id, got:\n{}",
            code
        );
        Ok(())
    }

    #[test]
    fn test_create_condition_id_from_data_input() -> Result<()> {
        let mut graph = make_graph();
        // Create a data node that outputs the id string "data_id".
        let mut id_node = make_data_node("id_src", NodeType::StringConstant, "out_value");
        id_node.params = [("value".to_string(), ParamValue::Literal(serde_json::json!("data_id")))]
            .into();
        graph.threads[0].labels[0].nodes.insert("id_src".to_string(), id_node);

        add_flow_node(
            &mut graph,
            "n1",
            NodeType::CreateCondition,
            [(
                "condition".to_string(),
                ParamValue::Literal(serde_json::json!("Exposed_All")),
            )]
            .into(),
        );
        // The test helper only creates Flow ports; add the id data input port.
        graph.threads[0].labels[0]
            .nodes
            .get_mut("n1")
            .unwrap()
            .inputs
            .push(Port::new("id", PortType::String, "ID"));
        // Wire StringConstant.value -> CreateCondition.id
        connect_data(
            &mut graph.threads[0].labels[0],
            "id_src",
            "out_value",
            "n1",
            "id",
            PortType::String,
        );

        let code = generate_code(&graph)?;
        assert!(
            code.contains("var_n1_out_condition = CreateCondition(\"Exposed_All\", id=\"data_id\")"),
            "Expected CreateCondition id from data input, got:\n{}",
            code
        );
        Ok(())
    }

    #[test]
    fn test_create_item_condition_id_from_data_input() -> Result<()> {
        let mut graph = make_graph();
        let mut id_node = make_data_node("id_src", NodeType::StringConstant, "out_value");
        id_node.params = [("value".to_string(), ParamValue::Literal(serde_json::json!("item_id")))]
            .into();
        graph.threads[0].labels[0].nodes.insert("id_src".to_string(), id_node);

        add_flow_node(
            &mut graph,
            "n1",
            NodeType::CreateItemCondition,
            [("itemtype".to_string(), ParamValue::Literal(serde_json::json!("Key")))]
                .into(),
        );
        graph.threads[0].labels[0]
            .nodes
            .get_mut("n1")
            .unwrap()
            .inputs
            .push(Port::new("id", PortType::String, "ID"));
        graph.threads[0].labels[0]
            .nodes
            .get_mut("n1")
            .unwrap()
            .outputs
            .push(Port::new("out_condition", PortType::Object, "Condition"));
        connect_data(
            &mut graph.threads[0].labels[0],
            "id_src",
            "out_value",
            "n1",
            "id",
            PortType::String,
        );

        let code = generate_code(&graph)?;
        assert!(
            code.contains("var_n1_out_condition = CreateItemCondition(itemtype=\"Key\", id=\"item_id\")"),
            "Expected CreateItemCondition id from data input, got:\n{}",
            code
        );
        Ok(())
    }

    #[test]
    fn test_create_item_condition_skips_empty_id() -> Result<()> {
        let mut graph = make_graph();
        add_flow_node(
            &mut graph,
            "n1",
            NodeType::CreateItemCondition,
            [(
                "itemtype".to_string(),
                ParamValue::Literal(serde_json::json!("Key")),
            ),
            (
                "id".to_string(),
                ParamValue::Literal(serde_json::json!("")),
            )]
            .into(),
        );
        // CreateItemCondition 通用分支依赖 node.outputs 生成变量赋值，
        // 而测试 helper make_node 只创建默认 Flow 端口，这里手动补上 out_condition。
        graph.threads[0].labels[0]
            .nodes
            .get_mut("n1")
            .unwrap()
            .outputs
            .push(Port::new("out_condition", PortType::Object, "Condition"));
        let code = generate_code(&graph)?;
        assert!(
            code.contains("var_n1_out_condition = CreateItemCondition(itemtype=\"Key\")"),
            "Expected CreateItemCondition without empty id, got:\n{}",
            code
        );
        assert!(
            !code.contains("id=\"\""),
            "CreateItemCondition should not emit empty id, got:\n{}",
            code
        );
        Ok(())
    }

    #[test]
    fn test_generate_game_api_items_and_equipment() -> Result<()> {
        for ty in [
            NodeType::DropItem,
            NodeType::CollectItem,
            NodeType::SetVibrator,
            NodeType::SetPiston,
            NodeType::LockHandcuffs,
            NodeType::UnlockHandcuffs,
            NodeType::EquipCosplay,
            NodeType::UnequipCosplay,
            NodeType::UnequipAllCosplay,
            NodeType::OwnCosplay,
            NodeType::EquipAdultToy,
            NodeType::UnequipAdultToy,
        ] {
            assert_flow_node_generates(ty, HashMap::new())?;
        }
        Ok(())
    }

    #[test]
    fn test_generate_equip_cosplay_multiple_keys() -> Result<()> {
        let mut graph = make_graph();
        add_flow_node(
            &mut graph,
            "n1",
            NodeType::EquipCosplay,
            [(
                "cosplayKeys".to_string(),
                ParamValue::Literal(serde_json::json!(["Maid", "Bunny"])),
            )]
            .into(),
        );
        let code = generate_code(&graph)?;
        assert!(
            code.contains(r#"EquipCosplay(cosplayKeys=["Maid", "Bunny"])"#),
            "Expected multi-select cosplay keys to be generated as array, got:\n{}",
            code
        );
        Ok(())
    }

    #[test]
    fn test_generate_game_api_player_state() -> Result<()> {
        for ty in [
            NodeType::SetPlayerPosition,
            NodeType::SetStage,
            NodeType::SetCamera,
            NodeType::SetAction,
            NodeType::SetFutanari,
            NodeType::SetSkill,
            NodeType::SetPlayerData,
            NodeType::SetSkillShortcut,
            NodeType::GetSkillShortcut,
            NodeType::GetRandomPosition,
        ] {
            assert_flow_node_generates(ty, HashMap::new())?;
        }
        Ok(())
    }

    #[test]
    fn test_generate_game_api_control_and_graphics() -> Result<()> {
        for ty in [
            NodeType::CanGameOver,
            NodeType::TriggerGameOver,
            NodeType::PlaySoundEffect,
            NodeType::SetStageRankLimit,
            NodeType::GetStageRankLimit,
            NodeType::SetPortalEnabled,
            NodeType::GetAllWaypoints,
            NodeType::SetSexPosition,
            NodeType::DeactivateSex,
            NodeType::SetSexMenu,
            NodeType::ShowBlackscreen,
            NodeType::GetSnapshotData,
            NodeType::GetAllSnapshots,
            NodeType::DeleteSnapshot,
            NodeType::GetImageReference,
            NodeType::SetGraphicsOption,
            NodeType::GetGraphicsOption,
        ] {
            assert_flow_node_generates(ty, HashMap::new())?;
        }
        Ok(())
    }

    #[test]
    fn test_generate_game_api_stats() -> Result<()> {
        for ty in [
            NodeType::AddCurrentEarnRP,
            NodeType::SetCurrentEarnRP,
            NodeType::GetCurrentEarnRP,
            NodeType::AddCurrentRP,
            NodeType::SetCurrentRP,
            NodeType::GetCurrentRP,
            NodeType::SetEcstasy,
            NodeType::AddEcstasy,
            NodeType::GetEcstasy,
            NodeType::SetStamina,
            NodeType::AddStamina,
            NodeType::GetStamina,
            NodeType::SetMoisture,
            NodeType::AddMoisture,
            NodeType::GetMoisture,
            NodeType::SetItemCount,
            NodeType::AddItemCount,
            NodeType::GetItemCount,
        ] {
            assert_flow_node_generates(ty, HashMap::new())?;
        }
        Ok(())
    }

    #[test]
    fn test_generate_objects() -> Result<()> {
        for ty in [
            NodeType::CreateMissionPanel,
            NodeType::CreateMissionMenuItem,
            NodeType::CreateArea,
            NodeType::CreateZone,
            NodeType::CreateInteractArea,
            NodeType::CreateText,
            NodeType::CreateMessengerChat,
            NodeType::CreateAudio,
            NodeType::CreateGallery,
            NodeType::CreateSnapshot,
            NodeType::CreateNPC,
            NodeType::CreateInput,
            NodeType::CallMethod,
            NodeType::ForeachNode,
        ] {
            assert_flow_node_generates(ty, HashMap::new())?;
        }
        assert_flow_node_generates(NodeType::CallFunction, [(
            "function".to_string(),
            ParamValue::Literal(serde_json::json!("myFunc")),
        )].into())?;
        Ok(())
    }

    #[test]
    fn test_generate_string_file_list() -> Result<()> {
        for ty in [
            NodeType::Length,
            NodeType::Lower,
            NodeType::Upper,
            NodeType::Find,
            NodeType::SubString,
            NodeType::Format,
            NodeType::ToNumber,
            NodeType::FileExists,
            NodeType::GetFiles,
            NodeType::GetFileExtension,
            NodeType::CreateList,
            NodeType::Copy,
            NodeType::CreateListFromJson,
        ] {
            assert_flow_node_generates(ty, HashMap::new())?;
        }
        Ok(())
    }

    #[test]
    fn test_generate_editor_only() -> Result<()> {
        for ty in [NodeType::Meta, NodeType::Comment, NodeType::Group] {
            assert_flow_node_generates(ty, HashMap::new())?;
        }
        Ok(())
    }
}
