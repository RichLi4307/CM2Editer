use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::migration::{CURRENT_VERSION, migrate_to_latest};
use crate::error::{FlowError, Result};
use crate::graph::types::{NodeType, PortType};
use crate::graph::{Edge, EdgeEndpoint, Graph, Node, ParamValue, Port, Vec2};

/// 图文档，包含 `Graph` 以及 JSON 中的视图层/元数据信息
#[derive(Debug, Clone)]
pub struct GraphDocument {
    /// 图数据层
    pub graph: Graph,
    /// 任务元数据，不参与代码生成，直接透传
    pub meta: Value,
    /// 线程定义
    pub threads: Vec<Thread>,
    /// 注释节点
    pub comments: Vec<Comment>,
    /// 画布视口状态
    pub viewport: Viewport,
}

/// 线程定义，描述并发入口
#[derive(Debug, Clone)]
pub struct Thread {
    pub id: String,
    pub name: String,
    pub entry_label: String,
    pub parent: Option<String>,
    pub auto_start: bool,
}

/// 注释节点，不参与代码生成
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    #[serde(default)]
    pub text: String,
    pub position: Vec2,
    #[serde(default = "default_size")]
    pub size: Size,
}

/// 画布视口状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Viewport {
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
    #[serde(default = "default_zoom")]
    pub zoom: f32,
    #[serde(default = "default_grid_size")]
    pub grid_size: f32,
    #[serde(default = "default_show_grid")]
    pub show_grid: bool,
}

impl GraphDocument {
    /// 使用完整字段创建图文档
    pub fn from_graph(
        graph: Graph,
        meta: Value,
        viewport: Viewport,
        threads: Vec<Thread>,
        comments: Vec<Comment>,
    ) -> Self {
        Self {
            graph,
            meta,
            threads,
            comments,
            viewport,
        }
    }

    /// 消费当前文档，返回内部 `Graph`
    pub fn into_graph(self) -> Graph {
        self.graph
    }

    /// 序列化为紧凑 JSON 字符串
    pub fn to_json(&self) -> Result<String> {
        let serde_doc = GraphDocumentSerde::from(self);
        serde_json::to_string(&serde_doc).map_err(FlowError::from)
    }

    /// 序列化为格式化 JSON 字符串
    pub fn to_json_pretty(&self) -> Result<String> {
        let serde_doc = GraphDocumentSerde::from(self);
        serde_json::to_string_pretty(&serde_doc).map_err(FlowError::from)
    }

    /// 从 JSON 字符串反序列化，并自动执行版本迁移
    pub fn from_json(json: &str) -> Result<Self> {
        let value: Value = serde_json::from_str(json).map_err(FlowError::from)?;
        let migrated = migrate_to_latest(value)?;
        let serde_doc: GraphDocumentSerde =
            serde_json::from_value(migrated).map_err(FlowError::from)?;
        Ok(Self::from(serde_doc))
    }
}

/// 将 `Graph` 序列化为 JSON 字符串
///
/// `meta` 为 `None` 时写入空对象；线程/视口/注释使用默认值。
pub fn serialize_graph(graph: &Graph, meta: Option<Value>) -> Result<String> {
    let serde_doc = GraphDocumentSerde {
        version: CURRENT_VERSION.to_string(),
        meta: meta.unwrap_or_else(default_meta),
        nodes: graph.nodes.values().cloned().map(NodeData::from).collect(),
        edges: graph.edges.values().cloned().map(EdgeData::from).collect(),
        labels: graph.labels.clone(),
        threads: default_threads(),
        comments: Vec::new(),
        viewport: default_viewport(),
    };
    serde_json::to_string(&serde_doc).map_err(FlowError::from)
}

/// 从 JSON 字符串反序列化，返回包含元数据的图文档
pub fn deserialize_graph(json: &str) -> Result<GraphDocument> {
    GraphDocument::from_json(json)
}

