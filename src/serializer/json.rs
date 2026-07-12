use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::{FlowError, Result};
use crate::graph::container::{ContainerGraph, Viewport};

/// 图文档，包含容器化图以及视图层/元数据信息
#[derive(Debug, Clone)]
pub struct GraphDocument {
    /// 图数据层（新架构容器化图）
    pub graph: ContainerGraph,
    /// 任务元数据，不参与代码生成，直接透传
    pub meta: Value,
    /// 注释节点
    pub comments: Vec<Comment>,
    /// 画布视口状态
    pub viewport: Viewport,
}

/// 注释节点，不参与代码生成
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    #[serde(default)]
    pub text: String,
    pub position: crate::graph::Vec2,
    #[serde(default = "default_size")]
    pub size: Size,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Size {
    #[serde(default = "default_width")]
    pub width: f32,
    #[serde(default = "default_height")]
    pub height: f32,
}

impl GraphDocument {
    /// 使用完整字段创建图文档
    pub fn from_graph(
        graph: ContainerGraph,
        meta: Value,
        viewport: Viewport,
        comments: Vec<Comment>,
    ) -> Self {
        Self {
            graph,
            meta,
            comments,
            viewport,
        }
    }

    /// 消费当前文档，返回内部 `ContainerGraph`
    pub fn into_graph(self) -> ContainerGraph {
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

    /// 从 JSON 字符串反序列化
    ///
    /// 仅支持新架构 `version: "2.0"`。
    pub fn from_json(json: &str) -> Result<Self> {
        let serde_doc: GraphDocumentSerde =
            serde_json::from_str(json).map_err(FlowError::from)?;
        if serde_doc.version != "2.0" {
            return Err(FlowError::Validation(format!(
                "Unsupported graph version: {}. Only 2.0 is supported in this architecture refactor.",
                serde_doc.version
            )));
        }
        Ok(Self::from(serde_doc))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GraphDocumentSerde {
    version: String,
    #[serde(default = "default_meta")]
    meta: Value,
    #[serde(default)]
    graph: ContainerGraph,
    #[serde(default)]
    comments: Vec<Comment>,
    #[serde(default = "default_viewport")]
    viewport: Viewport,
}

impl From<&GraphDocument> for GraphDocumentSerde {
    fn from(doc: &GraphDocument) -> Self {
        Self {
            version: "2.0".to_string(),
            meta: doc.meta.clone(),
            graph: doc.graph.clone(),
            comments: doc.comments.clone(),
            viewport: doc.viewport,
        }
    }
}

impl From<GraphDocumentSerde> for GraphDocument {
    fn from(doc: GraphDocumentSerde) -> Self {
        Self {
            graph: doc.graph,
            meta: doc.meta,
            comments: doc.comments,
            viewport: doc.viewport,
        }
    }
}

fn default_meta() -> Value {
    Value::Object(serde_json::Map::new())
}

fn default_viewport() -> Viewport {
    Viewport::default()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::container::{LabelContainer, ThreadContainer};
    use crate::graph::node::{Node, ParamValue, Port, Vec2};
    use crate::graph::types::{NodeType, PortType};
    use std::collections::HashMap;

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

    fn make_simple_graph() -> ContainerGraph {
        let mut label = LabelContainer {
            id: "label_main".to_string(),
            name: "main".to_string(),
            params: vec![],
            nodes: HashMap::new(),
            edges: HashMap::new(),
            entry_pin: Vec2::default(),
            position: Vec2::default(),
        };
        label.nodes.insert("node_1".to_string(), make_node("node_1", NodeType::Log, 100.0, 100.0));
        label.nodes.insert("node_2".to_string(), make_node("node_2", NodeType::Log, 300.0, 100.0));

        let mut graph = ContainerGraph::default();
        graph.threads.push(ThreadContainer {
            id: "thread_main".to_string(),
            name: "main".to_string(),
            variable_name: "var_main_thread".to_string(),
            auto_start: true,
            labels: vec![label],
            listeners: vec![],
            position: Vec2::default(),
        });
        graph
    }

    #[test]
    fn test_simple_roundtrip() -> Result<()> {
        let graph = make_simple_graph();
        let doc = GraphDocument::from_graph(graph, default_meta(), Viewport::default(), Vec::new());
        let json = doc.to_json()?;
        let doc2 = GraphDocument::from_json(&json)?;
        assert_eq!(doc2.graph.threads.len(), 1);
        assert_eq!(doc2.graph.threads[0].labels.len(), 1);
        assert_eq!(doc2.graph.threads[0].labels[0].nodes.len(), 2);
        Ok(())
    }

    #[test]
    fn test_meta_passthrough() -> Result<()> {
        let graph = make_simple_graph();
        let meta = serde_json::json!({ "title": { "En": "Test" } });
        let doc = GraphDocument::from_graph(graph, meta.clone(), Viewport::default(), Vec::new());
        let json = doc.to_json()?;
        let doc2 = GraphDocument::from_json(&json)?;
        assert_eq!(doc2.meta, meta);
        Ok(())
    }

    #[test]
    fn test_param_values_roundtrip() -> Result<()> {
        let mut graph = make_simple_graph();
        let mut n1 = make_node("node_1", NodeType::Log, 0.0, 0.0);
        n1.set_param("output", ParamValue::Literal(serde_json::json!("hello")));
        n1.set_param("ref", ParamValue::from_ref("node_2", "out_value"));
        graph.threads[0].labels[0].nodes.insert("node_1".to_string(), n1);

        let doc = GraphDocument::from_graph(graph, default_meta(), Viewport::default(), Vec::new());
        let json = doc.to_json()?;
        let doc2 = GraphDocument::from_json(&json)?;
        let node = doc2.graph.threads[0].labels[0]
            .nodes
            .get("node_1")
            .ok_or_else(|| FlowError::Validation("node_1 should exist".to_string()))?;
        assert_eq!(
            node.params.get("output"),
            Some(&ParamValue::Literal(serde_json::json!("hello")))
        );
        assert_eq!(
            node.params.get("ref"),
            Some(&ParamValue::from_ref("node_2", "out_value"))
        );
        Ok(())
    }

    #[test]
    fn test_graph_document_preserves_viewport() -> Result<()> {
        let graph = make_simple_graph();
        let viewport = Viewport {
            x: 10.0,
            y: 20.0,
            zoom: 2.0,
            grid_size: 10.0,
            show_grid: false,
        };
        let doc = GraphDocument::from_graph(graph, default_meta(), viewport, Vec::new());
        let json = doc.to_json()?;
        let doc2 = GraphDocument::from_json(&json)?;
        assert_eq!(doc2.viewport.x, viewport.x);
        assert_eq!(doc2.viewport.y, viewport.y);
        assert_eq!(doc2.viewport.zoom, viewport.zoom);
        assert_eq!(doc2.viewport.grid_size, viewport.grid_size);
        assert_eq!(doc2.viewport.show_grid, viewport.show_grid);
        Ok(())
    }

    #[test]
    fn test_rejects_legacy_v1_0_format() {
        let json = r#"{
            "version": "1.0",
            "nodes": [
                {
                    "id": "node_1",
                    "type": "Start",
                    "position": { "x": 0, "y": 0 }
                }
            ],
            "edges": []
        }"#;
        let result = GraphDocument::from_json(json);
        assert!(result.is_err());
    }
}
