//! Floating namespace picker window for selecting game-specific keys such as
//! cosplay items, adult toys, avatar types, etc.
//!
//! The picker is stateless: callers hold a [`NamespacePickerState`] and apply
//! the returned keys to the target parameter.

use std::collections::HashSet;

use crate::api::namespace::NamespaceRegistry;
use crate::ui::i18n::I18n;
use crate::ui::theme::tokens;
use crate::ui::token_widgets;

/// Persistent state for the namespace picker window.
#[derive(Debug, Clone)]
pub struct NamespacePickerState {
    pub open: bool,
    pub namespace: String,
    /// Parameter key that will receive the selected value(s).
    pub param_key: String,
    /// If true, multiple entries can be selected and returned as a JSON array.
    pub multi: bool,
    pub search: String,
    pub selected: HashSet<String>,
}

impl NamespacePickerState {
    /// Create a new picker state for the given namespace and parameter.
    pub fn new(namespace: impl Into<String>, param_key: impl Into<String>, multi: bool) -> Self {
        Self {
            open: true,
            namespace: namespace.into(),
            param_key: param_key.into(),
            multi,
            search: String::new(),
            selected: HashSet::new(),
        }
    }

    /// Pre-select a list of existing keys.
    pub fn with_selected(mut self, keys: &[String]) -> Self {
        self.selected.extend(keys.iter().cloned());
        self
    }
}

/// A floating namespace picker window.
pub struct NamespacePicker;

