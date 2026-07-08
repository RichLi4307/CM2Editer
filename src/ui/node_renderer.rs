use egui::{Align2, CornerRadius, FontId, Pos2, Rect, Stroke, Vec2};

use crate::api::definitions::NodeDefinition;
use crate::graph::node::{Node, ParamValue, Port};
use crate::graph::types::PortType;
use crate::ui::canvas::Canvas;
use crate::ui::theme::{Theme, category_color, port_color};

/// 节点渲染器配置。
pub struct NodeRenderer {
    /// 端口圆半径
    pub port_radius: f32,
    /// 标题栏高度
    pub header_height: f32,
    /// 圆角半径
    pub corner_radius: f32,
    /// 字体大小
    pub font_size: f32,
    /// 端口与边界的内边距
    pub port_padding: f32,
    /// 端口垂直间距
    pub port_spacing: f32,
    /// 节点最小宽度
    pub min_width: f32,
    /// 节点最小高度
    pub min_height: f32,
}

impl Default for NodeRenderer {
    fn default() -> Self {
        Self {
            port_radius: 6.0,
            header_height: 32.0,
            corner_radius: 8.0,
            font_size: 12.0,
            port_padding: 8.0,
            port_spacing: 20.0,
            min_width: 180.0,
            min_height: 80.0,
        }
    }
}

/// 端口渲染后的几何信息。
#[derive(Debug, Clone)]
pub struct PortGeometry {
    /// 端口 ID
    pub id: String,
    /// 端口标签
    pub label: String,
    /// 端口类型
    pub port_type: PortType,
    /// 端口圆心屏幕坐标
    pub center: Pos2,
    /// 是否为输入端口
    pub is_input: bool,
}

/// 节点渲染响应。
#[derive(Debug, Clone)]
pub struct NodeRenderResponse {
    /// 节点在屏幕上的矩形区域
    pub rect: Rect,
    /// 所有端口的渲染几何信息
    pub ports: Vec<PortGeometry>,
}

impl NodeRenderer {
    /// 计算节点在屏幕上的矩形区域。
    pub fn screen_rect(&self, canvas: &Canvas, node: &Node, canvas_rect: Rect) -> Rect {
        let screen_pos =
            canvas.world_to_screen(Pos2::new(node.position.x, node.position.y), canvas_rect);
        let height = if node.collapsed {
            self.collapsed_height(node)
        } else {
            let min_height = self.min_height.max(self.min_height_from_ports(node));
            node.size.y.max(min_height)
        };
        let size = Vec2::new(node.size.x.max(self.min_width), height);
        Rect::from_min_size(screen_pos, size)
    }

    /// 根据节点端口数量计算最小高度。
    fn min_height_from_ports(&self, node: &Node) -> f32 {
        let max_ports = node.inputs.len().max(node.outputs.len()) as f32;
        let ports_height = if max_ports <= 0.0 {
            0.0
        } else {
            self.port_padding * 2.0 + self.port_radius * 2.0 + (max_ports - 1.0) * self.port_spacing
        };
        self.header_height + ports_height
    }

    /// 计算折叠状态下的最小高度（标题栏 + 端口所需空间）。
    fn collapsed_height(&self, node: &Node) -> f32 {
        let max_ports = node.inputs.len().max(node.outputs.len()) as f32;
        let ports_height = if max_ports <= 0.0 {
            0.0
        } else {
            (max_ports - 1.0) * self.port_spacing
        };
        self.header_height + self.port_padding * 2.0 + self.port_radius * 2.0 + ports_height
    }

