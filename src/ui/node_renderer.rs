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
        let screen_pos =
            canvas.world_to_screen(Pos2::new(node.position.x, node.position.y), canvas_rect);
        let size = Vec2::new(
            node.size.x.max(self.min_width),
            node.size.y.max(self.min_height),
        );
        let rect = Rect::from_min_size(screen_pos, size);

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

        // 计算端口布局并渲染
        let ports = self.layout_and_paint_ports(ui, node, rect);

        // 绘制参数预览（如果未折叠）
        if !node.collapsed {
            self.paint_param_preview(ui, node, rect);
        }

        NodeRenderResponse { rect, ports }
    }

    /// 布局并渲染端口，返回端口几何信息。
    fn layout_and_paint_ports(&self, ui: &egui::Ui, node: &Node, rect: Rect) -> Vec<PortGeometry> {
        let mut ports = Vec::new();
        let body_top = rect.min.y + self.header_height;
        let input_y_start = body_top + self.port_padding + self.port_radius;

        // 输入端口：Flow 在前，Data 在后，放在左侧
        let (flow_inputs, data_inputs): (Vec<&Port>, Vec<&Port>) = node
            .inputs
            .iter()
            .partition(|p| p.port_type == PortType::Flow);

        for (i, port) in flow_inputs.iter().chain(data_inputs.iter()).enumerate() {
            let center = Pos2::new(
                rect.min.x + self.port_radius,
                input_y_start + i as f32 * self.port_spacing,
            );
            self.paint_port(ui, center, port, true);
            ports.push(PortGeometry {
                id: port.id.clone(),
                port_type: port.port_type.clone(),
                center,
                is_input: true,
            });
        }

        // 输出端口：Flow 在前，Data 在后，放在右侧
        let (flow_outputs, data_outputs): (Vec<&Port>, Vec<&Port>) = node
            .outputs
            .iter()
            .partition(|p| p.port_type == PortType::Flow);

        for (i, port) in flow_outputs.iter().chain(data_outputs.iter()).enumerate() {
            let center = Pos2::new(
                rect.max.x - self.port_radius,
                input_y_start + i as f32 * self.port_spacing,
            );
            self.paint_port(ui, center, port, false);
            ports.push(PortGeometry {
                id: port.id.clone(),
                port_type: port.port_type.clone(),
                center,
                is_input: false,
            });
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
        let body_top = rect.min.y + self.header_height;
        let params_x = rect.min.x + self.min_width * 0.35;
        let mut params_y = body_top + self.port_padding + self.port_radius;

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
    fn test_param_preview() {
        assert_eq!(param_preview(&ParamValue::Null), "(null)");
        assert_eq!(
            param_preview(&ParamValue::from_ref("node_1", "out_value")),
            "→ node_1/out_value"
        );
        assert_eq!(
            param_preview(&ParamValue::Literal(serde_json::json!("hello"))),
            "\"hello\""
        );
    }
}
