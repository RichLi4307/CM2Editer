use crate::api::coordinate::CoordinateRegistry;
use crate::api::definitions::ParamType;
use crate::api::namespace::NamespaceRegistry;
use crate::api::registry::get_definition;
use crate::graph::container::LabelContainer;
use crate::graph::node::{Node, ParamValue};
use crate::graph::types::{DynamicPortTemplate, NodeType, PortType};
use crate::ui::i18n::I18n;
use crate::ui::panels::namespace_picker::NamespacePickerState;
use crate::ui::panels::coordinate_picker::CoordinatePickerState;
use crate::ui::panels::condition_editor::ConditionEditorState;
use crate::ui::panels::param_text_edit::{EditBuffers, ParamTextEdit};
use std::collections::HashMap;

/// 属性面板可能触发的动作。
///
/// 属性面板不直接操作图，而是把动作返回给 `App`，由 `App` 转换为可撤销命令。
#[derive(Debug, Clone)]
pub enum PropertiesPanelAction {
    /// 修改单个参数
    SetParam { key: String, value: ParamValue },
    /// 同时修改多个参数（如 CallMethod 选择方法时同时设置方法名与参数模板）
    SetParams { values: HashMap<String, ParamValue> },
    /// 添加动态端口/参数
    AddDynamicPort { group_id: String },
    /// 删除动态端口/参数
    RemoveDynamicPort { group_id: String, port_id: String },
}

/// 属性面板。
pub struct PropertiesPanel;

