use crate::api::namespace::NamespaceRegistry;
use crate::api::coordinate::CoordinateRegistry;
use crate::api::registry::get_definition;
use crate::graph::container::LabelContainer;
use crate::graph::node::{Node, ParamValue};
use crate::graph::types::{NodeType, PortType};
use crate::ui::i18n::I18n;
use crate::ui::panels::namespace_picker::NamespacePickerState;
use crate::ui::panels::coordinate_picker::CoordinatePickerState;
use crate::api::definitions::ParamType;
use crate::ui::panels::param_text_edit::{EditBuffers, ParamTextEdit};

/// 属性面板。
pub struct PropertiesPanel;

impl PropertiesPanel {
    /// 显示选中节点的可编辑参数，返回发生变更的参数键值（如果有）。
    pub fn show(
        ui: &mut egui::Ui,
        i18n: &I18n,
        node: &Node,
        label: &LabelContainer,
        registry: &NamespaceRegistry,
        picker: &mut Option<NamespacePickerState>,
        coord: &CoordinateRegistry,
        coord_picker: &mut Option<CoordinatePickerState>,
        edit_bufs: &mut EditBuffers,
    ) -> Option<(String, ParamValue)> {
        // 节点标题 + 简介
        if let Some(def) = get_definition(node.node_type) {
            ui.heading(&def.display_name);
            ui.label(egui::RichText::new(format!("{:?}", node.node_type))
                .color(egui::Color32::from_gray(150))
                .size(11.0));
            ui.label(egui::RichText::new(&def.description)
                .color(egui::Color32::from_gray(130))
                .size(11.0));
        } else {
            ui.heading(i18n.text("panel.properties"));
            ui.label(i18n.format("label.type", &[&format!("{:?}", node.node_type)]));
        }
        ui.separator();

        let mut changed = None;
        for (key, value) in &node.params {
            let type_hint = param_type_label(node, key);
            // 纵向布局：标签在上，编辑器在下
            ui.vertical(|ui| {
                ui.label(format!("{} {}", key, type_hint));
                if let Some((new_key, new_value)) =
                    Self::param_editor(ui, i18n, node, label, registry, picker, coord, coord_picker, edit_bufs, key, value)
                {
                    changed = Some((new_key, new_value));
                }
            });
            ui.add_space(2.0);
            ui.separator();
        }

        if node.params.is_empty() {
            ui.label(i18n.text("label.none"));
        }

        ui.separator();
        ui.label(i18n.format(
            "label.position",
            &[&format!("{:.1}", node.position.x), &format!("{:.1}", node.position.y)],
        ));
        ui.label(i18n.format(
            "label.size",
            &[&format!("{:.0}", node.size.x), &format!("{:.0}", node.size.y)],
        ));

        changed
    }

