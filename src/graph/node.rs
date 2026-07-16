use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::types::{DynamicPortGroup, DynamicPortKind, DynamicPortTemplate, PortType};

/// 节点上的单个端口定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Port {
    /// 端口在节点内的唯一标识
    pub id: String,
    /// 端口数据类型
    #[serde(rename = "type")]
    pub port_type: PortType,
    /// 在 UI 上显示的文本
    pub label: String,
    /// 是否为必填端口（输入端口）
    #[serde(default)]
    pub required: bool,
}

impl Port {
    /// 创建一个新的端口
    pub fn new(id: &str, port_type: PortType, label: &str) -> Self {
        Self {
            id: id.to_string(),
            port_type,
            label: label.to_string(),
            required: false,
        }
    }

    /// 设置端口是否为必填
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
}

/// 节点参数值，支持直接字面量或引用其他节点的输出
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum ParamValue {
    /// 引用另一个节点的输出端口
    Ref {
        /// 被引用节点的 ID，序列化为 `"ref"`
        #[serde(rename = "ref")]
        node: String,
        /// 被引用端口的 ID
        port: String,
    },
    /// 空值，表示参数缺失
    Null,
    /// 任意 JSON 字面量
    Literal(serde_json::Value),
}

impl ParamValue {
    /// 从引用构造参数值
    pub fn from_ref(node: &str, port: &str) -> Self {
        Self::Ref {
            node: node.to_string(),
            port: port.to_string(),
        }
    }
}

/// 节点，编辑器画布中的基本单元
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// 全局唯一 ID
    pub id: String,
    /// 节点类型
    #[serde(rename = "type")]
    pub node_type: super::types::NodeType,
    /// 节点在画布上的位置
    pub position: Vec2,
    /// 节点在画布上的尺寸
    pub size: Vec2,
    /// 是否折叠
    pub collapsed: bool,
    /// 节点参数表，键为 API 参数名
    pub params: HashMap<String, ParamValue>,
    /// 输入端口列表
    pub inputs: Vec<Port>,
    /// 输出端口列表
    pub outputs: Vec<Port>,
    /// 动态端口/参数分组状态。
    ///
    /// Key 为 `DynamicPortGroup::id`，Value 为该组当前实际存在的端口/参数 ID 列表。
    /// 这些 ID 必须同时存在于 `inputs`、`outputs` 或 `params` 中。
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub dynamic_ports: HashMap<String, Vec<String>>,
    /// 节点分类，用于 UI 着色
    pub category: String,
}

impl Node {
    /// 使用指定类型和位置创建一个新节点
    pub fn new(node_type: super::types::NodeType, position: Vec2) -> Self {
        let id = format!("node_{}", &Uuid::new_v4().to_string()[..8]);
        Self {
            id,
            node_type,
            position,
            size: Vec2::new(180.0, 120.0),
            collapsed: false,
            params: HashMap::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            dynamic_ports: HashMap::new(),
            category: String::new(),
        }
    }

