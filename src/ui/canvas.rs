use egui::{Pos2, Rect, Response};

use crate::serializer::json::Viewport;
use crate::ui::theme::Theme;

/// 画布视口与交互状态。
///
/// 负责无限网格的渲染、中键平移、滚轮缩放，以及屏幕/世界坐标转换。
/// 内部持有的 `Viewport` 对应 JSON 中的 `viewport` 字段，可被序列化保存。
pub struct Canvas {
    /// 画布视口，对应 JSON 中的 `viewport` 字段
    pub viewport: Viewport,
}

impl Canvas {
    /// 使用默认视口创建画布。
    pub fn new() -> Self {
        Self {
            viewport: Viewport::default(),
        }
    }

    /// 使用指定视口创建画布。
    pub fn with_viewport(viewport: Viewport) -> Self {
        Self { viewport }
    }

    /// 屏幕坐标 -> 世界坐标。
    ///
    /// 屏幕坐标系原点在画布左上角，世界坐标系以 `viewport.x/y` 为画布中心。
    pub fn screen_to_world(&self, screen_pos: Pos2, canvas_rect: Rect) -> Pos2 {
        let center = canvas_rect.center();
        let dx = (screen_pos.x - center.x) / self.viewport.zoom;
        let dy = (screen_pos.y - center.y) / self.viewport.zoom;
        Pos2::new(self.viewport.x + dx, self.viewport.y + dy)
    }

    /// 世界坐标 -> 屏幕坐标。
    pub fn world_to_screen(&self, world_pos: Pos2, canvas_rect: Rect) -> Pos2 {
        let center = canvas_rect.center();
        let dx = (world_pos.x - self.viewport.x) * self.viewport.zoom;
        let dy = (world_pos.y - self.viewport.y) * self.viewport.zoom;
        Pos2::new(center.x + dx, center.y + dy)
    }

    /// 更新画布：处理输入、渲染网格。
    ///
    /// 返回画布响应，包含 egui 交互响应、画布矩形和当前鼠标悬停的世界坐标。
    pub fn update(&mut self, ui: &mut egui::Ui) -> CanvasResponse {
        let canvas_rect = ui.available_rect_before_wrap();

        // 绘制背景
        ui.painter()
            .rect_filled(canvas_rect, 0.0, Theme::BACKGROUND);

        // 处理交互
        let response = ui.interact(
            canvas_rect,
            ui.id().with("canvas"),
            egui::Sense::click_and_drag(),
        );

        // 平移（中键拖拽）
        if response.dragged_by(egui::PointerButton::Middle) {
            let delta = response.drag_delta();
            self.viewport.x -= delta.x / self.viewport.zoom;
            self.viewport.y -= delta.y / self.viewport.zoom;
        }

        // 缩放（滚轮，以鼠标为中心）
        if response.hovered() {
            let scroll = ui.input(|i| i.raw_scroll_delta.y);
            if scroll != 0.0 {
                let mouse_pos = response.hover_pos().unwrap_or_else(|| canvas_rect.center());
                self.zoom_at(mouse_pos, canvas_rect, scroll);
            }
        }

        // 绘制网格
        if self.viewport.show_grid {
            self.paint_grid(ui, canvas_rect);
        }

        let hover_world_pos = response
            .hover_pos()
            .map(|p| self.screen_to_world(p, canvas_rect));

        CanvasResponse {
            response,
            canvas_rect,
            hover_world_pos,
        }
    }

    /// 以指定屏幕点为中心进行缩放。
    fn zoom_at(&mut self, screen_pos: Pos2, canvas_rect: Rect, scroll: f32) {
        let old_zoom = self.viewport.zoom;
        let new_zoom = (old_zoom * (1.0 + scroll * 0.1)).clamp(0.1, 4.0);
        if (new_zoom - old_zoom).abs() < f32::EPSILON {
            return;
        }

        let m = screen_pos - canvas_rect.center();
        let factor = 1.0 / old_zoom - 1.0 / new_zoom;
        self.viewport.x += m.x * factor;
        self.viewport.y += m.y * factor;
        self.viewport.zoom = new_zoom;
    }

