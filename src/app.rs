use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use egui::{Align2, FontId, Pos2, Rect, Vec2 as EVec2};

use crate::api::definitions::{NodeDefinition, PortDefinition};
use crate::api::registry::get_definition;
use crate::code_gen::generator::generate_code_to_file;
use crate::error::{FlowError, Result};
use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::node::{Node, ParamValue, Port, Vec2};
use crate::graph::types::{NodeType, PortType};
use crate::graph::validation::GraphValidator;
use crate::serializer::json::GraphDocument;
use crate::ui::canvas::Canvas;
use crate::ui::edge_renderer::EdgeRenderer;
use crate::ui::interaction::InteractionController;
use crate::ui::node_renderer::{NodeRenderer, PortGeometry};
use crate::ui::panels::{
    json_preview::JsonPreviewPanel, node_library::NodeLibraryPanel, properties::PropertiesPanel,
    status_bar::StatusBarPanel,
};
use crate::ui::theme::Theme;

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
    pub current_file: Option<PathBuf>,
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
}

impl App {
    /// 创建默认应用，并加载中文字体。
    pub fn new(cc: &eframe::CreationContext) -> Self {
        setup_fonts(&cc.egui_ctx);
        let graph = Graph::default();
        Self {
            canvas: Canvas::new(),
            selected_nodes: HashSet::new(),
            selected_edges: HashSet::new(),
            error_nodes: HashSet::new(),
            interaction: InteractionController::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            clipboard: Clipboard::default(),
            current_file: None,
            validation_errors: Vec::new(),
            search_window_open: false,
            search_query: String::new(),
            status_message: String::from("就绪"),
            graph,
            show_welcome_hint: true,
            cached_json: String::from("{}"),
            cached_json_version: 0,
            graph_version: 0,
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
        self.selected_nodes.clear();
        self.selected_nodes.insert(node.id.clone());
        self.show_welcome_hint = false;
        self.push_command(Command::AddNode { node });
        self.status_message = format!("已添加 {}", def.display_name);
    }

    /// 删除选中的节点和边。
    fn delete_selected(&mut self) {
        let mut edges_to_remove = Vec::new();
        for edge in self.graph.edges.values() {
            if self.selected_edges.contains(&edge.id)
                || self.selected_nodes.contains(&edge.from.node_id)
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

    /// 保存 JSON 到当前文件或弹出对话框选择路径。
    fn save_json(&mut self) -> Result<()> {
        let path = if let Some(ref p) = self.current_file {
            p.clone()
        } else if let Some(p) = rfd::FileDialog::new().add_filter("JSON", &["json"]).save_file() {
            p
        } else {
            self.status_message = "已取消保存".to_string();
            return Ok(());
        };
        let doc = GraphDocument::from_graph(
            self.graph.clone(),
            serde_json::Value::Object(serde_json::Map::new()),
            self.canvas.viewport.clone(),
            Vec::new(),
            Vec::new(),
        );
        let json = doc.to_json_pretty()?;
        std::fs::write(&path, json)?;
        self.current_file = Some(path.clone());
        self.status_message = format!("已保存到 {}", path.display());
        Ok(())
    }

    /// 从 JSON 文件加载图。
    fn load_json(&mut self, path: &PathBuf) -> Result<()> {
        let json = std::fs::read_to_string(path)?;
        let doc = GraphDocument::from_json(&json)?;
        self.graph = doc.graph;
        self.canvas = Canvas::with_viewport(doc.viewport);
        self.current_file = Some(path.clone());
        self.selected_nodes.clear();
        self.selected_edges.clear();
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.show_welcome_hint = self.graph.nodes.is_empty();
        self.validate();
        self.graph_version += 1;
        self.status_message = format!("已加载 {}", path.display());
        Ok(())
    }

    /// 导出 JSON（弹出保存对话框）。
    fn export_json(&mut self) -> Result<()> {
        let path = if let Some(p) = rfd::FileDialog::new().add_filter("JSON", &["json"]).save_file() {
            p
        } else {
            self.status_message = "已取消导出".to_string();
            return Ok(());
        };
        let doc = GraphDocument::from_graph(
            self.graph.clone(),
            serde_json::Value::Object(serde_json::Map::new()),
            self.canvas.viewport.clone(),
            Vec::new(),
            Vec::new(),
        );
        let json = doc.to_json_pretty()?;
        std::fs::write(&path, json)?;
        self.status_message = format!("已导出 JSON 到 {}", path.display());
        Ok(())
    }

    /// 导出 `.code` 文件（弹出保存对话框）。
    fn export_code(&mut self) -> Result<()> {
        let path = if let Some(p) = rfd::FileDialog::new().add_filter("Code", &["code"]).save_file() {
            p
        } else {
            self.status_message = "已取消导出".to_string();
            return Ok(());
        };
        generate_code_to_file(&self.graph, &path)?;
        self.status_message = format!("已导出 .code 到 {}", path.display());
        Ok(())
    }

    /// 新建空图，若有未保存节点则先确认。
    fn confirm_new_graph(&mut self) {
        if !self.graph.nodes.is_empty() {
            let confirmed = rfd::MessageDialog::new()
                .set_title("确认新建")
                .set_description("当前画布有未保存的节点，是否新建？")
                .set_buttons(rfd::MessageButtons::YesNo)
                .show();
            if !matches!(confirmed, rfd::MessageDialogResult::Yes) {
                return;
            }
        }
        self.new_graph();
    }

    /// 新建空图。
    fn new_graph(&mut self) {
        self.graph = Graph::default();
        self.canvas = Canvas::new();
        self.selected_nodes.clear();
        self.selected_edges.clear();
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.current_file = None;
        self.show_welcome_hint = true;
        self.validate();
        self.graph_version += 1;
        self.status_message = String::from("新建工程");
    }

    /// 运行预览（当前仅提示，需要在游戏中加载 .code 文件）。
    fn run_preview(&mut self) {
        self.status_message = String::from(
            "运行预览：请将导出的 .code 文件放入游戏 CustomMissions2 文件夹后启动游戏",
        );
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
        doc.to_json_pretty().unwrap_or_else(|e| format!("序列化失败: {}", e))
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 全局快捷键（在 UI 绘制之前处理，防止被控件消费）
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Space)) {
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
            if let Err(e) = self.save_json() {
                self.status_message = format!("保存失败: {}", e);
            }
        }

        // 顶部工具栏
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("新建").clicked() {
                    self.confirm_new_graph();
                }
                if ui.button("打开 JSON").clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("JSON", &["json"]).pick_file() {
                        if let Err(e) = self.load_json(&path) {
                            self.status_message = format!("加载失败: {}", e);
                            rfd::MessageDialog::new()
                                .set_title("加载失败")
                                .set_description(&format!("无法加载 JSON: {}", e))
                                .set_buttons(rfd::MessageButtons::Ok)
                                .show();
                        }
                    }
                }
                if ui.button("保存 (Ctrl+S)").clicked() {
                    if let Err(e) = self.save_json() {
                        self.status_message = format!("保存失败: {}", e);
                    }
                }
                if ui.button("导出 JSON").clicked() {
                    if let Err(e) = self.export_json() {
                        self.status_message = format!("导出 JSON 失败: {}", e);
                    }
                }
                if ui.button("导出 .code").clicked() {
                    if let Err(e) = self.export_code() {
                        self.status_message = format!("导出 .code 失败: {}", e);
                    }
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

        // 左栏节点库
        egui::SidePanel::left("node_library")
            .width_range(180.0..=300.0)
            .show(ctx, |ui| {
                let spawn_pos = self
                    .hover_world_pos(ctx, self.canvas_rect(ctx))
                    .map(|p| Vec2::new(p.x, p.y));
                if let Some(node_type) =
                    NodeLibraryPanel::show(ui, &mut self.search_query, &mut self.search_window_open)
                {
                    let pos = spawn_pos.unwrap_or(Vec2::new(0.0, 0.0));
                    self.add_node_at(node_type, pos);
                }
            });

        // 右栏属性面板
        egui::SidePanel::right("properties")
            .width_range(200.0..=400.0)
            .show(ctx, |ui| {
                if let Some(node_id) = self.selected_nodes.iter().next().cloned() {
                    if let Some(node) = self.graph.nodes.get(&node_id).cloned() {
                        if let Some((key, value)) = PropertiesPanel::show(ui, &node) {
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
                }
            });

        // 底部状态栏
        egui::TopBottomPanel::bottom("status_bar")
            .default_height(28.0)
            .show(ctx, |ui| {
                StatusBarPanel::show(
                    ui,
                    &self.status_message,
                    &self.validation_errors,
                    self.hover_world_pos(ctx, self.canvas_rect(ctx)),
                    self.canvas.viewport.zoom,
                );
            });

        // 底部 JSON 预览
        if self.graph_version != self.cached_json_version {
            self.cached_json = Self::serialize_graph(&self.graph, &self.canvas.viewport);
            self.cached_json_version = self.graph_version;
        }
        egui::TopBottomPanel::bottom("json_preview")
            .default_height(120.0)
            .show(ctx, |ui| {
                JsonPreviewPanel::show(ui, &self.cached_json);
            });

        // 中央画布
        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).fill(Theme::BACKGROUND))
            .show(ctx, |ui| {
                self.update_canvas(ctx, ui);
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
    }
}

impl App {
    /// 获取当前画布矩形（近似）。
    fn canvas_rect(&self, ctx: &egui::Context) -> Rect {
        // 由于无法直接获取 CentralPanel 剩余矩形，使用整个屏幕区域减去已知面板。
        // 实际交互中由 canvas.update 提供精确矩形。
        ctx.available_rect()
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
        let mut node_data: Vec<(&Node, &NodeDefinition, Rect, Vec<PortGeometry>, bool, bool)> = Vec::new();
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
        for edge in self.graph.edges.values() {
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
                .map(|wp| self.canvas.world_to_screen(Pos2::new(wp.x, wp.y), canvas_rect))
                .collect();
            let hit_rect = edge_renderer.hit_rect(from_pos, to_pos, &waypoints);
            if cull_rect.intersects(hit_rect) {
                let is_selected = self.selected_edges.contains(&edge.id);
                edge_renderer.render_edge(ui, from_pos, to_pos, &edge.edge_type, &waypoints, is_selected);
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
                if let Some(status) = self.interaction.edge_target_status(&self.graph, end_pos, &port_hits) {
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
            &mut self.canvas,
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
    let bundled_bold =
        r"assets\fonts\思源黑体\OTF\SimplifiedChinese\SourceHanSansSC-Bold.otf";

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
            fonts
                .font_data
                .insert("cjk-bold".to_owned(), egui::FontData::from_owned(bytes).into());
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
