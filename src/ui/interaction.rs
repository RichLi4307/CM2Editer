use std::collections::HashSet;

use egui::{Pos2, Rect, Response};

use crate::app::Command;
use crate::graph::edge::{Edge, EdgeEndpoint};
use crate::graph::graph::Graph;
use crate::graph::node::Vec2;
use crate::graph::types::PortType;
use crate::ui::canvas::Canvas;
use crate::ui::canvas::CanvasResponse;

/// 画布交互状态机。
#[derive(Debug, Clone, Default)]
pub enum CanvasState {
    /// 默认状态
    #[default]
    Idle,
    /// 左键拖拽节点
    DraggingNode {
        node_id: String,
        start_node_pos: Vec2,
        start_mouse: Pos2,
    },
    /// 从端口拖出连线
    DrawingEdge {
        source_node: String,
        source_port: String,
        source_type: PortType,
    },
    /// 框选
    BoxSelecting { start: Pos2, current: Pos2 },
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

    /// 返回当前框选矩形（如果有）。
    pub fn selection_rect(&self) -> Option<Rect> {
        match &self.state {
            CanvasState::BoxSelecting { start, current } => {
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
        _ui: &mut egui::Ui,
        canvas_response: &CanvasResponse,
        node_hits: &[(String, Rect)],
        port_hits: &[(String, String, Pos2, PortType, bool)],
        graph: &mut Graph,
        selected_nodes: &mut HashSet<String>,
        selected_edges: &mut HashSet<String>,
        canvas: &mut Canvas,
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
                response,
                canvas_rect,
                mouse_pos,
                graph,
                node_hits,
                port_hits,
                selected_nodes,
                selected_edges,
                status_message,
            ),
            CanvasState::DraggingNode {
                node_id,
                start_node_pos,
                start_mouse,
            } => self.handle_dragging_node(
                response,
                graph,
                canvas,
                &node_id,
                start_node_pos,
                start_mouse,
                &mut commands,
                status_message,
            ),
            CanvasState::DrawingEdge {
                source_node,
                source_port,
                source_type,
            } => self.handle_drawing_edge(
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
                response,
                start,
                node_hits,
                selected_nodes,
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
        _ctx: &egui::Context,
        response: &Response,
        canvas_rect: Rect,
        _mouse_pos: Option<Pos2>,
        graph: &Graph,
        node_hits: &[(String, Rect)],
        port_hits: &[(String, String, Pos2, PortType, bool)],
        selected_nodes: &mut HashSet<String>,
        selected_edges: &mut HashSet<String>,
        _status_message: &mut String,
    ) {
        // 左键按下
        if response.drag_started_by(egui::PointerButton::Primary) {
            let start_pos = response
                .interact_pointer_pos()
                .unwrap_or(canvas_rect.center());

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
                    };
                    selected_edges.clear();
                    return;
                }
            }

            // 检查是否点在节点上
            if let Some(node_id) = find_node_at(start_pos, node_hits) {
                if !selected_nodes.contains(&node_id) {
                    selected_nodes.clear();
                    selected_nodes.insert(node_id.clone());
                }
                selected_edges.clear();
                if let Some(node) = graph.nodes.get(&node_id) {
                    self.state = CanvasState::DraggingNode {
                        node_id,
                        start_node_pos: node.position,
                        start_mouse: start_pos,
                    };
                }
                return;
            }

            // 空白处：开始框选
            self.state = CanvasState::BoxSelecting {
                start: start_pos,
                current: start_pos,
            };
            return;
        }

        // 右键点击：打开上下文菜单
        if response.secondary_clicked() {
            if let Some(pos) = response.hover_pos() {
                if let Some(node_id) = find_node_at(pos, node_hits) {
                    self.context_menu = Some(pos);
                    self.context_node = Some(node_id);
                    selected_nodes.clear();
                } else {
                    self.context_menu = None;
                    self.context_node = None;
                }
            }
            return;
        }

