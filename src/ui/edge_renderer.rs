use egui::{Color32, Pos2, Rect, Stroke};

use crate::graph::types::PortType;
use crate::ui::theme::Theme;

/// 连线渲染器配置。
pub struct EdgeRenderer {
    /// 普通连线宽度
    pub normal_width: f32,
    /// 高亮连线宽度
    pub highlighted_width: f32,
    /// 命中测试时曲线采样点数
    pub hit_samples: usize,
}

impl Default for EdgeRenderer {
    fn default() -> Self {
        Self {
            normal_width: 2.0,
            highlighted_width: 3.5,
            hit_samples: 12,
        }
    }
}

impl EdgeRenderer {
    /// 渲染一条连线。
    ///
    /// `from` 和 `to` 为屏幕坐标；`waypoints` 为中间经过点的屏幕坐标。
    /// `edge_type` 决定连线颜色：Flow 为白色，Data 为对应端口类型颜色。
    /// Flow 使用贝塞尔曲线实线；Data 使用沿路径的虚线，以区分数据流与执行流。
    /// `is_highlighted` 为 true 时加粗并绘制蓝色发光外框。
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
        let points = collect_points(from, to, waypoints);

        let is_flow = *edge_type == PortType::Flow;

        if is_highlighted {
            let glow_stroke: egui::Stroke =
                Stroke::new(width + 6.0, Theme::SELECTED_GLOW.gamma_multiply(0.4));
            for window in points.windows(2) {
                let segment_from = window[0];
                let segment_to = window[1];
                if is_flow {
                    let (cp1, cp2) = control_points(segment_from, segment_to);
                    ui.painter().add(egui::epaint::CubicBezierShape {
                        points: [segment_from, cp1, cp2, segment_to],
                        closed: false,
                        fill: Color32::TRANSPARENT,
                        stroke: glow_stroke.into(),
                    });
                } else {
                    draw_dashed_line(
                        ui.painter(),
                        segment_from,
                        segment_to,
                        Theme::SELECTED_GLOW.gamma_multiply(0.4),
                        width + 6.0,
                        5.0,
                        3.0,
                    );
                }
            }
        }

        for window in points.windows(2) {
            let segment_from = window[0];
            let segment_to = window[1];
            if is_flow {
                let (cp1, cp2) = control_points(segment_from, segment_to);
                ui.painter().add(egui::epaint::CubicBezierShape {
                    points: [segment_from, cp1, cp2, segment_to],
                    closed: false,
                    fill: Color32::TRANSPARENT,
                    stroke: Stroke::new(width, color).into(),
                });
            } else {
                draw_dashed_line(
                    ui.painter(),
                    segment_from,
                    segment_to,
                    color,
                    width,
                    5.0,
                    3.0,
                );
            }
        }
    }

    /// 使用指定颜色渲染一条临时连线（不依赖端口类型）。
    pub fn render_edge_with_color(
        &self,
        ui: &mut egui::Ui,
        from: Pos2,
        to: Pos2,
        color: Color32,
        waypoints: &[Pos2],
    ) {
        let stroke: egui::Stroke = Stroke::new(self.normal_width, color);
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

    /// 计算连线的近似命中矩形，用于点击/框选检测。
    ///
    /// 以贝塞尔曲线采样点的外接框向外扩展一定边距作为命中区域。
    pub fn hit_rect(&self, from: Pos2, to: Pos2, waypoints: &[Pos2]) -> Rect {
        const PADDING: f32 = 6.0;
        let points = collect_points(from, to, waypoints);
        let mut min = from;
        let mut max = from;
        for window in points.windows(2) {
            let a = window[0];
            let b = window[1];
            let (cp1, cp2) = control_points(a, b);
            for t in 0..=self.hit_samples {
                let t = t as f32 / self.hit_samples as f32;
                let p = cubic_bezier(a, cp1, cp2, b, t);
                min.x = min.x.min(p.x);
                min.y = min.y.min(p.y);
                max.x = max.x.max(p.x);
                max.y = max.y.max(p.y);
            }
        }
        Rect::from_min_max(min, max).expand(PADDING)
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

/// 计算三次贝塞尔曲线上 t ∈ [0, 1] 处的点。
fn cubic_bezier(p0: Pos2, p1: Pos2, p2: Pos2, p3: Pos2, t: f32) -> Pos2 {
    let u = 1.0 - t;
    let a = u * u * u;
    let b = 3.0 * u * u * t;
    let c = 3.0 * u * t * t;
    let d = t * t * t;
    Pos2::new(
        a * p0.x + b * p1.x + c * p2.x + d * p3.x,
        a * p0.y + b * p1.y + c * p2.y + d * p3.y,
    )
}

/// 根据连线类型和高亮状态返回颜色。
fn edge_color(edge_type: &PortType, is_highlighted: bool) -> Color32 {
    let base = match edge_type {
        PortType::Flow => Theme::WIRE_DEFAULT,
        _ => crate::ui::theme::port_color(edge_type),
    };
    if is_highlighted {
        Theme::SELECTED_GLOW
    } else {
        base
    }
}

/// 绘制从 `from` 到 `to` 的虚线，用于 Data 流连线。
fn draw_dashed_line(
    painter: &egui::Painter,
    from: Pos2,
    to: Pos2,
    color: Color32,
    width: f32,
    dash_len: f32,
    gap_len: f32,
) {
    let dir = to - from;
    let len = dir.length();
    if len < 1e-5 {
        return;
    }
    let unit = dir / len;
    let mut distance = 0.0;
    while distance < len {
        let start = from + unit * distance;
        let end = from + unit * (distance + dash_len).min(len);
        painter.line_segment([start, end], Stroke::new(width, color));
        distance += dash_len + gap_len;
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

    #[test]
    fn test_hit_rect_contains_curve() {
        let renderer = EdgeRenderer::default();
        let from = Pos2::new(0.0, 0.0);
        let to = Pos2::new(100.0, 0.0);
        let rect = renderer.hit_rect(from, to, &[]);
        assert!(rect.contains(Pos2::new(50.0, 0.0)));
    }
}
