use crate::api::registry::all_node_definitions;
use crate::graph::types::NodeType;
use crate::ui::theme::category_color;

/// 节点库面板。
pub struct NodeLibraryPanel;

impl NodeLibraryPanel {
    /// 在侧边栏中显示节点库，返回用户选中的节点类型（如果有）。
    pub fn show(
        ui: &mut egui::Ui,
        search_query: &mut String,
        search_window_open: &mut bool,
    ) -> Option<NodeType> {
        ui.heading("节点库");
        ui.text_edit_singleline(search_query);
        ui.separator();

        let defs = all_node_definitions();
        let mut categories: std::collections::HashMap<
            &str,
            Vec<&crate::api::definitions::NodeDefinition>,
        > = std::collections::HashMap::new();
        for def in defs {
            categories.entry(&def.category).or_default().push(def);
        }

        let mut created = None;
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (category, items) in categories {
                let filtered: Vec<_> = items
                    .iter()
                    .filter(|d| {
                        search_query.is_empty()
                            || d.display_name
                                .to_lowercase()
                                .contains(&search_query.to_lowercase())
                            || format!("{:?}", d.node_type)
                                .to_lowercase()
                                .contains(&search_query.to_lowercase())
                    })
                    .copied()
                    .collect();
                if filtered.is_empty() {
                    continue;
                }

                ui.collapsing(category, |ui| {
                    for def in filtered {
                        let color = category_color(&def.category);
                        ui.horizontal(|ui| {
                            ui.painter().circle_filled(
                                ui.cursor().min + egui::vec2(8.0, 8.0),
                                6.0,
                                color,
                            );
                            ui.add_space(16.0);
                            if ui.button(&def.display_name).clicked() {
                                created = Some(def.node_type);
                            }
                        });
                    }
                });
            }
        });

        if ui.button("Space 搜索添加节点").clicked() {
            *search_window_open = !*search_window_open;
        }

        created
    }

    /// 在弹出窗口中显示搜索界面，返回选中的节点类型（如果有）。
    pub fn show_search(ui: &mut egui::Ui, search_query: &mut String) -> Option<NodeType> {
        ui.text_edit_singleline(search_query);
        ui.separator();

        let query = search_query.to_lowercase();
        let defs = all_node_definitions();
        let matched: Vec<_> = defs
            .iter()
            .filter(|d| {
                query.is_empty()
                    || d.display_name.to_lowercase().contains(&query)
                    || format!("{:?}", d.node_type).to_lowercase().contains(&query)
                    || d.category.to_lowercase().contains(&query)
            })
            .collect();

        let mut created = None;
        egui::ScrollArea::vertical()
            .max_height(300.0)
            .show(ui, |ui| {
                for def in matched {
                    if ui
                        .button(format!("{} [{}]", def.display_name, def.category))
                        .clicked()
                    {
                        created = Some(def.node_type);
                    }
                }
            });
        created
    }
}
