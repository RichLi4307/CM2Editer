# Agent 开发指令集 — CustomMissions2 流编辑器

> 版本：v1.0  
> 用途：直接复制粘贴给 AI Agent，作为开发上下文和约束条件  
> 语言：Rust  
> 框架：待定（egui / iced / Tauri+Web）

---

## 一、项目背景（你必须知道）

我们在开发一个**节点式流编辑器（Node-based Flow Editor）**，用于编辑一款游戏的**自定义任务脚本**。

- 目标用户：游戏 Mod 作者（非专业程序员）
- 输出格式：可视化画布 → JSON 文件 → 游戏加载器读取
- 技术栈：Rust（后端逻辑 + 可能的桌面 GUI）
- 许可证：MIT

---

## 二、领域模型（核心概念）

### 2.1 执行模型

游戏脚本语言有以下特征：
- **标签（Label）**：代码入口点，类似函数名。例：`main:`, `m1:`
- **线程（Thread）**：通过 `CreateThread("label")` 创建并发执行流
- **监听器（Listener）**：每帧运行的循环，通过 `CreateListener("label")` 创建
- **Goto 跳转**：线程内通过 `thread.Goto("label")` 切换执行位置
- **全局状态 `_state`**：只读，每帧更新，包含玩家位置、NPC、物品等
- **事件系统**：`SetEvent("name", data)` / `GetEvent("name")` 跨帧通信

### 2.2 数据类型

只有 5 种基本类型：
- `Number`（整数或浮点）
- `String`
- `Boolean`
- `List`（键值对集合，索引自动转字符串）
- `Object`（游戏对象引用，如 Thread、Area、NPC）

### 2.3 节点 = 函数调用

编辑器中的每个方块 = 文档中的一个函数调用或控制结构。

例：
- `DropItem(itemtype="Coat", stage="Residence", x=-26.6, y=-0.1, z=-120)`
- `If (condition) → True分支 / False分支`
- `CreateThread("m1")`

---

## 三、开发约束（必须遵守）

### 3.1 代码规范

```rust
// 1. 所有错误用 Result<T, FlowError> 处理，禁止 unwrap
// 错误示例：
let value = map.get("key").unwrap(); // ❌ 禁止

// 正确示例：
let value = map.get("key").ok_or(FlowError::NodeNotFound("key".to_string()))?; // ✅

// 2. 公共 API 必须写文档注释（///）
/// 向图中添加一个节点
pub fn add_node(&mut self, node: Node) -> Result<()> { ... }

// 3. 禁止 unsafe 代码
// 4. 使用 Rust 2021 Edition
// 5. 数据结构必须 derive Clone（除非有明确理由）
// 6. 函数参数优先用 &str 而非 String，返回优先用 String
// 7. 集合类型优先用 HashMap / Vec，避免 LinkedList
```

### 3.2 模块边界

```
api/         → 只定义静态数据（节点类型、参数模板），不依赖其他模块
graph/       → 核心数据结构，只依赖 api::types
serializer/  → JSON 读写，依赖 graph
code_gen/    → 生成 .code 文件，依赖 graph + api
ui/          → 界面，依赖 graph + api
app/         → 主循环，依赖所有模块
```

### 3.3 错误处理策略

| 场景 | 处理方式 |
|------|---------|
| 用户操作（如删除节点） | 返回 Result，失败时显示 Toast 提示 |
| 文件加载失败 | 返回 Result，上层显示错误对话框 |
| 图验证失败 | 收集所有错误，批量显示在底部面板 |
| 内部不变量被破坏 | 使用 `debug_assert!`，Release 模式不 panic |

---

## 四、JSON 格式契约（编辑器 ↔ 加载器）

编辑器保存的文件格式：

```json
{
  "version": "1.0",
  "meta": { /* meta.json 内容 */ },
  "nodes": [
    {
      "id": "node_xxx",
      "type": "DropItem",
      "position": { "x": 200, "y": 150 },
      "params": { "itemtype": "Coat", "stage": "Residence", "x": -26.6 },
      "inputs": [{ "id": "in_flow", "type": "Flow", "label": "执行" }],
      "outputs": [{ "id": "out_flow", "type": "Flow", "label": "下一步" }]
    }
  ],
  "edges": [
    {
      "id": "edge_xxx",
      "from": { "node": "node_a", "port": "out_flow" },
      "to": { "node": "node_b", "port": "in_flow" },
      "type": "Flow"
    }
  ],
  "labels": {
    "main": ["node_001", "node_002"],
    "m1": ["node_003"]
  }
}
```

> 规则：
> - `version` 必须存在，加载时检查兼容性
> - `meta` 直接透传，编辑器不解析内容
> - `nodes` 中 `id` 全局唯一
> - `edges` 中端点必须指向存在的节点和端口
> - `labels` 定义标签到节点序列的映射

---

## 五、UI 设计规范（如果做 GUI）

### 5.1 布局

```
┌─────────────────────────────────────────────────────────────┐
│  [工具栏]  保存 | 撤销 | 重做 | 导出JSON | 运行预览           │
├──────────┬──────────────────────────────┬─────────────────┤
│          │                              │                 │
│  节点库   │      无限画布                 │   属性面板       │
│  (左栏)   │      (网格背景)               │   (右栏)        │
│  [搜索]   │                              │                 │
│  ───────  │      节点 + 连线               │  选中节点参数    │
│  分类A    │                              │  ───────────    │
│  ├ 节点1  │                              │  输入框/下拉/开关 │
│  └ 节点2  │                              │                 │
│          │                              │                 │
├──────────┴──────────────────────────────┴─────────────────┤
│  [底部]  JSON预览 | 错误列表 | 状态栏                        │
└─────────────────────────────────────────────────────────────┘
```