    /// 获取某个动态端口组的当前成员 ID 列表。
    pub fn dynamic_port_ids(&self, group_id: &str) -> &[String] {
        self.dynamic_ports.get(group_id).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// 添加一个动态端口/参数到指定组，返回新成员的 ID。
    ///
    /// 如果该组在 `dynamic_ports` 中不存在则自动创建。
    pub fn add_dynamic_port(&mut self, group: &DynamicPortGroup) -> String {
        let existing = self.dynamic_ports.entry(group.id.clone()).or_default();
        let index = existing.len();
        let id = format!("{}_{}", group.prefix, index);
        match &group.template {
            DynamicPortTemplate::Port(def) => {
                let port = Port::new(&id, def.port_type.clone(), &def.label);
                if group.kind == DynamicPortKind::Input {
                    self.inputs.push(port);
                } else {
                    self.outputs.push(port);
                }
            }
            DynamicPortTemplate::Param(def) => {
                self.params.insert(id.clone(), def.default_value());
            }
        }
        existing.push(id.clone());
        id
    }

    /// 从指定组删除一个动态端口/参数，并返回是否删除成功。
    ///
    /// 删除后不会重排其余成员的 ID，以保持连接稳定。
    pub fn remove_dynamic_port(&mut self, group: &DynamicPortGroup, port_id: &str) -> bool {
        let Some(existing) = self.dynamic_ports.get_mut(&group.id) else {
            return false;
        };
        if !existing.contains(&port_id.to_string()) {
            return false;
        }
        existing.retain(|id| id != port_id);
        match &group.template {
            DynamicPortTemplate::Port(_) => {
                if group.kind == DynamicPortKind::Input {
                    self.inputs.retain(|p| p.id != port_id);
                } else {
                    self.outputs.retain(|p| p.id != port_id);
                }
            }
            DynamicPortTemplate::Param(_) => {
                self.params.remove(port_id);
            }
        }
        true
    }

    /// 返回某个动态端口组的当前成员数量。
    pub fn dynamic_port_count(&self, group_id: &str) -> usize {
        self.dynamic_ports.get(group_id).map(|v| v.len()).unwrap_or(0)
    }

    /// 批量设置节点的输入和输出端口
    pub fn with_ports(mut self, inputs: Vec<Port>, outputs: Vec<Port>) -> Self {
        self.inputs = inputs;
        self.outputs = outputs;
        self
    }

    /// 设置节点的分类
    pub fn with_category(mut self, category: &str) -> Self {
        self.category = category.to_string();
        self
    }

    /// 设置一个参数值
    pub fn set_param(&mut self, name: &str, value: ParamValue) -> &mut Self {
        self.params.insert(name.to_string(), value);
        self
    }

    /// 获取指定端口
    ///
    /// `is_input` 为 `true` 时搜索输入端口，否则搜索输出端口
    pub fn get_port(&self, port_id: &str, is_input: bool) -> Option<&Port> {
        let ports = if is_input {
            &self.inputs
        } else {
            &self.outputs
        };
        ports.iter().find(|p| p.id == port_id)
    }
}

/// 2D 向量，用于节点位置和尺寸
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq)]
pub struct Vec2 {
    /// X 坐标
    pub x: f32,
    /// Y 坐标
    pub y: f32,
}

impl Vec2 {
    /// 创建一个新的 2D 向量
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// 零向量
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl std::ops::DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_node_new_has_unique_id() {
        let n1 = Node::new(super::super::types::NodeType::Log, Vec2::new(10.0, 20.0));
        let n2 = Node::new(super::super::types::NodeType::Log, Vec2::new(10.0, 20.0));
        assert!(!n1.id.is_empty());
        assert!(!n2.id.is_empty());
        assert_ne!(n1.id, n2.id);
    }

    #[test]
    fn test_node_with_ports() {
        let inputs = vec![Port::new("in_flow", PortType::Flow, "Execute")];
        let outputs = vec![
            Port::new("out_flow", PortType::Flow, "Next"),
            Port::new("out_result", PortType::String, "Return"),
        ];
        let node =
            Node::new(super::super::types::NodeType::Log, Vec2::ZERO).with_ports(inputs, outputs);
        assert_eq!(node.inputs.len(), 1);
        assert_eq!(node.outputs.len(), 2);
        assert!(node.get_port("in_flow", true).is_some());
        assert!(node.get_port("out_result", false).is_some());
        assert!(node.get_port("missing", true).is_none());
    }

    #[test]
    fn test_param_value_literal_roundtrip() {
        let value = ParamValue::Literal(json!("hello"));
        let json_str = serde_json::to_string(&value).unwrap();
        assert_eq!(json_str, "\"hello\"");
        let back: ParamValue = serde_json::from_str(&json_str).unwrap();
        assert!(matches!(back, ParamValue::Literal(_)));
    }