    /// 绘制无限网格。
    fn paint_grid(&self, ui: &egui::Ui, canvas_rect: Rect) {
        let world_min = self.screen_to_world(canvas_rect.min, canvas_rect);
        let world_max = self.screen_to_world(canvas_rect.max, canvas_rect);

        let grid_size = self.viewport.grid_size.max(1.0);
        let x_start = (world_min.x / grid_size).floor() * grid_size;
        let x_end = (world_max.x / grid_size).ceil() * grid_size;
        let y_start = (world_min.y / grid_size).floor() * grid_size;
        let y_end = (world_max.y / grid_size).ceil() * grid_size;

        let color = Theme::GRID;
        let mut x = x_start;
        while x <= x_end {
            let p0 = self.world_to_screen(Pos2::new(x, world_min.y), canvas_rect);
            let p1 = self.world_to_screen(Pos2::new(x, world_max.y), canvas_rect);
            ui.painter().line_segment([p0, p1], (1.0, color));
            x += grid_size;
        }

        let mut y = y_start;
        while y <= y_end {
            let p0 = self.world_to_screen(Pos2::new(world_min.x, y), canvas_rect);
            let p1 = self.world_to_screen(Pos2::new(world_max.x, y), canvas_rect);
            ui.painter().line_segment([p0, p1], (1.0, color));
            y += grid_size;
        }
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}

/// 画布响应。
pub struct CanvasResponse {
    /// egui 交互响应
    pub response: Response,
    /// 画布在屏幕上的矩形区域
    pub canvas_rect: Rect,
    /// 鼠标悬停处的世界坐标
    pub hover_world_pos: Option<Pos2>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_world_roundtrip() {
        let canvas = Canvas::new();
        let rect = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(800.0, 600.0));
        let world = Pos2::new(100.0, -50.0);
        let screen = canvas.world_to_screen(world, rect);
        let world2 = canvas.screen_to_world(screen, rect);
        assert!((world2.x - world.x).abs() < 1e-4);
        assert!((world2.y - world.y).abs() < 1e-4);
    }

    #[test]
    fn test_viewport_offset_applies_to_transform() {
        let viewport = Viewport {
            x: 50.0,
            y: -30.0,
            zoom: 2.0,
            grid_size: 20.0,
            show_grid: true,
        };
        let canvas = Canvas::with_viewport(viewport);
        let rect = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(800.0, 600.0));
        // 画布中心的世界坐标应为视口中心
        let center_world = canvas.screen_to_world(rect.center(), rect);
        assert!((center_world.x - 50.0).abs() < 1e-4);
        assert!((center_world.y - (-30.0)).abs() < 1e-4);
    }

    #[test]
    fn test_zoom_limits() {
        let mut canvas = Canvas::new();
        let rect = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(800.0, 600.0));
        canvas.zoom_at(Pos2::new(400.0, 300.0), rect, 100.0);
        assert!(canvas.viewport.zoom <= 4.0);
        canvas.zoom_at(Pos2::new(400.0, 300.0), rect, -100.0);
        assert!(canvas.viewport.zoom >= 0.1);
    }

    #[test]
    fn test_zoom_centered_preserves_world_under_cursor() {
        let mut canvas = Canvas::new();
        let rect = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(800.0, 600.0));
        let cursor = Pos2::new(400.0, 300.0); // 屏幕中心
        let world_before = canvas.screen_to_world(cursor, rect);
        canvas.zoom_at(cursor, rect, 1.0);
        let world_after = canvas.screen_to_world(cursor, rect);
        assert!((world_after.x - world_before.x).abs() < 1e-4);
        assert!((world_after.y - world_before.y).abs() < 1e-4);
    }
}
