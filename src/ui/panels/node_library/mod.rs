use crate::api::registry::all_node_definitions;
use crate::graph::types::NodeType;
use crate::ui::i18n::I18n;
use crate::ui::panels::node_library::catalog::SceneCatalog;
use crate::ui::theme::scene_category_color;

mod catalog;

/// 节点库操作结果。
#[derive(Debug)]
pub enum NodeLibraryAction {
    Create(NodeType),
    DragStart(NodeType),
    None,
}

/// 节点库当前过滤/搜索状态。
#[derive(Debug, Clone, Default)]
pub struct NodeLibraryFilter {
    /// 搜索关键词。
    pub query: String,
    /// 选中的场景分类 ID；空字符串表示“全部分类”。
    pub scene_filter: String,
}

/// 将节点类型记录到最近使用列表（去重、置顶、最多保留 10 项）。
pub fn record_recent(recent: &mut Vec<NodeType>, node_type: NodeType) {
    recent.retain(|&n| n != node_type);
    recent.insert(0, node_type);
    recent.truncate(10);
}

/// 节点库面板。
pub struct NodeLibraryPanel;

impl NodeLibraryPanel {
    pub fn show(
        ui: &mut egui::Ui,
        i18n: &I18n,
        filter: &mut NodeLibraryFilter,
        search_window_open: &mut bool,
        recent_node_types: &[NodeType],
        max_height: f32,
    ) -> NodeLibraryAction {
        let categories = SceneCatalog::categories();
        let mut action = NodeLibraryAction::None;
        egui::ScrollArea::vertical()
            .id_salt("node_library_scroll")
            .max_height(max_height)
            .auto_shrink([false, true])
            .show(ui, |ui| {
                ui.heading(i18n.text("node_library.title"));

                // 搜索框独占一行，避免与过滤下拉框挤在同一行导致左栏宽度异常。
                ui.text_edit_singleline(&mut filter.query);
                // 过滤下拉框：标签 + 下拉框，下拉框占满剩余宽度。
                ui.horizontal(|ui| {
                    ui.label(i18n.text("node_library.filter_label"));
                    let combo_width = ui.available_width().clamp(80.0, 160.0);
                    egui::ComboBox::from_id_salt("node_library_scene_filter")
                        .width(combo_width)
                        .selected_text(selected_filter_label(i18n, &filter.scene_filter))
                        .show_ui(ui, |ui| {
                            if ui
                                .selectable_label(
                                    filter.scene_filter.is_empty(),
                                    i18n.text("node_library.filter_all"),
                                )
                                .clicked()
                            {
                                filter.scene_filter.clear();
                            }
                            for category in &categories {
                                if ui
                                    .selectable_label(
                                        filter.scene_filter == category.id,
                                        i18n.text(category.id),
                                    )
                                    .clicked()
                                {
                                    filter.scene_filter = category.id.to_string();
                                }
                            }
                        });
                });
                ui.separator();

                // 最近使用
                if !recent_node_types.is_empty() {
                    ui.label(egui::RichText::new(i18n.text("node_library.recent")).strong());
                    ui.horizontal_wrapped(|ui| {
                        for &node_type in recent_node_types {
                            if ui.button(i18n.node_display_name(node_type)).clicked() {
                                action = NodeLibraryAction::Create(node_type);
                            }
                        }
                    });
                    ui.separator();
                }

                for category in &categories {
                    if !filter.scene_filter.is_empty() && filter.scene_filter != category.id {
                        continue;
                    }
                    let mut visible_subs = Vec::new();
                    for sub in &category.subcategories {
                        let filtered: Vec<_> = sub
                            .nodes
                            .iter()
                            .filter(|node_type| {
                                matches_filter(i18n, **node_type, &filter.query)
                            })
                            .copied()
                            .collect();
                        if !filtered.is_empty() {
                            visible_subs.push((sub, filtered));
                        }
                    }
                    if visible_subs.is_empty() {
                        continue;
                    }

                    egui::CollapsingHeader::new(i18n.text(category.id))
                        .id_salt(format!("scene_cat_{}", category.id))
                        .show(ui, |ui| {
                            for (sub, filtered) in visible_subs {
                                egui::CollapsingHeader::new(i18n.text(sub.id))
                                    .id_salt(format!(
                                        "scene_sub_{}_{}",
                                        category.id, sub.id
                                    ))
                                    .show(ui, |ui| {
                                        for node_type in filtered {
                                            let color = scene_category_color(category.id);
                                            let display_name =
                                                i18n.node_display_name(node_type);
                                            let resp = ui.horizontal(|ui| {
                                                ui.painter().circle_filled(
                                                    ui.cursor().min + egui::vec2(8.0, 8.0),
                                                    6.0,
                                                    color,
                                                );
                                                ui.add_space(16.0);
                                                ui.add(
                                                    egui::Button::new(display_name)
                                                        .sense(egui::Sense::drag()),
                                                )
                                            });
                                            if resp.inner.clicked() {
                                                action = NodeLibraryAction::Create(node_type);
                                            }
                                            if resp.inner.drag_started() {
                                                action = NodeLibraryAction::DragStart(node_type);
                                            }
                                        }
                                    });
                            }
                        });
                }
            });

        if ui.button(i18n.text("button.search_add")).clicked() {
            *search_window_open = !*search_window_open;
        }

        action
    }

