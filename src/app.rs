#![allow(non_snake_case)]

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use egui::{Align2, FontId, Pos2, Rect, Vec2 as EVec2};

use crate::api::definitions::{NodeDefinition, PortDefinition};
use crate::api::namespace::NamespaceRegistry;
use crate::api::registry::get_definition;
use crate::code_gen::generator::generate_code_to_file;
use crate::error::{FlowError, Result};
use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::node::{Node, ParamValue, Port, Vec2};
use crate::graph::types::{NodeType, PortType};
use crate::graph::validation::GraphValidator;
use crate::project::Project;
use crate::serializer::json::GraphDocument;
use crate::ui::canvas::Canvas;
use crate::ui::edge_renderer::EdgeRenderer;
use crate::ui::interaction::InteractionController;
use crate::ui::node_renderer::{NodeRenderer, PortGeometry};
use crate::ui::panels::{
    code_editor::CodeEditorPanel,
    data_menu::DataMenuPanel,
    json_preview::JsonPreviewPanel,
    meta_editor::MetaEditorPanel,
    namespace_picker::{NamespacePicker, NamespacePickerState},
    node_library::NodeLibraryPanel,
    project_tree::{ProjectTreeAction, ProjectTreePanel},
    properties::PropertiesPanel,
    status_bar::StatusBarPanel,
    status_bar::{ErrorDetailWindow, StatusBarEvent},
};
use crate::ui::theme::Theme;

/// 左栏当前显示的标签页。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum LeftPanelTab {
    /// 工程文件树。
    #[default]
    Project,
    /// 节点库。
    NodeLibrary,
}

/// 可撤销/重做命令。
#[derive(Debug, Clone)]
pub enum Command {
    /// 移动节点
    MoveNode {
        node_id: String,
        from: Vec2,
        to: Vec2,
    },
    /// 添加节点
    AddNode { node: Node },
    /// 删除节点（同时保存被级联删除的边）
    RemoveNode { node: Node, edges: Vec<Edge> },
    /// 添加连线
    AddEdge { edge: Edge },
    /// 删除连线
    RemoveEdge { edge: Edge },
    /// 修改参数
    SetParam {
        node_id: String,
        key: String,
        from: ParamValue,
        to: ParamValue,
    },
    /// 将当前选中节点复制到剪贴板（无 graph 变更）
    CopySelected,
    /// 在指定屏幕坐标处粘贴剪贴板内容（由 App 转换为世界坐标）
    PasteAt { screen_pos: Pos2 },
}

impl Command {
    /// 应用命令。
    fn apply(&self, graph: &mut Graph, _clipboard: &mut Clipboard) {
        match self {
            Self::MoveNode { node_id, to, .. } => {
                if let Some(node) = graph.nodes.get_mut(node_id) {
                    node.position = *to;
                }
            }
            Self::AddNode { node } => {
                graph.add_node(node.clone());
            }
            Self::RemoveNode { node, .. } => {
                let _ = graph.remove_node(&node.id);
            }
            Self::AddEdge { edge } => {
                let _ = graph.add_edge(edge.clone());
            }
            Self::RemoveEdge { edge } => {
                let _ = graph.remove_edge(&edge.id);
            }
            Self::SetParam {
                node_id, key, to, ..
            } => {
                if let Some(node) = graph.nodes.get_mut(node_id) {
                    node.set_param(key, to.clone());
                }
            }
            Self::CopySelected => {
                // 无 graph 变更，由 App::push_command 单独处理
            }
            Self::PasteAt { .. } => {
                // 实际粘贴逻辑在 App::push_command 中处理，避免命令结构膨胀
            }
        }
    }

    /// 撤销命令。
    fn undo(&self, graph: &mut Graph) {
        match self {
            Self::MoveNode { node_id, from, .. } => {
                if let Some(node) = graph.nodes.get_mut(node_id) {
                    node.position = *from;
                }
            }
            Self::AddNode { node } => {
                let _ = graph.remove_node(&node.id);
            }
            Self::RemoveNode { node, edges } => {
                graph.add_node(node.clone());
                for edge in edges {
                    let _ = graph.add_edge(edge.clone());
                }
            }
            Self::AddEdge { edge } => {
                let _ = graph.remove_edge(&edge.id);
            }
            Self::RemoveEdge { edge } => {
                let _ = graph.add_edge(edge.clone());
            }
            Self::SetParam {
                node_id, key, from, ..
            } => {
                if let Some(node) = graph.nodes.get_mut(node_id) {
                    node.set_param(key, from.clone());
                }
            }
            Self::CopySelected | Self::PasteAt { .. } => {
                // 剪贴板/粘贴操作在 update_canvas 或全局快捷键中直接处理，不进入 Undo 栈。
            }
        }
    }
}

/// 剪贴板内容。
#[derive(Debug, Clone, Default)]
pub struct Clipboard {
    /// 被复制的节点
    pub nodes: Vec<Node>,
    /// 被复制节点之间的内部连线
    pub edges: Vec<Edge>,
}

/// 应用主状态。
pub struct App {
    pub graph: Graph,
    pub canvas: Canvas,
    pub selected_nodes: HashSet<String>,
    pub selected_edges: HashSet<String>,
    pub interaction: InteractionController,
    pub undo_stack: Vec<Command>,
    pub redo_stack: Vec<Command>,
    pub clipboard: Clipboard,
    /// 当前打开的工程。
    pub project: Option<Project>,
    /// 是否显示 meta.json 而非节点图属性。
    pub show_meta_editor: bool,
    pub validation_errors: Vec<FlowError>,
    pub error_nodes: HashSet<String>,
    pub search_window_open: bool,
    pub search_query: String,
    pub status_message: String,
    /// 是否显示空画布欢迎提示
    pub show_welcome_hint: bool,
    /// JSON 预览缓存，避免每帧序列化
    pub cached_json: String,
    /// 缓存对应的图版本号
    pub cached_json_version: u64,
    /// 图版本号，变化时重新生成缓存 JSON
    pub graph_version: u64,
    /// 底部面板高度，用于持久化拖拽高度
    pub bottom_panel_height: f32,
    /// 命名空间注册表，加载 assets/namespaces 下的 JSON 文件。
    pub namespace_registry: NamespaceRegistry,
    /// 命名空间选择器窗口状态。
    pub namespace_picker: Option<NamespacePickerState>,
    /// 是否显示错误详情弹窗。
    pub show_error_detail: bool,
    /// 左栏当前标签页
    left_panel_tab: LeftPanelTab,
    /// 新建工程对话框状态
    new_project_dialog_open: bool,
    new_project_parent: Option<PathBuf>,
    new_project_name: String,
    /// 新建 .code 对话框状态
    new_code_dialog_open: bool,
    new_code_name: String,
    /// 重命名 .code 对话框状态
    rename_code_dialog_open: bool,
    rename_old_name: String,
    rename_new_name: String,
    /// 导出工程对话框状态
    export_project_dialog_open: bool,
    export_destination: Option<PathBuf>,
}

