# CustomMissions2 流编辑器 — Rust 项目骨架

> 版本：v1.0  
> 用途：定义 Rust 项目目录结构、模块划分和初始代码  
> 阅读对象：Agent（后端实现）  
> 相关文档：
>
> - 节点清单：[node_types.md](node_types.md)
> - JSON 规范：[json_schema.md](json_schema.md)
> - 开发约束：[agent_prompt.md](agent_prompt.md)
> - 示例任务：[docs/examples/new npc type/](examples/new%20npc%20type/)（包含 [main.code](examples/new%20npc%20type/main.code) 与 [meta.json](examples/new%20npc%20type/meta.json)）

---

## 一、目录结构

```text
CM2Editer/
├── Cargo.toml                 # 项目配置
├── Cargo.lock                 # 依赖锁定（自动生成）
├── LICENSE                    # MIT License
├── README.md                  # 项目说明
├── assets/                    # 静态资源（图标、字体、主题）
│   ├── icons/
│   └── themes/
├── src/
│   ├── main.rs                # 入口点
│   ├── lib.rs                 # 库入口（可选，如果要做成库）
│   ├── app.rs                 # 应用主循环 / 状态管理
│   ├── config.rs              # 配置加载与保存
│   ├── error.rs               # 全局错误类型定义
│   ├── graph/                 # 图数据结构（核心）
│   │   ├── mod.rs
│   │   ├── node.rs            # Node 结构体 + 端口
│   │   ├── edge.rs            # Edge 结构体
│   │   ├── graph.rs           # Graph 容器（节点+边集合）
│   │   ├── types.rs           # 节点类型枚举、端口类型枚举
│   │   └── validation.rs      # 图验证器（环检测、类型检查等）
│   ├── serializer/            # 序列化 / 反序列化
│   │   ├── mod.rs
│   │   ├── json.rs            # JSON ↔ Graph 转换
│   │   └── migration.rs       # 版本迁移逻辑
│   ├── code_gen/              # 代码生成（Graph → .code）
│   │   ├── mod.rs
│   │   ├── generator.rs       # 主生成器
│   │   ├── formatter.rs       # 缩进与格式化
│   │   └── templates/         # 代码模板
│   │       └── node_templates.json
│   ├── ui/                    # UI 层（前端交互）
│   │   ├── mod.rs
│   │   ├── canvas.rs          # 画布（无限网格、平移缩放）
│   │   ├── node_renderer.rs   # 节点渲染
│   │   ├── edge_renderer.rs   # 连线渲染
│   │   ├── panels/            # 面板
│   │   │   ├── mod.rs
│   │   │   ├── node_library.rs    # 左栏：节点库
│   │   │   ├── properties.rs      # 右栏：属性面板
│   │   │   ├── json_preview.rs    # 底部：JSON 预览
│   │   │   └── status_bar.rs      # 底部：状态栏
│   │   ├── interaction.rs     # 交互逻辑（拖拽、框选、连线）
│   │   └── theme.rs           # 主题与颜色配置
│   └── api/                   # 游戏 API 定义（节点清单的数据层）
│       ├── mod.rs
│       ├── definitions.rs       # 所有函数/对象的静态定义
│       └── registry.rs          # 节点注册表（运行时查询）
├── tests/                     # 集成测试
│   ├── fixtures/              # 测试数据（示例 JSON、.code 文件）
│   ├── test_graph.rs
│   ├── test_serializer.rs
│   └── test_validation.rs
└── docs/                      # 文档
    ├── agent_prompt.md
    ├── node_types.md          # 节点清单
    ├── json_schema.md         # JSON 规范
    └── rust_project_skeleton.md
```

---

## 二、Cargo.toml

