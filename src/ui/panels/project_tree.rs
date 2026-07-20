use std::path::PathBuf;

use crate::graph::container::{LabelContainer, ListenerContainer, ThreadContainer};
use crate::project::Project;
use crate::ui::i18n::I18n;

/// 左栏工程文件树面板。
pub struct ProjectTreePanel;

impl ProjectTreePanel {
    /// 显示工程文件树，返回用户触发的动作。
    pub fn show(
        ui: &mut egui::Ui,
        project: Option<&mut Project>,
        selected_code: &str,
        selected_thread: usize,
        selected_container: Option<ContainerKind>,
        i18n: &I18n,
    ) -> ProjectTreeAction {
        let mut action = ProjectTreeAction::None;

        ui.horizontal(|ui| {
            ui.heading(i18n.text("project_tree.title"));
            if ui.small_button(i18n.text("project_tree.new_project")).clicked() {
                action = ProjectTreeAction::NewProjectDialog;
            }
            if ui.small_button(i18n.text("project_tree.open_project")).clicked() {
                action = ProjectTreeAction::OpenProjectDialog;
            }
        });
        ui.separator();

        let Some(project) = project else {
            ui.label(i18n.text("project_tree.no_project"));
            return action;
        };

        ui.label(i18n.format("label.path", &[&project.root.display().to_string()]));
        ui.separator();

        // meta.json
        let is_meta = project.active_code.is_empty();
        let meta_response = ui.selectable_label(is_meta, "[meta] meta.json");
        if meta_response.clicked() {
            action = ProjectTreeAction::SelectMeta;
        }

        ui.separator();

        egui::ScrollArea::vertical()
            .id_salt("project_tree_scroll")
            .max_height(ui.available_height())
            .auto_shrink([false, true])
            .show(ui, |ui| {
                ui.label(i18n.text("project_tree.code_files"));
                for (cf_idx, code_file) in project.code_files.iter().enumerate() {
                    let is_active = project.active_code == code_file.name
                        || (selected_code == code_file.name && project.active_code.is_empty());
                    let header_id = ui.id().with(("code_file", cf_idx));
                    egui::collapsing_header::CollapsingState::load_with_default_open(
                        ui.ctx(),
                        header_id,
                        true,
                    )
                    .show_header(ui, |ui| {
                        ui.horizontal(|ui| {
                            let name_label = format!("[{}].code", code_file.name);
                            let response = ui.selectable_label(is_active, &name_label);
                            if response.clicked() {
                                action = ProjectTreeAction::SelectCode(code_file.name.clone());
                            }
                            if !is_active && ui.small_button("Del").on_hover_text(i18n.text("project_tree.delete")).clicked()
                            {
                                action = ProjectTreeAction::DeleteCode(code_file.name.clone());
                            }
                            if ui.small_button("Ren").on_hover_text(i18n.text("project_tree.rename")).clicked() {
                                action = ProjectTreeAction::RenameCode(code_file.name.clone());
                            }
                            if ui.small_button("Th+")
                                .on_hover_text(i18n.text("project_tree.add_thread"))
                                .clicked()
                            {
                                action = ProjectTreeAction::AddThread {
                                    code_name: code_file.name.clone(),
                                };
                            }
                        });
                    })
                    .body(|ui| {
                        Self::show_threads(
                            ui,
                            &code_file.graph_doc.graph.threads,
                            &code_file.name,
                            selected_code == code_file.name,
                            selected_thread,
                            selected_container,
                            &mut action,
                            i18n,
                        );
                    });
                }

                ui.separator();
                if ui.button(i18n.text("project_tree.new_code")).clicked() {
                    action = ProjectTreeAction::NewCodeDialog;
                }
                if ui.button(i18n.text("project_tree.save_project")).clicked() {
                    action = ProjectTreeAction::SaveProject;
                }
                if ui.button(i18n.text("project_tree.export_project")).clicked() {
                    action = ProjectTreeAction::ExportProjectDialog;
                }
            });

        action
    }

