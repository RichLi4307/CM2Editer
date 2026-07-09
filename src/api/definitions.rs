use serde::{Deserialize, Serialize};

use crate::api::enums::{
    to_strings, ACTIONS, CONDITION_TYPES, DROP_ITEM_TYPES, GRAPHICS_OPTIONS,
    HANDCUFFS_TYPES, ITEMS, PISTON_STRENGTHS, SEX_POSITIONS, SKILLS, SOUND_EFFECTS, STAGE_TYPES,
    VIBRATOR_STRENGTHS,
};
use crate::graph::node::ParamValue;
use crate::graph::types::{NodeType, PortType};

/// Logical parameter type used in node definitions.
///
/// This is a richer type than [`PortType`] because parameters may carry
/// UI-specific semantics (such as color or vector) while still being
/// transmitted as [`PortType::List`] through data ports.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum ParamType {
    Number,
    String,
    Boolean,
    List,
    Object,
    Color,
    Vector,
    Quaternion,
    /// 枚举/命名空间类型，显示为下拉选择框。
    Enum,
}

impl ParamType {
    /// Returns the data port type that should be used for values of this
    /// parameter type.
    pub const fn port_type(&self) -> PortType {
        match self {
            ParamType::Number => PortType::Number,
            ParamType::String | ParamType::Enum => PortType::String,
            ParamType::Boolean => PortType::Boolean,
            ParamType::List | ParamType::Color | ParamType::Vector | ParamType::Quaternion => {
                PortType::List
            }
            ParamType::Object => PortType::Object,
        }
    }

    /// Returns a sensible default value for this parameter type.
    pub fn default_value(self) -> ParamValue {
        default_param_value(self)
    }
}

/// Static definition of a single port on a node.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortDefinition {
    /// Port identifier, unique within the node.
    pub id: String,
    /// Port data type (flow or concrete data type).
    pub port_type: PortType,
    /// Label shown in the UI.
    pub label: String,
    /// Whether the port must be connected for the node to be valid.
    pub required: bool,
}

impl PortDefinition {
    /// Creates a new port definition.
    pub fn new(id: &str, port_type: PortType, label: &str) -> Self {
        Self {
            id: id.to_string(),
            port_type,
            label: label.to_string(),
            required: false,
        }
    }

    /// Sets whether the port is required.
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
}

/// Static definition of a single parameter on a node.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParamDefinition {
    /// API parameter name (e.g. `itemtype`).
    pub name: String,
    /// Display name in the UI.
    pub display_name: String,
    /// Logical parameter type.
    pub param_type: ParamType,
    /// Default value, if any.
    pub default: Option<ParamValue>,
    /// Whether the parameter must be provided.
    pub required: bool,
    /// Fixed list of options, if the parameter is a dropdown.
    pub options: Option<Vec<String>>,
    /// Tooltip description.
    pub description: Option<String>,
}

impl ParamDefinition {
    /// Creates a new parameter definition.
    pub fn new(name: &str, display_name: &str, param_type: ParamType) -> Self {
        Self {
            name: name.to_string(),
            display_name: display_name.to_string(),
            param_type,
            default: None,
            required: false,
            options: None,
            description: None,
        }
    }

    /// Marks the parameter as required.
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Sets the default value for the parameter.
    pub fn with_default(mut self, default: ParamValue) -> Self {
        self.default = Some(default);
        self
    }

    /// Sets a fixed list of options for the parameter.
    pub fn with_options(mut self, options: Vec<String>) -> Self {
        self.options = Some(options);
        self
    }

    /// Sets the tooltip description for the parameter.
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Returns a sensible default value for this parameter definition.
    pub fn default_value(&self) -> ParamValue {
        self.default
            .clone()
            .unwrap_or_else(|| default_param_value(self.param_type))
    }
}

/// Static definition of a node type.
///
/// Each variant of [`NodeType`] has a corresponding `NodeDefinition` that
/// describes its category, ports, parameters and visual appearance.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NodeDefinition {
    /// The node type this definition describes.
    pub node_type: NodeType,
    /// Category used for grouping in the node library.
    pub category: String,
    /// Display name in the UI.
    pub display_name: String,
    /// Tooltip description.
    pub description: String,
    /// Input port definitions.
    pub inputs: Vec<PortDefinition>,
    /// Output port definitions.
    pub outputs: Vec<PortDefinition>,
    /// Parameter definitions.
    pub params: Vec<ParamDefinition>,
    /// Title bar color in RGBA.
    pub color: [u8; 4],
}

