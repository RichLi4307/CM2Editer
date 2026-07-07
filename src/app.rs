use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use egui::{Align2, FontId, Pos2, Rect, Vec2 as EVec2};

use crate::api::definitions::PortDefinition;
use crate::api::registry::get_definition;
use crate::code_gen::generator::generate_code_to_file;
use crate::error::Result;
use crate::graph::edge::{Edge, EdgeEndpoint};
use crate::graph::graph::Graph;
use crate::graph::node::{Node, ParamValue, Port, Vec2};
use crate::graph::types::{NodeType, PortType};
use crate::graph::validation::GraphValidator;
use crate::serializer::json::GraphDocument;
use crate::ui::canvas::Canvas;
use crate::ui::edge_renderer::EdgeRenderer;
use crate::ui::interaction::InteractionController;
use crate::ui::node_renderer::NodeRenderer;
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
}

impl Command {
    /// 应用命令。
    fn apply(&self, graph: &mut Graph) {
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
        }
    }
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
    pub clipboard: Vec<Node>,
    pub current_file: Option<PathBuf>,
    pub validation_errors: Vec<crate::error::FlowError>,
    pub error_nodes: HashSet<String>,
    pub search_window_open: bool,
    pub search_query: String,
    pub status_message: String,
}

impl App {
    /// 创建默认应用，并加载一个示例图与中文字体。
    pub fn new(cc: &eframe::CreationContext) -> Self {
        setup_fonts(&cc.egui_ctx);
        let graph = build_sample_graph();
        Self {
            canvas: Canvas::new(),
            selected_nodes: HashSet::new(),
            selected_edges: HashSet::new(),
            error_nodes: {
                let mut set = HashSet::new();
                set.insert(id_by_type(&graph, NodeType::Log));
                set
            },
            interaction: InteractionController::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            clipboard: Vec::new(),
            current_file: None,
            validation_errors: Vec::new(),
            search_window_open: false,
            search_query: String::new(),
            status_message: String::from("就绪"),
            graph,
        }
    }

