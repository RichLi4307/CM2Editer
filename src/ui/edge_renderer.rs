use egui::{Color32, Pos2, Stroke};

use crate::graph::types::PortType;
use crate::ui::theme::Theme;

/// 连线渲染器配置。
pub struct EdgeRenderer {
    /// 普通连线宽度
    pub normal_width: f32,
    /// 高亮连线宽度
    pub highlighted_width: f32,
}

impl Default for EdgeRenderer {
    fn default() -> Self {
        Self {
            normal_width: 2.0,
            highlighted_width: 3.0,
        }
    }
}

impl EdgeRenderer {
    /// 渲染一条贝塞尔连线。
    ///
    /// `from` 和 `to` 为屏幕坐标；`waypoints` 为中间经过点的屏幕坐标。
    /// `edge_type` 决定连线颜色：Flow 为白色，Data 为对应端口类型颜色。
    /// `is_highlighted` 为 true 时加粗并提高亮度。
    pub fn render_edge(
        &self,
        ui: &mut egui::Ui,
        from: Pos2,
        to: Pos2,
        edge_type: &PortType,
        waypoints: &[Pos2],
        is_highlighted: bool,
    ) {
        let color = edge_color(edge_type, is_highlighted);
        let width = if is_highlighted {
            self.highlighted_width
        } else {
            self.normal_width
        };
        let stroke: egui::Stroke = Stroke::new(width, color);

        let points = collect_points(from, to, waypoints);
        for window in points.windows(2) {
            let segment_from = window[0];
            let segment_to = window[1];
            let (cp1, cp2) = control_points(segment_from, segment_to);
            ui.painter().add(egui::epaint::CubicBezierShape {
                points: [segment_from, cp1, cp2, segment_to],
                closed: false,
                fill: Color32::TRANSPARENT,
                stroke: stroke.into(),
            });
        }
    }
}

/// 收集连线经过的所有点（包含起点和终点）。
fn collect_points(from: Pos2, to: Pos2, waypoints: &[Pos2]) -> Vec<Pos2> {
    let mut points = Vec::with_capacity(waypoints.len() + 2);
    points.push(from);
    points.extend(waypoints);
    points.push(to);
    points
}

/// 计算两点间三次贝塞尔的控制点。
fn control_points(from: Pos2, to: Pos2) -> (Pos2, Pos2) {
    let dx = (to.x - from.x).abs();
    let offset = (dx * 0.5).max(20.0);
    let cp1 = Pos2::new(from.x + offset, from.y);
    let cp2 = Pos2::new(to.x - offset, to.y);
    (cp1, cp2)
}

/// 根据连线类型和高亮状态返回颜色。
fn edge_color(edge_type: &PortType, is_highlighted: bool) -> Color32 {
    let base = match edge_type {
        PortType::Flow => Theme::WIRE_DEFAULT,
        _ => crate::ui::theme::port_color(edge_type),
    };
    if is_highlighted {
        // 高亮时提亮颜色
        base.additive()
    } else {
        base
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control_points() {
        let from = Pos2::new(0.0, 0.0);
        let to = Pos2::new(100.0, 0.0);
        let (cp1, cp2) = control_points(from, to);
        assert!(cp1.x > from.x);
        assert!(cp2.x < to.x);
    }

    #[test]
    fn test_collect_points_with_waypoints() {
        let from = Pos2::new(0.0, 0.0);
        let to = Pos2::new(100.0, 100.0);
        let wp = vec![Pos2::new(50.0, 0.0)];
        let points = collect_points(from, to, &wp);
        assert_eq!(points.len(), 3);
        assert_eq!(points[0], from);
        assert_eq!(points[1], wp[0]);
        assert_eq!(points[2], to);
    }
}
