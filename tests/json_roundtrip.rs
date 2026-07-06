use std::collections::HashMap;

use CM2Editer::error::Result;
use CM2Editer::graph::edge::{Edge, EdgeEndpoint};
use CM2Editer::graph::graph::Graph;
use CM2Editer::graph::node::{Node, ParamValue, Port, Vec2};
use CM2Editer::graph::types::{NodeType, PortType};
use CM2Editer::serializer::{deserialize_graph, serialize_graph};
use serde_json::json;

fn make_node(id: &str, node_type: NodeType, x: f32, y: f32) -> Node {
    Node {
        id: id.to_string(),
        node_type,
        position: Vec2::new(x, y),
        size: Vec2::new(180.0, 120.0),
        collapsed: false,
        params: HashMap::new(),
        inputs: vec![Port::new("in_flow", PortType::Flow, "执行")],
        outputs: vec![Port::new("out_flow", PortType::Flow, "下一步")],
        category: "Control".to_string(),
    }
}

fn make_data_node(id: &str, x: f32, y: f32) -> Node {
    Node {
        id: id.to_string(),
        node_type: NodeType::Random,
        position: Vec2::new(x, y),
        size: Vec2::new(180.0, 120.0),
        collapsed: false,
        params: HashMap::new(),
        inputs: vec![Port::new("in_flow", PortType::Flow, "执行")],
        outputs: vec![
            Port::new("out_flow", PortType::Flow, "下一步"),
            Port::new("out_value", PortType::Number, "值"),
        ],
        category: "Math".to_string(),
    }
}

#[test]
fn graph_roundtrip_preserves_nodes_and_edges() -> Result<()> {
    let mut graph = Graph::default();
    let mut start = make_node("node_start", NodeType::Start, 100.0, 100.0);
    start.params.insert(
        "mission_name".to_string(),
        ParamValue::Literal(json!("roundtrip test")),
    );
    let log = make_node("node_log", NodeType::Log, 300.0, 100.0);
    let random = make_data_node("node_random", 300.0, 300.0);

    graph.add_node(start);
    graph.add_node(log);
    graph.add_node(random);
    graph.add_label(
        "main",
        vec!["node_start".to_string(), "node_log".to_string()],
    );

    let flow_edge = Edge::new(
        EdgeEndpoint::new("node_start", "out_flow"),
        EdgeEndpoint::new("node_log", "in_flow"),
        PortType::Flow,
    );
    let data_edge = Edge::new(
        EdgeEndpoint::new("node_random", "out_value"),
        EdgeEndpoint::new("node_log", "in_value"),
        PortType::Number,
    );
    graph.add_edge(flow_edge)?;
    // 直接插入 data_edge，避免 log 节点缺少 in_value 端口导致 add_edge 失败
    graph.edges.insert(data_edge.id.clone(), data_edge);

    let json = serialize_graph(&graph, None)?;
    let doc = deserialize_graph(&json)?;
    let restored = doc.into_graph();

    assert_eq!(restored.nodes.len(), 3);
    assert_eq!(restored.edges.len(), 2);
    assert!(restored.nodes.contains_key("node_start"));
    assert!(restored.nodes.contains_key("node_log"));
    assert!(restored.nodes.contains_key("node_random"));
    assert_eq!(restored.labels.get("main").map(Vec::len), Some(2));

    let start_node = restored
        .nodes
        .get("node_start")
        .ok_or_else(|| CM2Editer::error::FlowError::Validation("node_start missing".to_string()))?;
    assert_eq!(
        start_node.params.get("mission_name"),
        Some(&ParamValue::Literal(json!("roundtrip test")))
    );

    Ok(())
}

#[test]
fn graph_roundtrip_preserves_meta() -> Result<()> {
    let mut graph = Graph::default();
    graph.add_node(make_node("node_1", NodeType::Start, 0.0, 0.0));
    let meta = json!({
        "title": { "En": "Integration Test", "Ja": "統合テスト" },
        "defaultactive": true
    });

    let json = serialize_graph(&graph, Some(meta.clone()))?;
    let doc = deserialize_graph(&json)?;

    assert_eq!(doc.meta, meta);
    Ok(())
}

#[test]
fn graph_roundtrip_preserves_param_refs() -> Result<()> {
    let mut graph = Graph::default();
    let mut n1 = make_node("node_1", NodeType::Log, 0.0, 0.0);
    n1.set_param("output", ParamValue::from_ref("node_2", "out_result"));
    graph.add_node(n1);
    graph.add_node(make_node("node_2", NodeType::Random, 100.0, 0.0));

    let json = serialize_graph(&graph, None)?;
    let doc = deserialize_graph(&json)?;
    let node = doc
        .graph
        .nodes
        .get("node_1")
        .ok_or_else(|| CM2Editer::error::FlowError::Validation("node_1 missing".to_string()))?;

    assert_eq!(
        node.params.get("output"),
        Some(&ParamValue::from_ref("node_2", "out_result"))
    );
    Ok(())
}

#[test]
fn graph_roundtrip_preserves_waypoints() -> Result<()> {
    let mut graph = Graph::default();
    let n1 = make_node("node_1", NodeType::Start, 0.0, 0.0);
    let n2 = make_node("node_2", NodeType::Log, 200.0, 0.0);
    graph.add_node(n1);
    graph.add_node(n2);

    let edge = Edge::new(
        EdgeEndpoint::new("node_1", "out_flow"),
        EdgeEndpoint::new("node_2", "in_flow"),
        PortType::Flow,
    )
    .add_waypoint(50.0, 50.0)
    .add_waypoint(100.0, 50.0);
    graph.add_edge(edge)?;

    let json = serialize_graph(&graph, None)?;
    let doc = deserialize_graph(&json)?;
    let restored_edge =
        doc.graph.edges.values().next().ok_or_else(|| {
            CM2Editer::error::FlowError::Validation("expected one edge".to_string())
        })?;

    assert_eq!(restored_edge.waypoints.len(), 2);
    assert_eq!(restored_edge.waypoints[0], Vec2::new(50.0, 50.0));
    assert_eq!(restored_edge.waypoints[1], Vec2::new(100.0, 50.0));
    Ok(())
}