```toml
[package]
name = "CM2Editer"
version = "0.1.0"
edition = "2021"
authors = ["richli"]
license = "MIT"
description = "A node-based visual editor for CustomMissions2 task scripts"
repository = "https://github.com/richli/CM2Editer"

[dependencies]
# 序列化
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# 错误处理
thiserror = "1.0"
anyhow = "1.0"

# 日志
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# 命令行参数
clap = { version = "4.5", features = ["derive"] }

# 配置加载
config = "0.14"

# 唯一 ID 生成
uuid = { version = "1.8", features = ["v4", "serde"] }

# 数值计算（向量/颜色）
 glam = "0.27"

# GUI 框架（选择其一，推荐 egui 或 iced）
# egui = "0.27"
# eframe = { version = "0.27", features = ["default"] }
# 或
# iced = "0.12"

# 文件对话框（如果需要原生文件对话框）
# rfd = "0.14"

[dev-dependencies]
# 测试辅助
tempfile = "3.10"
pretty_assertions = "1.4"

[profile.release]
opt-level = 3
lto = true
strip = true
```text

### 依赖说明

| 依赖 | 用途 |
| ------ | ------ |
| `serde` / `serde_json` | JSON 序列化与反序列化 |
| `thiserror` / `anyhow` | 错误处理 |
| `tracing` | 结构化日志 |
| `clap` | 命令行参数解析 |
| `uuid` | 全局唯一 ID 生成 |
| `glam` | 向量、颜色等数值计算 |
| `egui` / `eframe`（可选） | 即时模式 GUI |
| `tempfile`（dev） | 测试临时目录 |
| `pretty_assertions`（dev） | 更易读的测试失败输出 |

> 注意：GUI 框架待选定。如果做桌面端，推荐 `egui`（即时模式，开发快）或 `iced`（保留模式，更像 React）。如果嵌入 Web，用 `egui` + `wasm-bindgen`。若需要原生文件对话框，启用 `rfd`。

---

## 三、核心模块初始代码

### 3.1 src/error.rs — 全局错误类型

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FlowError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Node not found: {0}")]
    NodeNotFound(String),

    #[error("Edge connection error: {0}")]
    ConnectionError(String),

    #[error("Type mismatch: expected {expected}, got {actual}")]
    TypeMismatch { expected: String, actual: String },

    #[error("Unknown node type: {0}")]
    UnknownNodeType(String),

    #[error("Cycle detected in graph")]
    CycleDetected,

    #[error("Version mismatch: file version {file}, supported {supported}")]
    VersionMismatch { file: String, supported: String },
}

pub type Result<T> = std::result::Result<T, FlowError>;
```

### 3.2 src/graph/types.rs — 类型系统

```rust
use serde::{Deserialize, Serialize};