impl App {
    /// 创建默认应用，并加载中文字体。
    pub fn new(cc: &eframe::CreationContext) -> Self {
        setup_fonts(&cc.egui_ctx);
        let graph = Graph::default();
        Self {
            graph,
            canvas: Canvas::new(),
            selected_nodes: HashSet::new(),
            selected_edges: HashSet::new(),
            error_nodes: HashSet::new(),
            interaction: InteractionController::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            clipboard: Clipboard::default(),
            project: None,
            namespace_registry: NamespaceRegistry::load_bundled(),
            namespace_picker: None,
            show_error_detail: false,
            show_meta_editor: false,
            validation_errors: Vec::new(),
            search_window_open: false,
            search_query: String::new(),
            status_message: String::new(),
            show_welcome_hint: false,
            cached_json: String::new(),
            cached_json_version: 0,
            graph_version: 0,
            bottom_panel_height: 200.0,
            left_panel_tab: LeftPanelTab::default(),
            new_project_dialog_open: false,
            new_project_parent: None,
            new_project_name: String::new(),
            new_code_dialog_open: false,
            new_code_name: String::new(),
            rename_code_dialog_open: false,
            rename_old_name: String::new(),
            rename_new_name: String::new(),
            export_project_dialog_open: false,
            export_destination: None,
        }

    }