    /// 在弹出窗口中显示搜索界面，返回选中的节点类型（如果有）。
    pub fn show_search(
        ui: &mut egui::Ui,
        i18n: &I18n,
        filter: &mut NodeLibraryFilter,
        recent_node_types: &[NodeType],
    ) -> Option<NodeType> {
        ui.text_edit_singleline(&mut filter.query);

        let categories = SceneCatalog::categories();
        ui.horizontal(|ui| {
            ui.label(i18n.text("node_library.filter_label"));
            egui::ComboBox::from_id_salt("node_search_scene_filter")
                .width(160.0)
                .selected_text(selected_filter_label(i18n, &filter.scene_filter))
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_label(
                            filter.scene_filter.is_empty(),
                            i18n.text("node_library.filter_all"),
                        )
                        .clicked()
                    {
                        filter.scene_filter.clear();
                    }
                    for category in &categories {
                        if ui
                            .selectable_label(
                                filter.scene_filter == category.id,
                                i18n.text(category.id),
                            )
                            .clicked()
                        {
                            filter.scene_filter = category.id.to_string();
                        }
                    }
                });
        });
        ui.separator();

        let mut created = None;
        let mut matched = Vec::new();
        for def in all_node_definitions() {
            if !filter.scene_filter.is_empty() {
                let mut in_category = false;
                for category in &categories {
                    if category.id != filter.scene_filter {
                        continue;
                    }
                    for sub in &category.subcategories {
                        if sub.nodes.contains(&def.node_type) {
                            in_category = true;
                            break;
                        }
                    }
                    if in_category {
                        break;
                    }
                }
                if !in_category {
                    continue;
                }
            }
            if matches_filter(i18n, def.node_type, &filter.query) {
                matched.push(def);
            }
        }

        egui::ScrollArea::vertical()
            .id_salt("node_search_scroll")
            .max_height(300.0)
            .show(ui, |ui| {
                // 最近使用
                if !recent_node_types.is_empty() {
                    ui.label(egui::RichText::new(i18n.text("node_library.recent")).strong());
                    for &node_type in recent_node_types {
                        if ui.button(i18n.node_display_name(node_type)).clicked() {
                            created = Some(node_type);
                        }
                    }
                    ui.separator();
                }

                for def in matched {
                    let display_name = i18n.node_display_name(def.node_type);
                    if ui
                        .button(format!("{} [{}]", display_name, def.category))
                        .clicked()
                    {
                        created = Some(def.node_type);
                    }
                }
            });
        created
    }
}

fn selected_filter_label(i18n: &I18n, scene_filter: &str) -> String {
    if scene_filter.is_empty() {
        i18n.text("node_library.filter_all")
    } else {
        i18n.text(scene_filter)
    }
}

/// 模糊匹配节点名称、NodeType 名、API 分类名和所属场景分类名。
fn matches_filter(i18n: &I18n, node_type: NodeType, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    let name = i18n.node_display_name(node_type).to_lowercase();
    let debug = format!("{:?}", node_type).to_lowercase();
    if fuzzy_match(query, &name) || fuzzy_match(query, &debug) {
        return true;
    }
    let def = match crate::api::registry::get_definition(node_type) {
        Some(d) => d,
        None => return false,
    };
    if fuzzy_match(query, &def.category.to_lowercase()) {
        return true;
    }
    // 同时匹配场景分类/子分类标签
    for category in SceneCatalog::categories() {
        for sub in &category.subcategories {
            if sub.nodes.contains(&node_type) {
                if fuzzy_match(query, &i18n.text(category.id).to_lowercase())
                    || fuzzy_match(query, &i18n.text(sub.id).to_lowercase())
                {
                    return true;
                }
            }
        }
    }
    false
}

/// 简单字符级模糊匹配：query 中的字符按顺序在 target 中出现即匹配。
/// 若 query 是 target 的连续子串则优先命中。
fn fuzzy_match(query: &str, target: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    if target.contains(query) {
        return true;
    }
    let mut qi = query.chars();
    let mut current = qi.next();
    for c in target.chars() {
        if let Some(qc) = current {
            if c == qc {
                current = qi.next();
            }
        }
    }
    current.is_none()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_match() {
        assert!(fuzzy_match("", "hello"));
        assert!(fuzzy_match("set", "setplayerposition"));
        assert!(fuzzy_match("stp", "setplayerposition"));
        assert!(fuzzy_match("setplayer", "setplayerposition"));
        assert!(!fuzzy_match("xyz", "setplayerposition"));
    }

    #[test]
    fn test_record_recent_dedup_and_limit() {
        let mut recent = vec![NodeType::Log, NodeType::If];
        record_recent(&mut recent, NodeType::Log);
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0], NodeType::Log);
        record_recent(&mut recent, NodeType::While);
        assert_eq!(recent[0], NodeType::While);
        // 15 次插入相同节点只会保留 1 个
        for _ in 0..15 {
            record_recent(&mut recent, NodeType::For);
        }
        assert_eq!(recent.len(), 4);
        // 用不同节点测试上限
        for &nt in &[
            NodeType::Break,
            NodeType::Return,
            NodeType::Wait,
            NodeType::WaitForEvent,
            NodeType::CallFunction,
            NodeType::CallMethod,
            NodeType::TriggerGameOver,
        ] {
            record_recent(&mut recent, nt);
        }
        assert_eq!(recent.len(), 10);
    }
}