### 5.2 节点外观

- 矩形卡片，圆角 8px
- 标题栏高度 32px，颜色按分类区分
- 端口：直径 12px 的圆，左侧输入、右侧输出
- 执行流端口（白色）位于最上方/最下方
- 选中状态：2px 蓝色发光边框

### 5.3 交互

- 中键拖拽：平移画布
- 滚轮：缩放（以鼠标为中心）
- 双击空白处：呼出快速搜索创建节点
- 从端口拖出线：创建连线，靠近兼容端口时高亮
- 右键节点：菜单（复制、删除、折叠）
- Ctrl+Z / Ctrl+Y：撤销/重做（至少 50 步）

---

## 六、节点类型注册表（关键数据结构）

每个节点类型必须有以下元数据：

```rust
pub struct NodeDefinition {
    pub node_type: NodeType,
    pub category: String,           // "Control" / "Game Functions" / "Objects"
    pub display_name: String,       // 显示名称（如 "掉落物品"）
    pub description: String,        // 文档说明
    pub inputs: Vec<PortDefinition>,
    pub outputs: Vec<PortDefinition>,
    pub params: Vec<ParamDefinition>, // 节点参数（非端口连接的常量值）
    pub color: [u8; 4],             // 标题栏颜色
}

pub struct PortDefinition {
    pub id: String,
    pub port_type: PortType,
    pub label: String,
    pub required: bool,
}

pub struct ParamDefinition {
    pub name: String,               // 对应 API 参数名（如 "itemtype"）
    pub display_name: String,       // 显示名（如 "物品类型"）
    pub param_type: ParamType,      // Number / String / Boolean / List / Object
    pub default: Option<ParamValue>,
    pub required: bool,
    pub options: Option<Vec<String>>, // 下拉选项（如 StageType 枚举值）
}
```

---

## 七、测试要求

每个模块必须包含单元测试：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_node() {
        let mut graph = Graph::default();
        let node = Node::new(NodeType::Log, Vec2::new(0.0, 0.0));
        graph.add_node(node.clone());
        assert_eq!(graph.nodes.len(), 1);
        assert!(graph.nodes.contains_key(&node.id));
    }

    #[test]
    fn test_cycle_detection() {
        let mut graph = Graph::default();
        let n1 = Node::new(NodeType::Start, Vec2::ZERO);
        let n2 = Node::new(NodeType::Log, Vec2::ZERO);
        let n3 = Node::new(NodeType::Log, Vec2::ZERO);

        graph.add_node(n1.clone());
        graph.add_node(n2.clone());
        graph.add_node(n3.clone());

        // 创建环：n1 → n2 → n3 → n1
        graph.add_edge(Edge::new(
            EdgeEndpoint { node_id: n1.id.clone(), port_id: "out".to_string() },
            EdgeEndpoint { node_id: n2.id.clone(), port_id: "in".to_string() },
            PortType::Flow,
        )).unwrap();

        graph.add_edge(Edge::new(
            EdgeEndpoint { node_id: n2.id.clone(), port_id: "out".to_string() },
            EdgeEndpoint { node_id: n3.id.clone(), port_id: "in".to_string() },
            PortType::Flow,
        )).unwrap();

        graph.add_edge(Edge::new(
            EdgeEndpoint { node_id: n3.id.clone(), port_id: "out".to_string() },
            EdgeEndpoint { node_id: n1.id.clone(), port_id: "in".to_string() },
            PortType::Flow,
        )).unwrap();

        assert!(GraphValidator::validate(&graph).is_err());
    }
}
```

---

## 八、常见陷阱（避免踩坑）

1. **不要假设节点有固定数量端口** — 某些节点（如 `Format`）支持可变参数
2. **List 类型既是数组也是字典** — 内部键始终为字符串，整数索引自动转换
3. **Goto 不是函数调用** — 它是状态跳转，不产生新作用域（除非用 Thread）
4. **Listener 每帧运行** — 生成代码时不能把它展开成普通循环，必须保留事件监听语义
5. **`_state` 只读** — 编辑器中不应提供修改 `_state` 的节点，只提供读取节点
6. **参数引用 vs 常量** — 端口连接 = 动态传值；`params` = 编译期常量。两者在 JSON 中格式不同

---

## 九、任务分配模板（给 Agent 发任务时复制）

```markdown
请实现以下功能：

**模块**：`src/xxx/xxx.rs`
**依赖**：`graph::types`, `graph::node`
**输入**：...
**输出**：...
**约束**：
- 使用 Result 错误处理，禁止 unwrap
- 写文档注释
- 包含单元测试
- 不引入 unsafe

**验收标准**：
1. [ ] 编译通过 `cargo check`
2. [ ] 测试通过 `cargo test`
3. [ ] Clippy 无警告 `cargo clippy`
4. [ ] 代码格式化 `cargo fmt`
```

---

## 十、参考资源

- 节点清单：`docs/node_types.md`
- JSON 规范：`docs/json_schema.md`
- UI 设计规范：`docs/ui_spec.md`
- 项目骨架：`docs/rust_project_skeleton.md`
- API 原文档：`docs/documentation_zh.html`（游戏官方文档）

---

> 最后提醒：这不是一个普通 GUI 项目，而是一个**领域特定语言（DSL）的可视化编辑器**。核心难点不是画方块，而是**正确地把一张图翻译成游戏脚本语言的执行语义**。务必先理解 Thread/Listener/Goto 的执行模型，再动手写代码生成器。