    /// 为单个参数绘制合适的编辑控件。
    ///
    /// 参数现在支持两种数据源：
    /// - 字面量：直接在节点属性中编辑。
    /// - 数据端口：通过节点边框的 Data 端口连接或下拉框选择其他节点的输出。
    fn param_editor(
        ui: &mut egui::Ui,
        i18n: &I18n,
        node: &Node,
        label: &LabelContainer,
        registry: &NamespaceRegistry,
        picker: &mut Option<NamespacePickerState>,
        _coord: &CoordinateRegistry,
        coord_picker: &mut Option<CoordinatePickerState>,
        edit_bufs: &mut EditBuffers,
        key: &str,
        value: &ParamValue,
    ) -> Option<(String, ParamValue)> {
        // 如果该参数对应的 Data 输入端口已被连接，则只读显示来源。
        if let Some((src_node, src_port)) = connected_data_source(label, &node.id, key) {
            ui.label(format!("🔗 {}.{}", src_node, src_port));
            return None;
        }

        // 如果参数有命名空间选择器，显示选择按钮。
        if let Some((namespace, multi)) = namespace_for_param(node.node_type, key) {
            if registry.get(namespace).is_some() {
                let current = selected_keys_from_value(value);
                if ui.button(i18n.text("button.select")).clicked() {
                    *picker = Some(
                        NamespacePickerState::new(namespace, key, multi)
                            .with_selected(&current),
                    );
                }
                ui.label(i18n.format("label.selected_count", &[&current.len().to_string()]));
                return None;
            }
        }

        // If/While 条件模板下拉框.
        if (node.node_type == NodeType::If || node.node_type == NodeType::While)
            && key == "condition"
        {
            return Self::condition_template_editor(ui, key, value, i18n);
        }

        // Vector 参数：坐标预设选择器按钮
        if let Some(param_def) = get_definition(node.node_type)
            .and_then(|def| def.params.iter().find(|p| p.name == key))
        {
            if param_def.param_type == ParamType::Vector || param_def.param_type == ParamType::Quaternion {
                if ui.button("...").on_hover_text(i18n.text("tooltip.coord_picker")).clicked() {
                    *coord_picker = Some(CoordinatePickerState::new(key));
                }
            }
        }

        // GetPosition 节点：coord_id 字段也提供坐标选择器。
        if node.node_type == NodeType::GetPosition && key == "coord_id" {
            if ui.button(i18n.text("button.coord_select")).on_hover_text(i18n.text("tooltip.coord_picker")).clicked() {
                *coord_picker = Some(CoordinatePickerState::new("__getposition__"));
            }
        }

        // 如果参数有固定枚举选项，直接显示枚举下拉框。
        if let Some(param_def) = get_definition(node.node_type)
            .and_then(|def| def.params.iter().find(|p| p.name == key))
        {
            if let Some(options) = param_def.options.as_ref() {
                return Self::enum_editor(ui, key, value, options);
            }
        }

        // Object/List 类型无合适 Data 源，跳过 ComboBox 直通文本编辑器
        let skip_combo = matches!(value, ParamValue::Literal(v) if v.is_object() || v.is_array());
        if skip_combo {
            return Self::literal_editor(ui, key, value, &node.id, edit_bufs, i18n);
        }

        // 否则使用数据源选择器 + 字面量编辑器。
        Self::source_editor(ui, node, label, key, value, edit_bufs, i18n)
    }

    /// 枚举参数下拉框编辑器。
    fn enum_editor(
        ui: &mut egui::Ui,
        key: &str,
        value: &ParamValue,
        options: &[String],
    ) -> Option<(String, ParamValue)> {
        let selected = match value {
            ParamValue::Literal(v) => v.as_str().unwrap_or_default().to_string(),
            _ => String::new(),
        };

        let mut changed = None;
        egui::ComboBox::from_id_salt(format!("{}_enum", key))
            .width(120.0)
            .selected_text(selected.clone())
            .show_ui(ui, |ui| {
                for option in options {
                    if ui.selectable_label(selected == *option, option).clicked() {
                        changed = Some(option.clone());
                    }
                }
            });

        changed.map(|new_value| (key.to_string(), ParamValue::Literal(serde_json::json!(new_value))))
    }

    /// 数据源选择编辑器：下拉框选择字面量或数据端口引用。
    fn source_editor(
        ui: &mut egui::Ui,
        node: &Node,
        label: &LabelContainer,
        key: &str,
        value: &ParamValue,
        edit_bufs: &mut EditBuffers,
        i18n: &I18n,
    ) -> Option<(String, ParamValue)> {
        // 收集可选数据源：所有兼容类型的非 Flow 输出端口。
        let options = available_data_sources(label, node, key);

        let selected_label = match value {
            ParamValue::Ref { node, port } => format!("{}.{} (ref)", node, port),
            _ => i18n.text("label.literal"),
        };

        let mut picked_source = None;
        egui::ComboBox::from_id_salt(format!("{}_source", key))
            .width(100.0)
            .selected_text(&selected_label)
            .show_ui(ui, |ui| {
                if ui
                    .selectable_label(selected_label == i18n.text("label.literal"), i18n.text("label.literal"))
                    .clicked()
                {
                    picked_source = Some(ParamValue::Null);
                }
                for (label, source_value) in &options {
                    if ui
                        .selectable_label(label == &selected_label, label)
                        .clicked()
                    {
                        picked_source = Some(source_value.clone());
                    }
                }
            });

        if let Some(picked) = picked_source {
            return match picked {
                // 选择“字面量”时，如果原值已经是字面量则保留，否则回退到默认空字面量。
                ParamValue::Null => {
                    if matches!(value, ParamValue::Ref { .. }) {
                        Some((key.to_string(), infer_default_literal(node, key)))
                    } else {
                        None
                    }
                }
                source => Some((key.to_string(), source)),
            };
        }

        // 当选择“字面量”时，显示对应的原生编辑器。
        match value {
            ParamValue::Ref { .. } => None,
            _ => Self::literal_editor(ui, key, value, &node.id, edit_bufs, i18n),
        }
    }

