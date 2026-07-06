use std::collections::HashMap;

use CM2Editer::api::definitions::PortDefinition;
use CM2Editer::api::registry::get_definition;
use CM2Editer::graph::{
    edge::{Edge, EdgeEndpoint},
    graph::Graph,
    node::{Node, ParamValue, Port, Vec2},
    types::{NodeType, PortType},
};
use CM2Editer::ui::canvas::Canvas;
use CM2Editer::ui::edge_renderer::EdgeRenderer;
use CM2Editer::ui::node_renderer::NodeRenderer;
use CM2Editer::ui::theme::{Theme, category_color};

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "CM2Editer",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(Cm2EditorApp::new()))),
    )
}

struct Cm2EditorApp {
    graph: Graph,
    canvas: Canvas,
    selected_nodes: Vec<String>,
    error_nodes: Vec<String>,
}

impl Cm2EditorApp {
    fn new() -> Self {
        let graph = build_sample_graph();
        Self {
            selected_nodes: vec![id_by_type(&graph, NodeType::If)],
            error_nodes: vec![id_by_type(&graph, NodeType::Log)],
            graph,
            canvas: Canvas::new(),
        }
    }
}

impl eframe::App for Cm2EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).fill(Theme::BACKGROUND))
            .show(ctx, |ui| {
                let canvas_response = self.canvas.update(ui);
                let mut port_positions: HashMap<(String, String), egui::Pos2> = HashMap::new();
                let node_renderer = NodeRenderer::default();

                // 渲染节点
                for node in self.graph.nodes.values() {
                    let Some(definition) = get_definition(node.node_type) else {
                        continue;
                    };
                    let is_selected = self.selected_nodes.contains(&node.id);
                    let has_errors = self.error_nodes.contains(&node.id);
                    let response = node_renderer.render(
                        ui,
                        &self.canvas,
                        node,
                        definition,
                        is_selected,
                        has_errors,
                    );
                    for port in response.ports {
                        port_positions.insert((node.id.clone(), port.id), port.center);
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
                    let waypoints: Vec<egui::Pos2> = edge
                        .waypoints
                        .iter()
                        .map(|wp| {
                            self.canvas.world_to_screen(
                                egui::Pos2::new(wp.x, wp.y),
                                canvas_response.canvas_rect,
                            )
                        })
                        .collect();
                    edge_renderer.render_edge(
                        ui,
                        from_pos,
                        to_pos,
                        &edge.edge_type,
                        &waypoints,
                        false,
                    );
                }

                // 信息覆盖层
                let text = if let Some(world_pos) = canvas_response.hover_world_pos {
                    format!(
                        "World: ({:.1}, {:.1}) | Zoom: {:.2}x | 中键拖拽平移 | 滚轮缩放",
                        world_pos.x, world_pos.y, self.canvas.viewport.zoom
                    )
                } else {
                    format!(
                        "Zoom: {:.2}x | 中键拖拽平移 | 滚轮缩放",
                        self.canvas.viewport.zoom
                    )
                };
                ui.painter().text(
                    canvas_response.canvas_rect.min + egui::vec2(10.0, 10.0),
                    egui::Align2::LEFT_TOP,
                    text,
                    egui::FontId::proportional(14.0),
                    Theme::TEXT,
                );

                // 右下角分类色表预览
                draw_category_preview(ui, canvas_response.canvas_rect);
            });
    }
}

/// 在画布右下角绘制分类色表预览。
fn draw_category_preview(ui: &egui::Ui, canvas_rect: egui::Rect) {
    let categories = [
        "Control",
        "General Functions",
        "Game Functions: Stats",
        "Objects",
        "Math",
        "String",
        "File",
        "Special",
    ];
    let preview_width = 140.0;
    let preview_height = categories.len() as f32 * 22.0 + 8.0;
    let preview_rect = egui::Rect::from_min_size(
        canvas_rect.max - egui::vec2(preview_width + 10.0, preview_height + 10.0),
        egui::vec2(preview_width, preview_height),
    );
    ui.painter()
        .rect_filled(preview_rect, 4.0, Theme::NODE_BACKGROUND);
    for (i, category) in categories.iter().enumerate() {
        let color = category_color(category);
        let y = preview_rect.min.y + 4.0 + i as f32 * 22.0;
        let color_rect = egui::Rect::from_min_size(
            egui::Pos2::new(preview_rect.min.x + 6.0, y),
            egui::vec2(16.0, 16.0),
        );
        ui.painter().rect_filled(color_rect, 3.0, color);
        ui.painter().text(
            egui::Pos2::new(preview_rect.min.x + 28.0, y + 8.0),
            egui::Align2::LEFT_CENTER,
            *category,
            egui::FontId::proportional(12.0),
            Theme::TEXT,
        );
    }
}

/// 从端口定义构造运行时端口。
fn port_from_def(p: &PortDefinition) -> Port {
    Port::new(&p.id, p.port_type.clone(), &p.label).required(p.required)
}

/// 按类型查找图中节点 ID（用于演示）。
fn id_by_type(graph: &Graph, node_type: NodeType) -> String {
    graph
        .nodes
        .iter()
        .find(|(_, n)| n.node_type == node_type)
        .map(|(id, _)| id.clone())
        .unwrap_or_default()
}

/// 构建一个演示用的示例图：Start -> If -> Log。
fn build_sample_graph() -> Graph {
    let mut graph = Graph::default();

    let start_def = match get_definition(NodeType::Start) {
        Some(d) => d,
        None => return graph,
    };
    let if_def = match get_definition(NodeType::If) {
        Some(d) => d,
        None => return graph,
    };
    let log_def = match get_definition(NodeType::Log) {
        Some(d) => d,
        None => return graph,
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
