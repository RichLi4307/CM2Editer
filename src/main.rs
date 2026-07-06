pub mod api;
pub mod error;
pub mod graph;

use api::registry::all_node_definitions;
use graph::{
    edge::Edge, edge::EdgeEndpoint, graph::Graph, node::Node, node::ParamValue, node::Port,
    node::Vec2, types::NodeType, types::PortType, validation::GraphValidator,
};

fn main() {
    // UX-DEBT(Phase3): 当前启动直接进空画布，需评估欢迎页/模板启动。
    // 见 docs/interaction_spec.md §1.1
    // ── 构建一个完整的 Graph：Start → If → Label → Goto → Log ──
    let mut g = Graph::default();

    // 从注册表获取节点定义
    let defs = all_node_definitions();
    let Some(start_def) = defs.iter().find(|d| d.node_type == NodeType::Start) else {
        println!("❌ Start 定义不存在");
        return;
    };
    let Some(if_def) = defs.iter().find(|d| d.node_type == NodeType::If) else {
        println!("❌ If 定义不存在");
        return;
    };
    let Some(label_def) = defs.iter().find(|d| d.node_type == NodeType::Label) else {
        println!("❌ Label 定义不存在");
        return;
    };
    let Some(goto_def) = defs.iter().find(|d| d.node_type == NodeType::Goto) else {
        println!("❌ Goto 定义不存在");
        return;
    };
    let Some(log_def) = defs.iter().find(|d| d.node_type == NodeType::Log) else {
        println!("❌ Log 定义不存在");
        return;
    };

    // 从 PortDefinition 转换为 Port
    fn port(p: &api::definitions::PortDefinition) -> Port {
        Port::new(&p.id, p.port_type.clone(), &p.label).required(p.required)
    }

    // 创建节点
    let mut n_start = Node::new(NodeType::Start, Vec2::new(100.0, 100.0));
    n_start.outputs = start_def.outputs.iter().map(port).collect();

    let mut n_if = Node::new(NodeType::If, Vec2::new(300.0, 100.0));
    n_if.inputs = if_def.inputs.iter().map(port).collect();
    n_if.outputs = if_def.outputs.iter().map(port).collect();
    n_if.set_param("condition", ParamValue::Literal(serde_json::json!(true)));

    let mut n_label = Node::new(NodeType::Label, Vec2::new(500.0, 100.0));
    n_label.inputs = label_def.inputs.iter().map(port).collect();
    n_label.outputs = label_def.outputs.iter().map(port).collect();
    n_label.set_param("name", ParamValue::Literal(serde_json::json!("done")));

    let mut n_goto = Node::new(NodeType::Goto, Vec2::new(700.0, 100.0));
    n_goto.inputs = goto_def.inputs.iter().map(port).collect();
    n_goto.outputs = goto_def.outputs.iter().map(port).collect();
    n_goto.set_param("label", ParamValue::Literal(serde_json::json!("done")));

    let mut n_log = Node::new(NodeType::Log, Vec2::new(900.0, 100.0));
    n_log.inputs = log_def.inputs.iter().map(port).collect();
    n_log.outputs = log_def.outputs.iter().map(port).collect();
    n_log.set_param("output", ParamValue::Literal(serde_json::json!("流程结束")));

    g.add_node(n_start);
    g.add_node(n_if);
    g.add_node(n_label);
    g.add_node(n_goto);
    g.add_node(n_log);

    // 按类型查找节点 ID
    fn id_by_type(g: &Graph, node_type: NodeType) -> Option<String> {
        g.nodes
            .iter()
            .find(|(_, n)| n.node_type == node_type)
            .map(|(id, _)| id.clone())
    }
    let Some(id_start) = id_by_type(&g, NodeType::Start) else {
        println!("❌ Start 节点不存在");
        return;
    };
    let Some(id_if) = id_by_type(&g, NodeType::If) else {
        println!("❌ If 节点不存在");
        return;
    };
    let Some(id_label) = id_by_type(&g, NodeType::Label) else {
        println!("❌ Label 节点不存在");
        return;
    };
    let Some(id_goto) = id_by_type(&g, NodeType::Goto) else {
        println!("❌ Goto 节点不存在");
        return;
    };
    let Some(id_log) = id_by_type(&g, NodeType::Log) else {
        println!("❌ Log 节点不存在");
        return;
    };

    // 连线（执行流）
    if let Err(e) = g.add_edge(Edge::new(
        EdgeEndpoint::new(&id_start, "out_flow"),
        EdgeEndpoint::new(&id_if, "in_flow"),
        PortType::Flow,
    )) {
        println!("❌ 添加边失败: {e}");
        return;
    }
    if let Err(e) = g.add_edge(Edge::new(
        EdgeEndpoint::new(&id_if, "out_true"),
        EdgeEndpoint::new(&id_label, "in_flow"),
        PortType::Flow,
    )) {
        println!("❌ 添加边失败: {e}");
        return;
    }
    if let Err(e) = g.add_edge(Edge::new(
        EdgeEndpoint::new(&id_label, "out_flow"),
        EdgeEndpoint::new(&id_goto, "in_flow"),
        PortType::Flow,
    )) {
        println!("❌ 添加边失败: {e}");
        return;
    }
    if let Err(e) = g.add_edge(Edge::new(
        EdgeEndpoint::new(&id_goto, "out_flow"),
        EdgeEndpoint::new(&id_log, "in_flow"),
        PortType::Flow,
    )) {
        println!("❌ 添加边失败: {e}");
        return;
    }

    // 添加标签映射
    g.add_label("main", vec![id_start, id_if, id_label, id_goto, id_log]);

    // ── 验证 ──
    match GraphValidator::validate(&g) {
        Ok(()) => println!(
            "✅ 图验证通过！共 {} 个节点、{} 条边",
            g.nodes.len(),
            g.edges.len()
        ),
        Err(e) => println!("❌ 图验证失败: {e}"),
    }

    for (id, node) in &g.nodes {
        let Some(def) = defs.iter().find(|d| d.node_type == node.node_type) else {
            println!("  - {}: 未找到定义", id);
            continue;
        };
        println!(
            "  - {}: {} (类型: {:?}, 分类: {})",
            id, def.display_name, node.node_type, def.category
        );
    }
}
