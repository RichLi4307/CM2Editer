use std::collections::{HashMap, HashSet};

use egui::{Color32, Pos2, Rect, Response, Stroke};

use crate::app::{Clipboard, Command};
use crate::graph::edge::{Edge, EdgeEndpoint};
use crate::graph::graph::Graph;
use crate::graph::node::Vec2;
use crate::graph::types::PortType;
use crate::ui::canvas::Canvas;
use crate::ui::canvas::CanvasResponse;
use crate::ui::theme::Theme;

/// 画布交互状态机。
#[derive(Debug, Clone, Default)]
pub enum CanvasState {
    /// 默认状态
    #[default]
    Idle,
    /// 左键拖拽一个或多个节点
    DraggingNode {
        /// 被鼠标按住的节点 ID
        node_id: String,
        /// 拖拽起始时的鼠标屏幕坐标
        start_mouse: Pos2,
        /// 所有被拖拽节点的起始世界坐标（key = 节点 ID）
        start_positions: HashMap<String, Vec2>,
    },
    /// 从端口拖出连线
    DrawingEdge {
        source_node: String,
        source_port: String,
        source_type: PortType,
        /// 当前鼠标悬停的端口状态（用于视觉反馈）
        target_status: Option<PortTargetStatus>,
    },
    /// 框选
    BoxSelecting {
        start: Pos2,
        current: Pos2,
        mode: BoxSelectMode,
    },
}

/// 框选模式。
///
/// 依据 `docs/interaction_spec.md` 第 3.4 节：拖拽方向决定模式。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxSelectMode {
    /// 左 → 右：严格包含（Window）
    Window,
    /// 右 → 左：交叉（Crossing）
    Crossing,
}

/// 拖线过程中目标端口的状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortTargetStatus {
    /// 类型兼容且不会形成环/自环
    Compatible,
    /// 类型不兼容
    Incompatible,
    /// Data 输入端口已被占用
    Occupied,
    /// 会形成环（Flow）或自环
    Cycle,
}

impl PortTargetStatus {
    /// 返回对应视觉颜色。
    pub fn color(self) -> Color32 {
        match self {
            Self::Compatible => Theme::WIRE_DEFAULT,
            Self::Incompatible | Self::Cycle => Theme::WIRE_INVALID,
            Self::Occupied => Theme::WIRE_OCCUPIED,
        }
    }
}

/// 画布交互控制器。
#[derive(Debug, Default, Clone)]
pub struct InteractionController {
    pub state: CanvasState,
    /// 右键菜单位置（None 表示未打开）
    pub context_menu: Option<Pos2>,
    /// 右键菜单关联的节点 ID
    pub context_node: Option<String>,
}

impl InteractionController {
    /// 创建新的交互控制器。
    pub fn new() -> Self {
        Self::default()
    }

    /// 返回当前拖线的源端口（如果有）。
    pub fn edge_source(&self) -> Option<(String, String)> {
        match &self.state {
            CanvasState::DrawingEdge {
                source_node,
                source_port,
                ..
            } => Some((source_node.clone(), source_port.clone())),
            _ => None,
        }
    }

    /// 返回当前拖线源节点和端口信息（如果有）。
    fn edge_source_info(&self) -> Option<(String, String, PortType)> {
        match &self.state {
            CanvasState::DrawingEdge {
                source_node,
                source_port,
                source_type,
                ..
            } => Some((source_node.clone(), source_port.clone(), source_type.clone())),
            _ => None,
        }
    }

    /// 返回当前拖线末端应使用的颜色。
    pub fn edge_target_color(&self, default: &Color32) -> Color32 {
        match &self.state {
            CanvasState::DrawingEdge { target_status, .. } => {
                target_status.map_or(*default, |s| s.color())
            }
            _ => *default,
        }
    }

    /// 返回鼠标位置下目标端口的状态（仅当处于 DrawingEdge 状态时）。
    pub fn edge_target_status(
        &self,
        graph: &Graph,
        mouse_pos: Pos2,
        port_hits: &[(String, String, Pos2, PortType, bool)],
    ) -> Option<PortTargetStatus> {
        let (source_node, source_port, source_type) = self.edge_source_info()?;
        let (target_node, target_port, target_type, is_input) =
            find_port_at(mouse_pos, port_hits)?;
        Some(evaluate_target(
            graph,
            &source_node,
            &source_port,
            &source_type,
            &target_node,
            &target_port,
            &target_type,
            is_input,
        ))
    }