impl NamespacePicker {
    /// Show the picker window. Returns `Some(keys)` when the user confirms.
    pub fn show(
        ctx: &egui::Context,
        registry: &NamespaceRegistry,
        state: &mut NamespacePickerState,
        lang: &str,
        i18n: &I18n,
    ) -> Option<Vec<String>> {
        if !state.open {
            return None;
        }

        let mut confirmed = false;
        let mut cancelled = false;
        let title = if state.multi {
            i18n.text("namespace_picker.title_multi")
        } else {
            i18n.text("namespace_picker.title")
        };

        egui::Window::new(title)
            .id(egui::Id::new("namespace_picker"))
            .collapsible(false)
            .resizable(true)
            .default_size([360.0, 420.0])
            .show(ctx, |ui| {
                // Search bar
                ui.horizontal(|ui| {
                    ui.label(i18n.text("namespace_picker.search"));
                    ui.text_edit_singleline(&mut state.search);
                });
                ui.separator();

                let namespace = match registry.get(&state.namespace) {
                    Some(ns) => ns,
                    None => {
                        ui.label(i18n.format("namespace_picker.not_found", &[&state.namespace]));
                        return;
                    }
                };
                let query = state.search.clone();
                let entries = namespace.search(&query, lang);

                ui.label(i18n.format("namespace_picker.items_selected", &[&entries.len().to_string(), &state.selected.len().to_string()]));

                let has_cats = entries.iter().any(|e| e.category.is_some());

                if has_cats {
                    let mut by_cat: std::collections::BTreeMap<
                        String,
                        Vec<&crate::api::namespace::NamespaceEntry>,
                    > = std::collections::BTreeMap::new();
                    for e in &entries {
                        let cat = e.category.clone().unwrap_or_else(|| i18n.text("label.category_other"));
                        by_cat.entry(cat).or_default().push(e);
                    }
                    egui::ScrollArea::vertical()
                        .id_salt("namespace_picker_scroll")
                        .auto_shrink([false, true])
                        .max_height(280.0)
                        .show(ui, |ui| {
                            for (cat, cat_entries) in &by_cat {
                                let cat_header = i18n.format("label.items_count", &[cat, &cat_entries.len().to_string()]);
                                egui::CollapsingHeader::new(cat_header)
                                    .id_salt(format!("nspick_{}_{}", state.namespace, cat))
                                    .show(ui, |ui| {
                                        if state.multi {
                                            // 多选模式：使用 checkbox 列表，交互明确。
                                            for entry in cat_entries {
                                                let display = entry.display_name(lang);
                                                let label = if display == entry.key {
                                                    entry.key.clone()
                                                } else {
                                                    format!("{} ({})", display, entry.key)
                                                };
                                                let mut selected = state.selected.contains(&entry.key);
                                                if ui.checkbox(&mut selected, label).changed() {
                                                    if selected {
                                                        state.selected.insert(entry.key.clone());
                                                    } else {
                                                        state.selected.remove(&entry.key);
                                                    }
                                                }
                                            }
                                        } else {
                                            // 单选模式：使用卡片，点击即选中。
                                            ui.horizontal_wrapped(|ui| {
                                                for entry in cat_entries {
                                                    if ui.add(ns_picker_card(entry, state, lang)).clicked() {
                                                        state.selected.clear();
                                                        state.selected.insert(entry.key.clone());
                                                    }
                                                }
                                            });
                                        }
                                    });
                            }
                        });
                } else {
                    let row_height = 20.0;
                    egui::ScrollArea::vertical()
                        .id_salt("namespace_picker_scroll")
                        .auto_shrink([false, true])
                        .max_height(300.0)
                        .show_rows(ui, row_height, entries.len(), |ui, range| {
                            for entry in &entries[range] {
                                let display = entry.display_name(lang);
                                let label = if display == entry.key {
                                    entry.key.clone()
                                } else {
                                    format!("{} ({})", display, entry.key)
                                };
                                let is_selected = state.selected.contains(&entry.key);
                                if state.multi {
                                    let mut selected = is_selected;
                                    if ui.checkbox(&mut selected, label).changed() {
                                        if selected {
                                            state.selected.insert(entry.key.clone());
                                        } else {
                                            state.selected.remove(&entry.key);
                                        }
                                    }
                                } else {
                                    if ui.selectable_label(is_selected, label).clicked() {
                                        state.selected.clear();
                                        state.selected.insert(entry.key.clone());
                                    }
                                }
                            }
                        });
                }

                ui.separator();
                ui.horizontal(|ui| {
                    if token_widgets::button(ui, i18n.text("button.confirm")).clicked() {
                        confirmed = true;
                    }
                    if token_widgets::button(ui, i18n.text("button.cancel")).clicked() {
                        cancelled = true;
                    }
                    if token_widgets::button(ui, i18n.text("button.clear")).clicked() {
                        state.selected.clear();
                    }
                });
            });

        if cancelled {
            state.open = false;
            None
        } else if confirmed {
            state.open = false;
            Some(state.selected.iter().cloned().collect())
        } else {
            None
        }
    }
}

/// 命名空间选择器卡片 — 紧凑版，显示中文名，选中高亮。
/// 当前仅用于分类视图的单选模式。
fn ns_picker_card<'a>(
    entry: &'a crate::api::namespace::NamespaceEntry,
    state: &'a NamespacePickerState,
    lang: &'a str,
) -> impl egui::Widget + 'a {
    let is_selected = state.selected.contains(&entry.key);
    move |ui: &mut egui::Ui| {
        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(130.0, 40.0), egui::Sense::click());
        let accent = if is_selected {
            tokens::ACCENT
        } else {
            tokens::BORDER_SUBTLE
        };
        let fill = if response.hovered() || is_selected {
            accent.gamma_multiply(0.12)
        } else {
            tokens::BG_CARD
        };
        ui.painter().rect_filled(rect, 4.0, fill);
        ui.painter().rect_stroke(rect, 4.0, egui::Stroke::new(1.2, accent), egui::StrokeKind::Middle);

        let zh = entry.display_name(lang);
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            zh,
            egui::FontId::proportional(12.0),
            tokens::TEXT_PRIMARY,
        );
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace_picker_state_builder() {
        let state = NamespacePickerState::new("cosplay", "cosplayKeys", true)
            .with_selected(&["Maid".to_string(), "Bunny".to_string()]);
        assert!(state.open);
        assert!(state.multi);
        assert_eq!(state.selected.len(), 2);
    }
}