    /// 字面量编辑器。
    fn literal_editor(
        ui: &mut egui::Ui,
        key: &str,
        value: &ParamValue,
        node_id: &str,
        edit_bufs: &mut EditBuffers,
        i18n: &I18n,
    ) -> Option<(String, ParamValue)> {
        match value {
            ParamValue::Literal(v) if v.is_boolean() => {
                let mut b = v.as_bool().unwrap_or(false);
                if ui.checkbox(&mut b, "").changed() {
                    return Some((key.to_string(), ParamValue::Literal(serde_json::json!(b))));
                }
                None
            }
            ParamValue::Literal(v) if v.is_number() => {
                let mut num = v.as_f64().unwrap_or(0.0);
                if ui.add(egui::DragValue::new(&mut num)).changed() {
                    return Some((key.to_string(), ParamValue::Literal(serde_json::json!(num))));
                }
                None
            }
            // Vector/Color：专用多字段编辑器
            ParamValue::Literal(v) if v.is_array() => {
                if let Some(arr) = v.as_array() {
                    if !arr.is_empty() {
                        if let Some(v2) = Self::vector_editor(ui, key, arr) {
                            return Some((key.to_string(), v2));
                        }
                    }
                }
                None
            }
            // 字符串 / Null / 空数组：文本编辑（统一组件）
            _ => {
                let hint = if matches!(value, ParamValue::Literal(v) if v.is_array()) {
                    i18n.text("hint.json_array")
                } else if matches!(value, ParamValue::Literal(v) if v.is_object()) {
                    i18n.text("hint.json_object")
                } else {
                    String::new()
                };
                ParamTextEdit::show(ui, &format!("{node_id}.{key}"), value, edit_bufs, &hint, i18n)
            }
        }
    }

    /// 多字段向量编辑器（Vector / Position / Quaternion / Color）。
    fn vector_editor(
        ui: &mut egui::Ui,
        _key: &str,
        arr: &[serde_json::Value],
    ) -> Option<ParamValue> {
        let mut changed = false;
        let mut new_arr: Vec<serde_json::Value> =
            arr.iter().map(|v| v.clone()).collect();
        let labels = if arr.len() == 4 {
            &["rx", "ry", "rz", "rw"][..]
        } else if arr.len() == 3 {
            &["x", "y", "z"][..]
        } else {
            &[]
        };
        ui.horizontal(|ui| {
            for (i, v) in new_arr.iter_mut().enumerate() {
                if let Some(n) = v.as_f64() {
                    let mut num = n;
                    let label = labels.get(i).map_or("", |s| s);
                    ui.label(label);
                    if ui.add(egui::DragValue::new(&mut num)).changed() {
                        *v = serde_json::json!(num);
                        changed = true;
                    }
                }
            }
        });
        if changed {
            Some(ParamValue::Literal(serde_json::json!(new_arr)))
        } else {
            None
        }
    }
}

/// 查找参数对应的 Data 输入端口是否已被连接。
///
/// 参数名与 Data 输入端口 ID 相同。当存在入边时返回源节点和端口。
fn connected_data_source(
    label: &LabelContainer,
    node_id: &str,
    param_name: &str,
) -> Option<(String, String)> {
    label
        .edges
        .values()
        .find(|e| e.to.node_id == node_id && e.to.port_id == param_name && e.edge_type != PortType::Flow)
        .map(|e| (e.from.node_id.clone(), e.from.port_id.clone()))
}

