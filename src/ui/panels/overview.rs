use egui::{Align2, Color32, FontId, Pos2, Rect, Stroke, Vec2};
use std::collections::{HashMap, HashSet};

use crate::graph::container::{ContainerGraph, LabelContainer};
use crate::graph::node::ParamValue;
use crate::graph::types::NodeType;
use crate::ui::theme::Theme;

/// 概览图节点。
#[derive(Debug, Clone)]
struct OverviewNode {
    id: String,
    name: String,
    thread_idx: usize,
    kind: OverviewContainerKind,
    idx: usize,
}

/// 概览图边。
#[derive(Debug, Clone)]
struct OverviewEdge {
    from: String,
    to: String,
    kind: OverviewEdgeKind,
}

/// 容器类型（概览图内部使用）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverviewContainerKind {
    Label,
    Listener,
}

/// 概览图边类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OverviewEdgeKind {
    Goto,
    CreateThread,
    CreateListener,
    CreateListenerLocal,
    Foreach,
}

/// 概览面板动作。
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum OverviewAction {
    /// 无动作。
    #[default]
    None,
    /// 双击节点，请求切换到对应容器。
    SelectContainer {
        thread_idx: usize,
        kind: OverviewContainerKind,
        idx: usize,
    },
}

/// 线程概览图面板。
///
/// 以简单网格布局展示当前 `.code` 文件内的所有标签/监听器，以及它们之间通过
/// `Goto`、`CreateThread`、`CreateListener` 和 `ForeachNode` 建立的关系。
/// 双击节点可跳转到对应容器的画布视图。
pub struct OverviewPanel;

impl OverviewPanel {
    /// 显示概览图面板，返回用户触发的动作。
    pub fn show(ui: &mut egui::Ui, graph: &ContainerGraph) -> OverviewAction {
        let mut action = OverviewAction::None;
        let graph_data = build_overview_graph(graph);
        let layout = layout_nodes(&graph_data.nodes, ui.available_width());
        let rect = ui.available_rect_before_wrap();
        let painter = ui.painter();

        // 绘制边
        for edge in &graph_data.edges {
            if let (Some(&from), Some(&to)) = (layout.get(&edge.from), layout.get(&edge.to)) {
                draw_edge(painter, from, to, edge.kind);
            }
        }

        // 绘制节点
        for node in &graph_data.nodes {
            let Some(&center) = layout.get(&node.id) else {
                continue;
            };
            let node_rect = Rect::from_center_size(center, NODE_SIZE);
            if !rect.intersects(node_rect) {
                continue;
            }
            let response = ui.interact(node_rect, ui.id().with(&node.id), egui::Sense::click());
            let bg = if response.hovered() {
                Theme::ENTRY_PIN.gamma_multiply(0.3)
            } else {
                Theme::NODE_BACKGROUND
            };
            painter.rect_filled(node_rect, 6_u8, bg);
            painter.rect_stroke(
                node_rect,
                6_u8,
                Stroke::new(1.0, Theme::NODE_BORDER),
                egui::StrokeKind::Middle,
            );
            painter.text(
                node_rect.center(),
                Align2::CENTER_CENTER,
                &node.name,
                FontId::proportional(11.0),
                Theme::TEXT,
            );
            if response.double_clicked() {
                action = OverviewAction::SelectContainer {
                    thread_idx: node.thread_idx,
                    kind: node.kind,
                    idx: node.idx,
                };
            }
        }

        action
    }
}

const NODE_SIZE: Vec2 = Vec2::new(120.0, 40.0);
const NODE_GAP: Vec2 = Vec2::new(24.0, 32.0);

/// 为所有节点计算简单网格布局。
fn layout_nodes(nodes: &[OverviewNode], available_width: f32) -> HashMap<String, Pos2> {
    let columns = ((available_width + NODE_GAP.x) / (NODE_SIZE.x + NODE_GAP.x))
        .floor()
        .max(1.0) as usize;
    let mut layout = HashMap::new();
    for (i, node) in nodes.iter().enumerate() {
        let col = i % columns;
        let row = i / columns;
        let x = col as f32 * (NODE_SIZE.x + NODE_GAP.x) + NODE_SIZE.x * 0.5;
        let y = row as f32 * (NODE_SIZE.y + NODE_GAP.y) + NODE_SIZE.y * 0.5;
        layout.insert(node.id.clone(), Pos2::new(x, y));
    }
    layout
}

