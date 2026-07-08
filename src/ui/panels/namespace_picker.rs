//! Floating namespace picker window for selecting game-specific keys such as
//! cosplay items, adult toys, avatar types, etc.
//!
//! The picker is stateless: callers hold a [`NamespacePickerState`] and apply
//! the returned keys to the target parameter.

use std::collections::HashSet;

use crate::api::namespace::NamespaceRegistry;

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
    ) -> Option<Vec<String>> {
        if !state.open {
            return None;
        }

        let mut confirmed = false;
        let mut cancelled = false;

        egui::Window::new("命名空间选择器")
            .id(egui::Id::new("namespace_picker"))
            .collapsible(false)
            .resizable(true)
            .default_size([360.0, 420.0])
            .show(ctx, |ui| {
                // Search bar
                ui.horizontal(|ui| {
                    ui.label("搜索:");
                    ui.text_edit_singleline(&mut state.search);
                });
                ui.separator();

                let namespace = registry.get(&state.namespace);
                if namespace.is_none() {
                    ui.label(format!("未找到命名空间: {}", state.namespace));
                    return;
                }
                let namespace = namespace.unwrap();
                let query = state.search.clone();
                let entries = namespace.search(&query, "zh");

                ui.label(format!("{} 项 (已选 {})", entries.len(), state.selected.len()));

                let row_height = 20.0;
                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show_rows(ui, row_height, entries.len(), |ui, range| {
                        for entry in &entries[range] {
                            let display = entry.display_name("zh");
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

                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("确定").clicked() {
                        confirmed = true;
                    }
                    if ui.button("取消").clicked() {
                        cancelled = true;
                    }
                    if ui.button("清空").clicked() {
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