/// 收集当前图中可供参数引用的所有兼容数据输出。
fn available_data_sources(
    label: &LabelContainer,
    node: &Node,
    param_name: &str,
) -> Vec<(String, ParamValue)> {
    let expected_type = get_definition(node.node_type)
        .and_then(|def| def.params.iter().find(|p| p.name == param_name))
        .map(|p| p.param_type.port_type());

    let mut sources = Vec::new();
    for (other_id, other) in &label.nodes {
        if other_id == &node.id {
            continue;
        }
        for output in &other.outputs {
            if output.port_type == PortType::Flow {
                continue;
            }
            if let Some(expected) = expected_type.as_ref() {
                if !expected.is_compatible_with(&output.port_type) {
                    continue;
                }
            }
            sources.push((
                format!("{}.{} ({})", other_id, output.id, output.label),
                ParamValue::from_ref(other_id, &output.id),
            ));
        }
    }
    sources
}

/// 根据参数定义推断一个合理的默认字面量。
fn infer_default_literal(node: &Node, param_name: &str) -> ParamValue {
    get_definition(node.node_type)
        .and_then(|def| def.params.iter().find(|p| p.name == param_name))
        .map(|p| p.default_value())
        .unwrap_or(ParamValue::Null)
}

/// Maps a parameter to its namespace and whether it supports multiple selections.
fn namespace_for_param(node_type: NodeType, param_name: &str) -> Option<(&'static str, bool)> {
    match (node_type, param_name) {
        (NodeType::EquipCosplay | NodeType::UnequipCosplay | NodeType::OwnCosplay | NodeType::CheckCosplay, "cosplayKeys") => {
            Some(("cosplay", true))
        }
        _ => None,
    }
}

/// Extracts a list of selected keys from a literal JSON value.
fn selected_keys_from_value(value: &ParamValue) -> Vec<String> {
    match value {
        ParamValue::Literal(v) if v.is_array() => v
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|x| x.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default(),
        ParamValue::Literal(v) if v.is_string() => {
            v.as_str().map(|s| vec![s.to_string()]).unwrap_or_default()
        }
        _ => Vec::new(),
    }
}

/// If/While 条件模板编辑器：ComboBox 快速填充 + 文本框微调
impl PropertiesPanel {
    fn condition_template_editor(
        ui: &mut egui::Ui,
        key: &str,
        value: &ParamValue,
        i18n: &I18n,
    ) -> Option<(String, ParamValue)> {
        let current = match value {
            ParamValue::Literal(v) => v.as_str().unwrap_or_default().to_string(),
            _ => String::new(),
        };

        let mut picked = None;
        egui::ComboBox::from_id_salt("if_condition_template")
            .width(160.0)
            .selected_text(if current.is_empty() { i18n.text("label.select_template") } else { current.clone() })
            .show_ui(ui, |ui| {
                for &(label, expr) in IF_CONDITION_TEMPLATES {
                    if expr.is_empty() {
                        ui.separator();
                        ui.label(label);
                        continue;
                    }
                    if ui.selectable_label(false, label).clicked() {
                        picked = Some(expr.to_string());
                    }
                }
            });

        if let Some(expr) = picked {
            return Some((key.to_string(), ParamValue::Literal(serde_json::json!(expr))));
        }

        // 文本框允许手动微调
        let mut text = current;
        if ui.text_edit_singleline(&mut text).changed() {
            return Some((key.to_string(), ParamValue::Literal(serde_json::json!(text))));
        }
        None
    }
}

/// 条件模板列表
static IF_CONDITION_TEMPLATES: &[(&str, &str)] = &[
    ("✅ true", "true"),
    ("❌ false", "false"),
    ("── 角色状态 ──", ""),
    ("Futanari · 扶她", "_state.Futanari"),
    ("Sitting · 坐姿", "_state.Sitting"),
    ("Orgasm · 高潮", "_state.Orgasm"),
    ("Moving · 移动中", "_state.Moving"),
    ("Crouching · 蹲伏", "_state.Crouching"),
    ("Peeing · 排泄", "_state.Peeing"),
    ("Dashing · 奔跑", "_state.Dashing"),
    ("── 环境 ──", ""),
    ("InLight · 光照区", "_state.InLight"),
    ("NearNPC · 靠近NPC", "_state.NearNPC"),
    ("Watched · 被注视", "_state.Watched"),
    ("IsDayTime · 白天", "_state.IsDayTime"),
    ("FPCamera · 第一人称", "_state.FPCamera"),
    ("── 装备/拘束 ──", ""),
    ("蒙眼", "_state.Blindfolded"),
    ("隐身", "_state.Invisible"),
    ("有手铐", "_state.AdultToys.Handcuff != null"),
    ("无手铐", "_state.AdultToys.Handcuff == null"),
    ("有跳蛋", "_state.AdultToys.Vibrator != null"),
    ("── 数值比较 ──", ""),
    ("身体涂鸦 > 0", "_state.Bodypaint > 0"),
    ("快感 ≥", "_state.Ecstasy >= "),
    ("侦测 ≥", "_state.Detection >= "),
    ("体力 ≥", "_state.Stamina >= "),
    ("等级 ≥", "_state.Rank >= "),
    ("湿润度 ≥", "_state.Moisture >= "),
    ("心率 ≥", "_state.HeartRate >= "),
];