    /// 执行命令并压入 Undo 栈。
    fn push_command(&mut self, cmd: Command) {
        cmd.apply(&mut self.graph, &mut self.clipboard);
        self.undo_stack.push(cmd);
        if self.undo_stack.len() > 50 {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
        self.graph_version += 1;
        self.validate();
    }

    /// 撤销。
    fn undo(&mut self) {
        if let Some(cmd) = self.undo_stack.pop() {
            cmd.undo(&mut self.graph);
            self.redo_stack.push(cmd);
            self.graph_version += 1;
            self.validate();
            self.status_message = String::from("已撤销");
        }
    }

    /// 重做。
    fn redo(&mut self) {
        if let Some(cmd) = self.redo_stack.pop() {
            cmd.apply(&mut self.graph, &mut self.clipboard);
            self.undo_stack.push(cmd);
            self.graph_version += 1;
            self.validate();
            self.status_message = String::from("已重做");
        }
    }

    /// 验证图并更新错误列表。
    fn validate(&mut self) {
        self.validation_errors = GraphValidator::collect_errors(&self.graph);
        self.error_nodes = self
            .validation_errors
            .iter()
            .flat_map(FlowError::affected_node_ids)
            .collect();
    }

    /// 在指定世界坐标处创建一个节点。
    fn add_node_at(&mut self, node_type: NodeType, position: Vec2) {
        let Some(def) = get_definition(node_type) else {
            return;
        };
        let mut node = Node::new(node_type, position);
        node.inputs = def.inputs.iter().map(port_from_def).collect();
        node.outputs = def.outputs.iter().map(port_from_def).collect();
        node.category = def.category.clone();
        for param in &def.params {
            node.set_param(&param.name, param.default_value());
            let data_port = Port::new(
                &param.name,
                param.param_type.port_type(),
                &param.display_name,
            )
            .required(param.required);
            node.inputs.push(data_port);
        }
        self.selected_nodes.clear();
        self.selected_nodes.insert(node.id.clone());
        self.show_welcome_hint = false;
        self.show_meta_editor = false;
        self.push_command(Command::AddNode { node });
        self.status_message = format!("已添加 {}", def.display_name);
    }

    /// 删除选中的项。
    ///
    /// 当存在选中的连线时，优先仅删除连线；否则删除选中的节点及其连接。
    /// 这样可以单独删除 Data 虚线而不误删节点。
    fn delete_selected(&mut self) {
        if !self.selected_edges.is_empty() {
            // 优先只删除选中的连线
            let mut edges_to_remove = Vec::new();
            for edge in self.graph.edges.values() {
                if self.selected_edges.contains(&edge.id) {
                    edges_to_remove.push(edge.clone());
                }
            }
            for edge in &edges_to_remove {
                self.push_command(Command::RemoveEdge { edge: edge.clone() });
            }
            self.selected_edges.clear();
            self.status_message = format!("已删除 {} 条连线", edges_to_remove.len());
            return;
        }

        let mut edges_to_remove = Vec::new();
        for edge in self.graph.edges.values() {
            if self.selected_nodes.contains(&edge.from.node_id)
                || self.selected_nodes.contains(&edge.to.node_id)
            {
                edges_to_remove.push(edge.clone());
            }
        }
        for edge in &edges_to_remove {
            self.push_command(Command::RemoveEdge { edge: edge.clone() });
        }

        let mut nodes_to_remove = Vec::new();
        for node in self.graph.nodes.values() {
            if self.selected_nodes.contains(&node.id) {
                nodes_to_remove.push(node.clone());
            }
        }
        for node in nodes_to_remove {
            self.push_command(Command::RemoveNode {
                node,
                edges: Vec::new(),
            });
        }

        self.selected_nodes.clear();
        self.selected_edges.clear();
        self.status_message = String::from("已删除选中项");
    }

    /// 将当前选中的节点复制到剪贴板。
    fn copy_selected(&mut self) {
        if self.selected_nodes.is_empty() {
            return;
        }
        let mut nodes: Vec<Node> = Vec::new();
        let mut old_to_new = HashMap::new();
        for id in &self.selected_nodes {
            if let Some(node) = self.graph.nodes.get(id) {
                let mut copy = node.clone();
                let old_id = copy.id.clone();
                copy.id = uuid::Uuid::new_v4().to_string();
                old_to_new.insert(old_id, copy.id.clone());
                nodes.push(copy);
            }
        }

        let mut edges: Vec<Edge> = Vec::new();
        for edge in self.graph.edges.values() {
            if let (Some(from_new), Some(to_new)) = (
                old_to_new.get(&edge.from.node_id),
                old_to_new.get(&edge.to.node_id),
            ) {
                let mut copy = edge.clone();
                copy.id = uuid::Uuid::new_v4().to_string();
                copy.from.node_id = from_new.clone();
                copy.to.node_id = to_new.clone();
                edges.push(copy);
            }
        }

        self.clipboard = Clipboard { nodes, edges };
        self.status_message = format!("已复制 {} 个节点", self.selected_nodes.len());
    }

    /// 在指定世界坐标处粘贴剪贴板内容。
    fn paste_at(&mut self, position: Vec2) {
        if self.clipboard.nodes.is_empty() {
            return;
        }

        // 先克隆剪贴板内容，避免借用冲突
        let nodes: Vec<Node> = self.clipboard.nodes.clone();
        let edges: Vec<Edge> = self.clipboard.edges.clone();

        // 计算剪贴板节点的中心，并生成新的 ID 映射
        let mut old_to_new = HashMap::new();
        let mut center = Vec2::ZERO;
        for node in &nodes {
            center += node.position;
            old_to_new.insert(node.id.clone(), uuid::Uuid::new_v4().to_string());
        }
        center /= nodes.len() as f32;
        let offset = position - center;

        let mut pasted_ids = HashSet::new();
        for node in &nodes {
            let mut copy = node.clone();
            if let Some(new_id) = old_to_new.get(&node.id) {
                copy.id = new_id.clone();
            }
            copy.position += offset;
            pasted_ids.insert(copy.id.clone());
            self.push_command(Command::AddNode { node: copy });
        }
        for edge in &edges {
            let mut copy = edge.clone();
            copy.id = uuid::Uuid::new_v4().to_string();
            if let (Some(from_new), Some(to_new)) = (
                old_to_new.get(&edge.from.node_id),
                old_to_new.get(&edge.to.node_id),
            ) {
                copy.from.node_id = from_new.clone();
                copy.to.node_id = to_new.clone();
                self.push_command(Command::AddEdge { edge: copy });
            }
        }

        self.selected_nodes = pasted_ids;
        self.selected_edges.clear();
        self.graph_version += 1;
        self.show_welcome_hint = false;
        self.status_message = format!("已粘贴 {} 个节点", nodes.len());
    }

    /// 保存工程。若当前没有工程，则打开新建工程对话框。
    fn save_project(&mut self) -> Result<()> {
        if self.project.is_none() {
            self.new_project_dialog_open = true;
            self.status_message = String::from("请先创建或打开工程");
            return Ok(());
        }
        self.sync_active_to_project();
        if let Some(project) = &mut self.project {
            project.save()?;
            self.status_message = format!("已保存工程 {}", project.root.display());
        }
        Ok(())
    }

    /// 从工程文件夹加载工程。
    fn load_project(&mut self, root: PathBuf) -> Result<()> {
        let root_display = root.display().to_string();
        let project = Project::open(root)?;
        self.project = Some(project);
        self.load_active_code();
        self.status_message = format!("已打开工程 {}", root_display);
        Ok(())
    }

    /// 将当前 App 中的图同步到工程当前激活的 `.code` 文件。
    fn sync_active_to_project(&mut self) {
        if let Some(project) = &mut self.project {
            let viewport = self.canvas.viewport.clone();
            let _ = project.sync_active_code(self.graph.clone(), viewport);
        }
    }

    /// 将工程当前激活的 `.code` 文件加载到 App 的图与画布中。
    fn load_active_code(&mut self) {
        if let Some(project) = &self.project {
            if let Some(code_file) = project.active_code_file() {
                self.graph = code_file.graph_doc.graph.clone();
                self.canvas = Canvas::with_viewport(code_file.graph_doc.viewport.clone());
                self.selected_nodes.clear();
                self.selected_edges.clear();
                self.undo_stack.clear();
                self.redo_stack.clear();
                self.show_welcome_hint = self.graph.nodes.is_empty();
                self.show_meta_editor = false;
                // 为缺少必填参数的节点补默认值，兼容旧 JSON
                for node in self.graph.nodes.values_mut() {
                    if let Some(def) = get_definition(node.node_type) {
                        for param in &def.params {
                            if !node.params.contains_key(&param.name) {
                                node.set_param(&param.name, param.default_value());
                            }
                            if !node.inputs.iter().any(|p| p.id == param.name) {
                                let data_port = Port::new(
                                    &param.name,
                                    param.param_type.port_type(),
                                    &param.display_name,
                                )
                                .required(param.required);
                                node.inputs.push(data_port);
                            }
                        }
                    }
                }
                self.validate();
                self.graph_version += 1;
            }
        }
    }

    /// 切换到指定 `.code` 文件或 `meta.json`。
    fn switch_to_code(&mut self, name: &str) {
        self.sync_active_to_project();
        if let Some(project) = &mut self.project {
            if let Err(e) = project.set_active_code(name) {
                self.status_message = format!("切换失败: {}", e);
                return;
            }
        }
        self.load_active_code();
        self.status_message = format!("切换到 {}.code", name);
    }

    /// 切换到 meta.json 编辑。
    fn switch_to_meta(&mut self) {
        self.sync_active_to_project();
        self.show_meta_editor = true;
        if let Some(project) = &mut self.project {
            project.active_code = String::new();
            let _ = project.refresh_meta_text();
        }
        self.status_message = String::from("编辑 meta.json");
    }

    /// 新建空图（在工程模式下清空当前激活的代码图）。
    fn confirm_new_graph(&mut self) {
        if !self.graph.nodes.is_empty() {
            let confirmed = rfd::MessageDialog::new()
                .set_title("确认新建")
                .set_description("当前画布有未保存的节点，是否清空？")
                .set_buttons(rfd::MessageButtons::YesNo)
                .show();
            if !matches!(confirmed, rfd::MessageDialogResult::Yes) {
                return;
            }
        }
        self.new_graph();
    }

    /// 清空当前激活的代码图。
    fn new_graph(&mut self) {
        self.graph = Graph::default();
        self.canvas = Canvas::new();
        self.selected_nodes.clear();
        self.selected_edges.clear();
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.show_welcome_hint = true;
        self.show_meta_editor = false;
        self.validate();
        self.graph_version += 1;
        self.status_message = String::from("已清空当前代码图");
    }

    /// 导出 `.code` 文件（弹出保存对话框），保留单文件导出能力。
    fn export_code(&mut self) -> Result<()> {
        let path = if let Some(p) = rfd::FileDialog::new()
            .add_filter("Code", &["code"])
            .save_file()
        {
            p
        } else {
            self.status_message = "已取消导出".to_string();
            return Ok(());
        };
        generate_code_to_file(&self.graph, &path)?;
        if self.graph.nodes.is_empty() {
            rfd::MessageDialog::new()
                .set_title("导出为空")
                .set_description("图为空，导出的 .code 文件内容为空。")
                .set_buttons(rfd::MessageButtons::Ok)
                .show();
            self.status_message = "已导出 .code（图为空）".to_string();
        } else {
            self.status_message = format!("已导出 .code 到 {}", path.display());
        }
        Ok(())
    }

    /// 导出工程到指定文件夹。
    fn export_project(&mut self, destination: &std::path::Path) -> Result<()> {
        if self.project.is_none() {
            self.status_message = String::from("没有打开的工程");
            return Ok(());
        }
        self.sync_active_to_project();
        if let Some(project) = &self.project {
            project.export(destination)?;
            self.status_message = format!(
                "已导出工程到 {}",
                destination
                    .join(project.root.file_name().unwrap_or_default())
                    .display()
            );
        }
        Ok(())
    }

    /// 运行预览（当前仅提示，需要在游戏中加载 .code 文件）。
    fn run_preview(&mut self) {
        self.status_message =
            String::from("运行预览：请将工程导出到 CustomMissions2 文件夹后启动游戏");
    }

    /// 获取当前鼠标悬停的世界坐标（如果可用）。
    fn hover_world_pos(&self, ctx: &egui::Context, canvas_rect: Rect) -> Option<Pos2> {
        ctx.input(|i| {
            i.pointer
                .hover_pos()
                .map(|p| self.canvas.screen_to_world(p, canvas_rect))
        })
    }

    fn serialize_graph(graph: &Graph, viewport: &crate::serializer::json::Viewport) -> String {
        let doc = GraphDocument::from_graph(
            graph.clone(),
            serde_json::Value::Object(serde_json::Map::new()),
            viewport.clone(),
            Vec::new(),
            Vec::new(),
        );
        doc.to_json_pretty()
            .unwrap_or_else(|e| format!("序列化失败: {}", e))
    }

    /// 处理工程文件树触发的动作。
    fn handle_project_action(&mut self, action: ProjectTreeAction) {
        match action {
            ProjectTreeAction::None => {}
            ProjectTreeAction::NewProjectDialog => {
                self.new_project_dialog_open = true;
                self.new_project_parent = None;
                self.new_project_name.clear();
            }
            ProjectTreeAction::OpenProjectDialog => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    if let Err(e) = self.load_project(path) {
                        self.status_message = format!("打开工程失败: {}", e);
                        rfd::MessageDialog::new()
                            .set_title("打开失败")
                            .set_description(format!("无法打开工程: {}", e))
                            .set_buttons(rfd::MessageButtons::Ok)
                            .show();
                    }
                }
            }
            ProjectTreeAction::SaveProject => {
                if let Err(e) = self.save_project() {
                    self.status_message = format!("保存失败: {}", e);
                }
            }
            ProjectTreeAction::ExportProjectDialog => {
                self.export_project_dialog_open = true;
                self.export_destination = None;
            }
            ProjectTreeAction::SelectMeta => {
                self.switch_to_meta();
            }
            ProjectTreeAction::SelectCode(name) => {
                self.switch_to_code(&name);
            }
            ProjectTreeAction::NewCodeDialog => {
                self.new_code_dialog_open = true;
                self.new_code_name.clear();
            }
            ProjectTreeAction::DeleteCode(name) => {
                if let Some(project) = &mut self.project {
                    if project.code_files.len() <= 1 {
                        self.status_message = String::from("至少保留一个 .code 文件");
                        return;
                    }
                    let confirmed = rfd::MessageDialog::new()
                        .set_title("确认删除")
                        .set_description(format!("是否删除 {}.code？", name))
                        .set_buttons(rfd::MessageButtons::YesNo)
                        .show();
                    if matches!(confirmed, rfd::MessageDialogResult::Yes) {
                        if let Err(e) = project.remove_code_file(&name) {
                            self.status_message = format!("删除失败: {}", e);
                        } else {
                            self.load_active_code();
                            self.status_message = format!("已删除 {}.code", name);
                        }
                    }
                }
            }
            ProjectTreeAction::RenameCode(name) => {
                self.rename_old_name = name;
                self.rename_new_name.clear();
                self.rename_code_dialog_open = true;
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 全局快捷键（在 UI 绘制之前处理，防止被控件消费）
        // Space 仅在画布未捕获键盘输入时触发搜索，避免在文本框中输入空格时打开搜索窗口
        if !ctx.wants_keyboard_input()
            && ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Space))
        {
            self.search_window_open = !self.search_window_open;
        }
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::Z)) {
            self.undo();
        }
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::Y)) {
            self.redo();
        }
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::C)) {
            self.copy_selected();
        }
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::V)) {
            if let Some(pos) = self.hover_world_pos(ctx, self.canvas_rect(ctx)) {
                self.paste_at(Vec2::new(pos.x, pos.y));
            }
        }
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Delete)) {
            self.delete_selected();
        }

        // 全局剪贴板快捷键：eframe 将 Ctrl+C/V 转换为 Event::Copy/Paste，
        // 而不是 Key::C/V。搜索窗口打开时让 TextEdit 保留原行为。
        let paste_pos = self.hover_world_pos(ctx, self.canvas_rect(ctx));
        let search_open = self.search_window_open;
        ctx.input_mut(|i| {
            let mut copied = false;
            let mut pasted = false;
            i.events.retain(|event| {
                if search_open {
                    return true;
                }
                match event {
                    egui::Event::Copy if !copied => {
                        copied = true;
                        self.copy_selected();
                        false
                    }
                    egui::Event::Paste(_) if !pasted => {
                        pasted = true;
                        if let Some(pos) = paste_pos {
                            self.paste_at(Vec2::new(pos.x, pos.y));
                        }
                        false
                    }
                    _ => true,
                }
            });
        });

        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::S)) {
            if let Err(e) = self.save_project() {
                self.status_message = format!("保存失败: {}", e);
            }
        }

        // 顶部工具栏
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("新建工程").clicked() {
                    self.new_project_dialog_open = true;
                    self.new_project_parent = None;
                    self.new_project_name.clear();
                }
                if ui.button("打开工程").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        if let Err(e) = self.load_project(path) {
                            self.status_message = format!("加载失败: {}", e);
                            rfd::MessageDialog::new()
                                .set_title("加载失败")
                                .set_description(format!("无法加载工程: {}", e))
                                .set_buttons(rfd::MessageButtons::Ok)
                                .show();
                        }
                    }
                }
                if ui.button("保存工程 (Ctrl+S)").clicked() {
                    if let Err(e) = self.save_project() {
                        self.status_message = format!("保存失败: {}", e);
                    }
                }
                if ui.button("导出工程").clicked() {
                    self.export_project_dialog_open = true;
                    self.export_destination = None;
                }
                if ui.button("导出 .code").clicked() {
                    if let Err(e) = self.export_code() {
                        self.status_message = format!("导出 .code 失败: {}", e);
                    }
                }
                if ui.button("清空当前图").clicked() {
                    self.confirm_new_graph();
                }
                if ui.button("运行预览").clicked() {
                    self.run_preview();
                }
                ui.separator();
                if ui.button("撤销 (Ctrl+Z)").clicked() {
                    self.undo();
                }
                if ui.button("重做 (Ctrl+Y)").clicked() {
                    self.redo();
                }
                ui.separator();
                if ui.button("复制 (Ctrl+C)").clicked() {
                    self.copy_selected();
                }
                if ui.button("粘贴 (Ctrl+V)").clicked() {
                    if let Some(pos) = self.hover_world_pos(ctx, self.canvas_rect(ctx)) {
                        self.paste_at(Vec2::new(pos.x, pos.y));
                    }
                }
                if ui.button("删除 (Del)").clicked() {
                    self.delete_selected();
                }
                if ui.button("添加节点 (Space)").clicked() {
                    self.search_window_open = !self.search_window_open;
                }
            });
        });

        // 左栏：工程文件树 / 节点库 标签页
        let mut left_action = ProjectTreeAction::None;
        egui::SidePanel::left("side_panel")
            .width_range(180.0..=300.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let project_tab =
                        ui.selectable_label(self.left_panel_tab == LeftPanelTab::Project, "工程");
                    if project_tab.clicked() {
                        self.left_panel_tab = LeftPanelTab::Project;
                    }
                    let node_tab = ui.selectable_label(
                        self.left_panel_tab == LeftPanelTab::NodeLibrary,
                        "节点库",
                    );
                    if node_tab.clicked() {
                        self.left_panel_tab = LeftPanelTab::NodeLibrary;
                    }
                });
                ui.separator();

                match self.left_panel_tab {
                    LeftPanelTab::Project => {
                        left_action = ProjectTreePanel::show(ui, self.project.as_mut());
                    }
                    LeftPanelTab::NodeLibrary => {
                        let spawn_pos = self
                            .hover_world_pos(ctx, self.canvas_rect(ctx))
                            .map(|p| Vec2::new(p.x, p.y));
                        if let Some(node_type) = NodeLibraryPanel::show(
                            ui,
                            &mut self.search_query,
                            &mut self.search_window_open,
                        ) {
                            let pos = spawn_pos.unwrap_or(Vec2::new(0.0, 0.0));
                            self.add_node_at(node_type, pos);
                        }
                    }
                }
            });
        self.handle_project_action(left_action);

        // 右栏：属性面板 / meta 编辑器
        let mut edited_node_id = None;
        egui::SidePanel::right("properties")
            .width_range(200.0..=400.0)
            .show(ctx, |ui| {
                if self.show_meta_editor {
                    if let Some(project) = self.project.as_mut() {
                        let _changed = MetaEditorPanel::show(ui, project);
                    }
                } else if let Some(node_id) = self.selected_nodes.iter().next().cloned() {
                    edited_node_id = Some(node_id.clone());
                    if let Some(node) = self.graph.nodes.get(&node_id).cloned() {
                        if let Some((key, value)) = PropertiesPanel::show(
                            ui,
                            &node,
                            &self.graph,
                            &self.namespace_registry,
                            &mut self.namespace_picker,
                        ) {
                            if let Some(n) = self.graph.nodes.get(&node_id) {
                                let from = n.params.get(&key).cloned().unwrap_or(ParamValue::Null);
                                self.push_command(Command::SetParam {
                                    node_id: node_id.clone(),
                                    key,
                                    from,
                                    to: value,
                                });
                            }
                        }
                    }
                } else if self.project.is_some() {
                    ui.heading("工程设置");
                    ui.label("点击左侧 meta.json 或选择节点进行编辑");
                } else {
                    ui.heading("属性");
                    ui.label("打开工程后编辑节点属性");
                }
            });

        // 命名空间选择器悬浮窗口
        if let Some(picker) = self.namespace_picker.as_mut() {
            if picker.open {
                if let Some(keys) = NamespacePicker::show(ctx, &self.namespace_registry, picker) {
                    if let Some(node_id) = edited_node_id {
                        let value = if picker.multi {
                            ParamValue::Literal(serde_json::json!(keys))
                        } else {
                            ParamValue::Literal(serde_json::json!(
                                keys.into_iter().next().unwrap_or_default()
                            ))
                        };
                        let key = picker.param_key.clone();
                        if let Some(n) = self.graph.nodes.get(&node_id) {
                            let from = n.params.get(&key).cloned().unwrap_or(ParamValue::Null);
                            self.push_command(Command::SetParam {
                                node_id,
                                key,
                                from,
                                to: value,
                            });
                        }
                    }
                }
            } else {
                self.namespace_picker = None;
            }
        }

        // 底部状态栏
        egui::TopBottomPanel::bottom("status_bar")
            .default_height(28.0)
            .show(ctx, |ui| {
                let event = StatusBarPanel::show(
                    ui,
                    &self.status_message,
                    &self.validation_errors,
                    self.hover_world_pos(ctx, self.canvas_rect(ctx)),
                    self.canvas.viewport.zoom,
                );
                if event == StatusBarEvent::OpenErrorDetails {
                    self.show_error_detail = true;
                }
            });

        // ──────────────────────────────────────────────
        // 底部面板（代码 ┃ JSON ┃ DataFlow）— 统一可拖拽
        // ──────────────────────────────────────────────
        if self.graph_version != self.cached_json_version {
            self.cached_json = Self::serialize_graph(&self.graph, &self.canvas.viewport);
            self.cached_json_version = self.graph_version;
        }

        egui::TopBottomPanel::bottom("bottom_main")
            .resizable(true)
            .default_height(260.0)
            .min_height(140.0)
            .show(ctx, |ui| {
                let panel_w = ui.available_width().max(200.0);
                let panel_h = ui.available_height().max(20.0);

                // 两个可拖拽分隔线的比例（左/（左+中+右）总宽）
                let s1_id = egui::Id::new("split_code_json");
                let s2_id = egui::Id::new("split_json_data");
                let data = ui.ctx().data_mut(|d| {
                    (d.get_temp::<f32>(s1_id).unwrap_or(0.33),
                     d.get_temp::<f32>(s2_id).unwrap_or(0.60))
                });
                let mut split1 = data.0;
                let mut split2 = data.1;

                // 三列计算
                let w1 = (panel_w * split1).round() as f32;
                let w2 = (panel_w * split2).round() as f32;

                // ── 列 1：代码预览 ──
                let rc1 = egui::Rect::from_min_size(
                    ui.next_widget_position(),
                    egui::vec2(w1 - 1.0, panel_h),
                );
                ui.allocate_new_ui(egui::UiBuilder::new().max_rect(rc1), |ui| {
                    if let Some(project) = self.project.as_mut() {
                        let _changed = CodeEditorPanel::show(ui, project);
                    } else {
                        ui.label("打开工程后查看 .code 代码");
                    }
                });

                // ── 分隔线 1 ──
                let rs1 = egui::Rect::from_min_size(
                    rc1.right_top(),
                    egui::vec2(6.0, panel_h),
                );
                let r1 = ui.allocate_rect(rs1, egui::Sense::drag());
                if r1.hovered() || r1.dragged() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
                }
                if r1.dragged() {
                    split1 = (split1 + r1.drag_delta().x / panel_w).clamp(0.15, split2 - 0.1);
                    ui.ctx().data_mut(|d| d.insert_temp(s1_id, split1));
                }

                // ── 列 2：JSON 预览 ──
                let rc2 = egui::Rect::from_min_size(
                    egui::pos2(rs1.right_top().x, rc1.top()),
                    egui::vec2(w2 - rs1.right_top().x - 1.0, panel_h),
                );
                ui.allocate_new_ui(egui::UiBuilder::new().max_rect(rc2), |ui| {
                    JsonPreviewPanel::show(ui, &self.cached_json);
                });

                // ── 分隔线 2 ──
                let rs2 = egui::Rect::from_min_size(
                    rc2.right_top(),
                    egui::vec2(6.0, panel_h),
                );
                let r2 = ui.allocate_rect(rs2, egui::Sense::drag());
                if r2.hovered() || r2.dragged() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
                }
                if r2.dragged() {
                    split2 = (split2 + r2.drag_delta().x / panel_w).clamp(split1 + 0.1, 0.85);
                    ui.ctx().data_mut(|d| d.insert_temp(s2_id, split2));
                }

                // ── 列 3：DataFlow ──
                let rc3 = egui::Rect::from_min_size(
                    rs2.right_top(),
                    egui::vec2(panel_w - rs2.right_top().x, panel_h),
                );
                ui.allocate_new_ui(egui::UiBuilder::new().max_rect(rc3), |ui| {
                    if let Some(node_id) =
                        DataMenuPanel::show(ui, &self.graph, &self.selected_nodes)
                    {
                        self.selected_nodes.clear();
                        self.selected_nodes.insert(node_id);
                        self.selected_edges.clear();
                    }
                });
            });

        // 错误详情弹窗
        if self.show_error_detail {
            ErrorDetailWindow::show(
                &mut self.show_error_detail,
                &self.validation_errors,
                ctx,
            );
        }

        // 中央画布
        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).fill(Theme::BACKGROUND))
            .show(ctx, |ui| {
                if self.show_meta_editor {
                    self.show_meta_editor_view(ui);
                } else {
                    self.update_canvas(ctx, ui);
                }
            });

        // 搜索窗口
        if self.search_window_open {
            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.search_window_open = false;
            }
            egui::Window::new("添加节点")
                .collapsible(false)
                .show(ctx, |ui| {
                    if ui.button("X 关闭").clicked() {
                        self.search_window_open = false;
                        return;
                    }
                    let spawn_pos = self
                        .hover_world_pos(ctx, self.canvas_rect(ctx))
                        .map(|p| Vec2::new(p.x, p.y));
                    if let Some(node_type) =
                        NodeLibraryPanel::show_search(ui, &mut self.search_query)
                    {
                        let pos = spawn_pos.unwrap_or(Vec2::new(0.0, 0.0));
                        self.add_node_at(node_type, pos);
                        self.search_window_open = false;
                    }
                });
        }

        // 对话框
        self.draw_dialogs(ctx);
    }
}

