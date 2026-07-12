# Agent 开发指令集 — CustomMissions2 流编辑器

> 版本：v3.0-architecture
> 用途：直接复制粘贴给 AI Agent，作为开发上下文和跨阶段约束条件
> 语言：Rust
> 框架：egui（默认 GUI） / iced / Tauri+Web
>
> **说明**：当前进入新架构阶段（v0.3.0-architecture），核心图模型从扁平 `nodes + edges + labels` 转向 `ThreadContainer / LabelContainer / ListenerContainer` 容器化结构。旧版文档已归档至 `docs/archive/`。

---

## 一、项目背景（你必须知道）

我们在开发一个**节点式流编辑器（Node-based Flow Editor）**，用于编辑一款游戏的**自定义任务脚本**。

- 目标用户：游戏 Mod 作者（非专业程序员）
- 输出格式：可视化画布 → 工程文件夹（`meta.json` + 多个 `.code` + 内部 `.cm2editor/*.code.json`） → 游戏加载器读取
- 技术栈：Rust（后端逻辑 + 桌面 GUI）
- 编辑器中间格式：每个 `.code` 文件对应一个 JSON 节点图（见 [json_schema.md](json_schema.md)）
- 游戏脚本格式：`.code`（由 `code_gen` 模块生成）
- 许可证：MIT

---

## 二、领域模型（核心概念）

### 2.1 执行模型（新架构）

游戏脚本语言 `.code` 有以下特征，新架构以这些语法结构为骨架：

- **ThreadContainer（线程容器）**：一个 `.code` 顶层线程，包含若干标签和监听器。代码中生成 `var_name = CreateThread("label")`。
- **LabelContainer（标签容器）**：线程内的命名代码块，类似函数/入口点。例：`main:`, `m1:`。同一容器内的 `Flow` 边仅表示顺序执行。
- **ListenerContainer（监听器容器）**：每帧或每秒调用一个标签的循环。代码中生成 `var_name = CreateListener("label")` 或 `CreateListenerLocal("label")`。
- **Goto 跳转**：同一 `ThreadContainer` 内通过 `thread.Goto("label")` 切换执行位置。
- **全局状态 `_state`**：只读，每帧更新，包含玩家位置、NPC、物品等。
- **事件系统**：`SetEvent("name", data)` / `GetEvent("name")` 跨帧通信。
- **Flow 边**：仅在同一 `LabelContainer` / `ListenerContainer` 内部表示顺序；跨容器关系通过名称引用或 `Data` 边表达。

### 2.2 旧架构兼容说明

- `Start` 和 `Label` 节点在新架构中已弃用，分别由 `ThreadContainer` / `LabelContainer` 替代。
- 保留反序列化兼容，加载旧工程时自动迁移提示。
- 旧 JSON v1.x 与新版 v2.0 不兼容，需通过迁移脚本转换。

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
// 1. 所有错误用 Result<T, FlowError> 处理，禁止 unwrap / expect
// 错误示例：
let value = map.get("key").unwrap(); // ❌ 禁止

// 正确示例：
let value = map.get("key").ok_or(FlowError::NodeNotFound("key".to_string()))?; // ✅

// 2. 公共 API 必须写文档注释（///）
/// 向图中添加一个节点
pub fn add_node(&mut self, node: Node) -> Result<()> { ... }

// 3. 禁止 unsafe 代码；如确需调用外部库，必须在上层封装并在 PR 中说明理由
// 4. 使用 Rust 2024 Edition 作为模板基线；实际项目以 `Cargo.toml` 中的 `edition` 字段为准（当前为 2024 Edition）
// 5. 数据结构必须 derive Clone（除非有明确理由，如持有非 Clone 资源）
// 6. 函数参数优先用 &str 而非 String，返回优先用 String
// 7. 集合类型优先用 HashMap / Vec，避免 LinkedList
// 8. 单元测试必须放在模块末尾的 #[cfg(test)] mod tests 中
```

### 3.2 模块边界

模块依赖关系必须遵守以下规则：

```text
api/         → 只定义静态数据（节点类型、参数模板、端口定义），不依赖其他模块
graph/       → 核心数据结构（Node / Edge / Graph），只依赖 api::types
serializer/  → JSON 读写与版本迁移，依赖 graph
             → 注意：serializer 不依赖 api，所有节点类型通过字符串反查
