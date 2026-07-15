//! 坐标预设选择器浮动窗口。
//!
//! 按场景（箱庭）分组展示坐标卡片，支持搜索和点选。

use crate::api::coordinate::{CoordinateEntry, CoordinateRegistry};
use crate::ui::i18n::I18n;

/// 坐标选择器持久状态。
#[derive(Debug, Clone)]
pub struct CoordinatePickerState {
    pub open: bool,
    pub search: String,
    pub param_key: String,
}

impl CoordinatePickerState {
    pub fn new(param_key: impl Into<String>) -> Self {
        Self {
            open: true,
            search: String::new(),
            param_key: param_key.into(),
        }
    }
}

/// 坐标选择器浮动窗口。
pub struct CoordinatePicker;

impl CoordinatePicker {
    /// 显示坐标选择器。返回 `Some(entry_id)` 当用户点选了一个坐标。
    pub fn show(
        ctx: &egui::Context,
        registry: &CoordinateRegistry,
        state: &mut CoordinatePickerState,
        i18n: &I18n,
    ) -> Option<String> {
        if !state.open {
            return None;
        }

        let mut picked = None;
        let mut closed = false;

        egui::Window::new(i18n.text("coordinate_picker.title"))
            .id(egui::Id::new("coordinate_picker"))
            .collapsible(false)
            .resizable(true)
            .default_size([480.0, 420.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(i18n.text("coordinate_picker.search"));
                    ui.text_edit_singleline(&mut state.search);
                });
                ui.separator();

                let entries = if state.search.is_empty() {
                    registry.entries.iter().collect()
                } else {
                    registry.search(&state.search)
                };

                if entries.is_empty() {
                    ui.label(i18n.text("coordinate_picker.no_match"));
                    return;
                }

                let by_stage: std::collections::BTreeMap<&str, Vec<&CoordinateEntry>> = {
                    let mut map: std::collections::BTreeMap<&str, Vec<&CoordinateEntry>> =
                        std::collections::BTreeMap::new();
                    for e in entries {
                        map.entry(e.stage.as_str()).or_default().push(e);
                    }
                    map
                };

                egui::ScrollArea::vertical()
                    .id_salt("coord_picker_scroll")
                    .show(ui, |ui| {
                        for (stage, stage_entries) in &by_stage {
                            let header = i18n.format("coordinate_picker.items_count", &[stage, &stage_entries.len().to_string()]);
                            egui::CollapsingHeader::new(header)
                                .id_salt(format!("coord_stage_{}", stage))
                                .show(ui, |ui| {
                                    ui.horizontal_wrapped(|ui| {
                                        for e in stage_entries {
                                            if ui.add(coordinate_card(e)).clicked() {
                                                picked = Some(e.id.clone());
                                            }
                                        }
                                    });
                                });
                        }
                    });

                ui.separator();
                if ui.button(i18n.text("button.close")).clicked() {
                    closed = true;
                }
            });

        if closed {
            state.open = false;
        }
        if picked.is_some() {
            state.open = false;
        }
        picked
    }
}

/// 坐标卡片 — 显示名称 + x/y/z + 场景标签。
fn coordinate_card(entry: &CoordinateEntry) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| {
        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(140.0, 50.0), egui::Sense::click());

        let stage_color = stage_tint(&entry.stage);
        let fill = if response.hovered() {
            stage_color.gamma_multiply(0.15)
        } else {
            egui::Color32::from_gray(32)
        };

        ui.painter().rect_filled(rect, 4.0, fill);
        ui.painter().rect_stroke(
            rect,
            4.0,
            egui::Stroke::new(1.5, stage_color.gamma_multiply(0.6)),
            egui::StrokeKind::Middle,
        );

        ui.painter().text(
            rect.left_top() + egui::vec2(6.0, 6.0),
            egui::Align2::LEFT_TOP,
            format!("{}", entry.name),
            egui::FontId::proportional(13.0),
            egui::Color32::WHITE,
        );

        ui.painter().text(
            rect.left_top() + egui::vec2(6.0, 24.0),
            egui::Align2::LEFT_TOP,
            entry.coord_text(),
            egui::FontId::proportional(10.0),
            egui::Color32::from_gray(180),
        );

        response
    }
}

fn stage_tint(stage: &str) -> egui::Color32 {
    let h = stage
        .bytes()
        .fold(0u32, |a, b| a.wrapping_mul(31).wrapping_add(b as u32));
    let palette: [egui::Color32; 8] = [
        egui::Color32::from_rgb(33, 150, 243),
        egui::Color32::from_rgb(76, 175, 80),
        egui::Color32::from_rgb(255, 152, 0),
        egui::Color32::from_rgb(156, 39, 176),
        egui::Color32::from_rgb(0, 188, 212),
        egui::Color32::from_rgb(233, 30, 99),
        egui::Color32::from_rgb(255, 235, 59),
        egui::Color32::from_rgb(121, 85, 72),
    ];
    palette[(h as usize) % palette.len()]
}