/// 构建概览图数据。
fn build_overview_graph(graph: &ContainerGraph) -> OverviewGraph {
    let mut nodes = Vec::new();
    let mut name_to_id: HashMap<String, String> = HashMap::new();
    let mut seen = HashSet::new();

    for (t_idx, thread) in graph.threads.iter().enumerate() {
        for (l_idx, label) in thread.labels.iter().enumerate() {
            let id = label.id.clone();
            name_to_id.insert(label.name.clone(), id.clone());
            nodes.push(OverviewNode {
                id,
                name: label.name.clone(),
                thread_idx: t_idx,
                kind: OverviewContainerKind::Label,
                idx: l_idx,
            });
        }
        for (l_idx, listener) in thread.listeners.iter().enumerate() {
            let id = listener.inner.id.clone();
            name_to_id.insert(listener.name().to_string(), id.clone());
            nodes.push(OverviewNode {
                id,
                name: listener.name().to_string(),
                thread_idx: t_idx,
                kind: OverviewContainerKind::Listener,
                idx: l_idx,
            });
        }
    }

    // 去重：同名标签或监听器可能同时出现，保留第一次遇到的 id。
    let mut unique_nodes = Vec::new();
    for node in nodes {
        if seen.insert(node.name.clone()) {
            unique_nodes.push(node);
        }
    }

    let mut edges = Vec::new();
    for thread in &graph.threads {
        for label in &thread.labels {
            collect_edges(label, &mut edges, &name_to_id);
        }
        for listener in &thread.listeners {
            collect_edges(&listener.inner, &mut edges, &name_to_id);
        }
    }

    OverviewGraph {
        nodes: unique_nodes,
        edges,
    }
}

/// 从单个容器中提取所有关系边。
fn collect_edges(
    label: &LabelContainer,
    edges: &mut Vec<OverviewEdge>,
    name_to_id: &HashMap<String, String>,
) {
    for node in label.nodes.values() {
        match node.node_type {
            NodeType::Goto => {
                if let Some(target) = literal_string_param(node.params.get("label")) {
                    if let Some(to) = name_to_id.get(&target) {
                        edges.push(OverviewEdge {
                            from: label.id.clone(),
                            to: to.clone(),
                            kind: OverviewEdgeKind::Goto,
                        });
                    }
                }
            }
            NodeType::CreateThread => {
                if let Some(target) = literal_string_param(node.params.get("labelName")) {
                    if let Some(to) = name_to_id.get(&target) {
                        edges.push(OverviewEdge {
                            from: label.id.clone(),
                            to: to.clone(),
                            kind: OverviewEdgeKind::CreateThread,
                        });
                    }
                }
            }
            NodeType::CreateListener => {
                if let Some(target) = literal_string_param(node.params.get("labelName")) {
                    if let Some(to) = name_to_id.get(&target) {
                        edges.push(OverviewEdge {
                            from: label.id.clone(),
                            to: to.clone(),
                            kind: OverviewEdgeKind::CreateListener,
                        });
                    }
                }
            }
            NodeType::CreateListenerLocal => {
                if let Some(target) = literal_string_param(node.params.get("labelName")) {
                    if let Some(to) = name_to_id.get(&target) {
                        edges.push(OverviewEdge {
                            from: label.id.clone(),
                            to: to.clone(),
                            kind: OverviewEdgeKind::CreateListenerLocal,
                        });
                    }
                }
            }
            NodeType::ForeachNode => {
                if let Some(target) = literal_string_param(node.params.get("threadVar")) {
                    if let Some(to) = name_to_id.get(&target) {
                        edges.push(OverviewEdge {
                            from: label.id.clone(),
                            to: to.clone(),
                            kind: OverviewEdgeKind::Foreach,
                        });
                    }
                }
            }
            _ => {}
        }
    }
}

/// 从参数值中提取字符串字面量。
fn literal_string_param(value: Option<&ParamValue>) -> Option<String> {
    match value {
        Some(ParamValue::Literal(v)) if v.is_string() => Some(v.as_str()?.to_string()),
        _ => None,
    }
}

