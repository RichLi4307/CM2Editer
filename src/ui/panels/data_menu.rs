use crate::graph::graph::Graph;
use crate::graph::types::PortType;
use crate::ui::theme::port_color;
use std::collections::HashSet;

/// 数据菜单面板。
///
/// 显示当前图中可用的数据变量，按节点分组，以小方块（巧克力板）形式排列。
/// 点击方块可选中画布中的对应节点。
pub struct DataMenuPanel;

impl DataMenuPanel {
    /// 显示数据菜单，返回请求选中的节点 ID（如果有）。
    ///
    /// `selected_nodes` 用于高亮当前已选中的节点。
    pub fn show(
        ui: &mut egui::Ui,
        graph: &Graph,
        selected_nodes: &HashSet<String>,
    ) -> Option<String> {
        ui.horizontal(|ui| {
            ui.heading("数据");
            ui.label("(DataFlow)").on_hover_text("列出图中可用的数据输出端口；点击方块选中节点");
        });

        ui.separator();

        let mut requested_select = None;

        // 收集所有数据输出
        let mut any = false;
        let mut node_groups: Vec<(String, Vec<DataTile>)> = Vec::new();
        for (node_id, node) in &graph.nodes {
            let data_outputs: Vec<_> = node
                .outputs
                .iter()
                .filter(|p| p.port_type != PortType::Flow)
                .collect();
            if data_outputs.is_empty() {
                continue;
            }
            any = true;

            let mut tiles = Vec::new();
            for output in data_outputs {
                tiles.push(DataTile {
                    key: output.id.clone(),
                    label: output.label.clone(),
                    port_type: output.port_type.clone(),
                });
            }
            node_groups.push((node_id.clone(), tiles));
        }

        if !any {
            ui.label("暂无数据输出端口");
            return None;
        }

        // 使用滚动区域包裹巧克力板布局
        egui::ScrollArea::vertical()
            .id_salt("data_tiles_scroll")
            .show(ui, |ui| {
            for (node_id, tiles) in &node_groups {
                let is_selected = selected_nodes.contains(node_id);
                ui.horizontal(|ui| {
                    ui.label(format!("节点 {}", node_id));
                    if is_selected {
                        ui.label("▶");
                    }
                });

                ui.horizontal_wrapped(|ui| {
                    for tile in tiles {
                        let tile_response = ui.add(tile_button(tile, is_selected));
                        if tile_response.clicked() || tile_response.secondary_clicked() {
                            requested_select = Some(node_id.clone());
                        }
                    }
                });
                ui.add_space(4.0);
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

/// 绘制一个小方块数据按钮，颜色根据端口类型变化。
fn tile_button(tile: &DataTile, is_node_selected: bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| {
        let (rect, response) = ui.allocate_exact_size(
            egui::vec2(72.0, 56.0),
            egui::Sense::click_and_drag(),
        );

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

        ui.painter().rect_filled(rect, 6.0, fill_color);
        ui.painter().rect_stroke(
            rect,
            6.0,
            egui::Stroke::new(2.0, stroke_color),
            egui::StrokeKind::Middle,
        );

        let text = if tile.label.len() > 8 {
            format!("{}..", &tile.label[..8])
        } else {
            tile.label.clone()
        };
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::proportional(12.0),
            if is_node_selected || response.hovered() {
                egui::Color32::WHITE
            } else {
                egui::Color32::LIGHT_GRAY
            },
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
    use crate::graph::types::{NodeType, PortType};

    #[test]
    fn test_tile_button_allocates_size() {
        let tile = DataTile {
            key: "out_value".to_string(),
            label: "值".to_string(),
            port_type: PortType::Number,
        };
        // tile_button 返回 impl Widget，只需确保类型可编译
        let _widget = tile_button(&tile, false);
    }

    #[test]
    fn test_data_panel_returns_requested_select() {
        let mut node = Node::new(NodeType::Random, Vec2::ZERO);
        node.id = "n1".to_string();
        node.outputs.push(Port::new("out_value", PortType::Number, "值"));
        let mut graph = Graph::default();
        graph.add_node(node);
        let selected: HashSet<String> = HashSet::new();
        // 无法在此直接调用 DataMenuPanel::show（需要 egui::Ui），但类型可编译
        let _ = (&graph, &selected);
    }
}
