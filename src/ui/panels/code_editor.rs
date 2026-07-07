use crate::project::Project;

/// 底部 `.code` 文本编辑器面板。
pub struct CodeEditorPanel;

impl CodeEditorPanel {
    /// 显示当前 `.code` 文件的文本内容，返回文本是否发生变化。
    pub fn show(ui: &mut egui::Ui, project: &mut Project) -> bool {
        ui.horizontal(|ui| {
            ui.heading("代码预览");
            if project
                .active_code_file()
                .map(|c| c.code_text_dirty)
                .unwrap_or(false)
            {
                ui.colored_label(ui.visuals().warn_fg_color, "● 已手动编辑");
            }
        });
        ui.separator();

        let mut changed = false;
        if let Some(code_file) = project.active_code_file_mut() {
            let mut text = code_file.code_text.clone();
            let response = ui.add(
                egui::TextEdit::multiline(&mut text)
                    .desired_rows(8)
                    .font(egui::TextStyle::Monospace)
                    .code_editor(),
            );
            if response.changed() {
                code_file.code_text = text;
                code_file.code_text_dirty = true;
                changed = true;
            }

            ui.horizontal(|ui| {
                if ui.button("🔄 从节点图生成").clicked() {
                    let _ = code_file.regenerate_code();
                }
                if ui
                    .small_button("重置")
                    .on_hover_text("丢弃手动修改并从节点图重新生成")
                    .clicked()
                {
                    let _ = code_file.regenerate_code();
                }
            });
        } else {
            ui.label("未选择 .code 文件");
        }
        changed
    }
}