/// 绘制概览图边。
fn draw_edge(painter: &egui::Painter, from: Pos2, to: Pos2, kind: OverviewEdgeKind) {
    let (color, width, dashed) = match kind {
        OverviewEdgeKind::Goto => (Theme::WIRE_DEFAULT, 2.0, false),
        OverviewEdgeKind::CreateThread => (Theme::SELECTED_GLOW, 1.5, true),
        OverviewEdgeKind::CreateListener => (Theme::WIRE_OCCUPIED, 1.5, true),
        OverviewEdgeKind::CreateListenerLocal => (Theme::WIRE_OCCUPIED, 1.5, true),
        OverviewEdgeKind::Foreach => (Theme::TEXT_DIM, 1.5, true),
    };

    let from = edge_anchor(from, to, NODE_SIZE * 0.5);
    let to = edge_anchor(to, from, NODE_SIZE * 0.5);

    if dashed {
        draw_dashed_line(painter, from, to, color, width, 5.0, 3.0);
    } else {
        painter.line_segment([from, to], Stroke::new(width, color));
    }
}

/// 计算从矩形中心出发、朝向目标方向的边缘点。
fn edge_anchor(center: Pos2, target: Pos2, half_size: Vec2) -> Pos2 {
    let dx = target.x - center.x;
    let dy = target.y - center.y;
    if dx == 0.0 && dy == 0.0 {
        return center;
    }
    let angle = dy.atan2(dx);
    let x = half_size.x * angle.cos();
    let y = half_size.y * angle.sin();
    Pos2::new(center.x + x, center.y + y)
}

/// 绘制虚线。
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

struct OverviewGraph {
    nodes: Vec<OverviewNode>,
    edges: Vec<OverviewEdge>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::container::{ContainerGraph, LabelContainer, ListenerContainer, ListenerKind};
    use crate::graph::node::{Node, ParamValue, Port, Vec2};
    use crate::graph::types::{NodeType, PortType};
    use std::collections::HashMap;

    fn make_label(name: &str) -> LabelContainer {
        LabelContainer {
            id: format!("label_{}", name),
            name: name.to_string(),
            ..Default::default()
        }
    }

    fn make_node(id: &str, node_type: NodeType) -> Node {
        Node {
            id: id.to_string(),
            node_type,
            position: Vec2::default(),
            size: Vec2::new(180.0, 120.0),
            collapsed: false,
            params: HashMap::new(),
            inputs: vec![Port::new("in_flow", PortType::Flow, "执行")],
            outputs: vec![Port::new("out_flow", PortType::Flow, "下一步")],
            category: "Control".to_string(),
        }
    }

    #[test]
    fn test_build_overview_graph_finds_goto() {
        let mut graph = ContainerGraph::default_main();
        let mut goto = make_node("goto1", NodeType::Goto);
        goto.set_param("label", ParamValue::Literal(serde_json::json!("sub")));
        graph.threads[0].labels.push(make_label("sub"));
        graph.threads[0].labels[0].nodes.insert("goto1".to_string(), goto);

        let data = build_overview_graph(&graph);
        assert_eq!(data.nodes.len(), 2);
        assert!(data.edges.iter().any(|e| {
            e.from == "label_main" && e.to == "label_sub" && e.kind == OverviewEdgeKind::Goto
        }));
    }

    #[test]
    fn test_layout_nodes_produces_unique_positions() {
        let mut nodes = Vec::new();
        for i in 0..4 {
            nodes.push(OverviewNode {
                id: format!("n{}", i),
                name: format!("node{}", i),
                thread_idx: 0,
                kind: OverviewContainerKind::Label,
                idx: i,
            });
        }
        let layout = layout_nodes(&nodes, 400.0);
        assert_eq!(layout.len(), 4);
        assert!(layout.values().any(|p| p.x > 0.0));
    }

    #[test]
    fn test_build_overview_graph_create_listener() {
        let mut graph = ContainerGraph::default_main();
        let mut create_listener = make_node("cl", NodeType::CreateListener);
        create_listener.set_param(
            "labelName",
            ParamValue::Literal(serde_json::json!("check_loop")),
        );
        graph.threads[0].labels[0].nodes.insert("cl".to_string(), create_listener);

        let listener_label = LabelContainer {
            id: "label_check_loop".to_string(),
            name: "check_loop".to_string(),
            ..Default::default()
        };
        let listener = ListenerContainer {
            inner: listener_label,
            kind: ListenerKind::Listener,
            variable_name: "var_check_loop_listener".to_string(),
        };
        graph.threads[0].listeners.push(listener);

        let data = build_overview_graph(&graph);
        assert_eq!(data.nodes.len(), 2);
        assert!(data.edges.iter().any(|e| {
            e.from == "label_main"
                && e.to == "label_check_loop"
                && e.kind == OverviewEdgeKind::CreateListener
        }));
    }
}
