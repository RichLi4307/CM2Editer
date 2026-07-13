use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    edge::Edge,
    node::{Node, ParamValue, Vec2},
    types::PortType,
};

/// 标签容器
///
/// 一个 `LabelContainer` 对应 `.code` 中的一个标签体。它包含一组节点和边，
/// 这些节点通过 `Flow` 边表示同一标签内的顺序执行。
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LabelContainer {
    pub id: String,
    pub name: String,
    pub params: Vec<LabelParam>,
    pub nodes: HashMap<String, Node>,
    pub edges: HashMap<String, Edge>,
    pub entry_pin: Vec2,
    pub position: Vec2,
}

impl LabelContainer {
    /// 返回当前标签的入口节点 ID。
    ///
    /// 规则：
    /// 1. 候选节点必须带有 Flow 类型输出端口（如 `out_flow`）。
    /// 2. 优先选择没有 Flow 边连入 `in_flow` 的节点。
    /// 3. 若存在多个候选，选择位置最靠左上（x 最小，其次 y 最小）的节点，保证稳定。
    /// 4. 若无无入边节点，则回退到任意 Flow 输出节点（同样按左上位置）。
    pub fn entry_node_id(&self) -> Option<String> {
        let has_incoming_flow = |node_id: &str| {
            self.edges.values().any(|e| {
                e.edge_type == PortType::Flow
                    && e.to.node_id == node_id
                    && e.to.port_id == "in_flow"
            })
        };

        let mut candidates: Vec<&Node> = self
            .nodes
            .values()
            .filter(|n| n.outputs.iter().any(|p| p.port_type == PortType::Flow))
            .collect();

        let with_no_incoming: Vec<&Node> = candidates
            .iter()
            .copied()
            .filter(|n| !has_incoming_flow(&n.id))
            .collect();

        if !with_no_incoming.is_empty() {
            candidates = with_no_incoming;
        }

        candidates.sort_by(|a, b| {
            a.position
                .x
                .partial_cmp(&b.position.x)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| {
                    a.position
                        .y
                        .partial_cmp(&b.position.y)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
        });

        candidates.first().map(|n| n.id.clone())
    }
}

/// 标签参数签名
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LabelParam {
    pub name: String,
    pub param_type: String,
    pub default: Option<ParamValue>,
}

/// 监听器容器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenerContainer {
    pub inner: LabelContainer,
    pub kind: ListenerKind,
    pub variable_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ListenerKind {
    Listener,
    LocalListener,
}

impl Default for ListenerKind {
    fn default() -> Self {
        ListenerKind::Listener
    }
}

impl ListenerContainer {
    pub fn id(&self) -> &str {
        &self.inner.id
    }
    pub fn name(&self) -> &str {
        &self.inner.name
    }
    pub fn nodes(&self) -> &HashMap<String, Node> {
        &self.inner.nodes
    }
    pub fn nodes_mut(&mut self) -> &mut HashMap<String, Node> {
        &mut self.inner.nodes
    }
    pub fn edges(&self) -> &HashMap<String, Edge> {
        &self.inner.edges
    }
    pub fn edges_mut(&mut self) -> &mut HashMap<String, Edge> {
        &mut self.inner.edges
    }
}

/// 线程容器
///
/// 一个 `ThreadContainer` 对应 `.code` 中的一个并发线程。它包含若干标签和监听器，
/// 这些标签共享同一个 `_this` 线程引用。
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ThreadContainer {
    pub id: String,
    pub name: String,
    pub variable_name: String,
    #[serde(default)]
    pub auto_start: bool,
    pub labels: Vec<LabelContainer>,
    pub listeners: Vec<ListenerContainer>,
    pub position: Vec2,
}

impl ThreadContainer {
    /// 查找指定名称的标签容器
    pub fn label(&self, name: &str) -> Option<&LabelContainer> {
        self.labels.iter().find(|l| l.name == name)
    }

    /// 查找指定名称的监听器容器
    pub fn listener(&self, name: &str) -> Option<&ListenerContainer> {
        self.listeners.iter().find(|l| l.name() == name)
    }

    /// 返回所有标签名
    pub fn label_names(&self) -> Vec<String> {
        self.labels.iter().map(|l| l.name.clone()).collect()
    }

    /// 返回所有监听器名
    pub fn listener_names(&self) -> Vec<String> {
        self.listeners.iter().map(|l| l.name().to_string()).collect()
    }
}

/// 容器化图
///
/// 新架构的核心数据结构。由若干 `ThreadContainer` 组成，每个线程容器包含自己的标签和监听器。
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ContainerGraph {
    pub threads: Vec<ThreadContainer>,
    pub viewport: Viewport,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
    #[serde(default = "default_grid_size")]
    pub grid_size: f32,
    #[serde(default = "default_show_grid")]
    pub show_grid: bool,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
            grid_size: 20.0,
            show_grid: true,
        }
    }
}

fn default_grid_size() -> f32 {
    20.0
}

fn default_show_grid() -> bool {
    true
}

impl ContainerGraph {
    /// 创建一个默认图，包含一个 main 线程和 main 标签
    pub fn default_main() -> Self {
        let mut graph = Self::default();
        graph.threads.push(ThreadContainer {
            id: "thread_main".to_string(),
            name: "main".to_string(),
            variable_name: "var_main_thread".to_string(),
            auto_start: true,
            labels: vec![LabelContainer {
                id: "label_main".to_string(),
                name: "main".to_string(),
                params: vec![],
                nodes: HashMap::new(),
                edges: HashMap::new(),
                entry_pin: Vec2::default(),
                position: Vec2::default(),
            }],
            listeners: vec![],
            position: Vec2::default(),
        });
        graph
    }

    /// 查找包含指定节点的线程和标签
    pub fn find_node_location(
        &self,
        node_id: &str,
    ) -> Option<(&ThreadContainer, ContainerLocation<'_>)> {
        for thread in &self.threads {
            for label in &thread.labels {
                if label.nodes.contains_key(node_id) {
                    return Some((thread, ContainerLocation::Label(label)));
                }
            }
            for listener in &thread.listeners {
                if listener.inner.nodes.contains_key(node_id) {
                    return Some((thread, ContainerLocation::Listener(listener)));
                }
            }
        }
        None
    }

    /// 返回所有标签名（跨线程全局）
    pub fn all_label_names(&self) -> Vec<String> {
        self.threads
            .iter()
            .flat_map(|t| t.label_names())
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ContainerLocation<'a> {
    Label(&'a LabelContainer),
    Listener(&'a ListenerContainer),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_main() {
        let g = ContainerGraph::default_main();
        assert_eq!(g.threads.len(), 1);
        assert_eq!(g.threads[0].name, "main");
        assert_eq!(g.threads[0].labels.len(), 1);
        assert_eq!(g.threads[0].labels[0].name, "main");
    }
}