/// 节点类型枚举（覆盖 API 文档中的所有函数/对象）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum NodeType {
    // 控制流
    Start,
    Label,
    Goto,
    If,
    While,
    For,
    Break,
    Return,
    Wait,
    WaitForEvent,

    // 通用函数
    Log,
    Global,
    Local,
    GetType,
    GetLanguage,
    DumpVariables,
    DumpVariable,
    CallFunction,
    CallMethod,
    Color,
    Range,
    SetEvent,
    GetEvent,

    // 游戏函数（按子分类）
    DropItem,
    CollectItem,
    SetVibrator,
    SetPiston,
    LockHandcuffs,
    UnlockHandcuffs,
    EquipCosplay,
    UnequipCosplay,
    UnequipAllCosplay,
    OwnCosplay,
    EquipAdultToy,
    UnequipAdultToy,
    SetPlayerPosition,
    SetStage,
    SetCamera,
    SetAction,
    SetFutanari,
    SetSkill,
    SetPlayerData,
    SetSkillShortcut,
    GetSkillShortcut,
    GetRandomPosition,
    AddCurrentEarnRP,
    SetCurrentEarnRP,
    GetCurrentEarnRP,
    AddCurrentRP,
    SetCurrentRP,
    GetCurrentRP,
    SetEcstasy,
    AddEcstasy,
    GetEcstasy,
    SetStamina,
    AddStamina,
    GetStamina,
    SetMoisture,
    AddMoisture,
    GetMoisture,
    SetItemCount,
    AddItemCount,
    GetItemCount,
    CanGameOver,
    TriggerGameOver,
    PlaySoundEffect,
    SetStageRankLimit,
    GetStageRankLimit,
    SetPortalEnabled,
    GetAllWaypoints,
    SetSexPosition,
    DeactivateSex,
    SetSexMenu,

    // 附加游戏函数
    ShowBlackscreen,
    GetSnapshotData,
    GetAllSnapshots,
    DeleteSnapshot,
    GetImageReference,

    // 图形
    SetGraphicsOption,
    GetGraphicsOption,

    // 数学
    Random,
    RandomInt,
    Sin, Cos, Tan,
    Asin, Acos, Atan,
    Floor, Ceil, Round, Trunc,
    Sign, Abs,
    LogN, Log2, Log10,
    Min, Max,
    Vector,
    Quaternion,
    Vector3Length,
    Vector3SqrLength,
    Vector3Add,
    Vector3Sub,
    Vector3Scale,
    Vector3Dot,
    Vector3Cross,
    Vector3Rotate,
    Vector3Distance,

    // 字符串
    Length,
    Lower,
    Upper,
    Find,
    SubString,
    Format,
    ToNumber,

    // 文件
    FileExists,
    GetFiles,
    GetFileExtension,

    // 对象构造
    CreateList,
    Copy,
    CreateListFromJson,
    CreateThread,
    CreateListener,
    CreateListenerLocal,
    CreateMissionPanel,
    CreateMissionMenuItem,
    CreateArea,
    CreateZone,
    CreateCondition,
    CreateItemCondition,
    CreateInteractArea,
    CreateText,
    CreateMessengerChat,
    CreateAudio,
    CreateGallery,
    CreateSnapshot,
    CreateNPC,
    CreateInput,

    // 特殊
    Meta,
    Comment,
    Group,
}

/// 端口数据类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum PortType {
    Flow,       // 执行流
    Number,     // 数字
    String,     // 字符串
    Boolean,    // 布尔
    List,       // 列表
    Object,     // 对象（通用）
    Any,        // 任意类型（动态）
}

impl PortType {
    /// 检查两种类型是否兼容（可连接）
    pub fn is_compatible_with(&self, other: &PortType) -> bool {
        match (self, other) {
            (a, b) if a == b => true,
            (PortType::Any, _) | (_, PortType::Any) => true,
            // Number 和 String 在某些情况下可隐式转换（如 ToNumber）
            // 但编辑器中默认不允许，除非显式转换节点
            _ => false,
        }
    }

    /// 颜色编码（供 UI 使用）
    pub fn color(&self) -> [u8; 4] {
        match self {
            PortType::Flow => [255, 255, 255, 255],    // 白色
            PortType::Number => [66, 165, 245, 255],   // 蓝色
            PortType::String => [244, 143, 177, 255],  // 粉色
            PortType::Boolean => [239, 83, 80, 255],   // 红色
            PortType::List => [255, 202, 40, 255],     // 黄色
            PortType::Object => [102, 187, 106, 255],  // 绿色
            PortType::Any => [189, 189, 189, 255],      // 灰色
        }
    }
}
```text

### 3.3 src/graph/node.rs — 节点结构

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::types::{NodeType, PortType};
use super::node::Vec2;

/// 端口定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Port {
    pub id: String,
    pub port_type: PortType,
    pub label: String,
    pub required: bool,      // 是否必填
}

/// 节点参数值（支持引用其他节点输出）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParamValue {
    Literal(serde_json::Value),           // 直接值
    Ref { node: String, port: String },   // 引用其他节点
    Null,
}

/// 节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub node_type: NodeType,
    pub position: Vec2,
    pub size: Vec2,
    pub collapsed: bool,
    pub params: std::collections::HashMap<String, ParamValue>,
    pub inputs: Vec<Port>,
    pub outputs: Vec<Port>,
    pub category: String,
}

impl Node {
    pub fn new(node_type: NodeType, position: Vec2) -> Self {
        let id = format!("node_{}", &Uuid::new_v4().to_string()[..8]);
        Self {
            id,
            node_type,
            position,
            size: Vec2::new(180.0, 120.0),
            collapsed: false,
            params: std::collections::HashMap::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            category: String::new(),
        }
    }

    pub fn with_ports(mut self, inputs: Vec<Port>, outputs: Vec<Port>) -> Self {
        self.inputs = inputs;
        self.outputs = outputs;
        self
    }

    /// 获取指定端口
    pub fn get_port(&self, port_id: &str, is_input: bool) -> Option<&Port> {
        let ports = if is_input { &self.inputs } else { &self.outputs };
        ports.iter().find(|p| p.id == port_id)
    }
}

/// 2D 向量（用于坐标）
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
}
```

