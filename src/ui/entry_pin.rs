use egui::{Align2, FontId, Pos2, Rect, Stroke, Vec2};

use crate::graph::container::LabelContainer;
use crate::graph::node::Vec2 as NodeVec2;
use crate::graph::types::PortType;
use crate::ui::canvas::Canvas;
use crate::ui::edge_renderer::EdgeRenderer;
use crate::ui::i18n::I18n;
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
    /// 逻辑与代码生成器保持一致：使用 `LabelContainer::entry_node_id()` 选择
    /// 最靠左上的无 Flow 入边节点，并返回其 `in_flow` 输入端口。
    pub fn find_entry_port(label: &LabelContainer) -> Option<(String, String)> {
        label.entry_node_id().map(|id| (id, "in_flow".to_string()))
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
    pub fn render_pin(&self, ui: &mut egui::Ui, screen_pos: Pos2, label_name: &str, i18n: &I18n) {
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
            i18n.format("entry_pin.label", &[label_name]),
            FontId::proportional(self.font_size),
            Theme::TEXT_DIM,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::container::LabelContainer;
    use crate::graph::node::{Node, Port, Vec2};
    use crate::graph::types::{NodeType, PortType};

    #[test]
    fn test_find_entry_port_prefers_top_left_no_incoming_flow() {
        let mut label = LabelContainer::default();
        label.id = "label_main".to_string();
        label.name = "main".to_string();

        let mut a = Node::new(NodeType::Log, Vec2::new(100.0, 0.0));
        a.id = "a".to_string();
        a.inputs = vec![Port::new("in_flow", PortType::Flow, "Execute")];
        a.outputs = vec![Port::new("out_flow", PortType::Flow, "Next")];

        let mut b = Node::new(NodeType::Log, Vec2::new(0.0, 0.0));
        b.id = "b".to_string();
        b.inputs = vec![Port::new("in_flow", PortType::Flow, "Execute")];
        b.outputs = vec![Port::new("out_flow", PortType::Flow, "Next")];

        label.nodes.insert(a.id.clone(), a);
        label.nodes.insert(b.id.clone(), b);

        assert_eq!(
            EntryPinRenderer::find_entry_port(&label),
            Some(("b".to_string(), "in_flow".to_string()))
        );
    }

    #[test]
    fn test_find_entry_port_empty_label() {
        let label = LabelContainer::default();
        assert_eq!(EntryPinRenderer::find_entry_port(&label), None);
    }
}