    /// 计算节点端口在屏幕上的几何位置（不渲染）。
    pub fn port_positions(&self, node: &Node, rect: Rect) -> Vec<PortGeometry> {
        let mut ports = Vec::new();
        let body_top = rect.min.y + self.header_height;
        let input_y_start = body_top + self.port_padding + self.port_radius;

        let (flow_inputs, data_inputs): (Vec<&Port>, Vec<&Port>) = node
            .inputs
            .iter()
            .partition(|p| p.port_type == PortType::Flow);

        for (i, port) in flow_inputs.iter().chain(data_inputs.iter()).enumerate() {
            let center = Pos2::new(
                rect.min.x + self.port_radius,
                input_y_start + i as f32 * self.port_spacing,
            );
            ports.push(PortGeometry {
                id: port.id.clone(),
                label: port.label.clone(),
                port_type: port.port_type.clone(),
                center,
                is_input: true,
            });
        }

        let (flow_outputs, data_outputs): (Vec<&Port>, Vec<&Port>) = node
            .outputs
            .iter()
            .partition(|p| p.port_type == PortType::Flow);

        for (i, port) in flow_outputs.iter().chain(data_outputs.iter()).enumerate() {
            let center = Pos2::new(
                rect.max.x - self.port_radius,
                input_y_start + i as f32 * self.port_spacing,
            );
            ports.push(PortGeometry {
                id: port.id.clone(),
                label: port.label.clone(),
                port_type: port.port_type.clone(),
                center,
                is_input: false,
            });
        }

        ports
    }

    /// 渲染单个节点卡片。
    ///
    /// `is_selected` 为 true 时绘制蓝色发光外框；`has_errors` 为 true 时绘制红色边框。
    /// 若节点分类为空，则回退到 `definition.category`。
    pub fn render(
        &self,
        ui: &mut egui::Ui,
        canvas: &Canvas,
        node: &Node,
        definition: &NodeDefinition,
        is_selected: bool,
        has_errors: bool,
    ) -> NodeRenderResponse {
        let canvas_rect = ui.available_rect_before_wrap();
        let rect = self.screen_rect(canvas, node, canvas_rect);
        let ports = self.port_positions(node, rect);
        self.render_with_data(ui, node, definition, rect, &ports, is_selected, has_errors);
        NodeRenderResponse { rect, ports }
    }

    /// 使用已计算好的几何数据渲染节点。
    ///
    /// 用于分阶段渲染：先收集数据做裁剪和排序，再统一绘制。
    pub fn render_with_data(
        &self,
        ui: &mut egui::Ui,
        node: &Node,
        definition: &NodeDefinition,
        rect: Rect,
        ports: &[PortGeometry],
        is_selected: bool,
        has_errors: bool,
    ) {
        let category = if node.category.is_empty() {
            &definition.category
        } else {
            &node.category
        };
        let title_color = category_color(category);
        let body_color = Theme::NODE_BACKGROUND;

        // 绘制选中状态的发光外框
        if is_selected {
            let glow_rect = rect.expand(4.0);
            let glow = Theme::SELECTED_GLOW;
            let glow_faded =
                egui::Color32::from_rgba_premultiplied(glow.r(), glow.g(), glow.b(), 128);
            ui.painter()
                .rect_filled(glow_rect, self.corner_radius, glow_faded);
        }

        // 绘制错误状态边框
        let border_color = if has_errors {
            Theme::ERROR
        } else {
            Theme::NODE_BORDER
        };
        let border_stroke = Stroke::new(if has_errors { 2.0 } else { 1.0 }, border_color);

        // 绘制节点主体
        ui.painter()
            .rect_filled(rect, self.corner_radius as u8, body_color);
        ui.painter().rect_stroke(
            rect,
            self.corner_radius as u8,
            border_stroke,
            egui::StrokeKind::Middle,
        );

        // 绘制标题栏
        let header_rect =
            Rect::from_min_size(rect.min, Vec2::new(rect.width(), self.header_height));
        let header_corner_radius = CornerRadius {
            nw: self.corner_radius as u8,
            ne: self.corner_radius as u8,
            sw: 0,
            se: 0,
        };
        ui.painter().add(egui::epaint::RectShape::filled(
            header_rect,
            header_corner_radius,
            title_color,
        ));

        // 标题文字
        ui.painter().text(
            header_rect.center(),
            Align2::CENTER_CENTER,
            &definition.display_name,
            FontId::proportional(self.font_size),
            Theme::TEXT,
        );

        // 渲染端口
        for port in ports {
            self.paint_port_with_geometry(ui, port);
        }

        // 绘制参数预览（如果未折叠）
        if !node.collapsed {
            self.paint_param_preview(ui, node, rect);
        }
    }