    /// 返回当前框选矩形（如果有）。
    pub fn selection_rect(&self) -> Option<Rect> {
        match &self.state {
            CanvasState::BoxSelecting { start, current, .. } => {
                Some(Rect::from_two_pos(*start, *current))
            }
            _ => None,
        }
    }

    /// 处理画布输入事件。
    #[allow(clippy::too_many_arguments)]
    pub fn handle_input(
        &mut self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        canvas_response: &CanvasResponse,
        node_hits: &[(String, Rect)],
        edge_hits: &[(String, Rect)],
        port_hits: &[(String, String, Pos2, PortType, bool)],
        graph: &mut Graph,
        selected_nodes: &mut HashSet<String>,
        selected_edges: &mut HashSet<String>,
        clipboard: &mut Clipboard,
        canvas: &Canvas,
        status_message: &mut String,
    ) -> Vec<Command> {
        let response = &canvas_response.response;
        let canvas_rect = canvas_response.canvas_rect;
        let mouse_pos = response.hover_pos();
        let mut commands = Vec::new();

        // 处理右键菜单状态：点击菜单外部关闭
        if self.context_menu.is_some() && response.clicked() {
            self.context_menu = None;
            self.context_node = None;
        }

        let state = self.state.clone();
        match state {
            CanvasState::Idle => self.handle_idle(
                ctx,
                ui,
                response,
                canvas_rect,
                mouse_pos,
                graph,
                node_hits,
                edge_hits,
                port_hits,
                selected_nodes,
                selected_edges,
                clipboard,
                status_message,
            ),
            CanvasState::DraggingNode {
                node_id,
                start_mouse,
                start_positions,
            } => self.handle_dragging_node(
                response,
                graph,
                canvas,
                &node_id,
                start_mouse,
                start_positions,
                &mut commands,
                status_message,
            ),
            CanvasState::DrawingEdge {
                source_node,
                source_port,
                source_type,
                ..
            } => self.handle_drawing_edge(
                ctx,
                ui,
                response,
                canvas_rect,
                mouse_pos,
                port_hits,
                &source_node,
                &source_port,
                &source_type,
                graph,
                selected_edges,
                &mut commands,
                status_message,
            ),
            CanvasState::BoxSelecting { start, .. } => self.handle_box_selecting(
                ctx,
                ui,
                response,
                start,
                node_hits,
                edge_hits,
                selected_nodes,
                selected_edges,
                status_message,
            ),
        }

        // 绘制右键菜单（简单弹出按钮）
        if let Some(pos) = self.context_menu {
            if let Some(node_id) = self.context_node.clone() {
                self.show_context_menu(
                    ctx,
                    pos,
                    &node_id,
                    graph,
                    selected_nodes,
                    clipboard,
                    &mut commands,
                    status_message,
                );
            } else {
                self.show_canvas_context_menu(
                    ctx,
                    pos,
                    clipboard,
                    &mut commands,
                    status_message,
                );
            }
        }

        commands
    }

