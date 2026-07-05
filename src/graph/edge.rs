use serde::{Deserialize, Serialize};

use super::node::Vec2;
use super::types::PortType;

/// 连线端点，标识一个节点上的特定端口
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EdgeEndpoint {
    /// 节点 ID
    pub node_id: String,
    /// 端口 ID
    pub port_id: String,
}

impl EdgeEndpoint {
    /// 创建一个新的端点
    pub fn new(node_id: &str, port_id: &str) -> Self {
        Self {
            node_id: node_id.to_string(),
            port_id: port_id.to_string(),
        }
    }
}

/// 连接两个端点的边
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    /// 边的唯一 ID
    pub id: String,
    /// 源端点
    pub from: EdgeEndpoint,
    /// 目标端点
    pub to: EdgeEndpoint,
    /// 边的类型（Flow 或 Data）
    #[serde(rename = "type")]
    pub edge_type: PortType,
    /// 连线中间点，用于调整 UI 显示路径
    pub waypoints: Vec<Vec2>,
}

impl Edge {
    /// 创建一条连接两个端点的新边
    ///
    /// ID 由端点信息自动生成，便于从连接关系快速查找
    pub fn new(from: EdgeEndpoint, to: EdgeEndpoint, edge_type: PortType) -> Self {
        let id = format!(
            "edge_{}_{}_{}_{}",
            from.node_id, from.port_id, to.node_id, to.port_id
        );
        Self {
            id,
            from,
            to,
            edge_type,
            waypoints: Vec::new(),
        }
    }

    /// 添加一个中间路径点
    pub fn add_waypoint(mut self, x: f32, y: f32) -> Self {
        self.waypoints.push(Vec2::new(x, y));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_new() {
        let from = EdgeEndpoint::new("node_1", "out_flow");
        let to = EdgeEndpoint::new("node_2", "in_flow");
        let edge = Edge::new(from, to, PortType::Flow);
        assert_eq!(edge.id, "edge_node_1_out_flow_node_2_in_flow");
        assert_eq!(edge.edge_type, PortType::Flow);
        assert!(edge.waypoints.is_empty());
    }

    #[test]
    fn test_edge_with_waypoints() {
        let edge = Edge::new(
            EdgeEndpoint::new("a", "out"),
            EdgeEndpoint::new("b", "in"),
            PortType::Number,
        )
        .add_waypoint(100.0, 50.0)
        .add_waypoint(150.0, 50.0);
        assert_eq!(edge.waypoints.len(), 2);
        assert_eq!(edge.waypoints[0], Vec2::new(100.0, 50.0));
    }

    #[test]
    fn test_edge_endpoint_equality() {
        let a = EdgeEndpoint::new("node_1", "port_1");
        let b = EdgeEndpoint::new("node_1", "port_1");
        let c = EdgeEndpoint::new("node_1", "port_2");
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