/// 返回参数类型简短标签，显示在参数名旁边帮助用户理解预期格式。
fn param_type_label(node: &Node, param_name: &str) -> String {
    use crate::api::registry::get_definition;
use crate::api::definitions::ParamType;
    let pt = get_definition(node.node_type)
        .and_then(|d| d.params.iter().find(|p| p.name == param_name))
        .map(|p| p.param_type);
    match pt {
        Some(ParamType::Boolean) => "[bool]".into(),
        Some(ParamType::String) => "[str]".into(),
        Some(ParamType::Enum) => "[enum]".into(),
        Some(ParamType::Number) => "[num]".into(),
        Some(ParamType::List) => "[list]".into(),
        Some(ParamType::Object) => "[obj]".into(),
        Some(ParamType::Vector) => "[xyz]".into(),
        Some(ParamType::Quaternion) => "[xyzw]".into(),
        Some(ParamType::Color) => "[rgb]".into(),
        None => "".into(),
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::edge::{Edge, EdgeEndpoint};
    use crate::graph::container::LabelContainer;
    use crate::graph::node::{Node, ParamValue, Port, Vec2};
    use crate::graph::types::{NodeType, PortType};

    fn make_node_with_data_output(id: &str, node_type: NodeType) -> Node {
        let mut node = Node::new(node_type, Vec2::ZERO);
        node.id = id.to_string();
        node.inputs = vec![Port::new("in_flow", PortType::Flow, "执行")];
        node.outputs = vec![
            Port::new("out_flow", PortType::Flow, "下一步"),
            Port::new("out_value", PortType::Number, "值"),
        ];
        node
    }

    #[test]
    fn test_connected_data_source_finds_data_edge() {
        let mut label = LabelContainer::default();
        let n1 = make_node_with_data_output("n1", NodeType::Random);
        let mut n2 = Node::new(NodeType::Log, Vec2::ZERO);
        n2.id = "n2".to_string();
        n2.inputs = vec![
            Port::new("in_flow", PortType::Flow, "执行"),
            Port::new("output", PortType::String, "输出"),
        ];
        n2.set_param("output", ParamValue::Literal(serde_json::json!("")));
        label.nodes.insert(n1.id.clone(), n1);
        label.nodes.insert(n2.id.clone(), n2);

        let edge = Edge::new(
            EdgeEndpoint::new("n1", "out_value"),
            EdgeEndpoint::new("n2", "output"),
            PortType::Number,
        );
        label.edges.insert(edge.id.clone(), edge);

        assert_eq!(
            super::connected_data_source(&label, "n2", "output"),
            Some(("n1".to_string(), "out_value".to_string()))
        );
        assert_eq!(super::connected_data_source(&label, "n1", "output"), None);
    }

    #[test]
    fn test_available_data_sources_filters_flow_and_compatible() {
        let mut label = LabelContainer::default();
        let n1 = make_node_with_data_output("n1", NodeType::Random);
        let mut n2 = Node::new(NodeType::SetEcstasy, Vec2::ZERO);
        n2.id = "n2".to_string();
        n2.inputs = vec![
            Port::new("in_flow", PortType::Flow, "执行"),
            Port::new("value", PortType::Number, "数值"),
        ];
        n2.set_param("value", ParamValue::Literal(serde_json::json!(0.0)));
        label.nodes.insert(n1.id.clone(), n1);

        let sources = super::available_data_sources(&label, &n2, "value");
        label.nodes.insert(n2.id.clone(), n2);

        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0].1, ParamValue::from_ref("n1", "out_value"));
    }
}
