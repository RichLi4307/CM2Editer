use egui::{Align2, FontId, Pos2, Rect, Stroke, Vec2};

use crate::graph::container::LabelContainer;
use crate::graph::node::Vec2 as NodeVec2;
use crate::graph::types::PortType;
use crate::ui::canvas::Canvas;
use crate::ui::edge_renderer::EdgeRenderer;
use crate::ui::theme::Theme;

/// 入口钉渲染器。
///
/// 入口钉表示当前 `LabelContainer` 的 Flow 执行起点，位于 `entry_pin` 处。
/// 它不是节点，不参与选择、拖拽和序列化，仅作为视觉参考。
#[derive(Debug, Clone, Copy)]
pub struct EntryPinRenderer {
    /// 入口钉圆点半径
    pub radius: f32,
    /// 标签文字偏移
    pub label_offset: f32,
    /// 标签文字大小
    pub font_size: f32,
}

impl Default for EntryPinRenderer {
    fn default() -> Self {
        Self {
            radius: 8.0,
            label_offset: 12.0,
            font_size: 11.0,
        }
    }
}

impl EntryPinRenderer {
    /// 计算入口钉在屏幕上的坐标。
    pub fn screen_pos(&self, canvas: &Canvas, pin: NodeVec2, canvas_rect: Rect) -> Pos2 {
        canvas.world_to_screen(Pos2::new(pin.x, pin.y), canvas_rect)
    }

    /// 查找当前标签的入口端口。
    ///
    /// 逻辑与代码生成器保持一致：优先返回有 `out_flow` 输出且没有 Flow 入边的
    /// 节点的 `in_flow` 输入端口；否则退回到第一个拥有 `out_flow` 输出和
    /// `in_flow` 输入的节点。
    pub fn find_entry_port(label: &LabelContainer) -> Option<(&str, &str)> {
        let has_incoming_flow = |node_id: &str| {
            label.edges.values().any(|e| {
                e.edge_type == PortType::Flow
                    && e.to.node_id == node_id
                    && e.to.port_id == "in_flow"
            })
        };

        for (id, node) in &label.nodes {
            let has_out_flow = node
                .outputs
                .iter()
                .any(|p| p.port_type == PortType::Flow);
            if has_out_flow && !has_incoming_flow(id) {
                if node.inputs.iter().any(|p| p.id == "in_flow") {
                    return Some((id.as_str(), "in_flow"));
                }
            }
        }

        label
            .nodes
            .values()
            .find(|n| {
                n.outputs.iter().any(|p| p.port_type == PortType::Flow)
                    && n.inputs.iter().any(|p| p.id == "in_flow")
            })
            .map(|n| (n.id.as_str(), "in_flow"))
    }

    /// 从入口钉到目标端口绘制 Flow 连线。
    pub fn render_edge(
        &self,
        ui: &mut egui::Ui,
        edge_renderer: &EdgeRenderer,
        from: Pos2,
        to: Pos2,
    ) {
        edge_renderer.render_edge(ui, from, to, &PortType::Flow, &[], false);
    }

    /// 绘制入口钉本体。
    pub fn render_pin(&self, ui: &mut egui::Ui, screen_pos: Pos2, label_name: &str) {
        let color = Theme::ENTRY_PIN;
        ui.painter().circle_filled(screen_pos, self.radius, color);
        ui.painter().circle_stroke(
            screen_pos,
            self.radius,
            Stroke::new(1.5, Theme::TEXT),
        );
        let label_pos = screen_pos + Vec2::new(self.radius + self.label_offset, 0.0);
        ui.painter().text(
            label_pos,
            Align2::LEFT_CENTER,
            format!("入口: {}", label_name),
            FontId::proportional(self.font_size),
            Theme::TEXT_DIM,
        );
    }
}
