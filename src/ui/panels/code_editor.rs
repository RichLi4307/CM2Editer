use crate::ui::i18n::I18n;
use crate::graph::container::ContainerGraph;
use crate::project::Project;

/// 底部 `.code` 文本编辑器面板。内容放入 ScrollArea 防止撑高父容器。
pub struct CodeEditorPanel;

impl CodeEditorPanel {
    pub fn show(ui: &mut egui::Ui, i18n: &I18n, project: &mut Project, graph: &ContainerGraph) -> bool {
        ui.horizontal(|ui| {
            ui.heading(i18n.text("code_editor.title"));
            if project
                .active_code_file()
                .map(|c| c.code_text_dirty)
                .unwrap_or(false)
            {
                ui.colored_label(ui.visuals().warn_fg_color, i18n.text("code_editor.edited"));
            }
        });
        ui.separator();

        let mut changed = false;
        let rows = (ui.available_height() / 14.0).max(4.0) as usize;

        egui::ScrollArea::vertical()
            .id_salt("code_preview_scroll")
            .show(ui, |ui| {
                if let Some(code_file) = project.active_code_file_mut() {
                    let mut text = code_file.code_text.clone();
                    let response = ui.add(
                        egui::TextEdit::multiline(&mut text)
                            .desired_rows(rows)
                            .font(egui::TextStyle::Monospace)
                            .code_editor(),
                    );
                    if response.changed() {
                        code_file.code_text = text;
                        code_file.code_text_dirty = true;
                        changed = true;
                    }

                    ui.horizontal(|ui| {
                        if ui.button(i18n.text("code_editor.regenerate")).clicked() {
                            code_file.graph_doc.graph = graph.clone();
                            let _ = code_file.regenerate_code();
                            changed = true;
                        }
                        if ui
                            .small_button(i18n.text("code_editor.reset"))
                            .on_hover_text(i18n.text("code_editor.reset_tooltip"))
                            .clicked()
                        {
                            code_file.graph_doc.graph = graph.clone();
                            let _ = code_file.regenerate_code();
                            changed = true;
                        }
                    });
                } else {
                    ui.label(i18n.text("code_editor.no_selection"));
                }
            });
        changed
    }
}