impl NodeDefinition {
    /// Creates a new node definition with no ports or parameters.
    pub fn new(
        node_type: NodeType,
        category: &str,
        display_name: &str,
        description: &str,
        color: [u8; 4],
    ) -> Self {
        Self {
            node_type,
            category: category.to_string(),
            display_name: display_name.to_string(),
            description: description.to_string(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            params: Vec::new(),
            color,
        }
    }

    /// Sets the input ports.
    pub fn with_inputs(mut self, inputs: Vec<PortDefinition>) -> Self {
        self.inputs = inputs;
        self
    }

    /// Sets the output ports.
    pub fn with_outputs(mut self, outputs: Vec<PortDefinition>) -> Self {
        self.outputs = outputs;
        self
    }

    /// Sets the parameters.
    pub fn with_params(mut self, params: Vec<ParamDefinition>) -> Self {
        self.params = params;
        self
    }
}

// -------------------------------------------------------------------------
// Category colors
// -------------------------------------------------------------------------

const CONTROL_COLOR: [u8; 4] = [156, 39, 176, 255]; // purple
const GENERAL_COLOR: [u8; 4] = [33, 150, 243, 255]; // blue
const GAME_COLOR: [u8; 4] = [76, 175, 80, 255]; // green
const STATS_COLOR: [u8; 4] = [255, 152, 0, 255]; // orange
const OBJECT_COLOR: [u8; 4] = [0, 188, 212, 255]; // cyan
const MATH_COLOR: [u8; 4] = [96, 125, 139, 255]; // grey
const STRING_COLOR: [u8; 4] = [233, 30, 99, 255]; // pink
const FILE_COLOR: [u8; 4] = [121, 85, 72, 255]; // brown
const WAIT_COLOR: [u8; 4] = [255, 235, 59, 255]; // yellow
const SPECIAL_COLOR: [u8; 4] = [117, 117, 117, 255]; // dark grey

// -------------------------------------------------------------------------
// Port helpers
// -------------------------------------------------------------------------

fn in_flow() -> PortDefinition {
    PortDefinition::new("in_flow", PortType::Flow, "执行").required(true)
}

fn out_flow() -> PortDefinition {
    PortDefinition::new("out_flow", PortType::Flow, "下一步").required(true)
}

fn out_true() -> PortDefinition {
    PortDefinition::new("out_true", PortType::Flow, "True").required(true)
}

fn out_false() -> PortDefinition {
    PortDefinition::new("out_false", PortType::Flow, "False").required(true)
}

fn out_break() -> PortDefinition {
    PortDefinition::new("out_break", PortType::Flow, "Break").required(true)
}

fn out_data(id: &str, port_type: PortType, label: &str) -> PortDefinition {
    PortDefinition::new(id, port_type, label).required(true)
}

// -------------------------------------------------------------------------
// Parameter helpers
// -------------------------------------------------------------------------

fn p(name: &str, display: &str, param_type: ParamType) -> ParamDefinition {
    ParamDefinition::new(name, display, param_type)
}

fn p_req(name: &str, display: &str, param_type: ParamType) -> ParamDefinition {
    p(name, display, param_type).required(true)
}

fn p_opt(name: &str, display: &str, param_type: ParamType) -> ParamDefinition {
    p(name, display, param_type).required(false)
}

/// 创建一个可枚举参数，提供固定下拉选项。
fn e(name: &str, display: &str, options: &[&str]) -> ParamDefinition {
    ParamDefinition::new(name, display, ParamType::Enum)
        .with_options(to_strings(options))
        .with_default(ParamValue::Literal(serde_json::json!(
            options.first().map_or("", |&s| s)
        )))
        .required(true)
}

/// 创建一个可枚举参数（可选）。
#[allow(dead_code)]
fn e_opt(name: &str, display: &str, options: &[&str]) -> ParamDefinition {
    ParamDefinition::new(name, display, ParamType::Enum)
        .with_options(to_strings(options))
        .with_default(ParamValue::Literal(serde_json::json!(
            options.first().map_or("", |&s| s)
        )))
        .required(false)
}

/// Returns a sensible default value for a parameter type.
fn default_param_value(param_type: ParamType) -> ParamValue {
    ParamValue::Literal(match param_type {
        ParamType::Number => serde_json::json!(0.0),
        ParamType::String => serde_json::json!(""),
        ParamType::Boolean => serde_json::json!(true),
        ParamType::List | ParamType::Color | ParamType::Vector | ParamType::Quaternion => {
            serde_json::json!([])
        }
        ParamType::Object => serde_json::json!({}),
        ParamType::Enum => serde_json::json!(""),
    })
}

// -------------------------------------------------------------------------
// All node definitions
// -------------------------------------------------------------------------

/// Returns the static definitions for every supported node type.
#[allow(clippy::too_many_lines)]
pub fn all_definitions() -> Vec<NodeDefinition> {
    vec![
        // -----------------------------------------------------------------
        // Control
        // -----------------------------------------------------------------
        NodeDefinition::new(
            NodeType::Start,
            "Control",
            "开始",
            "任务入口，每张图必须有且仅有一个",
            CONTROL_COLOR,
        )
        .with_outputs(vec![out_flow()]),
        NodeDefinition::new(
            NodeType::Label,
            "Control",
            "标签",
            "标签定义，可作为 Goto 目标",
            CONTROL_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![p_req("name", "名称", ParamType::String)]),
        NodeDefinition::new(
            NodeType::Goto,
            "Control",
            "跳转",
            "跳转到指定标签",
            CONTROL_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![
            p_req("label", "标签", ParamType::String),
            p_opt("params", "参数", ParamType::Object),
        ]),
        NodeDefinition::new(NodeType::If, "Control", "如果", "条件分支", CONTROL_COLOR)
            .with_inputs(vec![in_flow()])
            .with_outputs(vec![out_true(), out_false()])
            .with_params(vec![p_req("condition", "条件", ParamType::Boolean)]),
        NodeDefinition::new(
            NodeType::While,
            "Control",
            "循环",
            "条件成立时循环",
            CONTROL_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow(), out_break()])
        .with_params(vec![p_req("condition", "条件", ParamType::Boolean)]),
        NodeDefinition::new(NodeType::For, "Control", "遍历", "遍历列表", CONTROL_COLOR)
            .with_inputs(vec![in_flow()])
            .with_outputs(vec![out_flow(), out_break()])
            .with_params(vec![p_req("iterable", "列表", ParamType::List)]),
        NodeDefinition::new(
            NodeType::Break,
            "Control",
            "跳出",
            "提前退出循环",
            CONTROL_COLOR,
        )
        .with_inputs(vec![in_flow()]),
        NodeDefinition::new(
            NodeType::Return,
            "Control",
            "返回",
            "函数返回，设置 _result",
            CONTROL_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_params(vec![p_opt("value", "返回值", ParamType::List)]),
        NodeDefinition::new(
            NodeType::Wait,
            "Control",
            "等待",
            "延迟等待（秒）",
            WAIT_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![p_req("seconds", "秒数", ParamType::Number)]),
        NodeDefinition::new(
            NodeType::WaitForEvent,
            "Control",
            "等待事件",
            "阻塞当前线程直到事件触发",
            WAIT_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![p_req("eventName", "事件名", ParamType::String)]),
        // -----------------------------------------------------------------
        // General Functions
        // -----------------------------------------------------------------
        NodeDefinition::new(
            NodeType::Log,
            "General Functions",
            "日志",
            "控制台输出",
            GENERAL_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![p_req("output", "输出", ParamType::String)]),
        NodeDefinition::new(
            NodeType::Global,
            "General Functions",
            "全局变量",
            "读写全局变量",
            GENERAL_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_value", PortType::List, "值"),
        ])
        .with_params(vec![
            p_req("name", "变量名", ParamType::String),
            p_opt("value", "值", ParamType::List),
        ]),
        NodeDefinition::new(
            NodeType::Local,
            "General Functions",
            "局部变量",
            "读写局部变量（线程/标签作用域）",
            GENERAL_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_value", PortType::List, "值"),
        ])
        .with_params(vec![
            p_req("name", "变量名", ParamType::String),
            p_opt("value", "值", ParamType::List),
        ]),
        NodeDefinition::new(
            NodeType::GetType,
            "General Functions",
            "获取类型",
            "获取值的类型名称",
            GENERAL_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_type", PortType::String, "类型"),
        ])
        .with_params(vec![p_req("value", "值", ParamType::List)]),
        NodeDefinition::new(
            NodeType::GetLanguage,
            "General Functions",
            "获取语言",
            "获取当前语言代码",
            GENERAL_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_language", PortType::String, "语言"),
        ]),
        NodeDefinition::new(
            NodeType::DumpVariables,
            "General Functions",
            "打印所有变量",
            "打印所有变量到日志",
            GENERAL_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![p_opt("recursion", "递归深度", ParamType::Number)]),
        NodeDefinition::new(
            NodeType::DumpVariable,
            "General Functions",
            "打印变量",
            "打印单个变量到日志",
            GENERAL_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![
            p_req("var", "变量", ParamType::List),
            p_opt("recursion", "递归深度", ParamType::Number),
        ]),
        NodeDefinition::new(
            NodeType::CallFunction,
            "General Functions",
            "调用函数",
            "动态调用函数名",
            GENERAL_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_result", PortType::List, "结果"),
        ])
        .with_params(vec![
            p_req("function", "函数名", ParamType::String),
            p_opt("params", "参数", ParamType::Object),
        ]),
        NodeDefinition::new(
            NodeType::CallMethod,
            "General Functions",
            "调用方法",
            "动态调用对象方法",
            GENERAL_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_result", PortType::List, "结果"),
        ])
        .with_params(vec![
            p_req("thread", "线程对象", ParamType::Object),
            p_req("method", "方法名", ParamType::String),
            p_opt("params", "参数", ParamType::Object),
        ]),
        NodeDefinition::new(
            NodeType::Color,
            "General Functions",
            "颜色",
            "创建颜色列表",
            GENERAL_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_color", PortType::List, "颜色"),
        ])
        .with_params(vec![
            p_req("r", "红", ParamType::Number),
            p_req("g", "绿", ParamType::Number),
            p_req("b", "蓝", ParamType::Number),
            p_req("a", "透明度", ParamType::Number),
        ]),
        NodeDefinition::new(
            NodeType::Range,
            "General Functions",
            "范围",
            "生成数字范围",
            GENERAL_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_list", PortType::List, "列表"),
        ])
        .with_params(vec![
            p_req("start", "起始", ParamType::Number),
            p_req("stop", "结束", ParamType::Number),
            p_opt("step", "步长", ParamType::Number),
        ]),
        NodeDefinition::new(
            NodeType::SetEvent,
            "General Functions",
            "设置事件",
            "设置跨线程/跨帧事件数据",
            GENERAL_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![
            p_req("name", "事件名", ParamType::String),
            p_req("value", "值", ParamType::List),
        ]),
        NodeDefinition::new(
            NodeType::GetEvent,
            "General Functions",
            "获取事件",
            "获取事件数据",
            GENERAL_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_event", PortType::List, "事件数据"),
        ])
        .with_params(vec![p_req("name", "事件名", ParamType::String)]),
        // -----------------------------------------------------------------
        // Game Functions: Items & Equipment
        // -----------------------------------------------------------------
        NodeDefinition::new(
            NodeType::DropItem,
            "Game Functions: Items",
            "掉落物品",
            "在指定场景掉落物品",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_item", PortType::Object, "物品"),
        ])
        .with_params(vec![
            e("itemtype", "物品类型", DROP_ITEM_TYPES),
            e("stage", "场景", STAGE_TYPES),
            p_req("position", "位置", ParamType::Vector),
            p_opt("rotation", "旋转", ParamType::Quaternion),
            p_opt("compass", "指南针", ParamType::Boolean),
        ]),
        NodeDefinition::new(
            NodeType::CollectItem,
            "Game Functions: Items",
            "拾取物品",
            "捡起指定类型物品",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_collected", PortType::Boolean, "是否成功"),
        ])
        .with_params(vec![
            e("itemtype", "物品类型", DROP_ITEM_TYPES),
            p_req("position", "位置", ParamType::Vector),
        ]),
        NodeDefinition::new(
            NodeType::SetVibrator,
            "Game Functions: Items",
            "设置跳蛋",
            "设置跳蛋强度",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![e("strength", "强度", VIBRATOR_STRENGTHS)]),
        NodeDefinition::new(
            NodeType::SetPiston,
            "Game Functions: Items",
            "设置活塞",
            "设置活塞强度",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![e("strength", "强度", PISTON_STRENGTHS)]),
        NodeDefinition::new(
            NodeType::LockHandcuffs,
            "Game Functions: Items",
            "锁手铐",
            "给玩家锁上手铐",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![
            e("handcuffstype", "手铐类型", HANDCUFFS_TYPES),
            p_opt("attachtoobject", "绑定对象", ParamType::Boolean),
            p_opt("duration", "持续时间", ParamType::Number),
        ]),
        NodeDefinition::new(
            NodeType::UnlockHandcuffs,
            "Game Functions: Items",
            "解锁手铐",
            "解锁手铐",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()]),
        NodeDefinition::new(
            NodeType::EquipCosplay,
            "Game Functions: Items",
            "装备 Cosplay",
            "装备角色扮演服装",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![p_req("cosplayKeys", "服装键", ParamType::List)]),
        NodeDefinition::new(
            NodeType::UnequipCosplay,
            "Game Functions: Items",
            "卸下 Cosplay",
            "卸下角色扮演服装",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![p_req("cosplayKeys", "服装键", ParamType::List)]),
        NodeDefinition::new(
            NodeType::UnequipAllCosplay,
            "Game Functions: Items",
            "卸下全部 Cosplay",
            "卸下全部角色扮演服装",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()]),
        NodeDefinition::new(
            NodeType::OwnCosplay,
            "Game Functions: Items",
            "拥有 Cosplay",
            "设置服装拥有状态",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![
            p_req("owns", "是否拥有", ParamType::Boolean),
            p_req("cosplayKeys", "服装键", ParamType::List),
        ]),
        NodeDefinition::new(
            NodeType::EquipAdultToy,
            "Game Functions: Items",
            "装备成人玩具",
            "装备成人玩具",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![p_req("toyNames", "玩具名", ParamType::List)]),
        NodeDefinition::new(
            NodeType::UnequipAdultToy,
            "Game Functions: Items",
            "卸下成人玩具",
            "卸下成人玩具",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![p_req("toyNames", "玩具名", ParamType::List)]),
        // -----------------------------------------------------------------
        // Game Functions: Player State
        // -----------------------------------------------------------------
        NodeDefinition::new(
            NodeType::SetPlayerPosition,
            "Game Functions: Player",
            "设置玩家位置",
            "设置玩家的位置和旋转",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![
            p_req("position", "位置", ParamType::Vector),
            p_opt("rotation", "旋转", ParamType::Quaternion),
        ]),
        NodeDefinition::new(
            NodeType::SetStage,
            "Game Functions: Player",
            "切换场景",
            "切换到不同场景",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![
            e("stage", "场景", STAGE_TYPES),
            p_opt("daytime", "白天", ParamType::Boolean),
        ]),
        NodeDefinition::new(
            NodeType::SetCamera,
            "Game Functions: Player",
            "设置相机",
            "设置相机参数",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![
            p_opt("pitch", "俯仰", ParamType::Number),
            p_opt("yaw", "偏航", ParamType::Number),
            p_opt("lock", "锁定", ParamType::Boolean),
        ]),
        NodeDefinition::new(
            NodeType::SetAction,
            "Game Functions: Player",
            "设置动作",
            "设置玩家当前动作",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![e("action", "动作", ACTIONS)]),
        NodeDefinition::new(
            NodeType::SetFutanari,
            "Game Functions: Player",
            "设置双性状态",
            "设置扶她状态",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![p_req("active", "启用", ParamType::Boolean)]),
        NodeDefinition::new(
            NodeType::SetSkill,
            "Game Functions: Player",
            "设置技能",
            "启用或禁用技能",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![
            e("skill", "技能", SKILLS),
            p_req("enabled", "启用", ParamType::Boolean),
        ]),
        NodeDefinition::new(
            NodeType::SetPlayerData,
            "Game Functions: Player",
            "设置玩家数据",
            "设置任意玩家数据",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![
            p_req("dataName", "数据名", ParamType::String),
            p_req("value", "值", ParamType::List),
        ]),
        NodeDefinition::new(
            NodeType::SetSkillShortcut,
            "Game Functions: Player",
            "设置技能快捷栏",
            "设置技能快捷栏",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![
            p_req("slot", "槽位", ParamType::Number),
            p_req("actionIndex", "动作索引", ParamType::Number),
        ]),
        NodeDefinition::new(
            NodeType::GetSkillShortcut,
            "Game Functions: Player",
            "获取技能快捷栏",
            "获取技能快捷栏",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_index", PortType::Number, "索引"),
        ])
        .with_params(vec![p_req("slot", "槽位", ParamType::Number)]),
        NodeDefinition::new(
            NodeType::GetRandomPosition,
            "Game Functions: Player",
            "随机位置",
            "获取随机位置",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_position", PortType::List, "位置"),
        ])
        .with_params(vec![p_opt("minRange", "最小范围", ParamType::Number)]),
        // -----------------------------------------------------------------
        // Game Functions: Stats
        // -----------------------------------------------------------------
        NodeDefinition::new(
            NodeType::AddCurrentEarnRP,
            "Game Functions: Stats",
            "增加本次 RP",
            "增加本次外出赚取 RP",
            STATS_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_value", PortType::Number, "值"),
        ])
        .with_params(vec![p_req("value", "数值", ParamType::Number)]),
        NodeDefinition::new(
            NodeType::SetCurrentEarnRP,
            "Game Functions: Stats",
            "设置本次 RP",
            "设置本次外出赚取 RP",
            STATS_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_value", PortType::Number, "值"),
        ])
        .with_params(vec![p_req("value", "数值", ParamType::Number)]),
        NodeDefinition::new(
            NodeType::GetCurrentEarnRP,
            "Game Functions: Stats",
            "获取本次 RP",
            "获取本次外出赚取 RP",
            STATS_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_value", PortType::Number, "值"),
        ]),
        NodeDefinition::new(
            NodeType::AddCurrentRP,
            "Game Functions: Stats",
            "增加持有 RP",
            "增加持有 RP",
            STATS_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_value", PortType::Number, "值"),
        ])
        .with_params(vec![p_req("value", "数值", ParamType::Number)]),
        NodeDefinition::new(
            NodeType::SetCurrentRP,
            "Game Functions: Stats",
            "设置持有 RP",
            "设置持有 RP",
            STATS_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_value", PortType::Number, "值"),
        ])
        .with_params(vec![p_req("value", "数值", ParamType::Number)]),
        NodeDefinition::new(
            NodeType::GetCurrentRP,
            "Game Functions: Stats",
            "获取持有 RP",
            "获取持有 RP",
            STATS_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_value", PortType::Number, "值"),
        ]),
        NodeDefinition::new(
            NodeType::SetEcstasy,
            "Game Functions: Stats",
            "设置快感",
            "设置快感值",
            STATS_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_value", PortType::Number, "值"),
        ])
        .with_params(vec![p_req("value", "数值", ParamType::Number)]),
        NodeDefinition::new(
            NodeType::AddEcstasy,
            "Game Functions: Stats",
            "增加快感",
            "增加快感值",
            STATS_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_value", PortType::Number, "值"),
        ])
        .with_params(vec![p_req("value", "数值", ParamType::Number)]),
        NodeDefinition::new(
            NodeType::GetEcstasy,
            "Game Functions: Stats",
            "获取快感",
            "获取快感值",
            STATS_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_value", PortType::Number, "值"),
        ]),
        NodeDefinition::new(
            NodeType::SetStamina,
            "Game Functions: Stats",
            "设置体力",
            "设置体力",
            STATS_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_value", PortType::Number, "值"),
        ])
        .with_params(vec![p_req("value", "数值", ParamType::Number)]),
        NodeDefinition::new(
            NodeType::AddStamina,
            "Game Functions: Stats",
            "增加体力",
            "增加体力",
            STATS_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_value", PortType::Number, "值"),
        ])
        .with_params(vec![p_req("value", "数值", ParamType::Number)]),
        NodeDefinition::new(
            NodeType::GetStamina,
            "Game Functions: Stats",
            "获取体力",
            "获取体力",
            STATS_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_value", PortType::Number, "值"),
        ]),
        NodeDefinition::new(
            NodeType::SetMoisture,
            "Game Functions: Stats",
            "设置湿润度",
            "设置膀胱/湿润度",
            STATS_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_value", PortType::Number, "值"),
        ])
        .with_params(vec![p_req("value", "数值", ParamType::Number)]),
        NodeDefinition::new(
            NodeType::AddMoisture,
            "Game Functions: Stats",
            "增加湿润度",
            "增加膀胱/湿润度",
            STATS_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_value", PortType::Number, "值"),
        ])
        .with_params(vec![p_req("value", "数值", ParamType::Number)]),
        NodeDefinition::new(
            NodeType::GetMoisture,
            "Game Functions: Stats",
            "获取湿润度",
            "获取膀胱/湿润度",
            STATS_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_value", PortType::Number, "值"),
        ]),
        NodeDefinition::new(
            NodeType::SetItemCount,
            "Game Functions: Stats",
            "设置物品数量",
            "设置物品数量",
            STATS_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_count", PortType::Number, "数量"),
        ])
        .with_params(vec![
            e("item", "物品", ITEMS),
            p_req("count", "数量", ParamType::Number),
        ]),
        NodeDefinition::new(
            NodeType::AddItemCount,
            "Game Functions: Stats",
            "增加物品数量",
            "增加物品数量",
            STATS_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_count", PortType::Number, "数量"),
        ])
        .with_params(vec![
            e("item", "物品", ITEMS),
            p_req("count", "数量", ParamType::Number),
        ]),
        NodeDefinition::new(
            NodeType::GetItemCount,
            "Game Functions: Stats",
            "获取物品数量",
            "获取物品数量",
            STATS_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_count", PortType::Number, "数量"),
        ])
        .with_params(vec![p_req("item", "物品", ParamType::String)]),
        // -----------------------------------------------------------------
        // Game Functions: Game Control
        // -----------------------------------------------------------------
        NodeDefinition::new(
            NodeType::CanGameOver,
            "Game Functions: Control",
            "可游戏结束",
            "设置或获取是否可游戏结束",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_value", PortType::Boolean, "值"),
        ])
        .with_params(vec![p_opt("value", "是否可结束", ParamType::Boolean)]),
        NodeDefinition::new(
            NodeType::TriggerGameOver,
            "Game Functions: Control",
            "触发游戏结束",
            "强制触发游戏结束",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()]),
        NodeDefinition::new(
            NodeType::PlaySoundEffect,
            "Game Functions: Control",
            "播放音效",
            "播放音效",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![
            e("name", "音效名", SOUND_EFFECTS),
            p_opt("volume", "音量", ParamType::Number),
            p_opt("position", "位置", ParamType::Vector),
        ]),
        NodeDefinition::new(
            NodeType::SetStageRankLimit,
            "Game Functions: Control",
            "设置场景等级限制",
            "设置场景等级限制",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![
            e("stage", "场景", STAGE_TYPES),
            p_req("rank", "等级", ParamType::Number),
        ]),
        NodeDefinition::new(
            NodeType::GetStageRankLimit,
            "Game Functions: Control",
            "获取场景等级限制",
            "获取场景等级限制",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_rank", PortType::Number, "等级"),
        ])
        .with_params(vec![p_req("stage", "场景", ParamType::String)]),
        NodeDefinition::new(
            NodeType::SetPortalEnabled,
            "Game Functions: Control",
            "设置传送门",
            "启用或禁用传送门",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![
            e("stage", "场景", STAGE_TYPES),
            p_req("enabled", "启用", ParamType::Boolean),
        ]),
        NodeDefinition::new(
            NodeType::GetAllWaypoints,
            "Game Functions: Control",
            "获取路径点",
            "获取所有路径点",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_waypoints", PortType::List, "路径点"),
        ]),
        NodeDefinition::new(
            NodeType::SetSexPosition,
            "Game Functions: Control",
            "设置性爱体位",
            "设置性爱体位",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![e("position", "体位", SEX_POSITIONS)]),
        NodeDefinition::new(
            NodeType::DeactivateSex,
            "Game Functions: Control",
            "停用性爱",
            "停用性爱",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()]),
        NodeDefinition::new(
            NodeType::SetSexMenu,
            "Game Functions: Control",
            "设置性爱菜单",
            "配置性爱菜单",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![
            p_opt("canfinish", "可完成", ParamType::Boolean),
            p_opt("canposition", "可换体位", ParamType::List),
        ]),
        // -----------------------------------------------------------------
        // Additional Game Functions
        // -----------------------------------------------------------------
        NodeDefinition::new(
            NodeType::ShowBlackscreen,
            "Game Functions: Additional",
            "黑屏过渡",
            "显示全屏黑屏/颜色覆盖层",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![
            p_req("color", "颜色", ParamType::Color),
            p_opt("delay", "延迟", ParamType::Number),
            p_opt("fadein", "淡入", ParamType::Number),
            p_opt("duration", "持续", ParamType::Number),
            p_opt("fadeout", "淡出", ParamType::Number),
        ]),
        NodeDefinition::new(
            NodeType::GetSnapshotData,
            "Game Functions: Additional",
            "获取快照数据",
            "获取快照元数据",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_data", PortType::List, "数据"),
        ])
        .with_params(vec![p_req("imageRef", "图像引用", ParamType::String)]),
        NodeDefinition::new(
            NodeType::GetAllSnapshots,
            "Game Functions: Additional",
            "获取所有快照",
            "获取所有快照引用",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_list", PortType::List, "快照列表"),
        ])
        .with_params(vec![
            p_opt("deleted", "包含已删除", ParamType::Boolean),
            p_opt("hidden", "包含隐藏", ParamType::Boolean),
        ]),
        NodeDefinition::new(
            NodeType::DeleteSnapshot,
            "Game Functions: Additional",
            "删除快照",
            "标记删除快照",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![p_req("imageRef", "图像引用", ParamType::String)]),
        NodeDefinition::new(
            NodeType::GetImageReference,
            "Game Functions: Additional",
            "获取图像引用",
            "从文件路径获取图片引用",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_ref", PortType::String, "引用"),
        ])
        .with_params(vec![p_req("filePath", "文件路径", ParamType::String)]),
        // -----------------------------------------------------------------
        // Graphics
        // -----------------------------------------------------------------
        NodeDefinition::new(
            NodeType::SetGraphicsOption,
            "Graphics",
            "设置图形选项",
            "设置图形选项",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![out_flow()])
        .with_params(vec![
            e("option", "选项", GRAPHICS_OPTIONS),
            p_req("value", "值", ParamType::List),
        ]),
        NodeDefinition::new(
            NodeType::GetGraphicsOption,
            "Graphics",
            "获取图形选项",
            "获取图形选项值",
            GAME_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_value", PortType::List, "值"),
        ])
        .with_params(vec![e("option", "选项", GRAPHICS_OPTIONS)]),
        // -----------------------------------------------------------------
        // Math: Standard
        // -----------------------------------------------------------------
        NodeDefinition::new(NodeType::Random, "Math", "随机数", "随机浮点数", MATH_COLOR)
            .with_inputs(vec![in_flow()])
            .with_outputs(vec![
                out_flow(),
                out_data("out_value", PortType::Number, "值"),
            ])
            .with_params(vec![
                p_req("min", "最小值", ParamType::Number),
                p_req("max", "最大值", ParamType::Number),
            ]),
        NodeDefinition::new(
            NodeType::RandomInt,
            "Math",
            "随机整数",
            "随机整数",
            MATH_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_value", PortType::Number, "值"),
        ])
        .with_params(vec![
            p_req("min", "最小值", ParamType::Number),
            p_req("max", "最大值", ParamType::Number),
        ]),
        NodeDefinition::new(NodeType::Sin, "Math", "正弦", "正弦值", MATH_COLOR)
            .with_inputs(vec![in_flow()])
            .with_outputs(vec![
                out_flow(),
                out_data("out_value", PortType::Number, "值"),
            ])
            .with_params(vec![p_req("angle", "角度", ParamType::Number)]),
        NodeDefinition::new(NodeType::Cos, "Math", "余弦", "余弦值", MATH_COLOR)
            .with_inputs(vec![in_flow()])
            .with_outputs(vec![
                out_flow(),
                out_data("out_value", PortType::Number, "值"),
            ])
            .with_params(vec![p_req("angle", "角度", ParamType::Number)]),
        NodeDefinition::new(NodeType::Tan, "Math", "正切", "正切值", MATH_COLOR)
            .with_inputs(vec![in_flow()])
            .with_outputs(vec![
                out_flow(),
                out_data("out_value", PortType::Number, "值"),
            ])
            .with_params(vec![p_req("angle", "角度", ParamType::Number)]),
        NodeDefinition::new(NodeType::Asin, "Math", "反正弦", "反正弦", MATH_COLOR)
            .with_inputs(vec![in_flow()])
            .with_outputs(vec![
                out_flow(),
                out_data("out_value", PortType::Number, "值"),
            ])
            .with_params(vec![p_req("value", "值", ParamType::Number)]),
        NodeDefinition::new(NodeType::Acos, "Math", "反余弦", "反余弦", MATH_COLOR)
            .with_inputs(vec![in_flow()])
            .with_outputs(vec![
                out_flow(),
                out_data("out_value", PortType::Number, "值"),
            ])
            .with_params(vec![p_req("value", "值", ParamType::Number)]),
        NodeDefinition::new(NodeType::Atan, "Math", "反正切", "反正切", MATH_COLOR)
            .with_inputs(vec![in_flow()])
            .with_outputs(vec![
                out_flow(),
                out_data("out_value", PortType::Number, "值"),
            ])
            .with_params(vec![p_req("value", "值", ParamType::Number)]),
        NodeDefinition::new(NodeType::Floor, "Math", "向下取整", "向下取整", MATH_COLOR)
            .with_inputs(vec![in_flow()])
            .with_outputs(vec![
                out_flow(),
                out_data("out_value", PortType::Number, "值"),
            ])
            .with_params(vec![p_req("value", "值", ParamType::Number)]),
        NodeDefinition::new(NodeType::Ceil, "Math", "向上取整", "向上取整", MATH_COLOR)
            .with_inputs(vec![in_flow()])
            .with_outputs(vec![
                out_flow(),
                out_data("out_value", PortType::Number, "值"),
            ])
            .with_params(vec![p_req("value", "值", ParamType::Number)]),
        NodeDefinition::new(NodeType::Round, "Math", "四舍五入", "四舍五入", MATH_COLOR)
            .with_inputs(vec![in_flow()])
            .with_outputs(vec![
                out_flow(),
                out_data("out_value", PortType::Number, "值"),
            ])
            .with_params(vec![p_req("value", "值", ParamType::Number)]),
        NodeDefinition::new(NodeType::Trunc, "Math", "截断", "截断小数部分", MATH_COLOR)
            .with_inputs(vec![in_flow()])
            .with_outputs(vec![
                out_flow(),
                out_data("out_value", PortType::Number, "值"),
            ])
            .with_params(vec![p_req("value", "值", ParamType::Number)]),
        NodeDefinition::new(NodeType::Sign, "Math", "符号", "数值符号", MATH_COLOR)
            .with_inputs(vec![in_flow()])
            .with_outputs(vec![
                out_flow(),
                out_data("out_value", PortType::Number, "值"),
            ])
            .with_params(vec![p_req("value", "值", ParamType::Number)]),
        NodeDefinition::new(NodeType::Abs, "Math", "绝对值", "绝对值", MATH_COLOR)
            .with_inputs(vec![in_flow()])
            .with_outputs(vec![
                out_flow(),
                out_data("out_value", PortType::Number, "值"),
            ])
            .with_params(vec![p_req("value", "值", ParamType::Number)]),
        NodeDefinition::new(NodeType::LogN, "Math", "自然对数", "自然对数", MATH_COLOR)
            .with_inputs(vec![in_flow()])
            .with_outputs(vec![
                out_flow(),
                out_data("out_value", PortType::Number, "值"),
            ])
            .with_params(vec![p_req("value", "值", ParamType::Number)]),
        NodeDefinition::new(
            NodeType::Log2,
            "Math",
            "Log2",
            "以 2 为底的对数",
            MATH_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_value", PortType::Number, "值"),
        ])
        .with_params(vec![p_req("value", "值", ParamType::Number)]),
        NodeDefinition::new(
            NodeType::Log10,
            "Math",
            "Log10",
            "以 10 为底的对数",
            MATH_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_value", PortType::Number, "值"),
        ])
        .with_params(vec![p_req("value", "值", ParamType::Number)]),
        NodeDefinition::new(
            NodeType::Min,
            "Math",
            "最小值",
            "一组数字中的最小值",
            MATH_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_value", PortType::Number, "值"),
        ])
        .with_params(vec![p_req("values", "数值列表", ParamType::List)]),
        NodeDefinition::new(
            NodeType::Max,
            "Math",
            "最大值",
            "一组数字中的最大值",
            MATH_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_value", PortType::Number, "值"),
        ])
        .with_params(vec![p_req("values", "数值列表", ParamType::List)]),
        // -----------------------------------------------------------------
        // Math: Vector
        // -----------------------------------------------------------------
        NodeDefinition::new(
            NodeType::Vector,
            "Math: Vector",
            "向量",
            "创建 3D 向量",
            MATH_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_vector", PortType::List, "向量"),
        ])
        .with_params(vec![
            p_req("x", "X", ParamType::Number),
            p_req("y", "Y", ParamType::Number),
            p_req("z", "Z", ParamType::Number),
        ]),
        NodeDefinition::new(
            NodeType::Quaternion,
            "Math: Vector",
            "四元数",
            "创建四元数",
            MATH_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_quaternion", PortType::List, "四元数"),
        ])
        .with_params(vec![
            p_req("rx", "RX", ParamType::Number),
            p_req("ry", "RY", ParamType::Number),
            p_req("rz", "RZ", ParamType::Number),
            p_req("rw", "RW", ParamType::Number),
        ]),
        NodeDefinition::new(
            NodeType::Vector3Length,
            "Math: Vector",
            "向量长度",
            "向量长度",
            MATH_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_length", PortType::Number, "长度"),
        ])
        .with_params(vec![p_req("v", "向量", ParamType::List)]),
        NodeDefinition::new(
            NodeType::Vector3SqrLength,
            "Math: Vector",
            "向量长度平方",
            "向量长度平方",
            MATH_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_length", PortType::Number, "长度"),
        ])
        .with_params(vec![p_req("v", "向量", ParamType::List)]),
        NodeDefinition::new(
            NodeType::Vector3Add,
            "Math: Vector",
            "向量加",
            "向量相加",
            MATH_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_vector", PortType::List, "向量"),
        ])
        .with_params(vec![
            p_req("v1", "向量 1", ParamType::List),
            p_req("v2", "向量 2", ParamType::List),
        ]),
        NodeDefinition::new(
            NodeType::Vector3Sub,
            "Math: Vector",
            "向量减",
            "向量相减",
            MATH_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_vector", PortType::List, "向量"),
        ])
        .with_params(vec![
            p_req("v1", "向量 1", ParamType::List),
            p_req("v2", "向量 2", ParamType::List),
        ]),
        NodeDefinition::new(
            NodeType::Vector3Scale,
            "Math: Vector",
            "向量缩放",
            "向量缩放",
            MATH_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_vector", PortType::List, "向量"),
        ])
        .with_params(vec![
            p_req("v", "向量", ParamType::List),
            p_req("scalar", "缩放", ParamType::Number),
        ]),
        NodeDefinition::new(
            NodeType::Vector3Dot,
            "Math: Vector",
            "向量点积",
            "向量点积",
            MATH_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_value", PortType::Number, "值"),
        ])
        .with_params(vec![
            p_req("v1", "向量 1", ParamType::List),
            p_req("v2", "向量 2", ParamType::List),
        ]),
        NodeDefinition::new(
            NodeType::Vector3Cross,
            "Math: Vector",
            "向量叉积",
            "向量叉积",
            MATH_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_vector", PortType::List, "向量"),
        ])
        .with_params(vec![
            p_req("v1", "向量 1", ParamType::List),
            p_req("v2", "向量 2", ParamType::List),
        ]),
        NodeDefinition::new(
            NodeType::Vector3Rotate,
            "Math: Vector",
            "旋转向量",
            "用四元数旋转向量",
            MATH_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_vector", PortType::List, "向量"),
        ])
        .with_params(vec![
            p_req("q", "四元数", ParamType::List),
            p_req("v", "向量", ParamType::List),
        ]),
        NodeDefinition::new(
            NodeType::Vector3Distance,
            "Math: Vector",
            "向量距离",
            "两个向量之间的距离",
            MATH_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_distance", PortType::Number, "距离"),
        ])
        .with_params(vec![
            p_req("v1", "向量 1", ParamType::List),
            p_req("v2", "向量 2", ParamType::List),
        ]),
        // -----------------------------------------------------------------
        // String
        // -----------------------------------------------------------------
        NodeDefinition::new(
            NodeType::Length,
            "String",
            "字符串长度",
            "字符串长度",
            STRING_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_length", PortType::Number, "长度"),
        ])
        .with_params(vec![p_req("s", "字符串", ParamType::String)]),
        NodeDefinition::new(
            NodeType::Lower,
            "String",
            "转小写",
            "转换为小写",
            STRING_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_string", PortType::String, "字符串"),
        ])
        .with_params(vec![p_req("s", "字符串", ParamType::String)]),
        NodeDefinition::new(
            NodeType::Upper,
            "String",
            "转大写",
            "转换为大写",
            STRING_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_string", PortType::String, "字符串"),
        ])
        .with_params(vec![p_req("s", "字符串", ParamType::String)]),
        NodeDefinition::new(
            NodeType::Find,
            "String",
            "查找",
            "查找子串索引",
            STRING_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_index", PortType::Number, "索引"),
        ])
        .with_params(vec![
            p_req("sub", "子串", ParamType::String),
            p_req("s", "字符串", ParamType::String),
        ]),
        NodeDefinition::new(
            NodeType::SubString,
            "String",
            "截取",
            "提取子串",
            STRING_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_string", PortType::String, "字符串"),
        ])
        .with_params(vec![
            p_req("s", "字符串", ParamType::String),
            p_opt("start", "起始", ParamType::Number),
            p_opt("end", "结束", ParamType::Number),
            p_opt("length", "长度", ParamType::Number),
        ]),
        NodeDefinition::new(
            NodeType::Format,
            "String",
            "格式化",
            "格式化字符串",
            STRING_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_string", PortType::String, "字符串"),
        ])
        .with_params(vec![
            p_req("fmt", "格式", ParamType::String),
            p_opt("params", "参数", ParamType::Object),
        ]),
        NodeDefinition::new(
            NodeType::ToNumber,
            "String",
            "转数字",
            "字符串转数字",
            STRING_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_value", PortType::Number, "值"),
        ])
        .with_params(vec![p_req("s", "字符串", ParamType::String)]),
        // -----------------------------------------------------------------
        // File
        // -----------------------------------------------------------------
        NodeDefinition::new(
            NodeType::FileExists,
            "File",
            "文件存在",
            "文件是否存在",
            FILE_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_exists", PortType::Boolean, "是否存在"),
        ])
        .with_params(vec![p_req("path", "路径", ParamType::String)]),
        NodeDefinition::new(
            NodeType::GetFiles,
            "File",
            "获取文件",
            "获取文件列表",
            FILE_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_files", PortType::List, "文件列表"),
        ])
        .with_params(vec![
            p_req("path", "路径", ParamType::String),
            p_opt("subfolders", "包含子文件夹", ParamType::Boolean),
        ]),
        NodeDefinition::new(
            NodeType::GetFileExtension,
            "File",
            "获取扩展名",
            "获取文件扩展名",
            FILE_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_ext", PortType::String, "扩展名"),
        ])
        .with_params(vec![p_req("path", "路径", ParamType::String)]),
        // -----------------------------------------------------------------
        // Objects
        // -----------------------------------------------------------------
        NodeDefinition::new(
            NodeType::CreateList,
            "Objects",
            "创建列表",
            "创建列表",
            OBJECT_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_list", PortType::List, "列表"),
        ])
        .with_params(vec![p_opt("keyValues", "键值", ParamType::Object)]),
        NodeDefinition::new(
            NodeType::Copy,
            "Objects",
            "复制列表",
            "复制列表",
            OBJECT_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_list", PortType::List, "列表"),
        ])
        .with_params(vec![
            p_req("list", "列表", ParamType::List),
            p_opt("deepCopy", "深拷贝", ParamType::Boolean),
        ]),
        NodeDefinition::new(
            NodeType::CreateListFromJson,
            "Objects",
            "从 JSON 创建列表",
            "从 JSON 文件创建列表",
            OBJECT_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_list", PortType::List, "列表"),
        ])
        .with_params(vec![p_req("file", "文件", ParamType::String)]),
        NodeDefinition::new(
            NodeType::CreateThread,
            "Objects",
            "创建线程",
            "创建线程",
            OBJECT_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_thread", PortType::Object, "线程"),
        ])
        .with_params(vec![
            p_req("labelName", "标签名", ParamType::String),
            p_opt("params", "参数", ParamType::Object),
        ]),
        NodeDefinition::new(
            NodeType::CreateListener,
            "Objects",
            "创建监听器",
            "创建监听器（父作用域）",
            WAIT_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_listener", PortType::Object, "监听器"),
        ])
        .with_params(vec![
            p_req("labelName", "标签名", ParamType::String),
            p_opt("params", "参数", ParamType::Object),
        ]),
        NodeDefinition::new(
            NodeType::CreateListenerLocal,
            "Objects",
            "创建局部监听器",
            "创建监听器（当前作用域）",
            WAIT_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_listener", PortType::Object, "监听器"),
        ])
        .with_params(vec![
            p_req("labelName", "标签名", ParamType::String),
            p_opt("params", "参数", ParamType::Object),
        ]),
        NodeDefinition::new(
            NodeType::CreateMissionPanel,
            "Objects",
            "任务面板",
            "创建任务面板",
            OBJECT_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_panel", PortType::Object, "面板"),
        ]),
        NodeDefinition::new(
            NodeType::CreateMissionMenuItem,
            "Objects",
            "任务菜单项",
            "创建任务菜单项",
            OBJECT_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_item", PortType::Object, "菜单项"),
        ]),
        NodeDefinition::new(
            NodeType::CreateArea,
            "Objects",
            "创建区域",
            "创建区域",
            OBJECT_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_area", PortType::Object, "区域"),
        ])
        .with_params(vec![
            p_req("type", "类型", ParamType::String),
            e("stage", "场景", STAGE_TYPES),
            p_req("position", "位置", ParamType::Vector),
            p_req("r", "半径", ParamType::Number),
            p_req("h", "高度", ParamType::Number),
            p_opt("outline", "轮廓", ParamType::Boolean),
            p_opt("compass", "指南针", ParamType::String),
        ]),
        NodeDefinition::new(
            NodeType::CreateZone,
            "Objects",
            "创建地带",
            "创建地带（多区域组合）",
            OBJECT_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_zone", PortType::Object, "地带"),
        ])
        .with_params(vec![p_req("areas", "区域列表", ParamType::List)]),
        NodeDefinition::new(
            NodeType::CreateCondition,
            "Objects",
            "创建条件",
            "创建条件对象",
            OBJECT_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_condition", PortType::Object, "条件"),
        ])
        .with_params(vec![
            e("condition", "条件", CONDITION_TYPES),
            p_req("id", "ID", ParamType::String),
        ]),
        NodeDefinition::new(
            NodeType::CreateItemCondition,
            "Objects",
            "物品条件",
            "创建物品条件",
            OBJECT_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_condition", PortType::Object, "条件"),
        ])
        .with_params(vec![
            e("itemtype", "物品类型", DROP_ITEM_TYPES),
            p_opt("zone", "地带", ParamType::Object),
            p_opt("area", "区域", ParamType::Object),
            p_req("id", "ID", ParamType::String),
        ]),
        NodeDefinition::new(
            NodeType::CreateInteractArea,
            "Objects",
            "创建交互区域",
            "创建交互区域",
            OBJECT_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_area", PortType::Object, "区域"),
        ])
        .with_params(vec![
            e("stage", "场景", STAGE_TYPES),
            p_req("position", "位置", ParamType::Vector),
            p_req("r", "半径", ParamType::Number),
            p_req("text", "文本", ParamType::String),
            p_req("options", "选项", ParamType::List),
        ]),
        NodeDefinition::new(
            NodeType::CreateText,
            "Objects",
            "创建文本",
            "创建文本对象",
            OBJECT_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_text", PortType::Object, "文本"),
        ]),
        NodeDefinition::new(
            NodeType::CreateMessengerChat,
            "Objects",
            "创建聊天",
            "创建即时通讯聊天",
            OBJECT_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_chat", PortType::Object, "聊天"),
        ])
        .with_params(vec![
            p_req("title", "标题", ParamType::String),
            p_opt("iconText", "图标文本", ParamType::String),
            p_opt("iconTextColor", "图标文本颜色", ParamType::Color),
            p_opt("iconColor", "图标颜色", ParamType::Color),
            p_opt("iconFilename", "图标文件名", ParamType::String),
        ]),
        NodeDefinition::new(
            NodeType::CreateAudio,
            "Objects",
            "创建音频",
            "创建音频源",
            OBJECT_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_audio", PortType::Object, "音频"),
        ])
        .with_params(vec![p_req("filePath", "文件路径", ParamType::String)]),
        NodeDefinition::new(
            NodeType::CreateGallery,
            "Objects",
            "创建图库",
            "创建图库",
            OBJECT_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_gallery", PortType::Object, "图库"),
        ])
        .with_params(vec![
            p_req("callback", "回调", ParamType::String),
            p_opt("condition", "条件", ParamType::Object),
            p_opt("area", "区域", ParamType::Object),
            p_opt("zone", "地带", ParamType::Object),
        ]),
        NodeDefinition::new(
            NodeType::CreateSnapshot,
            "Objects",
            "创建快照相机",
            "创建快照相机",
            OBJECT_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_snapshot", PortType::Object, "快照相机"),
        ])
        .with_params(vec![
            p_req("position", "位置", ParamType::Vector),
            p_req("direction", "方向", ParamType::Vector),
            p_req("width", "宽度", ParamType::Number),
            p_req("height", "高度", ParamType::Number),
            p_req("fov", "视野", ParamType::Number),
        ]),
        NodeDefinition::new(
            NodeType::CreateNPC,
            "Objects",
            "创建 NPC",
            "创建或连接 NPC",
            OBJECT_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_npc", PortType::Object, "NPC"),
        ])
        .with_params(vec![
            p_req("avatarType", "外观类型", ParamType::String),
            p_req("position", "位置", ParamType::Vector),
            p_opt("rotation", "旋转", ParamType::Quaternion),
            p_opt("body", "身体", ParamType::Number),
            p_opt("hair", "发型", ParamType::Number),
            p_opt("face", "面容", ParamType::Number),
            p_opt("size", "大小", ParamType::Number),
            p_opt("id", "ID", ParamType::Number),
        ]),
        NodeDefinition::new(
            NodeType::CreateInput,
            "Objects",
            "创建输入",
            "创建输入检测",
            OBJECT_COLOR,
        )
        .with_inputs(vec![in_flow()])
        .with_outputs(vec![
            out_flow(),
            out_data("out_input", PortType::Object, "输入"),
        ])
        .with_params(vec![
            p_req("button", "按钮", ParamType::String),
            p_opt("modifier", "修饰键", ParamType::String),
            p_opt("interaction", "交互", ParamType::String),
        ]),
        // -----------------------------------------------------------------
        // Special
        // -----------------------------------------------------------------
        NodeDefinition::new(
            NodeType::Meta,
            "Special",
            "元数据",
            "任务元数据",
            SPECIAL_COLOR,
        )
        .with_params(vec![
            p_opt("title", "标题", ParamType::Object),
            p_opt("description", "描述", ParamType::Object),
            p_opt("settings", "设置", ParamType::List),
        ]),
        NodeDefinition::new(
            NodeType::Comment,
            "Special",
            "注释",
            "注释节点",
            SPECIAL_COLOR,
        )
        .with_params(vec![p_opt("text", "文本", ParamType::String)]),
        NodeDefinition::new(
            NodeType::Group,
            "Special",
            "分组",
            "可视化分组框",
            SPECIAL_COLOR,
        )
        .with_params(vec![
            p_opt("title", "标题", ParamType::String),
            p_opt("color", "颜色", ParamType::Color),
        ]),
        // ── Phase 6: Data-only 布尔/条件节点 ──
        // 纯数据输出，无 Flow 端口。通过 DataFlow 连入 If/While 的 condition。
        NodeDefinition::new(
            NodeType::Boolean,
            "Math",
            "布尔值",
            "输出布尔常量 true 或 false",
            MATH_COLOR,
        )
        .with_outputs(vec![out_data("out_value", PortType::Boolean, "布尔值")])
        .with_params(vec![e("value", "值", &["true", "false"])]),
        NodeDefinition::new(
            NodeType::GetStateBool,
            "Game Functions: Player",
            "读取布尔状态",
            "读取 _state 中任意布尔变量",
            GAME_COLOR,
        )
        .with_outputs(vec![out_data(
            "out_value",
            PortType::Boolean,
            "状态值",
        )])
        .with_params(vec![e(
            "stateKey",
            "状态键",
            &[
                "Futanari", "Sitting", "Orgasm", "Moving", "Crouching",
                "Peeing", "Dashing", "InLight", "NearNPC", "Watched",
                "ShowingOff", "Bukkake", "Blindfolded", "Invisible",
                "InOpenToilet", "FPCamera", "IsDayTime", "GameOver",
            ],
        )]),
        NodeDefinition::new(
            NodeType::GetStateNumber,
            "Game Functions: Player",
            "读取数值状态",
            "读取 _state 中任意数值变量",
            GAME_COLOR,
        )
        .with_outputs(vec![out_data(
            "out_value",
            PortType::Number,
            "状态值",
        )])
        .with_params(vec![e(
            "stateKey",
            "状态键",
            &[
                "Ecstasy", "Detection", "Rank", "HeartRate",
                "Stamina", "StaminaMax", "Moisture", "Bodypaint",
            ],
        )]),
        NodeDefinition::new(
            NodeType::CompareNumbers,
            "Math",
            "数值比较",
            "比较两个数值（>=、==、!=、>、<、<=）",
            MATH_COLOR,
        )
        .with_inputs(vec![
            PortDefinition::new("a", PortType::Number, "数值A").required(true),
            PortDefinition::new("b", PortType::Number, "数值B").required(true),
        ])
        .with_outputs(vec![out_data(
            "out_result",
            PortType::Boolean,
            "比较结果",
        )])
        .with_params(vec![
            p_req("a", "数值A", ParamType::Number),
            p_req("b", "数值B", ParamType::Number),
            e("operator", "操作符", &[">=", "==", "!=", ">", "<", "<="]),
        ]),
        NodeDefinition::new(
            NodeType::LogicAnd,
            "Math",
            "逻辑与",
            "两个布尔值的逻辑与（&&）",
            MATH_COLOR,
        )
        .with_inputs(vec![
            PortDefinition::new("a", PortType::Boolean, "输入A").required(true),
            PortDefinition::new("b", PortType::Boolean, "输入B").required(true),
        ])
        .with_outputs(vec![out_data(
            "out_result",
            PortType::Boolean,
            "结果",
        )]),
        NodeDefinition::new(
            NodeType::LogicOr,
            "Math",
            "逻辑或",
            "两个布尔值的逻辑或（||）",
            MATH_COLOR,
        )
        .with_inputs(vec![
            PortDefinition::new("a", PortType::Boolean, "输入A").required(true),
            PortDefinition::new("b", PortType::Boolean, "输入B").required(true),
        ])
        .with_outputs(vec![out_data(
            "out_result",
            PortType::Boolean,
            "结果",
        )]),
        NodeDefinition::new(
            NodeType::LogicNot,
            "Math",
            "逻辑非",
            "布尔值的逻辑取反（!）",
            MATH_COLOR,
        )
        .with_inputs(vec![
            PortDefinition::new("a", PortType::Boolean, "输入").required(true),
        ])
        .with_outputs(vec![out_data(
            "out_result",
            PortType::Boolean,
            "结果",
        )]),
        // ── Phase 7: 坐标系统 ──
        NodeDefinition::new(NodeType::GetPosition, "Game Functions: Player", "坐标预设", "从预设坐标库选取位置", GAME_COLOR)
            .with_outputs(vec![out_data("out_position", PortType::List, "坐标"), out_data("out_stage", PortType::String, "场景")])
            .with_params(vec![p_req("coord_id", "坐标ID", ParamType::String), e("stage", "场景", STAGE_TYPES), p_req("x", "X", ParamType::Number), p_req("y", "Y", ParamType::Number), p_req("z", "Z", ParamType::Number)]),
        NodeDefinition::new(NodeType::MakeVector, "Math: Vector", "构造向量", "x,y,z → Vector", MATH_COLOR)
            .with_inputs(vec![PortDefinition::new("x", PortType::Number, "X").required(true), PortDefinition::new("y", PortType::Number, "Y").required(true), PortDefinition::new("z", PortType::Number, "Z").required(true)])
            .with_outputs(vec![out_data("out_vec", PortType::List, "向量")])
            .with_params(vec![p_req("x", "X", ParamType::Number), p_req("y", "Y", ParamType::Number), p_req("z", "Z", ParamType::Number)]),
        NodeDefinition::new(
            NodeType::NumberConstant,
            "Math",
            "数值常量",
            "输出一个数值常量（如 0, 1, 90）",
            MATH_COLOR,
        )
        .with_outputs(vec![out_data("out_value", PortType::Number, "数值")])
        .with_params(vec![p_req("value", "值", ParamType::Number)]),
        NodeDefinition::new(NodeType::BreakVector, "Math: Vector", "拆分向量", "Vector → x,y,z", MATH_COLOR)
            .with_inputs(vec![PortDefinition::new("in_vec", PortType::List, "向量").required(true)])
            .with_outputs(vec![out_data("x", PortType::Number, "X"), out_data("y", PortType::Number, "Y"), out_data("z", PortType::Number, "Z")]),
        NodeDefinition::new(NodeType::CheckCondition, "Math", "检查条件", "条件对象→布尔值", MATH_COLOR)
            .with_inputs(vec![PortDefinition::new("cond", PortType::Object, "条件对象").required(true)])
            .with_outputs(vec![out_data("out_result", PortType::Boolean, "结果")]),
        NodeDefinition::new(NodeType::CheckEquipment, "Game Functions: Items", "检查装备", "是否装备指定玩具", GAME_COLOR)
            .with_outputs(vec![out_data("out_value", PortType::Boolean, "结果")])
            .with_params(vec![e("equipType", "装备类型", &["Handcuff","KeyHandcuff","TimerHandcuff","Vibrator","EyeMask","TitRotor","KuriRotor","PistonAnal","PistonPussy","AnalPlug"])]),
        NodeDefinition::new(NodeType::CheckCosplay, "Game Functions: Items", "检查服装", "是否穿着指定服装", GAME_COLOR)
            .with_outputs(vec![out_data("out_value", PortType::Boolean, "结果")])
            .with_params(vec![p_req("cosplayKey", "服装键", ParamType::String)]),
        NodeDefinition::new(NodeType::ForeachNode, "Flow", "Foreach", "遍历列表，每元素调用标签", WAIT_COLOR)
            .with_inputs(vec![in_flow()])
            .with_outputs(vec![out_flow()])
            .with_params(vec![p_req("list", "列表", ParamType::String), p_req("threadVar", "标签名", ParamType::String)]),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_variants_have_definition() {
        let all = all_definitions();
        assert_eq!(all.len(), 158);
        let mut seen = std::collections::HashSet::new();
        for definition in &all {
            assert!(
                seen.insert(definition.node_type.clone()),
                "duplicate definition for {:?}",
                definition.node_type
            );
        }
        assert_eq!(seen.len(), 158);
    }

    #[test]
    fn test_start_definition() {
        let all = all_definitions();
        let definition = find_definition(&all, NodeType::Start);
        assert!(definition.inputs.is_empty());
        assert_eq!(definition.outputs.len(), 1);
        assert_eq!(definition.outputs[0].port_type, PortType::Flow);
        assert!(definition.params.is_empty());
    }

    #[test]
    fn test_if_definition() {
        let all = all_definitions();
        let definition = find_definition(&all, NodeType::If);
        assert_eq!(definition.inputs.len(), 1);
        assert_eq!(definition.outputs.len(), 2);
        assert!(
            definition
                .outputs
                .iter()
                .any(|p| p.id == "out_true" && p.port_type == PortType::Flow)
        );
        assert!(
            definition
                .outputs
                .iter()
                .any(|p| p.id == "out_false" && p.port_type == PortType::Flow)
        );
    }

    #[test]
    fn test_color_param_type_to_port_type() {
        assert_eq!(ParamType::Color.port_type(), PortType::List);
        assert_eq!(ParamType::Vector.port_type(), PortType::List);
        assert_eq!(ParamType::Object.port_type(), PortType::Object);
    }

    fn find_definition<'a>(all: &'a [NodeDefinition], node_type: NodeType) -> &'a NodeDefinition {
        all.iter()
            .find(|d| d.node_type == node_type)
            .unwrap_or_else(|| panic!("missing definition for {:?}", node_type))
    }
}
