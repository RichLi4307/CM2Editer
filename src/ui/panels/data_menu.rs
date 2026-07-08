use crate::api::registry::get_definition;
use crate::graph::graph::Graph;
use crate::graph::types::PortType;
use std::collections::HashSet;

/// 数据菜单面板。
///
/// 显示当前图中可用的数据变量，按节点分组，便于用户拖拽/选择数据源。
pub struct DataMenuPanel;

impl DataMenuPanel {
    /// 显示数据菜单。
    ///
    /// `selected_nodes` 用于高亮当前选中节点提供的数据源。
    pub fn show(
        ui: &mut egui::Ui,
        graph: &Graph,
        selected_nodes: &HashSet<String>,
    ) {
        ui.horizontal(|ui| {
            ui.heading("数据");
            ui.label("(DataFlow)").on_hover_text("列出图中可用的数据输出端口");
        });

        ui.separator();

        let mut any = false;
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

            let is_selected = selected_nodes.contains(node_id);
            let node_name = get_definition(node.node_type)
                .map(|d| d.display_name.clone())
                .unwrap_or_else(|| format!("{:?}", node.node_type));
            let label = if is_selected {
                format!("▶ {} ({})", node_name, node_id)
            } else {
                format!("{} ({})", node_name, node_id)
            };
            let header_color = if is_selected {
                egui::Color32::from_rgb(100, 180, 255)
            } else {
                ui.visuals().text_color()
            };
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(&label).color(header_color));
            });
            for output in data_outputs {
                let port_label = format!("  • {} : {:?}", output.id, output.port_type);
                ui.label(port_label).on_hover_text(&output.label);
            }
        }

        if !any {
            ui.label("暂无数据输出端口");
        }
    }
}