    /// 执行命令并压入 Undo 栈。
    fn push_command(&mut self, cmd: Command) {
        cmd.apply(&mut self.graph);
        self.undo_stack.push(cmd);
        if self.undo_stack.len() > 50 {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
        self.validate();
    }

    /// 撤销。
    fn undo(&mut self) {
        if let Some(cmd) = self.undo_stack.pop() {
            cmd.undo(&mut self.graph);
            self.redo_stack.push(cmd);
            self.validate();
            self.status_message = String::from("已撤销");
        }
    }

    /// 重做。
    fn redo(&mut self) {
        if let Some(cmd) = self.redo_stack.pop() {
            cmd.apply(&mut self.graph);
            self.undo_stack.push(cmd);
            self.validate();
            self.status_message = String::from("已重做");
        }
    }

    /// 验证图并更新错误列表。
    fn validate(&mut self) {
        self.validation_errors = match GraphValidator::validate(&self.graph) {
            Ok(()) => Vec::new(),
            Err(e) => vec![e],
        };
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

    /// 保存 JSON 到当前文件或默认路径。
    fn save_json(&mut self) -> Result<()> {
        let path = self
            .current_file
            .clone()
            .unwrap_or_else(|| PathBuf::from("CM2Editer.json"));
        let doc = GraphDocument::from_graph(
            self.graph.clone(),
            serde_json::Value::Object(serde_json::Map::new()),
            self.canvas.viewport.clone(),
            Vec::new(),
            Vec::new(),
        );
        let json = doc.to_json_pretty()?;
        std::fs::write(&path, json)?;
        self.status_message = format!("已保存到 {}", path.display());
        Ok(())
    }

    /// 导出 JSON 到固定导出路径。
    fn export_json(&mut self) -> Result<()> {
        let path = PathBuf::from("CM2Editer_export.json");
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

    /// 导出 `.code` 文件。
    fn export_code(&mut self) -> Result<()> {
        let path = PathBuf::from("CM2Editer.code");
        generate_code_to_file(&self.graph, &path)?;
        self.status_message = format!("已导出 .code 到 {}", path.display());
        Ok(())
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
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 全局快捷键
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Space) {
                self.search_window_open = !self.search_window_open;
            }
            if i.modifiers.ctrl && i.key_pressed(egui::Key::Z) {
                self.undo();
            }
            if i.modifiers.ctrl && i.key_pressed(egui::Key::Y) {
                self.redo();
            }
            if i.modifiers.ctrl && i.key_pressed(egui::Key::S) {
                if let Err(e) = self.save_json() {
                    self.status_message = format!("保存失败: {}", e);
                }
            }
            if i.key_pressed(egui::Key::Delete) {
                self.delete_selected();
            }
        });

        // 顶部工具栏
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
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
                if ui.button("添加节点 (Space)").clicked() {
                    self.search_window_open = !self.search_window_open;
                }
                if ui.button("删除 (Del)").clicked() {
                    self.delete_selected();
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
                    self.validation_errors.len(),
                    self.hover_world_pos(ctx, self.canvas_rect(ctx)),
                    self.canvas.viewport.zoom,
                );
            });

        // 底部 JSON 预览
        egui::TopBottomPanel::bottom("json_preview")
            .default_height(120.0)
            .show(ctx, |ui| {
                JsonPreviewPanel::show(ui, &self.graph, &self.canvas.viewport);
            });

        // 中央画布
        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).fill(Theme::BACKGROUND))
            .show(ctx, |ui| {
                self.update_canvas(ctx, ui);
            });

        // 搜索窗口
        if self.search_window_open {
            egui::Window::new("添加节点")
                .collapsible(false)
                .show(ctx, |ui| {
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
        let mut port_positions: HashMap<(String, String), Pos2> = HashMap::new();
        let mut node_hits: Vec<(String, Rect)> = Vec::new();
        let mut port_hits: Vec<(String, String, Pos2, PortType, bool)> = Vec::new();
        let node_renderer = NodeRenderer::default();

        // 渲染节点并收集命中区域
        for node in self.graph.nodes.values() {
            let Some(definition) = get_definition(node.node_type) else {
                continue;
            };
            let is_selected = self.selected_nodes.contains(&node.id);
            let has_errors = self.error_nodes.contains(&node.id);
            let response =
                node_renderer.render(ui, &self.canvas, node, definition, is_selected, has_errors);
            node_hits.push((node.id.clone(), response.rect));
            for port in response.ports {
                port_positions.insert((node.id.clone(), port.id.clone()), port.center);
                port_hits.push((
                    node.id.clone(),
                    port.id,
                    port.center,
                    port.port_type,
                    port.is_input,
                ));
            }
        }

        // 渲染连线
        let edge_renderer = EdgeRenderer::default();
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
                .map(|wp| {
                    self.canvas
                        .world_to_screen(Pos2::new(wp.x, wp.y), canvas_rect)
                })
                .collect();
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

        // 绘制临时拖线
        if let Some((source_node, source_port)) = self.interaction.edge_source() {
            if let Some(&start_pos) = port_positions.get(&(source_node, source_port)) {
                let end_pos = ctx
                    .input(|i| i.pointer.hover_pos())
                    .unwrap_or(canvas_rect.center());
                edge_renderer.render_edge(ui, start_pos, end_pos, &PortType::Flow, &[], true);
            }
        }

        // 处理交互
        let commands = self.interaction.handle_input(
            ctx,
            ui,
            &canvas_response,
            &node_hits,
            &port_hits,
            &mut self.graph,
            &mut self.selected_nodes,
            &mut self.selected_edges,
            &mut self.canvas,
            &mut self.status_message,
        );
        for cmd in commands {
            self.push_command(cmd);
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

/// 构建示例图：Start -> If -> Log。
fn build_sample_graph() -> Graph {
    let mut graph = Graph::default();

    let Some(start_def) = get_definition(NodeType::Start) else {
        return graph;
    };
    let Some(if_def) = get_definition(NodeType::If) else {
        return graph;
    };
    let Some(log_def) = get_definition(NodeType::Log) else {
        return graph;
    };

    let mut start = Node::new(NodeType::Start, Vec2::new(-200.0, 0.0));
    start.outputs = start_def.outputs.iter().map(port_from_def).collect();
    start.category = start_def.category.clone();

    let mut if_node = Node::new(NodeType::If, Vec2::new(0.0, 0.0));
    if_node.inputs = if_def.inputs.iter().map(port_from_def).collect();
    if_node.outputs = if_def.outputs.iter().map(port_from_def).collect();
    if_node.set_param("condition", ParamValue::Literal(serde_json::json!(true)));
    if_node.category = if_def.category.clone();

    let mut log = Node::new(NodeType::Log, Vec2::new(240.0, 0.0));
    log.inputs = log_def.inputs.iter().map(port_from_def).collect();
    log.outputs = log_def.outputs.iter().map(port_from_def).collect();
    log.set_param("output", ParamValue::Literal(serde_json::json!("流程结束")));
    log.category = log_def.category.clone();

    graph.add_node(start);
    graph.add_node(if_node);
    graph.add_node(log);

    let id_start = id_by_type(&graph, NodeType::Start);
    let id_if = id_by_type(&graph, NodeType::If);
    let id_log = id_by_type(&graph, NodeType::Log);

    if !id_start.is_empty() && !id_if.is_empty() {
        let _ = graph.add_edge(Edge::new(
            EdgeEndpoint::new(&id_start, "out_flow"),
            EdgeEndpoint::new(&id_if, "in_flow"),
            PortType::Flow,
        ));
    }
    if !id_if.is_empty() && !id_log.is_empty() {
        let _ = graph.add_edge(Edge::new(
            EdgeEndpoint::new(&id_if, "out_true"),
            EdgeEndpoint::new(&id_log, "in_flow"),
            PortType::Flow,
        ));
    }

    graph
}

/// 按类型查找节点 ID。
fn id_by_type(graph: &Graph, node_type: NodeType) -> String {
    graph
        .nodes
        .iter()
        .find(|(_, n)| n.node_type == node_type)
        .map(|(id, _)| id.clone())
        .unwrap_or_default()
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