    #[allow(clippy::too_many_arguments)]
    fn show_threads(
        ui: &mut egui::Ui,
        threads: &[ThreadContainer],
        code_name: &str,
        code_active: bool,
        selected_thread: usize,
        selected_container: Option<ContainerKind>,
        action: &mut ProjectTreeAction,
        i18n: &I18n,
    ) {
        for (t_idx, thread) in threads.iter().enumerate() {
            let thread_active = code_active && selected_thread == t_idx;
            let thread_id = ui.id().with(("thread", t_idx));
            let thread_label = if thread.auto_start {
                format!("▶ {}", thread.name)
            } else {
                thread.name.clone()
            };
            egui::collapsing_header::CollapsingState::load_with_default_open(
                ui.ctx(),
                thread_id,
                true,
            )
            .show_header(ui, |ui| {
                ui.horizontal(|ui| {
                    let response = ui.selectable_label(thread_active, &thread_label);
                    if response.clicked() && !thread_active {
                        *action = ProjectTreeAction::SelectContainer {
                            thread_idx: t_idx,
                            container: ContainerKind::Label(0),
                        };
                    }
                    if ui
                        .small_button("Del")
                        .on_hover_text(i18n.text("project_tree.delete_thread"))
                        .clicked()
                    {
                        *action = ProjectTreeAction::DeleteThread {
                            code_name: code_name.to_string(),
                            thread_idx: t_idx,
                        };
                    }
                });
            })
            .body(|ui| {
                Self::show_labels(
                    ui,
                    &thread.labels,
                    code_name,
                    t_idx,
                    thread_active,
                    selected_container,
                    action,
                    i18n,
                );
                if !thread.listeners.is_empty() {
                    ui.separator();
                    Self::show_listeners(
                        ui,
                        &thread.listeners,
                        code_name,
                        t_idx,
                        thread_active,
                        selected_container,
                        action,
                        i18n,
                    );
                }
                ui.horizontal(|ui| {
                    if ui
                        .small_button("Lb+")
                        .on_hover_text(i18n.text("project_tree.add_label"))
                        .clicked()
                    {
                        *action = ProjectTreeAction::AddLabel {
                            code_name: code_name.to_string(),
                            thread_idx: t_idx,
                        };
                    }
                    if ui
                        .small_button("Ls+")
                        .on_hover_text(i18n.text("project_tree.add_listener"))
                        .clicked()
                    {
                        *action = ProjectTreeAction::AddListener {
                            code_name: code_name.to_string(),
                            thread_idx: t_idx,
                        };
                    }
                });
            });
        }

        if threads.is_empty() {
            ui.horizontal(|ui| {
                ui.label(i18n.text("project_tree.no_threads"));
                if ui
                    .small_button("Th+")
                    .on_hover_text(i18n.text("project_tree.add_thread"))
                    .clicked()
                {
                    *action = ProjectTreeAction::AddThread {
                        code_name: code_name.to_string(),
                    };
                }
            });
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn show_labels(
        ui: &mut egui::Ui,
        labels: &[LabelContainer],
        code_name: &str,
        thread_idx: usize,
        thread_active: bool,
        selected_container: Option<ContainerKind>,
        action: &mut ProjectTreeAction,
        i18n: &I18n,
    ) {
        ui.label(i18n.text("project_tree.labels"));
        for (l_idx, label) in labels.iter().enumerate() {
            let is_active =
                thread_active && selected_container == Some(ContainerKind::Label(l_idx));
            ui.horizontal(|ui| {
                let response = ui.selectable_label(
                    is_active,
                    format!("  {}: {}", label.name, label.nodes.len()),
                );
                if response.clicked() && !is_active {
                    *action = ProjectTreeAction::SelectContainer {
                        thread_idx,
                        container: ContainerKind::Label(l_idx),
                    };
                }
                if ui
                    .small_button("Del")
                    .on_hover_text(i18n.text("project_tree.delete_label"))
                    .clicked()
                {
                    *action = ProjectTreeAction::DeleteLabel {
                        code_name: code_name.to_string(),
                        thread_idx,
                        label_idx: l_idx,
                    };
                }
            });
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn show_listeners(
        ui: &mut egui::Ui,
        listeners: &[ListenerContainer],
        code_name: &str,
        thread_idx: usize,
        thread_active: bool,
        selected_container: Option<ContainerKind>,
        action: &mut ProjectTreeAction,
        i18n: &I18n,
    ) {
        ui.label(i18n.text("project_tree.listeners"));
        for (l_idx, listener) in listeners.iter().enumerate() {
            let is_active =
                thread_active && selected_container == Some(ContainerKind::Listener(l_idx));
            ui.horizontal(|ui| {
                let response = ui.selectable_label(
                    is_active,
                    format!("  {}: {}", listener.name(), listener.nodes().len()),
                );
                if response.clicked() && !is_active {
                    *action = ProjectTreeAction::SelectContainer {
                        thread_idx,
                        container: ContainerKind::Listener(l_idx),
                    };
                }
                if ui
                    .small_button("Del")
                    .on_hover_text(i18n.text("project_tree.delete_listener"))
                    .clicked()
                {
                    *action = ProjectTreeAction::DeleteListener {
                        code_name: code_name.to_string(),
                        thread_idx,
                        listener_idx: l_idx,
                    };
                }
            });
        }
    }

    /// 显示新建 `.code` 文件对话框。
    pub fn new_code_dialog(
        ctx: &egui::Context,
        open: &mut bool,
        name: &mut String,
        i18n: &I18n,
    ) -> Option<String> {
        let mut show = *open;
        let mut result = None;
        let mut close = false;
        egui::Window::new(i18n.text("dialog.new_code_title"))
            .collapsible(false)
            .open(&mut show)
            .show(ctx, |ui| {
                ui.label(i18n.text("dialog.new_code_name"));
                ui.text_edit_singleline(name);
                ui.horizontal(|ui| {
                    if ui.button(i18n.text("button.create")).clicked() && !name.trim().is_empty() {
                        result = Some(name.trim().to_string());
                        close = true;
                    }
                    if ui.button(i18n.text("button.cancel")).clicked() {
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
        i18n: &I18n,
    ) -> Option<(String, String)> {
        let mut show = *open;
        let mut result = None;
        let mut close = false;
        egui::Window::new(i18n.text("dialog.rename_code_title"))
            .collapsible(false)
            .open(&mut show)
            .show(ctx, |ui| {
                ui.label(i18n.format("dialog.original_name", &[old_name]));
                ui.label(i18n.text("dialog.new_name"));
                ui.text_edit_singleline(new_name);
                ui.horizontal(|ui| {
                    if ui.button(i18n.text("button.rename")).clicked() && !new_name.trim().is_empty() {
                        result = Some((old_name.to_string(), new_name.trim().to_string()));
                        close = true;
                    }
                    if ui.button(i18n.text("button.cancel")).clicked() {
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
        i18n: &I18n,
    ) -> Option<PathBuf> {
        let mut show = *open;
        let mut result = None;
        let mut close = false;
        egui::Window::new(i18n.text("dialog.new_project_title"))
            .collapsible(false)
            .open(&mut show)
            .show(ctx, |ui| {
                ui.label(i18n.text("dialog.project_name"));
                ui.text_edit_singleline(name);
                ui.horizontal(|ui| {
                    if ui.button(i18n.text("dialog.select_parent_folder")).clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            *parent = Some(path);
                        }
                    }
                    if let Some(p) = parent {
                        ui.label(i18n.format("dialog.parent_dir", &[&p.display().to_string()]));
                    }
                });
                ui.horizontal(|ui| {
                    let can_create = !name.trim().is_empty() && parent.is_some();
                    if ui
                        .add_enabled(can_create, egui::Button::new(i18n.text("button.create")))
                        .clicked()
                    {
                        if let Some(p) = parent {
                            result = Some(p.join(name.trim()));
                        }
                        close = true;
                    }
                    if ui.button(i18n.text("button.cancel")).clicked() {
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
    /// 选择容器（线程、标签或监听器）。
    SelectContainer {
        /// 线程在线程数组中的索引。
        thread_idx: usize,
        /// 选中的容器（标签或监听器）。
        container: ContainerKind,
    },
    /// 新建 `.code` 文件对话框。
    NewCodeDialog,
    /// 删除指定 `.code` 文件。
    DeleteCode(String),
    /// 重命名指定 `.code` 文件。
    RenameCode(String),
    /// 在指定 `.code` 文件中添加一个线程。
    AddThread { code_name: String },
    /// 删除指定 `.code` 文件中的线程。
    DeleteThread { code_name: String, thread_idx: usize },
    /// 在线程中添加一个标签容器。
    AddLabel { code_name: String, thread_idx: usize },
    /// 在线程中添加一个监听器容器。
    AddListener { code_name: String, thread_idx: usize },
    /// 删除线程中的标签容器。
    DeleteLabel { code_name: String, thread_idx: usize, label_idx: usize },
    /// 删除线程中的监听器容器。
    DeleteListener {
        code_name: String,
        thread_idx: usize,
        listener_idx: usize,
    },
}

/// 容器中可被编辑的实体类型（工程树专用）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerKind {
    /// 标签容器。
    Label(usize),
    /// 监听器容器。
    Listener(usize),
}
