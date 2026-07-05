use std::collections::HashMap;

use super::{edge::Edge, node::Node};
use crate::error::{FlowError, Result};

/// 图容器，持有节点、边以及标签映射
///
/// 这是编辑器后端的核心数据结构，所有不依赖 UI 的操作都应先在此层验证
#[derive(Debug, Default)]
pub struct Graph {
    /// 所有节点，以 ID 为键
    pub nodes: HashMap<String, Node>,
    /// 所有边，以 ID 为键
    pub edges: HashMap<String, Edge>,
    /// 标签名到节点 ID 列表的映射
    pub labels: HashMap<String, Vec<String>>,
}

impl Graph {
    /// 向图中添加一个节点
    ///
    /// 如果已存在相同 ID 的节点，将覆盖旧节点
    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    /// 从图中删除一个节点，并级联删除与之相连的所有边
    ///
    /// 同时从 `labels` 中移除该节点 ID
    pub fn remove_node(&mut self, node_id: &str) -> Result<()> {
        if !self.nodes.contains_key(node_id) {
            return Err(FlowError::NodeNotFound(node_id.to_string()));
        }

        let edges_to_remove: Vec<String> = self
            .edges
            .values()
            .filter(|e| e.from.node_id == node_id || e.to.node_id == node_id)
            .map(|e| e.id.clone())
            .collect();

        for edge_id in edges_to_remove {
            self.edges.remove(&edge_id);
        }
        self.nodes.remove(node_id);

        // 从 labels 中移除该节点 ID
        for node_ids in self.labels.values_mut() {
            node_ids.retain(|id| id != node_id);
        }

        Ok(())
    }

    /// 添加一条边，并验证端点节点与端口存在性
    pub fn add_edge(&mut self, edge: Edge) -> Result<()> {
        let from_node = self
            .nodes
            .get(&edge.from.node_id)
            .ok_or_else(|| FlowError::NodeNotFound(edge.from.node_id.clone()))?;
        let to_node = self
            .nodes
            .get(&edge.to.node_id)
            .ok_or_else(|| FlowError::NodeNotFound(edge.to.node_id.clone()))?;

        if from_node.get_port(&edge.from.port_id, false).is_none() {
            return Err(FlowError::ConnectionError(format!(
                "Output port {} not found on node {}",
                edge.from.port_id, edge.from.node_id
            )));
        }
        if to_node.get_port(&edge.to.port_id, true).is_none() {
            return Err(FlowError::ConnectionError(format!(
                "Input port {} not found on node {}",
                edge.to.port_id, edge.to.node_id
            )));
        }

        self.edges.insert(edge.id.clone(), edge);
        Ok(())
    }

    /// 删除一条边
    pub fn remove_edge(&mut self, edge_id: &str) -> Result<()> {
        if self.edges.remove(edge_id).is_none() {
            return Err(FlowError::ConnectionError(format!(
                "Edge not found: {}",
                edge_id
            )));
        }
        Ok(())
    }

    /// 获取指向指定节点的所有边
    pub fn incoming_edges(&self, node_id: &str) -> Vec<&Edge> {
        self.edges
            .values()
            .filter(|e| e.to.node_id == node_id)
            .collect()
    }

    /// 获取从指定节点出发的所有边
    pub fn outgoing_edges(&self, node_id: &str) -> Vec<&Edge> {
        self.edges
            .values()
            .filter(|e| e.from.node_id == node_id)
            .collect()
    }

    /// 添加或更新一个标签
    pub fn add_label(&mut self, name: &str, node_ids: Vec<String>) {
        self.labels.insert(name.to_string(), node_ids);
    }
}

#[cfg(test)]
mod tests {
    use super::super::edge::EdgeEndpoint;
    use super::super::node::{Port, Vec2};
    use super::super::types::{NodeType, PortType};
    use super::*;

    fn make_node(node_type: NodeType, id: &str) -> Node {
        Node {
            id: id.to_string(),
            node_type,
            position: Vec2::ZERO,
            size: Vec2::new(180.0, 120.0),
            collapsed: false,
            params: HashMap::new(),
            inputs: vec![Port::new("in_flow", PortType::Flow, "执行")],
            outputs: vec![Port::new("out_flow", PortType::Flow, "下一步")],
            category: String::new(),
        }
    }