impl PropertiesPanel {
    /// 显示选中节点的可编辑参数，返回发生变更的动作列表（可能为空）。
    #[allow(clippy::too_many_arguments)]
    pub fn show(
        ui: &mut egui::Ui,
        i18n: &I18n,
        node: &Node,
        label: &LabelContainer,
        registry: &NamespaceRegistry,
        picker: &mut Option<NamespacePickerState>,
        coord: &CoordinateRegistry,
        coord_picker: &mut Option<CoordinatePickerState>,
        condition_editor: &mut Option<ConditionEditorState>,
        edit_bufs: &mut EditBuffers,
    ) -> Vec<PropertiesPanelAction> {
        let mut actions = Vec::new();
        // 节点标题 + 简介
        if let Some(_def) = get_definition(node.node_type) {
            let display_name = i18n.node_display_name(node.node_type);
            let description = i18n.node_description(node.node_type);
            ui.heading(display_name);
            ui.label(egui::RichText::new(format!("{:?}", node.node_type))
                .color(egui::Color32::from_gray(150))
                .size(11.0));
            ui.label(egui::RichText::new(description)
                .color(egui::Color32::from_gray(130))
                .size(11.0));
        } else {
            ui.heading(i18n.text("panel.properties"));
            ui.label(i18n.format("label.type", &[&format!("{:?}", node.node_type)]));
        }
        ui.separator();

        // 按定义顺序排列参数；未在定义中的参数（动态参数）排在末尾。
        let mut ordered_keys: Vec<&str> = node.params.keys().map(|s| s.as_str()).collect();
        if let Some(def) = get_definition(node.node_type) {
            let order: std::collections::HashMap<&str, usize> = def
                .params
                .iter()
                .enumerate()
                .map(|(i, p)| (p.name.as_str(), i))
                .collect();
            ordered_keys.sort_by_key(|k| order.get(k).copied().unwrap_or(usize::MAX));
        }

        const COMMON_PARAM_COUNT: usize = 4;
        let common = &ordered_keys[..ordered_keys.len().min(COMMON_PARAM_COUNT)];
        let advanced = &ordered_keys[COMMON_PARAM_COUNT.min(ordered_keys.len())..];

        for key in common {
            if let Some(action) = Self::render_param_entry(
                ui,
                i18n,
                node,
                label,
                registry,
                picker,
                coord,
                coord_picker,
                condition_editor,
                edit_bufs,
                key,
            ) {
                actions.push(action);
            }
        }

        if !advanced.is_empty() {
            egui::CollapsingHeader::new(i18n.text("label.advanced_params"))
                .id_salt(format!("advanced_params_{}", node.id))
                .default_open(false)
                .show(ui, |ui| {
                    for key in advanced {
                        if let Some(action) = Self::render_param_entry(
                            ui,
                            i18n,
                            node,
                            label,
                            registry,
                            picker,
                            coord,
                            coord_picker,
                            condition_editor,
                            edit_bufs,
                            key,
                        ) {
                            actions.push(action);
                        }
                    }
                });
        }

        if node.params.is_empty() {
            ui.label(i18n.text("label.none"));
        }

        // 动态端口/参数管理
        if let Some(def) = get_definition(node.node_type) {
            Self::render_dynamic_port_groups(ui, i18n, node, def, &mut actions);
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

        actions
    }

    /// 渲染单个参数条目（标签 + 编辑器），返回触发的动作。
    #[allow(clippy::too_many_arguments)]
    fn render_param_entry(
        ui: &mut egui::Ui,
        i18n: &I18n,
        node: &Node,
        label: &LabelContainer,
        registry: &NamespaceRegistry,
        picker: &mut Option<NamespacePickerState>,
        coord: &CoordinateRegistry,
        coord_picker: &mut Option<CoordinatePickerState>,
        condition_editor: &mut Option<ConditionEditorState>,
        edit_bufs: &mut EditBuffers,
        key: &str,
    ) -> Option<PropertiesPanelAction> {
        let value = node.params.get(key)?;
        let type_hint = param_type_label(node, key);
        let param_label = i18n.param_display_name(node.node_type, key);
        let mut action = None;
        ui.vertical(|ui| {
            ui.label(format!("{} {}", param_label, type_hint));
            action = Self::param_editor(
                ui,
                i18n,
                node,
                label,
                registry,
                picker,
                coord,
                coord_picker,
                condition_editor,
                edit_bufs,
                key,
                value,
            );
        });
        ui.add_space(2.0);
        ui.separator();
        action
    }

    /// 为单个参数绘制合适的编辑控件。
    ///
    /// 参数现在支持两种数据源：
    /// - 字面量：直接在节点属性中编辑。
    /// - 数据端口：通过节点边框的 Data 端口连接或下拉框选择其他节点的输出。
    #[allow(clippy::too_many_arguments)]
    fn param_editor(
        ui: &mut egui::Ui,
        i18n: &I18n,
        node: &Node,
        label: &LabelContainer,
        registry: &NamespaceRegistry,
        picker: &mut Option<NamespacePickerState>,
        _coord: &CoordinateRegistry,
        coord_picker: &mut Option<CoordinatePickerState>,
        condition_editor: &mut Option<ConditionEditorState>,
        edit_bufs: &mut EditBuffers,
        key: &str,
        value: &ParamValue,
    ) -> Option<PropertiesPanelAction> {
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

        // If/While 条件模板下拉框；对 If 节点的动态 elseif 条件参数也使用同样编辑器。
        let is_if_condition = (node.node_type == NodeType::If || node.node_type == NodeType::While)
            && key == "condition";
        let is_elseif_condition =
            node.node_type == NodeType::If && key.starts_with("elseif_") && key.ends_with("_condition");
        if is_if_condition || is_elseif_condition {
            return Self::condition_template_editor(ui, key, value, i18n);
        }

        // CreateCondition 条件组合编辑器：用弹窗组合 AND/OR/NOT 与已有 ID。
        if node.node_type == NodeType::CreateCondition && key == "condition" {
            let current = match value {
                ParamValue::Literal(v) => v.as_str().unwrap_or_default().to_string(),
                _ => String::new(),
            };
            if ui.button(i18n.text("button.edit_condition"))
                .on_hover_text(i18n.text("tooltip.edit_condition"))
                .clicked()
            {
                *condition_editor = Some(ConditionEditorState::new(key, current));
            }
            ui.label(
                egui::RichText::new(i18n.text("condition_editor.inline_preview"))
                    .size(11.0)
                    .color(egui::Color32::from_gray(140)),
            );
            if let ParamValue::Literal(v) = value {
                if let Some(s) = v.as_str() {
                    ui.label(egui::RichText::new(s).monospace().size(12.0));
                }
            }
            return None;
        }

        // CreateCondition ID 说明：提示开发者 id 用于复用以及如何在弹窗中选择。
        if node.node_type == NodeType::CreateCondition && key == "id" {
            ui.label(
                egui::RichText::new(i18n.text("condition_editor.id_hint"))
                    .size(11.0)
                    .color(egui::Color32::from_gray(140)),
            );
        }

        // CallMethod 方法下拉：根据 thread 数据连接推断对象类型，列出方法并自动填充参数模板。
        if node.node_type == NodeType::CallMethod && key == "method" {
            return Self::method_editor(ui, key, value, node, label, edit_bufs, i18n);
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
    ) -> Option<PropertiesPanelAction> {
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

        changed.map(|new_value| PropertiesPanelAction::SetParam {
            key: key.to_string(),
            value: ParamValue::Literal(serde_json::json!(new_value)),
        })
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
    ) -> Option<PropertiesPanelAction> {
        // 收集可选数据源：所有兼容类型的非 Flow 输出端口。
        let options = available_data_sources(i18n, label, node, key);

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
                        Some(PropertiesPanelAction::SetParam {
                            key: key.to_string(),
                            value: infer_default_literal(node, key),
                        })
                    } else {
                        None
                    }
                }
                source => Some(PropertiesPanelAction::SetParam {
                    key: key.to_string(),
                    value: source,
                }),
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
    ) -> Option<PropertiesPanelAction> {
        match value {
            ParamValue::Literal(v) if v.is_boolean() => {
                let mut b = v.as_bool().unwrap_or(false);
                if ui.checkbox(&mut b, "").changed() {
                    return Some(PropertiesPanelAction::SetParam {
                        key: key.to_string(),
                        value: ParamValue::Literal(serde_json::json!(b)),
                    });
                }
                None
            }
            ParamValue::Literal(v) if v.is_number() => {
                let mut num = v.as_f64().unwrap_or(0.0);
                if ui.add(egui::DragValue::new(&mut num)).changed() {
                    return Some(PropertiesPanelAction::SetParam {
                        key: key.to_string(),
                        value: ParamValue::Literal(serde_json::json!(num)),
                    });
                }
                None
            }
            // Vector/Color：专用多字段编辑器
            ParamValue::Literal(v) if v.is_array() => {
                if let Some(arr) = v.as_array() {
                    if !arr.is_empty() {
                        if let Some(v2) = Self::vector_editor(ui, key, arr) {
                            return Some(PropertiesPanelAction::SetParam {
                                key: key.to_string(),
                                value: v2,
                            });
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
                    .map(|(k, v)| PropertiesPanelAction::SetParam { key: k, value: v })
            }
        }
    }

    /// 渲染动态端口/参数组管理 UI（+/- 按钮）。
    fn render_dynamic_port_groups(
        ui: &mut egui::Ui,
        i18n: &I18n,
        node: &Node,
        def: &crate::api::definitions::NodeDefinition,
        actions: &mut Vec<PropertiesPanelAction>,
    ) {
        if def.dynamic_ports.is_empty() {
            return;
        }
        ui.separator();
        ui.label(egui::RichText::new(i18n.text("label.dynamic_ports")).strong());
        for group in &def.dynamic_ports {
            let members_per_group = group.members.len().max(1);
            let count = node.dynamic_port_count(&group.id, members_per_group);
            let can_add = group.max_count.is_none_or(|max| count < max);
            let can_remove = count > group.min_count;
            ui.horizontal(|ui| {
                ui.label(&group.label);
                if ui.button("+").on_hover_text(i18n.text("tooltip.add_dynamic_port")).clicked() && can_add {
                    actions.push(PropertiesPanelAction::AddDynamicPort {
                        group_id: group.id.clone(),
                    });
                }
                if ui.button("-").on_hover_text(i18n.text("tooltip.remove_dynamic_port")).clicked() && can_remove {
                    // 删除该组最后一个成员（flat list 中的最后一个 ID 即可触发整组删除）
                    let ids = node.dynamic_port_ids(&group.id);
                    if let Some(last_id) = ids.get(ids.len().saturating_sub(1)) {
                        actions.push(PropertiesPanelAction::RemoveDynamicPort {
                            group_id: group.id.clone(),
                            port_id: last_id.clone(),
                        });
                    }
                }
            });
            // 列出当前成员。单成员组按 flat ID 显示；多成员组按逻辑成员分组显示。
            let ids = node.dynamic_port_ids(&group.id);
            if members_per_group == 1 {
                for id in ids {
                    let label = match &group.members[0].template {
                        DynamicPortTemplate::Port(def) => format!("{}: {}", id, def.label),
                        DynamicPortTemplate::Param(def) => format!("{}: {}", id, def.display_name),
                    };
                    ui.label(egui::RichText::new(label).size(11.0));
                }
            } else {
                for (logical_index, chunk) in ids.chunks(members_per_group).enumerate() {
                    ui.label(egui::RichText::new(format!("{} {}", group.label, logical_index)).size(11.0).strong());
                    for (id, member) in chunk.iter().zip(group.members.iter()) {
                        let detail = match &member.template {
                            DynamicPortTemplate::Port(def) => format!("  {}: {}", id, def.label),
                            DynamicPortTemplate::Param(def) => format!("  {}: {}", id, def.display_name),
                        };
                        ui.label(egui::RichText::new(detail).size(11.0));
                    }
                }
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
    i18n: &I18n,
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
            let port_label = i18n.port_display_name(other.node_type, &output.id);
            sources.push((
                format!("{}.{} ({})", other_id, output.id, port_label),
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
    ) -> Option<PropertiesPanelAction> {
        let current = match value {
            ParamValue::Literal(v) => v.as_str().unwrap_or_default().to_string(),
            _ => String::new(),
        };

        let mut picked = None;
        egui::ComboBox::from_id_salt("if_condition_template")
            .width(160.0)
            .selected_text(if current.is_empty() { i18n.text("label.select_template") } else { current.clone() })
            .show_ui(ui, |ui| {
                for &(label_key, expr) in IF_CONDITION_TEMPLATES {
                    let label = i18n.text(label_key);
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
            return Some(PropertiesPanelAction::SetParam {
                key: key.to_string(),
                value: ParamValue::Literal(serde_json::json!(expr)),
            });
        }

        // 文本框允许手动微调
        let mut text = current;
        if ui.text_edit_singleline(&mut text).changed() {
            return Some(PropertiesPanelAction::SetParam {
                key: key.to_string(),
                value: ParamValue::Literal(serde_json::json!(text)),
            });
        }
        None
    }
}

/// 条件模板列表。每个条目的第一个字段是 i18n key，第二个字段是表达式。
/// 空表达式表示仅作为分类标题。
static IF_CONDITION_TEMPLATES: &[(&str, &str)] = &[
    ("template.true", "true"),
    ("template.false", "false"),
    ("template.category.character_state", ""),
    ("template.futanari", "_state.Futanari"),
    ("template.sitting", "_state.Sitting"),
    ("template.orgasm", "_state.Orgasm"),
    ("template.moving", "_state.Moving"),
    ("template.crouching", "_state.Crouching"),
    ("template.peeing", "_state.Peeing"),
    ("template.dashing", "_state.Dashing"),
    ("template.category.environment", ""),
    ("template.inlight", "_state.InLight"),
    ("template.near_npc", "_state.NearNPC"),
    ("template.watched", "_state.Watched"),
    ("template.is_daytime", "_state.IsDayTime"),
    ("template.fp_camera", "_state.FPCamera"),
    ("template.category.equipment", ""),
    ("template.blindfolded", "_state.Blindfolded"),
    ("template.invisible", "_state.Invisible"),
    ("template.has_handcuffs", "_state.AdultToys.Handcuff != null"),
    ("template.no_handcuffs", "_state.AdultToys.Handcuff == null"),
    ("template.has_vibrator", "_state.AdultToys.Vibrator != null"),
    ("template.category.value_compare", ""),
    ("template.bodypaint", "_state.Bodypaint > 0"),
    ("template.ecstasy", "_state.Ecstasy >= "),
    ("template.detection", "_state.Detection >= "),
    ("template.stamina", "_state.Stamina >= "),
    ("template.rank", "_state.Rank >= "),
    ("template.moisture", "_state.Moisture >= "),
    ("template.heartrate", "_state.HeartRate >= "),
];

/// CallMethod 方法下拉编辑器：根据 thread 数据连接推断对象类型，列出方法并自动填充参数模板。
impl PropertiesPanel {
    fn method_editor(
        ui: &mut egui::Ui,
        key: &str,
        value: &ParamValue,
        node: &Node,
        label: &LabelContainer,
        edit_bufs: &mut EditBuffers,
        i18n: &I18n,
    ) -> Option<PropertiesPanelAction> {
        use crate::api::method_registry::{
            all_methods, methods_for_object_type, object_type_from_node_type,
        };

        let current = match value {
            ParamValue::Literal(v) => v.as_str().unwrap_or_default().to_string(),
            _ => String::new(),
        };

        let object_type = connected_data_source(label, &node.id, "thread")
            .and_then(|(src_id, _)| label.nodes.get(&src_id))
            .and_then(|src| object_type_from_node_type(src.node_type));

        let methods: Vec<&crate::api::method_registry::MethodSignature> = if let Some(ot) = object_type {
            methods_for_object_type(ot)
        } else {
            all_methods().iter().collect()
        };

        if methods.is_empty() {
            return Self::literal_editor(ui, key, value, &node.id, edit_bufs, i18n);
        }

        let mut picked = None;
        egui::ComboBox::from_id_salt("call_method_method")
            .width(180.0)
            .selected_text(if current.is_empty() {
                i18n.text("label.select_method")
            } else {
                current.clone()
            })
            .show_ui(ui, |ui| {
                let mut last_object_type = "";
                for method in methods {
                    if method.object_type != last_object_type {
                        if !last_object_type.is_empty() {
                            ui.separator();
                        }
                        ui.label(egui::RichText::new(method.object_type).strong().size(11.0));
                        last_object_type = method.object_type;
                    }
                    if ui
                        .selectable_label(current == method.method_name, method.full_label())
                        .clicked()
                    {
                        picked = Some(method);
                    }
                }
            });

        if let Some(method) = picked {
            let mut values = HashMap::new();
            values.insert(
                key.to_string(),
                ParamValue::Literal(serde_json::json!(method.method_name)),
            );
            values.insert(
                "params".to_string(),
                ParamValue::Literal(method.params_template()),
            );
            return Some(PropertiesPanelAction::SetParams { values });
        }

        Self::literal_editor(ui, key, value, &node.id, edit_bufs, i18n)
    }
}

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
        node.inputs = vec![Port::new("in_flow", PortType::Flow, "Execute")];
        node.outputs = vec![
            Port::new("out_flow", PortType::Flow, "Next"),
            Port::new("out_value", PortType::Number, "Value"),
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
            Port::new("in_flow", PortType::Flow, "Execute"),
            Port::new("output", PortType::String, "Output"),
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
            Port::new("in_flow", PortType::Flow, "Execute"),
            Port::new("value", PortType::Number, "Value"),
        ];
        n2.set_param("value", ParamValue::Literal(serde_json::json!(0.0)));
        let i18n = crate::ui::i18n::I18n::new();
        label.nodes.insert(n1.id.clone(), n1);

        let sources = super::available_data_sources(&i18n, &label, &n2, "value");
        label.nodes.insert(n2.id.clone(), n2);

        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0].1, ParamValue::from_ref("n1", "out_value"));
    }
}