### 3.4 src/graph/edge.rs — 连线结构

```rust
use serde::{Deserialize, Serialize};
use super::types::PortType;
use super::node::Vec2;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: String,
    pub from: EdgeEndpoint,
    pub to: EdgeEndpoint,
    pub edge_type: PortType,
    pub waypoints: Vec<Vec2>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeEndpoint {
    pub node_id: String,
    pub port_id: String,
}

impl Edge {
    pub fn new(from: EdgeEndpoint, to: EdgeEndpoint, edge_type: PortType) -> Self {
        let id = format!("edge_{}_{}_{}_{}", from.node_id, from.port_id, to.node_id, to.port_id);
        Self {
            id,
            from,
            to,
            edge_type,
            waypoints: Vec::new(),
        }
    }
}
```text

### 3.5 src/graph/graph.rs — 图容器

```rust
use std::collections::{HashMap, HashSet};
use crate::error::{FlowError, Result};
use super::{Edge, EdgeEndpoint, Node};

#[derive(Debug, Default)]
pub struct Graph {
    pub nodes: HashMap<String, Node>,
    pub edges: HashMap<String, Edge>,
    pub labels: HashMap<String, Vec<String>>, // label -> node_ids
}

impl Graph {
    /// 添加节点；若存在相同 ID，则覆盖旧节点
    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    /// 删除节点，并级联删除与之相连的所有边
    pub fn remove_node(&mut self, node_id: &str) -> Result<()> {
        if !self.nodes.contains_key(node_id) {
            return Err(FlowError::NodeNotFound(node_id.to_string()));
        }

        let edges_to_remove: Vec<String> = self.edges.values()
            .filter(|e| e.from.node_id == node_id || e.to.node_id == node_id)
            .map(|e| e.id.clone())
            .collect();

        for edge_id in edges_to_remove {
            self.edges.remove(&edge_id);
        }
        self.nodes.remove(node_id);
        Ok(())
    }

    /// 添加边，并验证端点节点与端口存在性
    pub fn add_edge(&mut self, edge: Edge) -> Result<()> {
        let from_node = self.nodes.get(&edge.from.node_id)
            .ok_or_else(|| FlowError::NodeNotFound(edge.from.node_id.clone()))?;
        let to_node = self.nodes.get(&edge.to.node_id)
            .ok_or_else(|| FlowError::NodeNotFound(edge.to.node_id.clone()))?;

        if from_node.get_port(&edge.from.port_id, false).is_none() {
            return Err(FlowError::ConnectionError(
                format!("Output port {} not found on node {}", edge.from.port_id, edge.from.node_id)
            ));
        }
        if to_node.get_port(&edge.to.port_id, true).is_none() {
            return Err(FlowError::ConnectionError(
                format!("Input port {} not found on node {}", edge.to.port_id, edge.to.node_id)
            ));
        }

        self.edges.insert(edge.id.clone(), edge);
        Ok(())
    }

    /// 获取节点的所有入边
    pub fn incoming_edges(&self, node_id: &str) -> Vec<&Edge> {
        self.edges.values()
            .filter(|e| e.to.node_id == node_id)
            .collect()
    }

    /// 获取节点的所有出边
    pub fn outgoing_edges(&self, node_id: &str) -> Vec<&Edge> {
        self.edges.values()
            .filter(|e| e.from.node_id == node_id)
            .collect()
    }
}
```