    #[test]
    fn test_param_value_ref_roundtrip() {
        let value = ParamValue::from_ref("node_002", "out_result");
        let json_str = serde_json::to_string(&value).unwrap();
        assert_eq!(json_str, r#"{"ref":"node_002","port":"out_result"}"#);
        let back: ParamValue = serde_json::from_str(&json_str).unwrap();
        assert!(matches!(back, ParamValue::Ref { .. }));
    }

    #[test]
    fn test_param_value_null_roundtrip() {
        let value = ParamValue::Null;
        let json_str = serde_json::to_string(&value).unwrap();
        assert_eq!(json_str, "null");
        let back: ParamValue = serde_json::from_str(&json_str).unwrap();
        assert!(matches!(back, ParamValue::Null));
    }

    #[test]
    fn test_dynamic_port_add_and_remove_output() {
        use super::super::types::{DynamicPortGroup, DynamicPortKind, DynamicPortTemplate};
        use crate::api::definitions::PortDefinition;

        let mut node = Node::new(super::super::types::NodeType::Log, Vec2::ZERO);
        let group = DynamicPortGroup::new(
            "extras",
            "Extras",
            DynamicPortKind::Output,
            "extra",
            DynamicPortTemplate::Port(PortDefinition::new("extra", PortType::String, "Extra")),
        );
        let id1 = node.add_dynamic_port(&group);
        let id2 = node.add_dynamic_port(&group);
        assert_eq!(node.dynamic_port_count("extras"), 2);
        assert!(node.get_port(&id1, false).is_some());
        assert!(node.get_port(&id2, false).is_some());
        assert!(node.remove_dynamic_port(&group, &id1));
        assert!(node.get_port(&id1, false).is_none());
        assert!(node.get_port(&id2, false).is_some());
    }

    #[test]
    fn test_dynamic_param_add_and_remove() {
        use super::super::types::{DynamicPortGroup, DynamicPortKind, DynamicPortTemplate};
        use crate::api::definitions::{ParamDefinition, ParamType};

        let mut node = Node::new(super::super::types::NodeType::Log, Vec2::ZERO);
        let group = DynamicPortGroup::new(
            "args",
            "Args",
            DynamicPortKind::Param,
            "arg",
            DynamicPortTemplate::Param(ParamDefinition::new("arg", "Arg", ParamType::String)),
        );
        let id1 = node.add_dynamic_port(&group);
        assert!(node.params.contains_key(&id1));
        assert!(node.remove_dynamic_port(&group, &id1));
        assert!(!node.params.contains_key(&id1));
    }

    #[test]
    fn test_dynamic_port_serialization_roundtrip() {
        use super::super::types::{DynamicPortGroup, DynamicPortKind, DynamicPortTemplate};
        use crate::api::definitions::{ParamDefinition, ParamType, PortDefinition};

        let mut node = Node::new(super::super::types::NodeType::Log, Vec2::ZERO);
        let out_group = DynamicPortGroup::new(
            "outs",
            "Outs",
            DynamicPortKind::Output,
            "out",
            DynamicPortTemplate::Port(PortDefinition::new("out", PortType::String, "Out")),
        );
        let _ = node.add_dynamic_port(&out_group);
        let param_group = DynamicPortGroup::new(
            "args",
            "Args",
            DynamicPortKind::Param,
            "arg",
            DynamicPortTemplate::Param(ParamDefinition::new("arg", "Arg", ParamType::String)),
        );
        let _ = node.add_dynamic_port(&param_group);

        let json = serde_json::to_string(&node).expect("serialize");
        let back: Node = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.dynamic_port_count("outs"), 1);
        assert_eq!(back.dynamic_port_count("args"), 1);
        assert_eq!(back.outputs.len(), 1);
        assert_eq!(back.params.len(), 1);
    }

    #[test]
    fn test_vec2_operations() {
        let v = Vec2::new(3.0, 4.0);
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 4.0);
        assert_eq!(Vec2::ZERO, Vec2::new(0.0, 0.0));
    }
}