impl App {
    /// 获取当前画布矩形（近似）。
    fn canvas_rect(&self, ctx: &egui::Context) -> Rect {
        ctx.available_rect()
    }

    /// 在 meta 编辑器模式下显示提示（避免画布空荡）。
    fn show_meta_editor_view(&mut self, ui: &mut egui::Ui) {
        ui.centered_and_justified(|ui| {
            ui.label(
                egui::RichText::new("正在编辑 meta.json\n请在右侧面板修改")
                    .size(18.0)
                    .color(Theme::TEXT_DIM),
            );
        });
    }

    /// 渲染各种弹出对话框。
    fn draw_dialogs(&mut self, ctx: &egui::Context) {
        if self.new_project_dialog_open {
            if let Some(root) = ProjectTreePanel::new_project_dialog(
                ctx,
                &mut self.new_project_dialog_open,
                &mut self.new_project_parent,
                &mut self.new_project_name,
            ) {
                match Project::create(
                    root.parent().unwrap_or(&root),
                    root.file_name()
                        .map(|n| n.to_string_lossy())
                        .unwrap_or_default()
                        .as_ref(),
                ) {
                    Ok(project) => {
                        let root_display = project.root.display().to_string();
                        self.project = Some(project);
                        self.load_active_code();
                        self.status_message = format!("已创建工程 {}", root_display);
                    }
                    Err(e) => {
                        self.status_message = format!("创建工程失败: {}", e);
                    }
                }
                self.new_project_dialog_open = false;
            }
        }

        if self.new_code_dialog_open {
            if let Some(name) = ProjectTreePanel::new_code_dialog(
                ctx,
                &mut self.new_code_dialog_open,
                &mut self.new_code_name,
            ) {
                if let Some(project) = &mut self.project {
                    if let Err(e) = project.add_code_file(&name) {
                        self.status_message = format!("创建失败: {}", e);
                    } else {
                        self.load_active_code();
                        self.status_message = format!("已创建 {}.code", name);
                    }
                }
                self.new_code_dialog_open = false;
            }
        }

        if self.rename_code_dialog_open {
            if let Some((old_name, new_name)) = ProjectTreePanel::rename_code_dialog(
                ctx,
                &mut self.rename_code_dialog_open,
                &self.rename_old_name,
                &mut self.rename_new_name,
            ) {
                if let Some(project) = &mut self.project {
                    if let Err(e) = project.rename_code_file(&old_name, &new_name) {
                        self.status_message = format!("重命名失败: {}", e);
                    } else {
                        if self.active_code_name() == old_name {
                            self.load_active_code();
                        }
                        self.status_message = format!("已重命名为 {}.code", new_name);
                    }
                }
                self.rename_code_dialog_open = false;
            }
        }

        if self.export_project_dialog_open {
            let mut open = self.export_project_dialog_open;
            egui::Window::new("导出工程")
                .collapsible(false)
                .open(&mut open)
                .show(ctx, |ui| {
                    ui.label("选择目标文件夹（通常为 CustomMissions2）:");
                    ui.horizontal(|ui| {
                        if ui.button("选择文件夹").clicked() {
                            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                self.export_destination = Some(path);
                            }
                        }
                        if let Some(p) = &self.export_destination {
                            ui.label(format!("目标: {}", p.display()));
                        }
                    });
                    ui.horizontal(|ui| {
                        let can_export =
                            self.export_destination.is_some() && self.project.is_some();
                        if ui
                            .add_enabled(can_export, egui::Button::new("导出"))
                            .clicked()
                        {
                            if let Some(dest) = self.export_destination.take() {
                                if let Err(e) = self.export_project(dest.as_path()) {
                                    self.status_message = format!("导出失败: {}", e);
                                }
                            }
                            self.export_project_dialog_open = false;
                        }
                        if ui.button("取消").clicked() {
                            self.export_project_dialog_open = false;
                        }
                    });
                });
            if !open {
                self.export_project_dialog_open = false;
            }
        }
    }

    /// 获取当前激活的代码文件名。
    fn active_code_name(&self) -> String {
        self.project
            .as_ref()
            .map(|p| p.active_code.clone())
            .unwrap_or_default()
    }

    /// 更新画布内容并处理交互。
    fn update_canvas(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let canvas_response = self.canvas.update(ui);
        let canvas_rect = canvas_response.canvas_rect;
        let cull_margin = 50.0;
        let cull_rect = canvas_rect.expand(cull_margin);
        let node_renderer = NodeRenderer::default();
        let edge_renderer = EdgeRenderer::default();

        // 第一遍：计算所有节点屏幕区域和端口几何（不渲染，支持裁剪和 z 序）
        let mut node_data: Vec<(&Node, &NodeDefinition, Rect, Vec<PortGeometry>, bool, bool)> =
            Vec::new();
        let mut port_positions: HashMap<(String, String), Pos2> = HashMap::new();
        let mut port_hits: Vec<(String, String, Pos2, PortType, bool)> = Vec::new();

        for node in self.graph.nodes.values() {
            let Some(definition) = get_definition(node.node_type) else {
                continue;
            };
            let rect = node_renderer.screen_rect(&self.canvas, node, canvas_rect);
            if !cull_rect.intersects(rect) {
                continue;
            }
            let ports = node_renderer.port_positions(node, rect);
            let is_selected = self.selected_nodes.contains(&node.id);
            let has_errors = self.error_nodes.contains(&node.id);
            for port in &ports {
                port_positions.insert((node.id.clone(), port.id.clone()), port.center);
                port_hits.push((
                    node.id.clone(),
                    port.id.clone(),
                    port.center,
                    port.port_type.clone(),
                    port.is_input,
                ));
            }
            node_data.push((node, definition, rect, ports, is_selected, has_errors));
        }

        let mut node_hits: Vec<(String, Rect)> = node_data
            .iter()
            .map(|(node, _, rect, _, _, _)| (node.id.clone(), *rect))
            .collect();
        // 命中测试顺序与渲染顺序一致：选中节点在最上层，优先命中
        node_hits.sort_by(|(a_id, _), (b_id, _)| {
            let a_selected = self.selected_nodes.contains(a_id);
            let b_selected = self.selected_nodes.contains(b_id);
            b_selected.cmp(&a_selected)
        });

        // 第二遍：渲染连线（在节点下方），并裁剪
        let mut edge_hits: Vec<(String, Rect)> = Vec::new();
        // DataFlow 边在单选相关节点时显示，或当该 Data 边本身被选中时显示，
        // 便于用户点选/框选虚线并单独删除。
        let selected_node = self.selected_nodes.iter().next().cloned();
        for edge in self.graph.edges.values() {
            if edge.edge_type != PortType::Flow {
                let related = selected_node.as_ref().is_some_and(|selected| {
                    edge.from.node_id == *selected || edge.to.node_id == *selected
                });
                let edge_selected = self.selected_edges.contains(&edge.id);
                if !related && !edge_selected {
                    continue;
                }
            }
            let Some(&from_pos) =
                port_positions.get(&(edge.from.node_id.clone(), edge.from.port_id.clone()))
            else {
                continue;
            };
            let Some(&to_pos) =
                port_positions.get(&(edge.to.node_id.clone(), edge.to.port_id.clone()))
            else {
                continue;
            };
            let waypoints: Vec<Pos2> = edge
                .waypoints
                .iter()
                .map(|wp| {
                    self.canvas
                        .world_to_screen(Pos2::new(wp.x, wp.y), canvas_rect)
                })
                .collect();
            let hit_rect = edge_renderer.hit_rect(from_pos, to_pos, &waypoints);
            if cull_rect.intersects(hit_rect) {
                let is_selected = self.selected_edges.contains(&edge.id);
                edge_renderer.render_edge(
                    ui,
                    from_pos,
                    to_pos,
                    &edge.edge_type,
                    &waypoints,
                    is_selected,
                );
            }
            edge_hits.push((edge.id.clone(), hit_rect));
        }

        // 第三遍：渲染非选中节点（下层）
        for (node, definition, rect, ports, _, has_errors) in node_data.iter().filter(|d| !d.4) {
            node_renderer.render_with_data(ui, node, definition, *rect, ports, false, *has_errors);
        }

        // 第四遍：渲染选中节点（上层 / 置顶）
        for (node, definition, rect, ports, _, has_errors) in node_data.iter().filter(|d| d.4) {
            node_renderer.render_with_data(ui, node, definition, *rect, ports, true, *has_errors);
        }

        // 绘制临时拖线及目标端口状态填充
        if let Some((source_node, source_port)) = self.interaction.edge_source() {
            if let Some(&start_pos) = port_positions.get(&(source_node, source_port)) {
                let end_pos = ctx
                    .input(|i| i.pointer.hover_pos())
                    .unwrap_or(canvas_rect.center());
                let target_color = self.interaction.edge_target_color(&Theme::WIRE_DEFAULT);
                edge_renderer.render_edge_with_color(ui, start_pos, end_pos, target_color, &[]);
                if let Some(status) =
                    self.interaction
                        .edge_target_status(&self.graph, end_pos, &port_hits)
                {
                    if let Some((_, _, center, _, _)) = port_hits
                        .iter()
                        .find(|(_, _, c, _, _)| c.distance(end_pos) <= 10.0)
                    {
                        let color = status.color();
                        ui.painter().circle_filled(*center, 6.0, color);
                    }
                }
            }
        }

        // 处理交互
        let commands = self.interaction.handle_input(
            ctx,
            ui,
            &canvas_response,
            &node_hits,
            &edge_hits,
            &port_hits,
            &mut self.graph,
            &mut self.selected_nodes,
            &mut self.selected_edges,
            &mut self.clipboard,
            &self.canvas,
            &mut self.status_message,
        );
        for cmd in commands {
            match cmd {
                Command::CopySelected => self.copy_selected(),
                Command::PasteAt { screen_pos } => {
                    let world = self
                        .canvas
                        .screen_to_world(screen_pos, canvas_response.canvas_rect);
                    self.paste_at(Vec2::new(world.x, world.y));
                }
                _ => self.push_command(cmd),
            }
        }

        // 空画布欢迎提示
        if self.show_welcome_hint && self.graph.nodes.is_empty() {
            let center = canvas_rect.center();
            ui.painter().text(
                center,
                Align2::CENTER_CENTER,
                "按 Space 添加第一个节点\n或从左侧节点库选择",
                FontId::proportional(18.0),
                Theme::TEXT_DIM,
            );
        }

        // 画布信息覆盖层
        let text = if let Some(world_pos) = canvas_response.hover_world_pos {
            format!(
                "World: ({:.1}, {:.1}) | Zoom: {:.2}x | 左键拖拽节点 | 中键平移 | 滚轮缩放 | Space 搜索",
                world_pos.x, world_pos.y, self.canvas.viewport.zoom
            )
        } else {
            format!(
                "Zoom: {:.2}x | 左键拖拽节点 | 中键平移 | 滚轮缩放 | Space 搜索",
                self.canvas.viewport.zoom
            )
        };
        ui.painter().text(
            canvas_rect.min + EVec2::new(10.0, 10.0),
            Align2::LEFT_TOP,
            text,
            FontId::proportional(14.0),
            Theme::TEXT,
        );
    }
}