code_gen/    → 生成 .code 文件，依赖 graph + api
project/     → 工程管理（meta.json、多 .code 文件、保存/导出），依赖 graph + serializer + code_gen + api
ui/          → 界面渲染与交互，依赖 graph + api + project
app/         → 主循环与状态管理，依赖所有上层模块
```

**禁止循环依赖**：

- `api` 不能依赖 `graph`、`serializer`、`code_gen`、`ui`、`app`
- `graph` 不能依赖 `serializer`、`code_gen`、`ui`、`app`
- `serializer` 不能依赖 `code_gen`、`ui`、`app`
- `code_gen` 不能依赖 `ui`、`app`

### 3.3 错误处理策略

| 场景 | 处理方式 |
| ------ | --------- |
| 用户操作（如删除节点） | 返回 Result，失败时显示 Toast 提示 |
| 文件加载失败 | 返回 Result，上层显示错误对话框 |
| 图验证失败 | 收集所有错误，批量显示在底部面板 |
| 内部不变量被破坏 | 使用 `debug_assert!`，Release 模式不 panic |

---

## 四、JSON 格式契约（编辑器 ↔ 加载器）

编辑器保存的文件格式（完整字段定义见 [json_schema.md](json_schema.md)）：

```json
{
  "version": "2.0",
  "meta": { /* meta.json 内容 */ },
  "threads": [
    {
      "id": "thread_main",
      "name": "main",
      "variable_name": "var_main_thread",
      "auto_start": true,
      "labels": [
        {
          "id": "label_main",
          "name": "main",
          "params": [],
          "nodes": [...],
          "edges": [...]
        }
      ],
      "listeners": [],
      "position": { "x": 0, "y": 0 }
    }
  ],
  "viewport": { "x": 0, "y": 0, "zoom": 1.0 }
}
```

> 规则：
>
> - `version` 必须存在，当前为 `"2.0"`；加载时检查兼容性，保存时始终写入最新版本
> - `meta` 直接透传，编辑器不解析内容
> - `threads` 是线程容器数组，每个线程包含 `labels` 和 `listeners`
> - `labels` 和 `listeners` 是独立的容器，内部包含 `nodes` 和 `edges`
> - `Flow` 边只能出现在同一 `LabelContainer` / `ListenerContainer` 内部
> - `Data` 边可以跨 `LabelContainer` 连接同一 `ThreadContainer` 内的节点
> - `params` 中的值可以是常量（字符串/数字/布尔/列表/对象），也可以是引用：`{ "ref": "node_id", "port": "port_id" }` 表示端口动态连接
> - `viewport` 为视图层状态，不影响逻辑

### 项目文件夹结构（工程管理）

编辑器保存的不仅是单一 JSON 文件，而是整个 Custom Missions 2 项目文件夹。一个完整工程必须包含：

```text
MyMission/
├── meta.json                    # 任务标题、描述、设置菜单
├── main.code                    # 主逻辑代码
├── common.code                  # 可选：公共函数/标签
└── Images/                      # 可选：图片资源
```

- `meta.json` 必须存在，定义多语言 `title`/`description` 和 `settings` 菜单；设置值通过全局变量 `_settings` 在代码中读取。
- 项目可拆分为多个 `.code` 文件，游戏加载时按字母顺序合并，因此跨文件标签/跳转必须可解析。
- 编辑器内部使用 `.json` 文件保存每个 `.code` 对应的节点图（每个 `.code` 对应一个 Graph），并与工程文件夹一起保存/加载。
- 新建/打开工程时必须选择项目文件夹并命名，不再使用单一文件对话框。

> 工程管理已 Phase 4.5 实现，入口在 `src/project.rs` 与 `src/app.rs`。新建/打开工程时使用文件夹对话框，保存时同步写回 `meta.json`、所有 `.code` 文件以及内部 `.cm2editor/*.code.json` 节点图。

---

## 五、节点类型注册表（关键数据结构）

每个节点类型必须有以下元数据。完整节点清单见 [node_types.md](node_types.md)。

> **NodeType 计数规则**：当前 `NodeType` 枚举包含 **168 个变体**。新架构下按 `.code` 语言概念重新分类：Threading & Concurrency、Control Flow、Variables & Globals、Literals、Math & Logic、Conditions & Queries、String/File/List、Game API、Objects、Editor-only。`Start` 和 `Label` 已弃用，保留反序列化兼容。

```rust
pub struct NodeDefinition {
    pub node_type: NodeType,
    pub category: String,           // "Control" / "General Functions" / "Game Functions" / "Objects"
    pub display_name: String,       // 显示名称（如 "掉落物品"）
    pub description: String,        // 文档说明
    pub inputs: Vec<PortDefinition>,
    pub outputs: Vec<PortDefinition>,
    pub params: Vec<ParamDefinition>, // 节点参数（非端口连接的常量值）
    pub color: [u8; 4],             // 标题栏颜色 RGBA
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
    pub param_type: ParamType,      // Number / String / Boolean / List / Object / Color / Vector / Quaternion
    pub default: Option<ParamValue>,
    pub required: bool,
    pub options: Option<Vec<String>>, // 下拉选项（如 StageType 枚举值）
    pub description: Option<String>,  // 参数说明（悬停提示）
}
```

---

## 六、测试要求

每个模块必须包含单元测试，并在 CI 中通过 `cargo test`。测试模板见 `docs/TODO.md` 末尾的快速参考。

### 6.1 推荐测试类型

| 测试类型 | 说明 | 示例 |
| ---------- | ------ | ------ |
| 单元测试 | 单个函数/方法的正确性 | `Node::get_port`、参数默认值 |
| 集成测试 | 多模块协作 | `Graph → JSON → Graph` 往返 |
| 属性测试 | 随机输入验证不变量 | `serde_json` 任意合法 JSON 不 panic |
| 快照测试 | 代码生成输出稳定 | `.code` 输出对比 |

### 6.2 测试数据

- 使用 `tests/fixtures/` 存放示例 JSON 和 `.code` 文件
- 不要依赖真实文件路径，使用 `tempfile` 创建临时目录
- 断言失败信息必须包含输入数据和期望输出

---

## 常用 UI 组件速查

### ParamTextEdit（参数文本输入）

文件 `src/ui/panels/param_text_edit.rs` 是统一的文本编辑组件。**禁止裸 `ui.text_edit_singleline`**——它在 `source_editor` 的 `ComboBox` 之后会失去键盘焦点。

```rust
// ✅ 用 ParamTextEdit
ParamTextEdit::show(ui, key, value, "占位提示")
// 返回 Option<(String, ParamValue)>
```

组件内置：`val_to_str`（ParamValue→String）、`str_to_param`（String→ParamValue）、`hint_text` 占位提示、`desired_width` 自适应。

### 其他可复用模式

| 场景 | 实现位置 |
|------|---------|
| 枚举下拉 | `properties.rs:enum_editor` — ComboBox with id_salt |
| 布尔勾选 | `properties.rs:literal_editor` — `ui.checkbox` |
| 数值拖拽 | `properties.rs:literal_editor` — `egui::DragValue` |
| Vector/xyz | `properties.rs:vector_editor` — 三字段 DragValue |
| 命名空间选择 | `namespace_picker.rs` — NavigatorPicker 窗口 |
| 坐标选择 | `coordinate_picker.rs` — CoordinatePicker 窗口 |
| 条件模板 | `properties.rs:condition_template_editor` |
| 文本输入 | **必须用 `param_text_edit.rs:ParamTextEdit::show()`** |

---

## 七、常见陷阱（避免踩坑）

1. **不要假设节点有固定数量端口** — 某些节点（如 `Format`）支持可变参数，端口定义需由 `api::definitions` 动态生成
2. **List 类型既是数组也是字典** — 内部键始终为字符串，整数索引自动转换；遍历时注意 `Count()` 与键集合的区别
3. **Goto 不是函数调用** — 它是状态跳转，不产生新作用域（除非用 Thread）
4. **Listener 每帧运行** — 生成代码时不能把它展开成普通循环，必须保留事件监听语义
5. **`_state` 只读** — 编辑器中不应提供修改 `_state` 的节点，只提供读取节点
6. **参数引用 vs 常量** — 端口连接 = 动态传值；`params` = 编译期常量。两者在 JSON 中格式不同
7. **线程作用域隔离** — 不同 `ThreadContainer` 中的同名变量互不影响；跨线程通信需使用 `SetEvent` / `GetEvent`
8. **NodeType 与字符串必须一致** — 序列化后的 `type` 字段必须与 `NodeType` 的 `PascalCase` 名称完全匹配
9. **Flow 边仅限容器内部** — `Flow` 边只能在同一 `LabelContainer` / `ListenerContainer` 内表示顺序；跨容器关系用名称引用或 `Data` 边表达
10. **保存前验证** — 导出 JSON 或 `.code` 前必须调用 `GraphValidator::validate` 并处理所有错误
11. **特殊节点不参与代码生成** — `Meta` / `Comment` / `Group` 只用于编辑期元数据或视觉组织，验证器应允许其存在，但跳过拓扑排序与代码生成路径
12. **不要把旧 `Start`/`Label` 节点带入新图** — 新工程中应使用 `ThreadContainer` / `LabelContainer` 的入口钉

---

## 八、参考资源

- [节点清单](node_types.md)
- [JSON 规范](json_schema.md)
- [Rust 项目骨架](rust_project_skeleton.md)
- [示例任务](examples/new%20npc%20type/main.code) 与 [meta.json](examples/new%20npc%20type/meta.json)
- [`.code` DSL 权威参考](code_api_reference.md)（基于官方英文文档 `documentation.html`）
- [项目进度与 backlog](TODO.md) — 当前进入新架构阶段：核心图模型容器化（ThreadContainer / LabelContainer / ListenerContainer）

## 九、交付与维护规则

### 9.1 每次交付必须做的事

1. **更新 `CHANGELOG.md`** — 每次功能交付后追加条目，记录新增/修复/变更/测试数据。发布到 GitHub 时可直接用作 Release Notes。
2. **更新 `docs/TODO.md`** — 标记已完成任务 ✅，追加工作日志条目（日期 + 任务编号 + 说明 + 状态）。
3. **`cargo test` 全过再 commit** — 108 项全部通过为提交门槛。`cargo clippy` 只允许 pre-existing 警告。
4. **任何任务完成后必须提交一次 commit** — 即使是验证、文档或配置调整，只要改动了文件，就应当以 commit 收尾；不要留下未提交的改动。
5. **commit message 用中文前缀** — 格式为 `<类型>: <简要描述>`，例如：`修复: Goto 标签自动注册`、`新增: 回归测试`、`文档: 更新交付规则`。类型可选：新增/修复/文档/重构/测试/杂项。方便中文项目维护者阅读 `git log`。 

### 9.2 重要文档归档

重大重构前后，将旧版文件归档到 `docs/archive/`，命名格式 `{文件名}_{YYYYMMDD}_v{序号}.{ext}`。

示例：

```text
docs/archive/TODO_20260708_v5.md
docs/archive/agent_prompt_v1_full.md
```

git 可以找回历史，但归档文件作为"快照"更方便追溯设计决策。

### 9.3 CHANGELOG 格式

```markdown
## [0.1.0] — 2026-07-08（续）

### 新增（功能模块）
- 功能点...

### 修复
- 修复点...

### 文档
- 文档变更...

### 测试
- 测试数据：N tests passed
```

---

> 最后提醒：这不是一个普通 GUI 项目，而是一个**领域特定语言（DSL）的可视化编辑器**。核心难点不是画方块，而是**正确地把一张图翻译成游戏脚本语言的执行语义**。务必先理解 Thread/Listener/Goto 的执行模型，再动手写代码生成器。

## 节点修改强制规则

修改或新增节点时，**必须先读 `docs/node_types.md`**。

该文件将 168 个节点按 `.code` 语言概念分为三类：

| 类别 | 定义 | 规则 |
|------|------|------|
| **A 类**（自定义代码生成） | 有显式 `match` 臂的节点（If/While/For/Goto/CallFunction/CallMethod/ForeachNode/Return/Break/Wait/DestroyListener/WaitForThread 等） | 新增 Data 输出端口**必须手动写** `var_{id}_{port} = ...`；端口值需被引用时**还必须在 `evaluate_data_output` 加分支** |
| **B 类**（通用代码生成） | 走 `generate_node_call` 的节点（大多数 Game API / Object 节点） | 新增 Data 输出端口**自动生成**，无需改代码生成器 |
| **C 类**（纯 Data 节点） | 无 Flow 端口的节点（Literals / Variables / Globals / Math / Logic / Conditions 等） | 新增需在 `evaluate_data_output` 加分支 |

**注意**：`Start` / `Label` 已弃用，不再出现在新分类中。旧工程反序列化时兼容，但新图不应使用。

**每次修改节点定义后必须同步更新 `docs/node_types.md`**。
