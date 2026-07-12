use std::collections::HashMap;

use super::{
    edge::Edge,
    node::{Node, ParamValue, Vec2},
    types::NodeType,
};

/// 标签容器
///
/// 一个 `LabelContainer` 对应 `.code` 中的一个标签体。它包含一组节点和边，
/// 这些节点通过 `Flow` 边表示同一标签内的顺序执行。
#[derive(Debug, Default, Clone)]
pub struct LabelContainer {
    pub id: String,
    pub name: String,
    pub params: Vec<LabelParam>,
    pub nodes: HashMap<String, Node>,
    pub edges: HashMap<String, Edge>,
    pub entry_pin: Vec2,
    pub position: Vec2,
}

/// 标签参数签名
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LabelParam {
    pub name: String,
    pub param_type: String,
    pub default: Option<ParamValue>,
}

/// 监听器容器
#[derive(Debug, Clone)]
pub struct ListenerContainer {
    pub inner: LabelContainer,
    pub kind: ListenerKind,
    pub variable_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Default, Clone)]
pub struct ThreadContainer {
    pub id: String,
    pub name: String,
    pub variable_name: String,
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
#[derive(Debug, Default, Clone)]
pub struct ContainerGraph {
    pub threads: Vec<ThreadContainer>,
    pub viewport: Viewport,
}

#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
        }
    }
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

    /// 从旧版 `Graph` 迁移到容器化图
    ///
    /// 迁移规则：
    /// - 每个 `graph.labels` 条目变成一个 `LabelContainer`。
    /// - 标签内的节点和边归入该容器。
    /// - `Start` 节点作为入口钉被移除。
    /// - `Label` 节点作为标签名被解析后移除。
    /// - 所有标签默认归入一个名为 `main` 的 `ThreadContainer`。
    pub fn from_legacy_graph(graph: &super::graph::Graph) -> Self {
        let mut containers = Vec::new();
        let mut main_thread = ThreadContainer {
            id: "thread_main".to_string(),
            name: "main".to_string(),
            variable_name: "var_main_thread".to_string(),
            auto_start: true,
            labels: Vec::new(),
            listeners: Vec::new(),
            position: Vec2::default(),
        };

        // 按标签名排序，保证确定性
        let mut label_names: Vec<&String> = graph.labels.keys().collect();
        label_names.sort();

        for label_name in label_names {
            let node_ids = graph.labels.get(label_name).cloned().unwrap_or_default();
            let node_id_set: std::collections::HashSet<String> = node_ids.iter().cloned().collect();

            // 提取节点
            let mut nodes = HashMap::new();
            let mut entry_pin = Vec2::default();
            for id in &node_ids {
                if let Some(node) = graph.nodes.get(id) {
                    if node.node_type == NodeType::Start {
                        entry_pin = node.position;
                    } else if node.node_type != NodeType::Label {
                        nodes.insert(id.clone(), node.clone());
                    }
                }
            }

            // 解析 Label 节点的名称参数（如果存在）
            let mut final_name = label_name.to_string();
            for id in &node_ids {
                if let Some(node) = graph.nodes.get(id) {
                    if node.node_type == NodeType::Label {
                        if let Some(ParamValue::Literal(v)) = node.params.get("name") {
                            if let Some(name) = v.as_str() {
                                final_name = name.to_string();
                            }
                        }
                    }
                }
            }

            // 提取容器内边
            let mut edges = HashMap::new();
            for edge in graph.edges.values() {
                let in_from = node_id_set.contains(&edge.from.node_id);
                let in_to = node_id_set.contains(&edge.to.node_id);
                if in_from && in_to {
                    edges.insert(edge.id.clone(), edge.clone());
                }
            }

            main_thread.labels.push(LabelContainer {
                id: format!("label_{}", final_name),
                name: final_name,
                params: vec![],
                nodes,
                edges,
                entry_pin,
                position: Vec2::default(),
            });
        }

        if main_thread.labels.is_empty() {
            // 兜底：没有任何标签时创建一个空 main 标签
            main_thread.labels.push(LabelContainer {
                id: "label_main".to_string(),
                name: "main".to_string(),
                params: vec![],
                nodes: HashMap::new(),
                edges: HashMap::new(),
                entry_pin: Vec2::default(),
                position: Vec2::default(),
            });
        }

        containers.push(main_thread);
        Self {
            threads: containers,
            viewport: Viewport::default(),
        }
    }

    /// 转换为旧版 `Graph`
    ///
    /// 用于兼容旧版 UI、序列化和验证器。新代码应优先使用容器化 API。
    pub fn to_legacy_graph(&self) -> super::graph::Graph {
        let mut graph = super::graph::Graph::default();
        let mut id_gen = IdGenerator::default();

        for thread in &self.threads {
            for label in &thread.labels {
                // 创建 Label 节点
                let label_node_id = id_gen.next("label_node");
                let mut label_node = Node {
                    id: label_node_id.clone(),
                    node_type: NodeType::Label,
                    position: label.position,
                    size: Vec2::new(180.0, 120.0),
                    collapsed: false,
                    params: HashMap::new(),
                    inputs: vec![],
                    outputs: vec![],
                    category: String::new(),
                };
                label_node.params.insert(
                    "name".to_string(),
                    ParamValue::Literal(serde_json::Value::String(label.name.clone())),
                );
                graph.add_node(label_node);

                // 创建 Start 节点（仅第一个标签）
                let start_node_id = id_gen.next("start");
                let start_node = Node {
                    id: start_node_id.clone(),
                    node_type: NodeType::Start,
                    position: label.entry_pin,
                    size: Vec2::new(180.0, 120.0),
                    collapsed: false,
                    params: HashMap::new(),
                    inputs: vec![],
                    outputs: vec![],
                    category: String::new(),
                };
                graph.add_node(start_node);

                // 记录标签节点 ID
                let mut label_node_ids = vec![start_node_id, label_node_id];

                // 复制标签内节点
                for (id, node) in &label.nodes {
                    graph.add_node(node.clone());
                    label_node_ids.push(id.clone());
                }

                // 复制标签内边
                for edge in label.edges.values() {
                    graph.add_edge(edge.clone()).ok();
                }

                graph.add_label(&label.name, label_node_ids);
            }

            for listener in &thread.listeners {
                let inner = &listener.inner;
                let label_node_id = id_gen.next("label_node");
                let mut label_node = Node {
                    id: label_node_id.clone(),
                    node_type: NodeType::Label,
                    position: inner.position,
                    size: Vec2::new(180.0, 120.0),
                    collapsed: false,
                    params: HashMap::new(),
                    inputs: vec![],
                    outputs: vec![],
                    category: String::new(),
                };
                label_node.params.insert(
                    "name".to_string(),
                    ParamValue::Literal(serde_json::Value::String(inner.name.clone())),
                );
                graph.add_node(label_node);

                let start_node_id = id_gen.next("start");
                let start_node = Node {
                    id: start_node_id.clone(),
                    node_type: NodeType::Start,
                    position: inner.entry_pin,
                    size: Vec2::new(180.0, 120.0),
                    collapsed: false,
                    params: HashMap::new(),
                    inputs: vec![],
                    outputs: vec![],
                    category: String::new(),
                };
                graph.add_node(start_node);

                let mut label_node_ids = vec![start_node_id, label_node_id];
                for (id, node) in &inner.nodes {
                    graph.add_node(node.clone());
                    label_node_ids.push(id.clone());
                }
                for edge in inner.edges.values() {
                    graph.add_edge(edge.clone()).ok();
                }
                graph.add_label(&inner.name, label_node_ids);
            }
        }

        graph
    }

    /// 查找包含指定节点的线程和标签
    pub fn find_node_location(&self, node_id: &str) -> Option<(&ThreadContainer, ContainerLocation<'_>)> {
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

/// 简单 ID 生成器，用于 to_legacy_graph
#[derive(Default)]
struct IdGenerator {
    counter: std::collections::HashMap<String, usize>,
}

impl IdGenerator {
    fn next(&mut self, prefix: &str) -> String {
        let idx = self.counter.entry(prefix.to_string()).or_insert(0);
        let id = format!("{}_{}", prefix, *idx);
        *idx += 1;
        id
    }
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