// ── 内部 serde 表示 ──

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GraphDocumentSerde {
    version: String,
    #[serde(default = "default_meta")]
    meta: Value,
    nodes: Vec<NodeData>,
    #[serde(default)]
    edges: Vec<EdgeData>,
    #[serde(default)]
    labels: HashMap<String, Vec<String>>,
    #[serde(default = "default_threads")]
    threads: Vec<ThreadSerde>,
    #[serde(default)]
    comments: Vec<Comment>,
    #[serde(default = "default_viewport")]
    viewport: Viewport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NodeData {
    id: String,
    #[serde(rename = "type")]
    node_type: NodeType,
    #[serde(default)]
    category: String,
    position: Vec2,
    #[serde(default = "default_size")]
    size: Size,
    #[serde(default)]
    collapsed: bool,
    #[serde(default)]
    params: HashMap<String, ParamValue>,
    #[serde(default)]
    ports: Ports,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct Ports {
    #[serde(default)]
    inputs: Vec<Port>,
    #[serde(default)]
    outputs: Vec<Port>,
}

/// 矩形尺寸，用于节点和注释框的 JSON 表示
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Size {
    #[serde(default = "default_width")]
    pub width: f32,
    #[serde(default = "default_height")]
    pub height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EdgeData {
    id: String,
    from: EdgeEndpointData,
    to: EdgeEndpointData,
    #[serde(rename = "type")]
    edge_type: PortType,
    #[serde(default)]
    waypoints: Vec<Vec2>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EdgeEndpointData {
    node: String,
    port: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ThreadSerde {
    id: String,
    #[serde(default)]
    name: String,
    entry_label: String,
    #[serde(default)]
    parent: Option<String>,
    #[serde(default = "default_auto_start")]
    auto_start: bool,
}

// ── 转换实现 ──

impl From<&GraphDocument> for GraphDocumentSerde {
    fn from(doc: &GraphDocument) -> Self {
        Self {
            version: CURRENT_VERSION.to_string(),
            meta: doc.meta.clone(),
            nodes: doc
                .graph
                .nodes
                .values()
                .cloned()
                .map(NodeData::from)
                .collect(),
            edges: doc
                .graph
                .edges
                .values()
                .cloned()
                .map(EdgeData::from)
                .collect(),
            labels: doc.graph.labels.clone(),
            threads: doc.threads.iter().cloned().map(ThreadSerde::from).collect(),
            comments: doc.comments.clone(),
            viewport: doc.viewport.clone(),
        }
    }
}

impl From<GraphDocumentSerde> for GraphDocument {
    fn from(doc: GraphDocumentSerde) -> Self {
        let mut graph = Graph::default();
        for node_data in doc.nodes {
            graph.add_node(Node::from(node_data));
        }
        for edge_data in doc.edges {
            graph
                .edges
                .insert(edge_data.id.clone(), Edge::from(edge_data));
        }
        for (label, ids) in doc.labels {
            graph.add_label(&label, ids);
        }
        Self {
            graph,
            meta: doc.meta,
            threads: doc.threads.into_iter().map(Thread::from).collect(),
            comments: doc.comments,
            viewport: doc.viewport,
        }
    }
}

impl From<Node> for NodeData {
    fn from(node: Node) -> Self {
        Self {
            id: node.id,
            node_type: node.node_type,
            category: node.category,
            position: node.position,
            size: Size {
                width: node.size.x,
                height: node.size.y,
            },
            collapsed: node.collapsed,
            params: node.params,
            ports: Ports {
                inputs: node.inputs,
                outputs: node.outputs,
            },
        }
    }
}

impl From<NodeData> for Node {
    fn from(data: NodeData) -> Self {
        Self {
            id: data.id,
            node_type: data.node_type,
            position: data.position,
            size: Vec2::new(data.size.width, data.size.height),
            collapsed: data.collapsed,
            params: data.params,
            inputs: data.ports.inputs,
            outputs: data.ports.outputs,
            category: data.category,
        }
    }
}

impl From<Edge> for EdgeData {
    fn from(edge: Edge) -> Self {
        Self {
            id: edge.id,
            from: EdgeEndpointData {
                node: edge.from.node_id,
                port: edge.from.port_id,
            },
            to: EdgeEndpointData {
                node: edge.to.node_id,
                port: edge.to.port_id,
            },
            edge_type: edge.edge_type,
            waypoints: edge.waypoints,
        }
    }
}

impl From<EdgeData> for Edge {
    fn from(data: EdgeData) -> Self {
        Self {
            id: data.id,
            from: EdgeEndpoint::new(&data.from.node, &data.from.port),
            to: EdgeEndpoint::new(&data.to.node, &data.to.port),
            edge_type: data.edge_type,
            waypoints: data.waypoints,
        }
    }
}

impl From<Thread> for ThreadSerde {
    fn from(t: Thread) -> Self {
        Self {
            id: t.id,
            name: t.name,
            entry_label: t.entry_label,
            parent: t.parent,
            auto_start: t.auto_start,
        }
    }
}

impl From<ThreadSerde> for Thread {
    fn from(t: ThreadSerde) -> Self {
        Self {
            id: t.id,
            name: t.name,
            entry_label: t.entry_label,
            parent: t.parent,
            auto_start: t.auto_start,
        }
    }
}

// ── 默认值函数 ──

fn default_meta() -> Value {
    Value::Object(serde_json::Map::new())
}

fn default_threads() -> Vec<ThreadSerde> {
    vec![ThreadSerde {
        id: "thread_main".to_string(),
        name: "main".to_string(),
        entry_label: "main".to_string(),
        parent: None,
        auto_start: true,
    }]
}

fn default_viewport() -> Viewport {
    Viewport {
        x: 0.0,
        y: 0.0,
        zoom: 1.0,
        grid_size: 20.0,
        show_grid: true,
    }
}

fn default_size() -> Size {
    Size {
        width: 180.0,
        height: 120.0,
    }
}

fn default_width() -> f32 {
    180.0
}

fn default_height() -> f32 {
    120.0
}

fn default_zoom() -> f32 {
    1.0
}

fn default_grid_size() -> f32 {
    20.0
}

fn default_show_grid() -> bool {
    true
}

fn default_auto_start() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;

    use super::*;
    use crate::graph::types::{NodeType, PortType};

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

    #[test]
    fn test_simple_roundtrip() -> Result<()> {
        let mut graph = Graph::default();
        let n1 = make_node("node_1", NodeType::Start, 100.0, 100.0);
        let n2 = make_node("node_2", NodeType::Log, 300.0, 100.0);
        graph.add_node(n1);
        graph.add_node(n2);
        let edge = Edge::new(
            EdgeEndpoint::new("node_1", "out_flow"),
            EdgeEndpoint::new("node_2", "in_flow"),
            PortType::Flow,
        );
        graph.add_edge(edge)?;

        let json = serialize_graph(&graph, None)?;
        let doc = deserialize_graph(&json)?;

        assert_eq!(doc.graph.nodes.len(), 2);
        assert_eq!(doc.graph.edges.len(), 1);
        assert!(doc.graph.nodes.contains_key("node_1"));
        assert!(doc.graph.nodes.contains_key("node_2"));
        Ok(())
    }

    #[test]
    fn test_meta_passthrough() -> Result<()> {
        let mut graph = Graph::default();
        graph.add_node(make_node("node_1", NodeType::Start, 0.0, 0.0));
        let meta = json!({ "title": { "En": "Test" } });
        let json = serialize_graph(&graph, Some(meta.clone()))?;
        let doc = deserialize_graph(&json)?;
        assert_eq!(doc.meta, meta);
        Ok(())
    }

    #[test]
    fn test_param_values_roundtrip() -> Result<()> {
        let mut graph = Graph::default();
        let mut n1 = make_node("node_1", NodeType::Log, 0.0, 0.0);
        n1.set_param("output", ParamValue::Literal(json!("hello")));
        n1.set_param("ref", ParamValue::from_ref("node_2", "out_value"));
        graph.add_node(n1);

        let json = serialize_graph(&graph, None)?;
        let doc = deserialize_graph(&json)?;
        let node = doc
            .graph
            .nodes
            .get("node_1")
            .ok_or_else(|| FlowError::Validation("node_1 should exist".to_string()))?;
        assert_eq!(
            node.params.get("output"),
            Some(&ParamValue::Literal(json!("hello")))
        );
        assert_eq!(
            node.params.get("ref"),
            Some(&ParamValue::from_ref("node_2", "out_value"))
        );
        Ok(())
    }

    #[test]
    fn test_graph_document_preserves_viewport_and_threads() -> Result<()> {
        let mut graph = Graph::default();
        graph.add_node(make_node("node_1", NodeType::Start, 0.0, 0.0));
        let viewport = Viewport {
            x: 10.0,
            y: 20.0,
            zoom: 2.0,
            grid_size: 10.0,
            show_grid: false,
        };
        let threads = vec![Thread {
            id: "thread_1".to_string(),
            name: "worker".to_string(),
            entry_label: "worker".to_string(),
            parent: None,
            auto_start: false,
        }];
        let doc =
            GraphDocument::from_graph(graph, default_meta(), viewport.clone(), threads, Vec::new());
        let json = doc.to_json()?;
        let doc2 = GraphDocument::from_json(&json)?;
        assert_eq!(doc2.viewport, viewport);
        assert_eq!(doc2.threads.len(), 1);
        assert_eq!(doc2.threads[0].id, "thread_1");
        Ok(())
    }

    #[test]
    fn test_loads_legacy_v1_0_format() -> Result<()> {
        let json = r#"{
            "version": "1.0",
            "nodes": [
                {
                    "id": "node_1",
                    "type": "Start",
                    "position": { "x": 0, "y": 0 },
                    "ports": {
                        "outputs": [{ "id": "out_flow", "type": "Flow", "label": "开始" }]
                    }
                }
            ],
            "edges": []
        }"#;
        let doc = deserialize_graph(json)?;
        assert_eq!(doc.graph.nodes.len(), 1);
        assert_eq!(doc.threads.len(), 1);
        assert_eq!(doc.viewport.grid_size, 20.0);
        assert!(doc.viewport.show_grid);
        Ok(())
    }
}