### 3.6 src/graph/validation.rs — 图验证器

```rust
use std::collections::{HashMap, HashSet, VecDeque};
use crate::error::{FlowError, Result};
use super::types::PortType;
use super::Graph;

pub struct GraphValidator;

impl GraphValidator {
    /// 执行全部验证
    pub fn validate(graph: &Graph) -> Result<()> {
        Self::check_unique_ids(graph)?;
        Self::check_edge_endpoints(graph)?;
        Self::check_type_compatibility(graph)?;
        Self::check_single_input_per_port(graph)?;
        Self::check_no_cycles(graph)?;
        Self::check_required_params(graph)?;
        Ok(())
    }

    /// 检查节点 ID 唯一性
    fn check_unique_ids(graph: &Graph) -> Result<()> {
        let mut ids = HashSet::new();
        for id in graph.nodes.keys() {
            if !ids.insert(id) {
                return Err(FlowError::Validation(format!("Duplicate node id: {}", id)));
            }
        }
        Ok(())
    }

    /// 检查连线端点有效性
    fn check_edge_endpoints(graph: &Graph) -> Result<()> {
        for edge in graph.edges.values() {
            if !graph.nodes.contains_key(&edge.from.node_id) {
                return Err(FlowError::NodeNotFound(edge.from.node_id.clone()));
            }
            if !graph.nodes.contains_key(&edge.to.node_id) {
                return Err(FlowError::NodeNotFound(edge.to.node_id.clone()));
            }
        }
        Ok(())
    }

    /// 检查端口类型兼容性
    fn check_type_compatibility(graph: &Graph) -> Result<()> {
        for edge in graph.edges.values() {
            let from_node = graph.nodes.get(&edge.from.node_id)
                .ok_or_else(|| FlowError::NodeNotFound(edge.from.node_id.clone()))?;
            let to_node = graph.nodes.get(&edge.to.node_id)
                .ok_or_else(|| FlowError::NodeNotFound(edge.to.node_id.clone()))?;

            let from_port = from_node.get_port(&edge.from.port_id, false)
                .ok_or_else(|| FlowError::ConnectionError(format!("Port not found: {}", edge.from.port_id)))?;
            let to_port = to_node.get_port(&edge.to.port_id, true)
                .ok_or_else(|| FlowError::ConnectionError(format!("Port not found: {}", edge.to.port_id)))?;

            if !from_port.port_type.is_compatible_with(&to_port.port_type) {
                return Err(FlowError::TypeMismatch {
                    expected: format!("{:?}", to_port.port_type),
                    actual: format!("{:?}", from_port.port_type),
                });
            }
        }
        Ok(())
    }

    /// 检查数据输入端口是否只有一条入边
    fn check_single_input_per_port(graph: &Graph) -> Result<()> {
        let mut port_connections: HashSet<(String, String)> = HashSet::new(); // (node_id, port_id)

        for edge in graph.edges.values() {
            if edge.edge_type != PortType::Flow {
                let key = (edge.to.node_id.clone(), edge.to.port_id.clone());
                if !port_connections.insert(key) {
                    return Err(FlowError::ConnectionError(
                        format!("Input port {} on node {} has multiple connections", 
                            edge.to.port_id, edge.to.node_id)
                    ));
                }
            }
        }
        Ok(())
    }

    /// 检查 Flow 边是否有环（DAG 要求）
    fn check_no_cycles(graph: &Graph) -> Result<()> {
        // 构建邻接表（仅 Flow 边）
        let mut adj: HashMap<String, Vec<String>> = HashMap::new();
        let mut in_degree: HashMap<String, usize> = HashMap::new();

        for node_id in graph.nodes.keys() {
            adj.insert(node_id.clone(), Vec::new());
            in_degree.insert(node_id.clone(), 0);
        }

        for edge in graph.edges.values() {
            if edge.edge_type == PortType::Flow {
                adj.get_mut(&edge.from.node_id).unwrap().push(edge.to.node_id.clone());
                *in_degree.get_mut(&edge.to.node_id).unwrap() += 1;
            }
        }

        // Kahn 算法拓扑排序
        let mut queue: VecDeque<String> = in_degree.iter()
            .filter(|(_, &deg)| deg == 0)
            .map(|(id, _)| id.clone())
            .collect();

        let mut visited = 0;
        let mut processed = HashSet::new();

        while let Some(node_id) = queue.pop_front() {
            if !processed.insert(node_id.clone()) { continue; }
            visited += 1;

            if let Some(neighbors) = adj.get(&node_id) {
                for neighbor in neighbors {
                    if let Some(deg) = in_degree.get_mut(neighbor) {
                        *deg -= 1;
                        if *deg == 0 {
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
        }

        if visited != graph.nodes.len() {
            return Err(FlowError::CycleDetected);
        }

        Ok(())
    }

    /// 检查必填参数
    fn check_required_params(_graph: &Graph) -> Result<()> {
        // TODO: 根据节点类型定义中的必填字段检查 params
        // 需要接入 api::definitions 中的节点元数据
        Ok(())
    }
}
```text

