use crate::project::Project;

/// 右栏 `meta.json` 编辑器面板。
pub struct MetaEditorPanel;

impl MetaEditorPanel {
    /// 显示 `meta.json` 文本编辑器，返回文本是否发生变化。
    pub fn show(ui: &mut egui::Ui, project: &mut Project) -> bool {
        ui.heading("工程设置 (meta.json)");
        ui.separator();

        if project.meta_text_invalid {
            ui.colored_label(
                ui.visuals().error_fg_color,
                "⚠ JSON 格式错误，无法保存到 meta 对象",
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