        // 左键单击空白：取消选中
        if response.clicked() {
            selected_nodes.clear();
            selected_edges.clear();
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn handle_dragging_node(
        &mut self,
        response: &Response,
        graph: &mut Graph,
        canvas: &Canvas,
        node_id: &str,
        start_node_pos: Vec2,
        start_mouse: Pos2,
        commands: &mut Vec<Command>,
        status_message: &mut String,
    ) {
        if response.dragged_by(egui::PointerButton::Primary) {
            if let Some(current) = response.hover_pos() {
                let dx = (current.x - start_mouse.x) / canvas.viewport.zoom;
                let dy = (current.y - start_mouse.y) / canvas.viewport.zoom;
                if let Some(node) = graph.nodes.get_mut(node_id) {
                    node.position.x = start_node_pos.x + dx;
                    node.position.y = start_node_pos.y + dy;
                }
            }
        } else if response.drag_stopped() {
            if let Some(node) = graph.nodes.get(node_id) {
                if node.position != start_node_pos {
                    commands.push(Command::MoveNode {
                        node_id: node_id.to_string(),
                        from: start_node_pos,
                        to: node.position,
                    });
                }
            }
            self.state = CanvasState::Idle;
            *status_message = String::from("节点移动完成");
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn handle_drawing_edge(
        &mut self,
        response: &Response,
        canvas_rect: Rect,
        mouse_pos: Option<Pos2>,
        port_hits: &[(String, String, Pos2, PortType, bool)],
        source_node: &str,
        source_port: &str,
        source_type: &PortType,
        _graph: &mut Graph,
        selected_edges: &mut HashSet<String>,
        commands: &mut Vec<Command>,
        status_message: &mut String,
    ) {
        let end_pos = mouse_pos.unwrap_or(canvas_rect.center());
        let _ = end_pos; // 未来可用于实时高亮检测

        if response.drag_stopped() {
            // 查找释放位置的目标端口
            if let Some((target_node, target_port, target_type, is_input)) =
                find_port_at(end_pos, port_hits)
            {
                if is_input
                    && target_node != source_node
                    && target_type.is_compatible_with(source_type)
                    && !self_loop(source_node, &target_node)
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

    fn handle_box_selecting(
        &mut self,
        ctx: &egui::Context,
        response: &Response,
        start: Pos2,
        node_hits: &[(String, Rect)],
        selected_nodes: &mut HashSet<String>,
        status_message: &mut String,
    ) {
        if response.dragged_by(egui::PointerButton::Primary) {
            let current = response.hover_pos().unwrap_or(start);
            self.state = CanvasState::BoxSelecting { start, current };
        } else if response.drag_stopped() {
            let end = response.hover_pos().unwrap_or(start);
            let rect = Rect::from_two_pos(start, end);

            let extend = ctx.input(|i| i.modifiers.shift);
            let subtract = ctx.input(|i| i.modifiers.ctrl);

            if !extend && !subtract {
                selected_nodes.clear();
            }

            for (node_id, node_rect) in node_hits {
                if rect.intersects(*node_rect) {
                    if subtract {
                        selected_nodes.remove(node_id);
                    } else {
                        selected_nodes.insert(node_id.clone());
                    }
                }
            }

            self.state = CanvasState::Idle;
            *status_message = format!("已选中 {} 个节点", selected_nodes.len());
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
                    selected_nodes.clear();
                    selected_nodes.insert(node_id.to_string());
                    self.context_menu = None;
                }
                if ui.button("删除").clicked() {
                    if let Some(node) = graph.nodes.get(node_id).cloned() {
                        commands.push(Command::RemoveNode {
                            node,
                            edges: Vec::new(),
                        });
                    }
                    selected_nodes.remove(node_id);
                    self.context_menu = None;
                    *status_message = String::from("已删除节点");
                }
            });
    }
}

/// 查找指定屏幕位置下的节点。
fn find_node_at(pos: Pos2, node_hits: &[(String, Rect)]) -> Option<String> {
    node_hits
        .iter()
        .find(|(_, rect)| rect.contains(pos))
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
