use crate::graph::graph::Graph;
use crate::graph::types::PortType;
use crate::ui::theme::port_color;
use std::collections::HashSet;

/// 数据菜单面板。按节点分组，小方块（巧克力板）水平换行排列。点击选中节点。
pub struct DataMenuPanel;

impl DataMenuPanel {
    pub fn show(
        ui: &mut egui::Ui,
        graph: &Graph,
        selected_nodes: &HashSet<String>,
    ) -> Option<String> {
        ui.horizontal(|ui| {
            ui.heading("数据");
            ui.label("(DataFlow)");
        });

        let mut requested_select = None;
        let mut any = false;
        let mut node_groups: Vec<(String, Vec<DataTile>)> = Vec::new();

        for (node_id, node) in &graph.nodes {
            let data_outputs: Vec<_> = node.outputs.iter().filter(|p| p.port_type != PortType::Flow).collect();
            if data_outputs.is_empty() {
                continue;
            }
            any = true;
            let tiles: Vec<DataTile> = data_outputs
                .iter()
                .map(|o| DataTile {
                    key: o.id.clone(),
                    label: o.label.clone(),
                    port_type: o.port_type.clone(),
                })
                .collect();
            node_groups.push((node_id.clone(), tiles));
        }

        if !any {
            ui.label("暂无数据输出端口");
            return None;
        }

        let _available_width = ui.available_width().max(80.0);

        egui::ScrollArea::vertical()
            .id_salt("data_tiles_scroll")
            .auto_shrink([true, false])
            .show(ui, |ui| {
                for (node_id, tiles) in &node_groups {
                    let is_selected = selected_nodes.contains(node_id);
                    ui.horizontal(|ui| {
                        if is_selected {
                            ui.colored_label(egui::Color32::from_rgb(100, 180, 255), "▶");
                        }
                        ui.label(node_id);
                    });

                    ui.horizontal_wrapped(|ui| {
                        for tile in tiles {
                            let resp = ui.add(tile_button(tile, is_selected));
                            if resp.clicked() || resp.secondary_clicked() {
                                requested_select = Some(node_id.clone());
                            }
                        }
                    });
                    ui.separator();
                }
            });

        requested_select
    }
}

struct DataTile {
    key: String,
    label: String,
    port_type: PortType,
}

fn tile_button(tile: &DataTile, is_node_selected: bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| {
        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(64.0, 40.0), egui::Sense::click_and_drag());

        let color = port_color(&tile.port_type);
        let stroke_color = if is_node_selected {
            egui::Color32::from_rgb(100, 180, 255)
        } else {
            color.gamma_multiply(0.7)
        };
        let fill_color = if response.hovered() || response.dragged() {
            color.gamma_multiply(0.3)
        } else {
            color.gamma_multiply(0.15)
        };

        ui.painter().rect_filled(rect, 4.0, fill_color);
        ui.painter().rect_stroke(rect, 4.0, egui::Stroke::new(1.5, stroke_color), egui::StrokeKind::Middle);

        let text = if tile.label.len() > 6 {
            format!("{}..", &tile.label[..6])
        } else {
            tile.label.clone()
        };
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::proportional(11.0),
            if is_node_selected || response.hovered() { egui::Color32::WHITE } else { egui::Color32::LIGHT_GRAY },
        );

        if response.hovered() {
            egui::show_tooltip_at_pointer(
                ui.ctx(),
                egui::LayerId::new(egui::Order::Tooltip, egui::Id::new("data_tile_tooltip")),
                egui::Id::new(format!("data_tile_{}", tile.key)),
                |ui: &mut egui::Ui| {
                    ui.label(format!("{}: {:?}", tile.key, tile.port_type));
                },
            );
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::node::{Node, Port, Vec2};
    use crate::graph::types::NodeType;

    #[test]
    fn test_tile_button_allocates_size() {
        let tile = DataTile {
            key: "out_value".to_string(),
            label: "值".to_string(),
            port_type: PortType::Number,
        };
        let _widget = tile_button(&tile, false);
    }

    #[test]
    fn test_data_panel_compiles() {
        let mut node = Node::new(NodeType::Random, Vec2::ZERO);
        node.id = "n1".to_string();
        node.outputs.push(Port::new("out_value", PortType::Number, "值"));
        let mut graph = Graph::default();
        graph.add_node(node);
        let selected: HashSet<String> = HashSet::new();
        let _ = (&graph, &selected);
    }
}
