use std::path::PathBuf;

use crate::project::Project;

/// 左栏工程文件树面板。
pub struct ProjectTreePanel;

impl ProjectTreePanel {
    /// 显示工程文件树，返回用户触发的动作。
    pub fn show(ui: &mut egui::Ui, project: Option<&mut Project>) -> ProjectTreeAction {
        let mut action = ProjectTreeAction::None;

        ui.horizontal(|ui| {
            ui.heading("工程");
            if ui.small_button("新建工程").clicked() {
                action = ProjectTreeAction::NewProjectDialog;
            }
            if ui.small_button("打开工程").clicked() {
                action = ProjectTreeAction::OpenProjectDialog;
            }
        });
        ui.separator();

        let Some(project) = project else {
            ui.label("未打开工程");
            return action;
        };

        ui.label(format!("路径: {}", project.root.display()));
        ui.separator();

        // meta.json
        let is_meta = project.active_code.is_empty();
        let meta_response = ui.selectable_label(is_meta, "[meta] meta.json");
        if meta_response.clicked() {
            action = ProjectTreeAction::SelectMeta;
        }

        ui.separator();
        ui.label(".code 文件");

        egui::ScrollArea::vertical()
            .id_salt("project_tree_scroll")
            .show(ui, |ui| {
            for code_file in &project.code_files {
                let is_active = project.active_code == code_file.name;
                ui.horizontal(|ui| {
                    let name_label = format!("[{name}].code", name=code_file.name);
                    let response = ui.selectable_label(is_active, &name_label);
                    if response.clicked() {
                        action = ProjectTreeAction::SelectCode(code_file.name.clone());
                    }
                    if !is_active && ui.small_button("Del").on_hover_text("删除").clicked() {
                        action = ProjectTreeAction::DeleteCode(code_file.name.clone());
                    }
                    if ui.small_button("Ren").on_hover_text("重命名").clicked() {
                        action = ProjectTreeAction::RenameCode(code_file.name.clone());
                    }
                });
            }
        });

        ui.separator();
        if ui.button("+ 新建 .code").clicked() {
            action = ProjectTreeAction::NewCodeDialog;
        }
        if ui.button("💾 保存工程").clicked() {
            action = ProjectTreeAction::SaveProject;
        }
        if ui.button("📤 导出工程").clicked() {
            action = ProjectTreeAction::ExportProjectDialog;
        }

        action
    }

    /// 显示新建 `.code` 文件对话框。
    pub fn new_code_dialog(
        ctx: &egui::Context,
        open: &mut bool,
        name: &mut String,
    ) -> Option<String> {
        let mut show = *open;
        let mut result = None;
        let mut close = false;
        egui::Window::new("新建 .code 文件")
            .collapsible(false)
            .open(&mut show)
            .show(ctx, |ui| {
                ui.label("文件名（不含扩展名）:");
                ui.text_edit_singleline(name);
                ui.horizontal(|ui| {
                    if ui.button("创建").clicked() && !name.trim().is_empty() {
                        result = Some(name.trim().to_string());
                        close = true;
                    }
                    if ui.button("取消").clicked() {
                        close = true;
                    }
                });
            });
        *open = show && !close;
        result
    }

    /// 显示重命名 `.code` 文件对话框。
    pub fn rename_code_dialog(
        ctx: &egui::Context,
        open: &mut bool,
        old_name: &str,
        new_name: &mut String,
    ) -> Option<(String, String)> {
        let mut show = *open;
        let mut result = None;
        let mut close = false;
        egui::Window::new("重命名 .code 文件")
            .collapsible(false)
            .open(&mut show)
            .show(ctx, |ui| {
                ui.label(format!("原名称: {}.code", old_name));
                ui.label("新名称（不含扩展名）:");
                ui.text_edit_singleline(new_name);
                ui.horizontal(|ui| {
                    if ui.button("重命名").clicked() && !new_name.trim().is_empty() {
                        result = Some((old_name.to_string(), new_name.trim().to_string()));
                        close = true;
                    }
                    if ui.button("取消").clicked() {
                        close = true;
                    }
                });
            });
        *open = show && !close;
        result
    }

    /// 显示新建工程对话框。
    pub fn new_project_dialog(
        ctx: &egui::Context,
        open: &mut bool,
        parent: &mut Option<PathBuf>,
        name: &mut String,
    ) -> Option<PathBuf> {
        let mut show = *open;
        let mut result = None;
        let mut close = false;
        egui::Window::new("新建工程")
            .collapsible(false)
            .open(&mut show)
            .show(ctx, |ui| {
                ui.label("工程名称:");
                ui.text_edit_singleline(name);
                ui.horizontal(|ui| {
                    if ui.button("选择父文件夹").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            *parent = Some(path);
                        }
                    }
                    if let Some(p) = parent {
                        ui.label(format!("父目录: {}", p.display()));
                    }
                });
                ui.horizontal(|ui| {
                    let can_create = !name.trim().is_empty() && parent.is_some();
                    if ui
                        .add_enabled(can_create, egui::Button::new("创建"))
                        .clicked()
                    {
                        if let Some(p) = parent {
                            result = Some(p.join(name.trim()));
                        }
                        close = true;
                    }
                    if ui.button("取消").clicked() {
                        close = true;
                    }
                });
            });
        *open = show && !close;
        result
    }
}

/// 工程文件树面板触发的动作。
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ProjectTreeAction {
    /// 无动作。
    #[default]
    None,
    /// 打开新建工程对话框。
    NewProjectDialog,
    /// 打开选择工程文件夹对话框。
    OpenProjectDialog,
    /// 保存工程。
    SaveProject,
    /// 打开导出工程对话框。
    ExportProjectDialog,
    /// 切换到 `meta.json` 编辑。
    SelectMeta,
    /// 切换到指定 `.code` 文件。
    SelectCode(String),
    /// 新建 `.code` 文件对话框。
    NewCodeDialog,
    /// 删除指定 `.code` 文件。
    DeleteCode(String),
    /// 重命名指定 `.code` 文件。
    RenameCode(String),
}