    /// 使用已有的几何信息绘制端口。
    fn paint_port_with_geometry(&self, ui: &egui::Ui, port: &PortGeometry) {
        let port_def = Port {
            id: port.id.clone(),
            port_type: port.port_type.clone(),
            label: port.label.clone(),
            required: false,
        };
        self.paint_port(ui, port.center, &port_def, port.is_input);
    }

    /// 布局并渲染端口，返回端口几何信息。
    #[allow(dead_code)]
    fn layout_and_paint_ports(&self, ui: &egui::Ui, node: &Node, rect: Rect) -> Vec<PortGeometry> {
        let ports = self.port_positions(node, rect);
        for port in &ports {
            self.paint_port_with_geometry(ui, port);
        }
        ports
    }

    /// 绘制单个端口圆点。
    fn paint_port(&self, ui: &egui::Ui, center: Pos2, port: &Port, is_input: bool) {
        let color = port_color(&port.port_type);
        ui.painter().circle_filled(center, self.port_radius, color);
        let label_pos = if is_input {
            Pos2::new(center.x + self.port_radius + 4.0, center.y)
        } else {
            Pos2::new(center.x - self.port_radius - 4.0, center.y)
        };
        let label_align = if is_input {
            Align2::LEFT_CENTER
        } else {
            Align2::RIGHT_CENTER
        };
        ui.painter().text(
            label_pos,
            label_align,
            &port.label,
            FontId::proportional(self.font_size - 1.0),
            Theme::TEXT_DIM,
        );
    }

    /// 在节点主体中绘制参数预览。
    fn paint_param_preview(&self, ui: &egui::Ui, node: &Node, rect: Rect) {
        // 参数预览放在所有端口下方，避免与端口标签重叠
        let port_count = node.inputs.len().max(node.outputs.len()) as f32;
        let body_top = rect.min.y + self.header_height;
        let params_x = rect.min.x + self.port_radius * 2.0 + 4.0;
        let mut params_y = body_top
            + self.port_padding
            + self.port_radius
            + (port_count - 1.0) * self.port_spacing
            + self.port_spacing * 0.5;

        for (name, value) in &node.params {
            if params_y > rect.max.y - self.port_padding {
                break;
            }
            let text = format!("{}: {}", name, param_preview(value));
            ui.painter().text(
                Pos2::new(params_x, params_y),
                Align2::LEFT_CENTER,
                text,
                FontId::proportional(self.font_size - 1.0),
                Theme::TEXT_DIM,
            );
            params_y += self.port_spacing * 0.8;
        }
    }
}