    #[allow(clippy::too_many_arguments)]
    fn handle_idle(
        &mut self,
        ctx: &egui::Context,
        _ui: &mut egui::Ui,
        response: &Response,
        canvas_rect: Rect,
        mouse_pos: Option<Pos2>,
        graph: &Graph,
        node_hits: &[(String, Rect)],
        edge_hits: &[(String, Rect)],
        port_hits: &[(String, String, Pos2, PortType, bool)],
        selected_nodes: &mut HashSet<String>,
        selected_edges: &mut HashSet<String>,
        _clipboard: &mut Clipboard,
        _status_message: &mut String,
    ) {
        // 左键按下
        if response.drag_started_by(egui::PointerButton::Primary) {
            let start_pos = response
                .interact_pointer_pos()
                .unwrap_or(canvas_rect.center());
            let extend = ctx.input(|i| i.modifiers.shift);
            let subtract = ctx.input(|i| i.modifiers.ctrl);

            // 检查是否点在端口上
            if let Some((node_id, port_id, port_type, is_input)) =
                find_port_at(start_pos, port_hits)
            {
                if !is_input {
                    // 只能从输出端口拖出连线
                    self.state = CanvasState::DrawingEdge {
                        source_node: node_id,
                        source_port: port_id,
                        source_type: port_type,
                        target_status: None,
                    };
                    selected_edges.clear();
                    return;
                }
            }

            // 检查是否点在节点上
            if let Some(node_id) = find_node_at(start_pos, node_hits) {
                // 按住 Shift/Ctrl 时，即使点在节点上也要开始框选
                if extend || subtract {
                    self.start_box_select(start_pos);
                    return;
                }

                if !selected_nodes.contains(&node_id) {
                    selected_nodes.clear();
                    selected_nodes.insert(node_id.clone());
                }
                selected_edges.clear();
                if let Some(_node) = graph.nodes.get(&node_id) {
                    // 记录所有当前选中节点的起始位置，以支持多选拖拽
                    let mut start_positions = HashMap::new();
                    for id in selected_nodes.iter() {
                        if let Some(n) = graph.nodes.get(id) {
                            start_positions.insert(id.clone(), n.position);
                        }
                    }
                    self.state = CanvasState::DraggingNode {
                        node_id,
                        start_mouse: start_pos,
                        start_positions,
                    };
                }
                return;
            }

            // 空白处：开始框选
            self.start_box_select(start_pos);
            return;
        }

        // 左键单击：更新节点/边选中
        if response.clicked() {
            let pos = mouse_pos.unwrap_or(canvas_rect.center());
            if let Some(node_id) = find_node_at(pos, node_hits) {
                selected_nodes.clear();
                selected_nodes.insert(node_id);
                selected_edges.clear();
            } else if let Some(edge_id) = find_edge_at(pos, edge_hits) {
                selected_nodes.clear();
                selected_edges.clear();
                selected_edges.insert(edge_id);
            } else {
                selected_nodes.clear();
                selected_edges.clear();
            }
            return;
        }

        // 右键点击：打开上下文菜单（节点或空白处）
        if response.secondary_clicked() {
            if let Some(pos) = mouse_pos {
                if let Some(node_id) = find_node_at(pos, node_hits) {
                    self.context_menu = Some(pos);
                    self.context_node = Some(node_id);
                } else {
                    self.context_menu = Some(pos);
                    self.context_node = None;
                }
            }
        }
    }

    fn start_box_select(&mut self, start_pos: Pos2) {
        self.state = CanvasState::BoxSelecting {
            start: start_pos,
            current: start_pos,
            mode: BoxSelectMode::Window,
        };
    }