### 3.7 src/serializer/json.rs — JSON 序列化

```rust
use serde_json::Value;
use crate::error::{FlowError, Result};
use crate::graph::{Edge, Graph, Node};
use crate::graph::types::PortType;

pub struct JsonSerializer;

impl JsonSerializer {
    /// Graph → JSON Value
    pub fn serialize(graph: &Graph, meta: Option<Value>) -> Result<Value> {
        let mut root = serde_json::Map::new();
        root.insert("version".to_string(), Value::String("1.0".to_string()));

        if let Some(m) = meta {
            root.insert("meta".to_string(), m);
        }

        let nodes: Vec<Value> = graph.nodes.values()
            .map(|n| serde_json::to_value(n).map_err(FlowError::Json))
            .collect::<Result<Vec<_>>>()?;
        root.insert("nodes".to_string(), Value::Array(nodes));

        let edges: Vec<Value> = graph.edges.values()
            .map(|e| serde_json::to_value(e).map_err(FlowError::Json))
            .collect::<Result<Vec<_>>>()?;
        root.insert("edges".to_string(), Value::Array(edges));

        let labels = serde_json::to_value(&graph.labels)?;
        root.insert("labels".to_string(), labels);

        Ok(Value::Object(root))
    }

    /// JSON Value → Graph
    pub fn deserialize(value: &Value) -> Result<(Graph, Option<Value>)> {
        let version = value.get("version")
            .and_then(|v| v.as_str())
            .ok_or_else(|| FlowError::Validation("Missing version field".to_string()))?;

        if version != "1.0" {
            return Err(FlowError::VersionMismatch {
                file: version.to_string(),
                supported: "1.0".to_string(),
            });
        }

        let meta = value.get("meta").cloned();
        let mut graph = Graph::default();

        // 解析节点
        if let Some(nodes) = value.get("nodes").and_then(|v| v.as_array()) {
            for node_val in nodes {
                let node: Node = serde_json::from_value(node_val.clone())
                    .map_err(|e| FlowError::Validation(format!("Failed to parse node: {}", e)))?;
                graph.add_node(node);
            }
        }

        // 解析连线
        if let Some(edges) = value.get("edges").and_then(|v| v.as_array()) {
            for edge_val in edges {
                let edge: Edge = serde_json::from_value(edge_val.clone())
                    .map_err(|e| FlowError::Validation(format!("Failed to parse edge: {}", e)))?;
                graph.add_edge(edge)?;
            }
        }

        // 解析标签
        if let Some(labels) = value.get("labels").and_then(|v| v.as_object()) {
            for (label, node_ids) in labels {
                if let Some(ids) = node_ids.as_array() {
                    let ids: Vec<String> = ids.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect();
                    graph.labels.insert(label.clone(), ids);
                }
            }
        }

        Ok((graph, meta))
    }
}
```