    #[test]
    fn test_add_node() {
        let mut graph = Graph::default();
        let node = make_node(NodeType::Log, "node_1");
        graph.add_node(node);
        assert_eq!(graph.nodes.len(), 1);
        assert!(graph.nodes.contains_key("node_1"));
    }

    #[test]
    fn test_remove_node_cascades_edges() {
        let mut graph = Graph::default();
        let n1 = make_node(NodeType::Log, "node_1");
        let n2 = make_node(NodeType::Log, "node_2");
        graph.add_node(n1);
        graph.add_node(n2);

        let edge = Edge::new(
            EdgeEndpoint::new("node_1", "out_flow"),
            EdgeEndpoint::new("node_2", "in_flow"),
            PortType::Flow,
        );
        graph.add_edge(edge).unwrap();
        assert_eq!(graph.edges.len(), 1);

        graph.remove_node("node_1").unwrap();
        assert!(!graph.nodes.contains_key("node_1"));
        assert!(graph.edges.is_empty());
    }

    #[test]
    fn test_remove_node_updates_labels() {
        let mut graph = Graph::default();
        let n1 = make_node(NodeType::Start, "node_1");
        let n2 = make_node(NodeType::Log, "node_2");
        graph.add_node(n1);
        graph.add_node(n2);
        graph.add_label("main", vec!["node_1".to_string(), "node_2".to_string()]);

        graph.remove_node("node_1").unwrap();
        assert_eq!(graph.labels["main"], vec!["node_2".to_string()]);
    }

    #[test]
    fn test_remove_node_not_found() {
        let mut graph = Graph::default();
        let result = graph.remove_node("missing");
        assert!(matches!(result, Err(FlowError::NodeNotFound(_))));
    }

    #[test]
    fn test_add_edge_checks_nodes() {
        let mut graph = Graph::default();
        let n1 = make_node(NodeType::Log, "node_1");
        graph.add_node(n1);

        let edge = Edge::new(
            EdgeEndpoint::new("node_1", "out_flow"),
            EdgeEndpoint::new("node_2", "in_flow"),
            PortType::Flow,
        );
        let result = graph.add_edge(edge);
        assert!(matches!(result, Err(FlowError::NodeNotFound(_))));
    }

    #[test]
    fn test_add_edge_checks_ports() {
        let mut graph = Graph::default();
        let n1 = make_node(NodeType::Log, "node_1");
        let n2 = make_node(NodeType::Log, "node_2");
        graph.add_node(n1);
        graph.add_node(n2);

        let edge = Edge::new(
            EdgeEndpoint::new("node_1", "out_missing"),
            EdgeEndpoint::new("node_2", "in_flow"),
            PortType::Flow,
        );
        let result = graph.add_edge(edge);
        assert!(matches!(result, Err(FlowError::ConnectionError(_))));
    }

    #[test]
    fn test_incoming_and_outgoing_edges() {
        let mut graph = Graph::default();
        let n1 = make_node(NodeType::Log, "node_1");
        let n2 = make_node(NodeType::Log, "node_2");
        graph.add_node(n1);
        graph.add_node(n2);

        let edge = Edge::new(
            EdgeEndpoint::new("node_1", "out_flow"),
            EdgeEndpoint::new("node_2", "in_flow"),
            PortType::Flow,
        );
        graph.add_edge(edge).unwrap();

        assert_eq!(graph.outgoing_edges("node_1").len(), 1);
        assert_eq!(graph.incoming_edges("node_2").len(), 1);
        assert!(graph.outgoing_edges("node_2").is_empty());
        assert!(graph.incoming_edges("node_1").is_empty());
    }

    #[test]
    fn test_add_edge_overwrites_duplicate() {
        let mut graph = Graph::default();
        let n1 = make_node(NodeType::Log, "node_1");
        let n2 = make_node(NodeType::Log, "node_2");
        graph.add_node(n1);
        graph.add_node(n2);

        let edge1 = Edge::new(
            EdgeEndpoint::new("node_1", "out_flow"),
            EdgeEndpoint::new("node_2", "in_flow"),
            PortType::Flow,
        );
        let edge2 = Edge::new(
            EdgeEndpoint::new("node_1", "out_flow"),
            EdgeEndpoint::new("node_2", "in_flow"),
            PortType::Flow,
        );
        graph.add_edge(edge1).unwrap();
        graph.add_edge(edge2).unwrap();
        assert_eq!(graph.edges.len(), 1);
    }
}