    #[allow(clippy::too_many_arguments)]
    fn handle_dragging_node(
        &mut self,
        response: &Response,
        graph: &mut Graph,
        canvas: &Canvas,
        _node_id: &str,
        start_mouse: Pos2,
        start_positions: HashMap<String, Vec2>,
        commands: &mut Vec<Command>,
        status_message: &mut String,
    ) {
        if response.dragged_by(egui::PointerButton::Primary) {
            if let Some(current) = response.hover_pos() {
                let dx = (current.x - start_mouse.x) / canvas.viewport.zoom;
                let dy = (current.y - start_mouse.y) / canvas.viewport.zoom;
                for (id, start_pos) in &start_positions {
                    if let Some(node) = graph.nodes.get_mut(id) {
                        node.position.x = start_pos.x + dx;
                        node.position.y = start_pos.y + dy;
                    }
                }
            }
        } else if response.drag_stopped() {
            for (id, start_pos) in &start_positions {
                if let Some(node) = graph.nodes.get(id) {
                    if node.position != *start_pos {
                        commands.push(Command::MoveNode {
                            node_id: id.clone(),
                            from: *start_pos,
                            to: node.position,
                        });
                    }
                }
            }
            self.state = CanvasState::Idle;
            *status_message = String::from("节点移动完成");
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn handle_drawing_edge(
        &mut self,
        _ctx: &egui::Context,
        ui: &mut egui::Ui,
        response: &Response,
        canvas_rect: Rect,
        mouse_pos: Option<Pos2>,
        port_hits: &[(String, String, Pos2, PortType, bool)],
        source_node: &str,
        source_port: &str,
        source_type: &PortType,
        graph: &Graph,
        selected_edges: &mut HashSet<String>,
        commands: &mut Vec<Command>,
        status_message: &mut String,
    ) {
        let end_pos = mouse_pos.unwrap_or(canvas_rect.center());
        let mut target_status = None;

        // 实时检测目标端口状态
        if let Some((target_node, target_port, target_type, is_input)) =
            find_port_at(end_pos, port_hits)
        {
            target_status = Some(evaluate_target(
                graph,
                source_node,
                source_port,
                source_type,
                &target_node,
                &target_port,
                &target_type,
                is_input,
            ));

            // 绘制端口高亮/提示
            self.paint_port_highlight(ui, end_pos, port_hits, target_status);
        }

        // 更新状态中的目标状态
        if let CanvasState::DrawingEdge { target_status: ts, .. } = &mut self.state {
            *ts = target_status;
        }

        if response.drag_stopped() {
            // 查找释放位置的目标端口
            if let Some((target_node, target_port, target_type, is_input)) =
                find_port_at(end_pos, port_hits)
            {
                if is_input
                    && target_node != source_node
                    && target_type.is_compatible_with(source_type)
                    && !self_loop(source_node, &target_node)
                    && !would_form_cycle(graph, source_node, &target_node)
                    && !data_port_occupied(graph, &target_node, &target_port)
                {
                    let edge = Edge::new(
                        EdgeEndpoint::new(source_node, source_port),
                        EdgeEndpoint::new(&target_node, &target_port),
                        source_type.clone(),
                    );
                    commands.push(Command::AddEdge { edge });
                    selected_edges.clear();
                    *status_message = String::from("已创建连线");
                } else {
                    *status_message = String::from("目标端口不兼容或无效");
                }
            }
            self.state = CanvasState::Idle;
        }
    }

    /// 在拖拽连线过程中绘制目标端口高亮提示。
    fn paint_port_highlight(
        &mut self,
        ui: &mut egui::Ui,
        mouse_pos: Pos2,
        port_hits: &[(String, String, Pos2, PortType, bool)],
        target_status: Option<PortTargetStatus>,
    ) {
        if let Some((_, _, center, _, _)) = port_hits
            .iter()
            .find(|(_, _, c, _, _)| c.distance(mouse_pos) <= 10.0)
        {
            let color = target_status.map_or(Theme::WIRE_DEFAULT, |s| s.color());
            let radius = 10.0;
            ui.painter()
                .circle_stroke(*center, radius, Stroke::new(2.0, color));
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn handle_box_selecting(
        &mut self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        response: &Response,
        start: Pos2,
        node_hits: &[(String, Rect)],
        edge_hits: &[(String, Rect)],
        selected_nodes: &mut HashSet<String>,
        selected_edges: &mut HashSet<String>,
        status_message: &mut String,
    ) {
        if response.dragged_by(egui::PointerButton::Primary) {
            let current = response.hover_pos().unwrap_or(start);
            let mode = box_select_mode(start, current);
            self.state = CanvasState::BoxSelecting {
                start,
                current,
                mode,
            };

            // 绘制选择框
            let rect = Rect::from_two_pos(start, current);
            let (stroke_color, fill_color) = match mode {
                BoxSelectMode::Window => (
                    Theme::BOX_SELECT_WINDOW,
                    Theme::BOX_SELECT_WINDOW.gamma_multiply(0.15),
                ),
                BoxSelectMode::Crossing => (
                    Theme::BOX_SELECT_CROSSING,
                    Theme::BOX_SELECT_CROSSING.gamma_multiply(0.15),
                ),
            };
            ui.painter().rect_filled(rect, 0.0, fill_color);
            match mode {
                BoxSelectMode::Window => {
                    ui.painter().rect_stroke(
                        rect,
                        0.0,
                        Stroke::new(2.0, stroke_color),
                        egui::StrokeKind::Middle,
                    );
                }
                BoxSelectMode::Crossing => {
                    draw_dashed_rect(ui.painter(), rect, stroke_color, 2.0, 5.0, 3.0);
                }
            }
        } else if response.drag_stopped() {
            let end = response.hover_pos().unwrap_or(start);
            let rect = Rect::from_two_pos(start, end);
            let mode = box_select_mode(start, end);

            let extend = ctx.input(|i| i.modifiers.shift);
            let subtract = ctx.input(|i| i.modifiers.ctrl);

            if !extend && !subtract {
                selected_nodes.clear();
                selected_edges.clear();
            }

            for (node_id, node_rect) in node_hits {
                if is_in_box(rect, mode, *node_rect) {
                    if subtract {
                        selected_nodes.remove(node_id);
                    } else {
                        selected_nodes.insert(node_id.clone());
                    }
                }
            }

            for (edge_id, edge_rect) in edge_hits {
                if is_in_box(rect, mode, *edge_rect) {
                    if subtract {
                        selected_edges.remove(edge_id);
                    } else {
                        selected_edges.insert(edge_id.clone());
                    }
                }
            }

            self.state = CanvasState::Idle;
            *status_message = format!(
                "已选中 {} 个节点、{} 条连线",
                selected_nodes.len(),
                selected_edges.len()
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn show_context_menu(
        &mut self,
        ctx: &egui::Context,
        pos: Pos2,
        node_id: &str,
        graph: &mut Graph,
        selected_nodes: &mut HashSet<String>,
        clipboard: &mut Clipboard,
        commands: &mut Vec<Command>,
        status_message: &mut String,
    ) {
        egui::Window::new("菜单")
            .fixed_pos(pos)
            .collapsible(false)
            .title_bar(false)
            .show(ctx, |ui| {
                if ui.button("折叠/展开").clicked() {
                    if let Some(node) = graph.nodes.get_mut(node_id) {
                        node.collapsed = !node.collapsed;
                    }
                    self.context_menu = None;
                }
                if ui.button("复制").clicked() {
                    // 若右键节点不在当前选区，则只复制该节点；否则复制整个选区
                    if !selected_nodes.contains(node_id) {
                        selected_nodes.clear();
                        selected_nodes.insert(node_id.to_string());
                    }
                    clipboard.nodes.clear();
                    clipboard.edges.clear();
                    commands.push(Command::CopySelected);
                    self.context_menu = None;
                }
                if !clipboard.nodes.is_empty() && ui.button("粘贴").clicked() {
                    commands.push(Command::PasteAt { screen_pos: pos });
                    self.context_menu = None;
                }
                if ui.button("删除").clicked() {
                    if let Some(node) = graph.nodes.get(node_id).cloned() {
                        // 收集该节点关联的边，支持撤销时恢复
                        let edges: Vec<Edge> = graph
                            .edges
                            .values()
                            .filter(|e| e.from.node_id == node_id || e.to.node_id == node_id)
                            .cloned()
                            .collect();
                        commands.push(Command::RemoveNode { node, edges });
                    }
                    selected_nodes.remove(node_id);
                    self.context_menu = None;
                    *status_message = String::from("已删除节点");
                }
            });
    }

    /// 空白处右键菜单，仅在有剪贴板内容时显示“粘贴”。
    fn show_canvas_context_menu(
        &mut self,
        ctx: &egui::Context,
        pos: Pos2,
        clipboard: &mut Clipboard,
        commands: &mut Vec<Command>,
        _status_message: &mut String,
    ) {
        egui::Window::new("画布菜单")
            .fixed_pos(pos)
            .collapsible(false)
            .title_bar(false)
            .show(ctx, |ui| {
                if !clipboard.nodes.is_empty() && ui.button("粘贴").clicked() {
                    commands.push(Command::PasteAt { screen_pos: pos });
                    self.context_menu = None;
                }
            });
    }
}

/// 根据拖拽方向决定框选模式。
fn box_select_mode(start: Pos2, current: Pos2) -> BoxSelectMode {
    if current.x >= start.x {
        BoxSelectMode::Window
    } else {
        BoxSelectMode::Crossing
    }
}

/// 判断节点/边矩形是否按指定模式位于框选矩形内。
fn is_in_box(rect: Rect, mode: BoxSelectMode, item_rect: Rect) -> bool {
    match mode {
        // Window：严格包含整个矩形
        BoxSelectMode::Window => rect.contains_rect(item_rect),
        // Crossing：只要相交即可
        BoxSelectMode::Crossing => rect.intersects(item_rect),
    }
}

/// 查找指定屏幕位置下的节点。
fn find_node_at(pos: Pos2, node_hits: &[(String, Rect)]) -> Option<String> {
    node_hits
        .iter()
        .find(|(_, rect)| rect.contains(pos))
        .map(|(id, _)| id.clone())
}

/// 查找指定屏幕位置下的连线。
fn find_edge_at(pos: Pos2, edge_hits: &[(String, Rect)]) -> Option<String> {
    const HIT_PADDING: f32 = 6.0;
    edge_hits
        .iter()
        .find(|(_, rect)| rect.expand(HIT_PADDING).contains(pos))
        .map(|(id, _)| id.clone())
}

/// 查找指定屏幕位置下的端口。
fn find_port_at(
    pos: Pos2,
    port_hits: &[(String, String, Pos2, PortType, bool)],
) -> Option<(String, String, PortType, bool)> {
    const HIT_RADIUS: f32 = 10.0;
    port_hits
        .iter()
        .find(|(_, _, center, _, _)| center.distance(pos) <= HIT_RADIUS)
        .map(|(node_id, port_id, _, port_type, is_input)| {
            (
                node_id.clone(),
                port_id.clone(),
                port_type.clone(),
                *is_input,
            )
        })
}

/// 检查是否会形成自环（同节点连接）。
fn self_loop(source_node: &str, target_node: &str) -> bool {
    source_node == target_node
}

/// 检查目标 Data 输入端口是否已经被占用。
fn data_port_occupied(graph: &Graph, target_node: &str, target_port: &str) -> bool {
    graph.edges.values().any(|e| {
        e.to.node_id == target_node && e.to.port_id == target_port && e.edge_type != PortType::Flow
    })
}

/// 检查连接是否会形成 Flow 环。
fn would_form_cycle(graph: &Graph, source_node: &str, target_node: &str) -> bool {
    // 仅在 Flow 连接上判断。若 target 可以通过已有 Flow 边到达 source，则连接 source->target 成环。
    let mut visited = HashSet::new();
    let mut stack = vec![target_node];
    while let Some(current) = stack.pop() {
        if current == source_node {
            return true;
        }
        if !visited.insert(current.to_string()) {
            continue;
        }
        for edge in graph.edges.values() {
            if edge.edge_type == PortType::Flow && edge.from.node_id == current {
                stack.push(&edge.to.node_id);
            }
        }
    }
    false
}

/// 评估目标端口状态。
fn evaluate_target(
    graph: &Graph,
    source_node: &str,
    _source_port: &str,
    source_type: &PortType,
    target_node: &str,
    target_port: &str,
    target_type: &PortType,
    is_input: bool,
) -> PortTargetStatus {
    if !is_input {
        return PortTargetStatus::Incompatible;
    }
    if target_node == source_node {
        return PortTargetStatus::Cycle;
    }
    if !target_type.is_compatible_with(source_type) {
        return PortTargetStatus::Incompatible;
    }
    if *source_type == PortType::Flow && would_form_cycle(graph, source_node, target_node) {
        return PortTargetStatus::Cycle;
    }
    if *source_type != PortType::Flow && data_port_occupied(graph, target_node, target_port) {
        return PortTargetStatus::Occupied;
    }
    PortTargetStatus::Compatible
}

/// 绘制虚线矩形边框。
fn draw_dashed_rect(
    painter: &egui::Painter,
    rect: Rect,
    color: Color32,
    width: f32,
    dash_len: f32,
    gap_len: f32,
) {
    let segments = [
        (rect.left_top(), rect.right_top()),
        (rect.right_top(), rect.right_bottom()),
        (rect.right_bottom(), rect.left_bottom()),
        (rect.left_bottom(), rect.left_top()),
    ];
    for (a, b) in segments {
        draw_dashed_line(painter, a, b, color, width, dash_len, gap_len);
    }
}

/// 绘制从 `from` 到 `to` 的虚线。
fn draw_dashed_line(
    painter: &egui::Painter,
    from: Pos2,
    to: Pos2,
    color: Color32,
    width: f32,
    dash_len: f32,
    gap_len: f32,
) {
    let dir = to - from;
    let len = dir.length();
    if len < 1e-5 {
        return;
    }
    let unit = dir / len;
    let mut distance = 0.0;
    while distance < len {
        let start = from + unit * distance;
        let end = from + unit * (distance + dash_len).min(len);
        painter.line_segment([start, end], Stroke::new(width, color));
        distance += dash_len + gap_len;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_node_at() {
        let hits = vec![(
            "node_1".to_string(),
            Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(100.0, 100.0)),
        )];
        assert_eq!(
            find_node_at(Pos2::new(50.0, 50.0), &hits),
            Some("node_1".to_string())
        );
        assert_eq!(find_node_at(Pos2::new(150.0, 150.0), &hits), None);
    }

    #[test]
    fn test_find_port_at() {
        let hits = vec![(
            "node_1".to_string(),
            "out_flow".to_string(),
            Pos2::new(10.0, 10.0),
            PortType::Flow,
            false,
        )];
        let result = find_port_at(Pos2::new(12.0, 12.0), &hits);
        assert!(result.is_some());
        let (node_id, port_id, port_type, is_input) = result.unwrap();
        assert_eq!(node_id, "node_1");
        assert_eq!(port_id, "out_flow");
        assert_eq!(port_type, PortType::Flow);
        assert!(!is_input);
    }
}
