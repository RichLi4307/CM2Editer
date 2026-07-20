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
    ToggleFavorite(NodeType),
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

/// 收藏节点数量上限。
const MAX_FAVORITES: usize = 30;

/// 判断某个节点是否已被收藏。
fn is_favorite(favorite_node_types: &[NodeType], node_type: NodeType) -> bool {
    favorite_node_types.contains(&node_type)
}

/// 切换节点的收藏状态（添加时置顶并截断到上限）。
pub fn toggle_favorite(favorite_node_types: &mut Vec<NodeType>, node_type: NodeType) {
    if let Some(pos) = favorite_node_types.iter().position(|&n| n == node_type) {
        favorite_node_types.remove(pos);
    } else {
        favorite_node_types.insert(0, node_type);
        favorite_node_types.truncate(MAX_FAVORITES);
    }
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
        favorite_node_types: &[NodeType],
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

                // 收藏（置顶）
                if !favorite_node_types.is_empty() {
                    ui.label(egui::RichText::new(i18n.text("node_library.favorites")).strong());
                    ui.horizontal_wrapped(|ui| {
                        for &node_type in favorite_node_types {
                            if ui
                                .button(format!(
                                    "{} {}",
                                    i18n.text("button.favorite"),
                                    i18n.node_display_name(node_type)
                                ))
                                .clicked()
                            {
                                action = NodeLibraryAction::ToggleFavorite(node_type);
                            }
                        }
                    });
                    ui.separator();
                }

                // 最近使用（过滤掉已收藏的节点，避免重复）
                let recent_not_favorite: Vec<_> = recent_node_types
                    .iter()
                    .copied()
                    .filter(|&n| !is_favorite(favorite_node_types, n))
                    .collect();
                if !recent_not_favorite.is_empty() {
                    ui.label(egui::RichText::new(i18n.text("node_library.recent")).strong());
                    ui.horizontal_wrapped(|ui| {
                        for &node_type in &recent_not_favorite {
                            if ui
                                .button(format!(
                                    "{} {}",
                                    i18n.text("button.favorite"),
                                    i18n.node_display_name(node_type)
                                ))
                                .clicked()
                            {
                                action = NodeLibraryAction::ToggleFavorite(node_type);
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
                                            let fav_text = if is_favorite(favorite_node_types, node_type) {
                                                i18n.text("button.unfavorite")
                                            } else {
                                                i18n.text("button.favorite")
                                            };
                                            let resp = ui.horizontal(|ui| {
                                                ui.painter().circle_filled(
                                                    ui.cursor().min + egui::vec2(8.0, 8.0),
                                                    6.0,
                                                    color,
                                                );
                                                ui.add_space(16.0);
                                                let name_resp = ui.add(
                                                    egui::Button::new(display_name)
                                                        .sense(egui::Sense::drag()),
                                                );
                                                let star_resp = ui.button(fav_text);
                                                (name_resp, star_resp)
                                            });
                                            if resp.inner.0.clicked() {
                                                action = NodeLibraryAction::Create(node_type);
                                            }
                                            if resp.inner.0.drag_started() {
                                                action = NodeLibraryAction::DragStart(node_type);
                                            }
                                            if resp.inner.1.clicked() {
                                                action = NodeLibraryAction::ToggleFavorite(node_type);
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

    /// 在弹出窗口中显示搜索界面，返回操作结果。
    pub fn show_search(
        ui: &mut egui::Ui,
        i18n: &I18n,
        filter: &mut NodeLibraryFilter,
        recent_node_types: &[NodeType],
        favorite_node_types: &[NodeType],
    ) -> NodeLibraryAction {
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

        let mut action = NodeLibraryAction::None;
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
                // 收藏（置顶）
                if !favorite_node_types.is_empty() {
                    ui.label(egui::RichText::new(i18n.text("node_library.favorites")).strong());
                    for &node_type in favorite_node_types {
                        let display_name = i18n.node_display_name(node_type);
                        ui.horizontal(|ui| {
                            if ui.button(i18n.text("button.unfavorite")).clicked() {
                                action = NodeLibraryAction::ToggleFavorite(node_type);
                            }
                            if ui.button(display_name).clicked() {
                                action = NodeLibraryAction::Create(node_type);
                            }
                        });
                    }
                    ui.separator();
                }

                // 最近使用（过滤掉已收藏的节点）
                let recent_not_favorite: Vec<_> = recent_node_types
                    .iter()
                    .copied()
                    .filter(|&n| !is_favorite(favorite_node_types, n))
                    .collect();
                if !recent_not_favorite.is_empty() {
                    ui.label(egui::RichText::new(i18n.text("node_library.recent")).strong());
                    for &node_type in &recent_not_favorite {
                        let display_name = i18n.node_display_name(node_type);
                        ui.horizontal(|ui| {
                            if ui.button(i18n.text("button.favorite")).clicked() {
                                action = NodeLibraryAction::ToggleFavorite(node_type);
                            }
                            if ui.button(display_name).clicked() {
                                action = NodeLibraryAction::Create(node_type);
                            }
                        });
                    }
                    ui.separator();
                }

                for def in matched {
                    let display_name = i18n.node_display_name(def.node_type);
                    let fav_text = if is_favorite(favorite_node_types, def.node_type) {
                        i18n.text("button.unfavorite")
                    } else {
                        i18n.text("button.favorite")
                    };
                    ui.horizontal(|ui| {
                        if ui.button(fav_text).clicked() {
                            action = NodeLibraryAction::ToggleFavorite(def.node_type);
                        }
                        if ui
                            .button(format!("{} [{}]", display_name, def.category))
                            .clicked()
                        {
                            action = NodeLibraryAction::Create(def.node_type);
                        }
                    });
                }
            });
        action
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
            if sub.nodes.contains(&node_type)
                && (fuzzy_match(query, &i18n.text(category.id).to_lowercase())
                    || fuzzy_match(query, &i18n.text(sub.id).to_lowercase()))
            {
                return true;
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
    fn test_toggle_favorite_add_remove_and_limit() {
        let mut favorites = Vec::new();
        // 添加收藏
        toggle_favorite(&mut favorites, NodeType::Log);
        assert_eq!(favorites, vec![NodeType::Log]);
        // 再次点击同一节点应取消收藏
        toggle_favorite(&mut favorites, NodeType::Log);
        assert!(favorites.is_empty());
        // 重新添加并追加新收藏
        toggle_favorite(&mut favorites, NodeType::Log);
        toggle_favorite(&mut favorites, NodeType::If);
        assert_eq!(favorites, vec![NodeType::If, NodeType::Log]);
        // 取消收藏
        toggle_favorite(&mut favorites, NodeType::Log);
        assert_eq!(favorites, vec![NodeType::If]);
        // 测试上限：用大量不同节点填充
        let all_types: Vec<NodeType> = all_node_definitions()
            .into_iter()
            .map(|d| d.node_type)
            .collect();
        for &nt in &all_types {
            toggle_favorite(&mut favorites, nt);
        }
        assert_eq!(favorites.len(), MAX_FAVORITES);
        // 最后插入的节点应位于列表前端
        assert!(is_favorite(&favorites, *all_types.last().unwrap()));
    }

    #[test]
    fn test_is_favorite() {
        let favorites = vec![NodeType::Log, NodeType::If];
        assert!(is_favorite(&favorites, NodeType::Log));
        assert!(!is_favorite(&favorites, NodeType::While));
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
