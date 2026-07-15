use crate::project::Project;
use crate::ui::i18n::I18n;

/// 右栏 `meta.json` 编辑器面板。
pub struct MetaEditorPanel;

impl MetaEditorPanel {
    /// 显示 `meta.json` 文本编辑器，返回文本是否发生变化。
    pub fn show(ui: &mut egui::Ui, i18n: &I18n, project: &mut Project) -> bool {
        ui.heading(i18n.text("meta_editor.title"));
        ui.separator();

        if project.meta_text_invalid {
            ui.colored_label(
                ui.visuals().error_fg_color,
                i18n.text("meta_editor.json_error"),
            );
        }

        let mut text = project.meta_text.clone();
        let response = ui.add(
            egui::TextEdit::multiline(&mut text)
                .desired_rows(20)
                .font(egui::TextStyle::Monospace)
                .code_editor(),
        );

        if response.changed() {
            project.set_meta_text(&text);
            true
        } else {
            false
        }
    }
}