/// 从端口定义构造运行时端口。
fn port_from_def(p: &PortDefinition) -> Port {
    Port::new(&p.id, p.port_type.clone(), &p.label).required(p.required)
}

/// 加载字体以支持中文显示。
///
/// 优先使用仓库内置的思源黑体（Source Han Sans SC），路径为
/// `assets/fonts/思源黑体/OTF/SimplifiedChinese/`；若缺失则回退到 Windows
/// 系统字体（微软雅黑 / 黑体）。这样可在无系统字体或跨平台时仍正常显示中文。
fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // 1. 优先加载仓库内置的思源黑体 Regular
    let bundled_regular =
        r"assets\fonts\思源黑体\OTF\SimplifiedChinese\SourceHanSansSC-Regular.otf";
    let bundled_bold = r"assets\fonts\思源黑体\OTF\SimplifiedChinese\SourceHanSansSC-Bold.otf";

    if let Ok(bytes) = std::fs::read(bundled_regular) {
        fonts
            .font_data
            .insert("cjk".to_owned(), egui::FontData::from_owned(bytes).into());
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .push("cjk".to_owned());
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("cjk".to_owned());

        // 同时加载 Bold 以支持粗体中文
        if let Ok(bytes) = std::fs::read(bundled_bold) {
            fonts.font_data.insert(
                "cjk-bold".to_owned(),
                egui::FontData::from_owned(bytes).into(),
            );
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .push("cjk-bold".to_owned());
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("cjk-bold".to_owned());
        }
    } else {
        // 2. 回退：Windows 系统字体
        #[cfg(target_os = "windows")]
        {
            let candidates = [
                r"C:\Windows\Fonts\msyh.ttc",
                r"C:\Windows\Fonts\msyhbd.ttc",
                r"C:\Windows\Fonts\simhei.ttf",
            ];
            for path in &candidates {
                if let Ok(bytes) = std::fs::read(path) {
                    fonts
                        .font_data
                        .insert("cjk".to_owned(), egui::FontData::from_owned(bytes).into());
                    fonts
                        .families
                        .entry(egui::FontFamily::Proportional)
                        .or_default()
                        .push("cjk".to_owned());
                    fonts
                        .families
                        .entry(egui::FontFamily::Monospace)
                        .or_default()
                        .push("cjk".to_owned());
                    break;
                }
            }
        }
    }

    ctx.set_fonts(fonts);
}