---

## 四、模块划分原则

模块划分遵循**单向依赖**原则，避免循环引用。

| 层级 | 职责 | 依赖关系 |
| ------ | ------ | --------- |
| **api** | 静态定义所有节点类型、参数、端口 | 不依赖其他模块 |
| **graph** | 核心数据结构（节点、边、图） | 依赖 `api::types` |
| **serializer** | JSON 读写、版本迁移 | 依赖 `graph`；不依赖 `api` |
| **code_gen** | 生成 `.code` 文件 | 依赖 `graph`、`api` |
| **ui** | 用户界面、画布交互 | 依赖 `graph`、`api` |
| **app** | 应用状态、主循环、事件分发 | 依赖所有上层模块 |

### 依赖图

```text
           ┌─────┐
           │ api │
           └──┬──┘
              │
     ┌────────┼────────┐
     ▼        ▼        ▼
  ┌──────┐ ┌────┐ ┌──────────┐
  │graph │ │ ui │ │ code_gen │
  └──┬───┘ └────┘ └─────┬────┘
     │                  │
     ▼                  ▼
  ┌─────────┐      ┌─────────┐
  │serializer│      │   app   │
  └─────────┘      └─────────┘
```text

---

## 五、构建与运行

```bash
# 开发构建
cargo build

# 运行
cargo run

# 运行测试（单元测试 + 集成测试）
cargo test

# 运行测试并显示输出
cargo test -- --nocapture

# 发布构建
cargo build --release

# 格式化代码
cargo fmt

# 静态检查
cargo check

# Clippy 静态分析（视警告为错误）
cargo clippy -- -D warnings

# 生成文档
cargo doc --open
```

### 推荐 CI 流程

```yaml
# .github/workflows/ci.yml 示例
steps:
  - uses: actions/checkout@v4
  - uses: dtolnay/rust-toolchain@stable
  - run: cargo fmt --check
  - run: cargo clippy -- -D warnings
  - run: cargo test
```text

---

## 六、下一步任务（给 Agent）

按以下顺序实现可降低模块间耦合：

### 阶段 1：数据层

1. [ ] 实现 `api::definitions` — 为每个 `NodeType` 定义参数模板和端口模板
2. [ ] 实现 `api::registry` — 运行时节点查询（根据类型名获取定义）
3. [ ] 实现 `graph::validation::check_required_params` — 接入 `api::definitions` 完成必填参数检查

### 阶段 2：序列化与代码生成

1. [ ] 实现 `serializer::migration` — 版本迁移逻辑
2. [ ] 实现 `code_gen::generator` — 将 Graph 导出为 `.code` 文件
3. [ ] 实现 `code_gen::formatter` — 缩进与格式化

### 阶段 3：UI 层

1. [ ] 实现 `ui::canvas` — 无限画布（网格、平移、缩放）
2. [ ] 实现 `ui::node_renderer` — 节点卡片渲染（标题栏、端口、参数预览）
3. [ ] 实现 `ui::edge_renderer` — 连线渲染（支持 waypoints）
4. [ ] 实现 `ui::interaction` — 拖拽、框选、连线创建
5. [ ] 实现 `ui::panels::node_library` — 左栏分类树 + 搜索
6. [ ] 实现 `ui::panels::properties` — 右栏参数编辑表单
7. [ ] 实现 `ui::panels::json_preview` — 底部实时 JSON 预览

### 阶段 4：集成

1. [ ] 集成测试 — 端到端：创建图 → 保存 JSON → 加载 → 验证 → 生成代码
2. [ ] 示例任务导入测试 — 使用 `docs/examples/new npc type/` 验证序列化与代码生成

---

> 提示：开始实现前，请确保已阅读 `docs/agent_prompt.md` 中的开发约束与 `docs/node_types.md` 中的节点清单。