/// 生成参数值的简短预览文本。
fn param_preview(value: &ParamValue) -> String {
    match value {
        ParamValue::Null => "(null)".to_string(),
        ParamValue::Ref { node, port } => format!("→ {}/{}", node, port),
        ParamValue::Literal(v) => {
            let s = v.to_string();
            if s.len() > 20 {
                format!("{}…", &s[..20])
            } else {
                s
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collapsed_node_height_is_smaller() {
        use crate::graph::node::Vec2 as NodeVec2;
        use crate::graph::types::NodeType;
        use crate::serializer::json::Viewport;
        use crate::ui::canvas::Canvas;

        let viewport = Viewport::default();
        let canvas = Canvas::with_viewport(viewport);
        let canvas_rect = Rect::from_min_size(Pos2::new(0.0, 0.0), Vec2::new(800.0, 600.0));
        let renderer = NodeRenderer::default();

        let mut expanded = Node::new(NodeType::Log, NodeVec2::ZERO);
        expanded.collapsed = false;
        expanded.size = NodeVec2::new(180.0, 120.0);
        expanded.inputs = vec![Port::new("in_flow", PortType::Flow, "执行")];
        expanded.outputs = vec![Port::new("out_flow", PortType::Flow, "下一步")];

        let mut collapsed = expanded.clone();
        collapsed.collapsed = true;

        let expanded_rect = renderer.screen_rect(&canvas, &expanded, canvas_rect);
        let collapsed_rect = renderer.screen_rect(&canvas, &collapsed, canvas_rect);

        assert!(
            collapsed_rect.height() < expanded_rect.height(),
            "collapsed height {} should be less than expanded height {}",
            collapsed_rect.height(),
            expanded_rect.height()
        );
    }

    #[test]
    fn test_collapsed_height_matches_ports() {
        use crate::graph::node::Vec2 as NodeVec2;
        use crate::graph::types::NodeType;
        use crate::serializer::json::Viewport;
        use crate::ui::canvas::Canvas;

        let viewport = Viewport::default();
        let canvas = Canvas::with_viewport(viewport);
        let canvas_rect = Rect::from_min_size(Pos2::new(0.0, 0.0), Vec2::new(800.0, 600.0));
        let renderer = NodeRenderer::default();

        let mut node = Node::new(NodeType::If, NodeVec2::ZERO);
        node.collapsed = true;
        node.size = NodeVec2::new(180.0, 120.0);
        node.inputs = vec![Port::new("in_flow", PortType::Flow, "执行")];
        node.outputs = vec![
            Port::new("out_true", PortType::Flow, "True"),
            Port::new("out_false", PortType::Flow, "False"),
        ];

        let rect = renderer.screen_rect(&canvas, &node, canvas_rect);
        let expected_min =
            renderer.header_height + renderer.port_padding * 2.0 + renderer.port_radius * 2.0;
        assert!(rect.height() >= expected_min);
        // 2 output ports means one spacing step between them
        let expected_with_ports = expected_min + renderer.port_spacing;
        assert!((rect.height() - expected_with_ports).abs() < 0.01);
    }

    #[test]
    fn test_node_height_adapts_to_port_count() {
        use crate::graph::node::Vec2 as NodeVec2;
        use crate::graph::types::NodeType;
        use crate::serializer::json::Viewport;
        use crate::ui::canvas::Canvas;

        let viewport = Viewport::default();
        let canvas = Canvas::with_viewport(viewport);
        let canvas_rect = Rect::from_min_size(Pos2::new(0.0, 0.0), Vec2::new(800.0, 600.0));
        let renderer = NodeRenderer::default();

        let mut few_ports = Node::new(NodeType::Log, NodeVec2::ZERO);
        few_ports.inputs = vec![Port::new("in_flow", PortType::Flow, "执行")];
        few_ports.outputs = vec![Port::new("out_flow", PortType::Flow, "下一步")];

        let mut many_ports = Node::new(NodeType::Color, NodeVec2::ZERO);
        many_ports.inputs = vec![
            Port::new("in_flow", PortType::Flow, "执行"),
            Port::new("r", PortType::Number, "红"),
            Port::new("g", PortType::Number, "绿"),
            Port::new("b", PortType::Number, "蓝"),
            Port::new("a", PortType::Number, "透明度"),
        ];
        many_ports.outputs = vec![
            Port::new("out_flow", PortType::Flow, "下一步"),
            Port::new("out_color", PortType::List, "颜色"),
        ];

        let few_rect = renderer.screen_rect(&canvas, &few_ports, canvas_rect);
        let many_rect = renderer.screen_rect(&canvas, &many_ports, canvas_rect);

        assert!(
            many_rect.height() > few_rect.height(),
            "多端口节点应比少端口节点高"
        );
    }
}
