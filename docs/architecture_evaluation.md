# CM2Editer 架构评估：以 .code 为中心的节点/图设计

> **状态**: 评估草案 | **优先级**: P0 / 架构级 | **影响面**: 核心图模型、节点类型、代码生成、UI、JSON 格式
> **日期**: 2026-07-13
> **背景**: 用户提出 "编辑器应该为 .code 的语法结构服务"，认为历史 Start 节点和节点分类偏离了 .code 的真实语义。

**摘要**: 当前 CM2Editer 的图模型把 .code 当作**扁平函数调用序列 + 控制流边**来处理。然而 .code 的真实结构是 **类 Python 的层级脚本 + 标签驱动的并发状态机**。这导致 Start 节点、Label 节点、Flow 边、以及按 API 表面分类的节点库都与 .code 语义存在结构性错配。本文档基于 docs/code_api_reference.md、docs/documentation.html、示例 .code 文件以及主流可视化脚本编辑器（UE Blueprints、Unity Bolt、Godot VisualScript、RPG Maker、Warcraft III 触发器等）进行系统评估，并提出 redesign 方向。

---

## 目录

- [1. 背景与问题](#1-背景与问题)
- [2. 研究方法](#2-研究方法)
- [3. .code 语义模型](#3-code-语义模型)
  - [3.1 执行模型：模块加载与顶层语句](#31-执行模型模块加载与顶层语句)
  - [3.2 标签的三重身份](#32-标签的三重身份)
  - [3.3 线程 vs 监听器 vs 局部监听器](#33-线程-vs-监听器-vs-局部监听器)
  - [3.4 变量作用域](#34-变量作用域)
  - [3.5 Goto 与 Return 的区别](#35-goto-与-return-的区别)
  - [3.6 多文件合并](#36-多文件合并)
- [4. 当前架构错配分析](#4-当前架构错配分析)
- [5. 目标架构设计](#5-目标架构设计)
- [6. 迁移路线图](#6-迁移路线图)
- [7. 风险与决策清单](#7-风险与决策清单)
- [8. 结论与建议](#8-结论与建议)
- [附录 A: 术语表](#附录-a-术语表)
- [附录 B: 关键代码引用](#附录-b-关键代码引用)
- [附录 C: 参考来源](#附录-c-参考来源)

---

## 1. 背景与问题

CM2Editer 是一个节点式流编辑器，目标输出是 Custom Missions 2 的 `.code` 脚本。`.code` 是一种**类 Python 的领域特定语言**，拥有自己的执行模型、作用域规则和并发原语。

然而，当前编辑器的图模型是在早期阶段按照"流程图转代码块"的直觉构建的：

- 用 `Start` 节点表示唯一入口。
- 用 `Flow` 边连接所有节点，表示执行顺序。
- 用 `Label` 节点作为标签体的视觉锚点。
- 按 API 函数表面分类节点（如 `Objects`、`Game Functions`、`Math`）。

这种模型与 `.code` 的真实语义产生严重错位。随着功能增加，错位越来越严重：

- `Start` 节点让初学者误以为 `main:` 是特殊语法，而实际上它只是普通标签。
- `Flow` 边被同时用于"标签内顺序"和"标签间引用"，导致验证器错误地禁止循环和菱形路径。
- 线程/监听器没有视觉容器，代码生成器只能用 BFS 可达性启发式推断子标签。
- 节点分类混乱，把全局变量、线程/监听器、字面量、逻辑运算混在 API 类别中。

因此，需要基于 `.code` 的真实语法结构，重新评估并设计编辑器的核心架构。

---

## 2. 研究方法

本评估综合以下来源：

1. **官方文档**
   - `docs/code_api_reference.md`：基于官方英文 `documentation.html` 和 80+ 个实战 `.code` 反推的 DSL 参考。
   - `docs/documentation.html`：官方英文 API 文档。
   - `docs/documentation_zh.html`：社区翻译中文版（部分章节缺失，作为辅助）。
2. **示例代码**
   - `tests/fixtures/*.code`：项目测试夹具。
3. **当前实现**
   - `src/graph/*.rs`：图数据结构、节点、边、验证器。
   - `src/code_gen/generator.rs`：代码生成器。
   - `src/api/definitions.rs`：节点定义和分类。
   - `src/project.rs`：工程创建和默认图。
4. **跨编辑器模式研究**
   - Unreal Engine Blueprints（事件图、函数、状态机、Latent 节点）。
   - Unity Bolt / Visual Scripting（Flow Graph、State Graph、coroutine 事件）。
   - Godot VisualScript（函数、yield、signal）。
   - RPG Maker 事件页 / Warcraft III GUI 触发器（事件、条件、动作、并行/自动运行）。
   - Yarn Spinner / Twine（段落、跳转、状态变量）。


---

## 3. `.code` 语义模型

### 3.1 执行模型：模块加载与顶层语句

当 CM2 加载任务时，所有 `.code` 文件按文件名排序合并为一个脚本，然后执行顶层语句。典型的顶层结构：

```code
var_main_thread = CreateThread("main")

main:
    thread = _this
    ...

step1:
    ...
    thread.Goto("step2")

check_loop:
    if _state.Futanari
        thread.Goto("step2")
    _result = null
```

关键点：

- 只有被顶层 `CreateThread("label")` 显式启动的标签才会在模块加载时执行一次。
- `main:` 不是特殊入口，只是一个约定俗成的顶层线程标签。
- 标签内部按顺序执行语句；结束时不自动循环，只是返回 `_result`（默认为 `null`）。

### 3.2 标签的三重身份

| 身份 | 触发方式 | 运行次数 | 作用域 |
|------|---------|---------|--------|
| 函数 | `labelname(args)` | 一次 | 新子作用域，继承父作用域 |
| 线程入口 | `CreateThread("label")` | 一次（创建时），之后由 `Goto` 切换 | 同一线程实例，`Goto` 时旧上下文销毁 |
| 监听器回调 | `CreateListener("label")` / `CreateListenerLocal("label")` | 每帧/每秒 | 捕获创建时的作用域 |

> **官方文档**："Labels are used to create custom functions and as entrypoints for threads."（`documentation.html`）
> **中文参考**："每个 `.code` 文件按文件名排序合并"（`code_api_reference.md`）

### 3.3 线程 vs 监听器 vs 局部监听器

| 特性 | `CreateThread` | `CreateListener` | `CreateListenerLocal` |
|------|----------------|------------------|------------------------|
| 父作用域 | 调用处作用域 | 标签**定义处**作用域 | 函数**调用处**作用域 |
| 生命周期 | 线程对象存活则上下文存活 | 定义处作用域存活则存活 | 调用处作用域存活则存活 |
| 执行频率 | 创建时执行一次，之后由 `Goto` 触发 | 每帧/每秒执行 | 每帧/每秒执行 |
| 返回值 | `_result` | `_result`（每帧都返回） | `_result` |

> **官方文档**："CreateListener creates the listener within the scope where the code is defined. CreateListenerLocal creates the listener within the scope where you called the function."（`documentation.html`）

### 3.4 变量作用域

- 函数和线程创建自己的局部作用域。
- 子作用域可以访问父作用域变量。
- 同名参数会遮蔽父作用域变量。
- 多个线程使用同一标签时，各自拥有变量实例，但共享父作用域变量。
- `_this` 是当前线程引用，在标签内可用。

示例：

```code
name = "Player"
function1(id=1)
function1(id=2)

function1:
    sqr = id*id
    Log(name + id + ": sqr=" + sqr)
# 输出：Player1: sqr=1 和 Player2: sqr=4
```

### 3.5 `Goto` 与 `Return` 的区别

| 特性 | `Goto` | `Return` / 函数调用 |
|------|--------|---------------------|
| 目的 | 线程状态切换 | 子程序调用/返回 |
| 目标作用域 | 替换当前线程上下文 | 创建新子作用域 |
| 是否返回调用者 | 否 | 是，返回 `_result` |
| 典型用途 | 任务阶段推进 | 辅助函数、计算 |

### 3.6 多文件合并

所有 `.code` 文件按文件名排序合并，标签在所有文件间共享命名空间。这意味着：

- 跨文件标签名必须唯一（或有意覆盖）。
- 线程由每个 `CreateThread` 调用独立创建，与文件无关。
- 编辑器的 `collect_labels` 扫描整个图并输出顶层 `CreateThread`，但这种做法会为所有非监听器标签生成顶层线程，与手写模组的实际习惯不符。


---

## 4. 当前架构错配分析

### 4.1 `Start` 节点：编辑器专用概念，无 `.code` 对应物

- `.code` 中没有 `Start` 关键字。`main:` 只是一个普通标签。
- 当前 `NodeType::Start` 被当作唯一入口点，但 `.code` 有多个入口点（`main`、监听器标签、回调标签、`Goto` 目标）。
- 验证器警告多 `Start` 节点，但多标签是 `.code` 的正常现象。

### 4.2 `Label` 节点：双重表示

标签在图中同时通过三种方式表示：

1. `Label` 节点（Control 类型，带 `name` 参数）。
2. `graph.labels: HashMap<String, Vec<String>>`。
3. `Goto` / `CreateThread` / `CreateListener` 参数中的字符串。

这三种表示可能不一致。`Label` 节点在代码生成中被当作 no-op 贯通（`src/code_gen/generator.rs`），说明它没有任何运行时语义，只是视觉锚点。

### 4.3 `Flow` 边：混淆了两种关系

当前 `PortType::Flow` 被用于：

1. **标签内顺序执行**（`Log` -> `SetStage`）。
2. **标签间引用**（`CreateThread` -> `Label`，`Goto` -> `Label`）。

但 `.code` 中：

- 标签内是语句序列，不是数据流。
- 标签间是名称引用，不需要边。
- `Goto` 创建循环；监听器回调可以回到任何标签；Flow 边的 DAG 约束和菱形警告因此是错误的。

### 4.4 线程/监听器无显式容器

当前图中没有线程或监听器的视觉容器。`CreateThread` 只是返回对象端口的节点，与标签体之间只通过字符串参数关联。这导致：

- 无法直观区分线程体、监听器体和普通函数体。
- 代码生成器必须用 BFS 可达性启发式来推断监听器子标签（`compute_child_labels`），这是脆弱且不可靠的。
- `WaitForThread` / `DestroyListener` 被归类为 `Objects`，但它们是生命周期/同步操作。

### 4.5 节点分类按 API 表面而非语言概念

| 问题 | 示例 |
|------|------|
| 全局变量被当作函数 | `GetSave`/`GetTime`/`GetSettings` 在 `General Functions` |
| 状态读取被当作玩家函数 | `GetStateBool`/`GetStateNumber` 在 `Game Functions: Player` |
| 字面量被当作数学 | `Boolean`/`NumberConstant`/`StringConstant` 在 `Math` |
| 逻辑运算被当作数学 | `LogicAnd`/`LogicOr`/`LogicNot` 在 `Math` |
| 线程/监听被当作对象 | `CreateThread`/`CreateListener`/`WaitForThread`/`DestroyListener` 在 `Objects` |
| 条件查询被当作数学/物品 | `CheckCondition` 在 `Math`；`CheckEquipment`/`CheckCosplay` 在 `Game Functions: Items` |

### 4.6 代码生成器的语义偏差

- 为每个非监听器标签生成顶层 `CreateThread`，导致 `tests/fixtures/example_npc_type.code` 这种双重创建模式。
- `Goto` 生成 `thread.Goto(...)`，但从不定义 `thread` 变量。
- 用 Flow 边可达性推断标签归属，而不是显式容器。
- `ForeachNode` 被当作 Flow 节点，但 `.code` 中 `Foreach(list, thread)` 是函数调用。


---

## 5. 目标架构设计

### 5.1 核心原则

1. **图结构 = `.code` 语法树的可视化表示**。
2. **标签是最小组织单元**；`Start`/`Label` 不作为节点类型，而是作为容器或入口钉。
3. **线程/监听器是显式容器**，拥有内部标签和流图。
4. **`Flow` 边仅表示标签内语句顺序**，不用于标签间引用。
5. **`Data` 边表示表达式、变量引用、参数传递**。
6. **节点分类围绕 `.code` 语言概念**，而非 API 表面。

### 5.2 视觉语法建议

参考 UE Blueprints、Unity Bolt、RPG Maker 等主流编辑器，建议的视觉语法：

| 元素 | 符号 | 说明 |
|------|------|------|
| **线程（Thread）** | 圆角矩形/面板 | 并发作用域，包含多个标签和监听器 |
| **标签（Label）** | 左侧入口钉 | 命名入口点，可双击展开内部流图 |
| **监听器（Listener）** | 带循环图标的入口钉 | 每帧/每秒触发 |
| **Wait/Delay** | 带时钟图标的节点 | 延迟输出，不阻塞其他线程 |
| **状态（State）** | 大状态节点 | 用于互斥模式（如 Idle/Combat） |
| **函数/子图** | 可折叠节点 | 同步辅助函数，可含输入/输出 |
| **Group/Comment** | 彩色区域 | 编辑器专用，无运行时语义 |

### 5.3 层级结构示例

```text
CM2Editer file (.code.json)
├── Thread: "Main"
│   ├── Label: "start"
│   │   └── [Log] -> [SetStage] -> [Return]
│   ├── Listener: "update"
│   │   └── [GetStateBool] -> [If] -> [Goto: "onTrigger"]
│   └── Label: "onTrigger"
│       └── [Wait 1.0] -> [Log]
├── Thread: "Audio"
│   └── Listener: "fade"
│       └── [GetTimeDiff] -> ...
└── State Machine: "GamePhase"
    ├── State: "Menu" -> Thread(s)
    ├── State: "Play" -> Thread(s)
    └── State: "GameOver" -> Thread(s)
```

### 5.4 新的节点分类建议

| 分类 | 成员 |
|------|------|
| **Threading & Concurrency** | `CreateThread`, `CreateListener`, `CreateListenerLocal`, `WaitForThread`, `Wait`, `WaitForEvent`, `DestroyListener` |
| **Control Flow** | `If`, `While`, `For`, `Break`, `Return`, `Goto` |
| **Variables & Globals** | `Global`, `Local`, `GetSave`, `GetTime`, `GetTimeDiff`, `GetSettings`, `GetMod`, `GetMods`, `GetCurrentThread`, `GetStateBool`, `GetStateNumber` |
| **Literals** | `Boolean`, `NumberConstant`, `StringConstant`, `Color` |
| **Math & Logic** | 算术、三角、向量运算；`LogicAnd`, `LogicOr`, `LogicNot` |
| **Conditions & Queries** | `CompareNumbers`, `CheckCondition`, `CheckEquipment`, `CheckCosplay` |
| **Game API** | `DropItem`, `SetStage`, `SetPlayerPosition`, `PlaySoundEffect`, `SetEcstasy` 等 |
| **Objects** | `CreateArea`, `CreateNPC`, `CreateGallery`, `CreateMessengerChat`, `CreateCondition` 等 |
| **String / File / List** | 字符串操作、文件操作、列表操作 |
| **Editor-only** | `Meta`, `Comment`, `Group` |

### 5.5 代码生成方式变化

- 顶层只生成用户明确指定的 `CreateThread`（或默认 `main` 线程）。
- 每个线程容器生成其标签定义和内部流图。
- 监听器在所属线程内生成 `CreateListener` / `CreateListenerLocal`。
- 标签间引用（`Goto`、`CreateThread` 目标）通过名称解析，不依赖 `Flow` 边。
- `Flow` 边仅在标签/线程内部表示顺序。


---

## 6. 迁移路线图

### 6.1 短期（兼容补丁）

1. **修复 `Start`/`Label` 兜底**：确保新建工程时 `main` 标签被正确写入 `graph.labels`；`collect_labels` 在 `labels` 为空时自动从 `Start` 节点推断 `main`（已在 2026-07-13 提交中部分实现）。
2. **修正代码生成**：`Goto` 默认使用 `_this` 或显式线程输入；`CreateThread` 不再为每个标签自动生成顶层线程。
3. **修正节点分类**：把全局变量、状态读取、逻辑运算、线程/监听器重新归类到语言概念类别。
4. **修正教程**：继续更新 `tutorial_make_code.md`，明确 `Start` 只是视觉入口，`Label` 是标签体入口。

### 6.2 中期（容器化重构）

1. **引入 `ThreadContainer` 和 `LabelContainer`**：
   - `ThreadContainer` 拥有线程变量名、一组 `LabelContainer`、可选的监听器。
   - `LabelContainer` 拥有名称、参数签名、内部节点和 `Flow` 边。
2. **移除 `Start` 和 `Label` 作为 `NodeType`**：
   - 用容器属性表示入口和标签名。
   - 为向后兼容，保留 `Start`/`Label` 的反序列化映射。
3. **限制 `Flow` 边语义**：
   - `Flow` 边仅在容器内部表示顺序。
   - 标签间引用改为名称或 `Data` 引用（如 `Goto` 的参数）。
4. **重构验证器**：移除 `Flow` DAG 约束和菱形警告，改为检查标签名唯一性、作用域一致性、未使用的标签等。

### 6.3 长期（全新图模型）

1. **JSON Schema 2.0**：
   - 顶层结构从 `nodes + edges + labels` 改为 `threads: [ThreadContainer]`。
   - 每个 `ThreadContainer` 包含 `name`, `labels: [LabelContainer]`, `listeners: [ListenerContainer]`。
   - `LabelContainer` 包含 `name`, `params`, `nodes`, `edges`。
2. **UI 层级编辑器**：
   - 左侧工程树显示线程/标签层级。
   - 画布显示当前选中标签的内部流图。
   - 提供线程概览图（状态机视图）。
3. **迁移脚本**：将旧 `nodes + edges + labels` 自动迁移到新容器结构。


---

## 7. 风险与决策清单

| 风险 | 说明 | 缓解措施 | 状态 |
|------|------|----------|------|
| 向后兼容 | 旧 `.code.json` 文件和节点类型需要迁移 | 提供版本迁移和反序列化兼容层 | 待决策 |
| 用户学习成本 | 新容器概念比当前流程图更复杂 | 提供清晰的教程和可视化区分 | 待决策 |
| 状态机与线程边界 | `.code` 没有显式状态机，需要决定是否引入 | 可选 `State Machine` 容器，不强制 | 待决策 |
| 多文件标签冲突 | 合并后标签名冲突仍可能 | 编辑器提供跨文件标签检查和命名空间提示 | 待决策 |
| `Foreach` 语义 | `.code` 中 `Foreach` 是函数调用，不是循环节点 | 新节点分类中明确为函数/对象操作 | 待决策 |
| Listener 频率 | 官方文档与中文参考对频率描述不一致 | 以官方英文文档为准（every frame） | 待确认 |
| 自定义变量 | 当前没有通用的变量赋值节点 | 引入 `Set Variable` / `Variable` 数据节点 | 待决策 |


---

## 8. 结论与建议

CM2Editer 的当前架构是围绕 **"流程图转代码块"** 的直觉构建的，而 `.code` 的真实语义是 **"类 Python 的层级脚本 + 标签驱动的并发状态机"**。这导致 `Start` 节点、`Label` 节点、`Flow` 边、以及 API 表面分类等核心概念与 `.code` 语法错配。

建议的 redesign 方向：

- 以 **线程/标签容器** 为骨架。
- 用 **入口钉** 替代 `Start`/`Label` 节点。
- 用 **`Flow` 边** 仅表示标签内顺序。
- 用 **`Data` 边** 表示表达式和变量引用。
- 重新组织节点分类，围绕 `.code` 语言概念。

**行动建议**：

1. 短期先修补兼容性问题（如已做的 `main` 标签兜底）。
2. 中期引入 `ThreadContainer`/`LabelContainer` 容器化图模型。
3. 长期实现以 `.code` 语法结构为中心的层级编辑器。


---

## 附录 A: 术语表

| 术语 | `.code` 含义 | 当前编辑器中的问题 |
|------|-------------|-------------------|
| **Label（标签）** | 命名代码块，可作为函数/线程入口/监听器回调 | 被 `Label` 节点 + `graph.labels` + 字符串参数三重表示 |
| **Thread（线程）** | `CreateThread` 创建的运行时对象，执行标签体 | 没有视觉容器，与标签体只有字符串关联 |
| **Listener（监听器）** | 每帧/每秒调用标签的循环 | 没有视觉容器，子标签归属靠 BFS 启发式推断 |
| **ListenerLocal（局部监听器）** | 绑定到调用处作用域的监听器 | 与 `CreateListener` 的作用域差异无法在图中表达 |
| **`_this`** | 当前线程引用 | 只能通过 `GetCurrentThread` 节点获取，无自动作用域绑定 |
| **`_result`** | 标签返回值 | 由 `Return` 节点生成，但每个标签末尾自动追加 `_result = null` 是噪音 |
| **Flow 边** | 当前编辑器中的控制流边 | 被错误用于标签间引用，与 `.code` 顺序执行语义不一致 |
| **Data 边** | 当前编辑器中的数据/表达式边 | 正确表达 `.code` 表达式和参数引用，但缺少变量节点支持 |
| **Top-level（顶层）** | `.code` 模块加载时执行的语句 | 编辑器没有顶层语句区域，所有节点都在 `main` 标签中 |


---

## 附录 B: 关键代码引用

| 代码位置 | 说明 |
|----------|------|
| `src/graph/types.rs:12` | `NodeType::Start` 定义 |
| `src/graph/types.rs:14` | `NodeType::Label` 定义 |
| `src/graph/types.rs:381` | `PortType::Flow` 定义 |
| `src/graph/graph.rs:16` | `Graph.labels: HashMap<String, Vec<String>>` |
| `src/code_gen/generator.rs:102-109` | 为每个非监听器标签生成顶层 `CreateThread` |
| `src/code_gen/generator.rs:167-168` | `Start`/`Label` 作为 no-op 贯通 |
| `src/code_gen/generator.rs:175` | `Goto` 生成 `thread.Goto(...)` |
| `src/code_gen/generator.rs:212-219` | `ForeachNode` 作为函数调用生成 |
| `src/code_gen/generator.rs:779-851` | `compute_child_labels` BFS 启发式 |
| `src/api/definitions.rs:317-324` | `Start` 节点定义 |
| `src/api/definitions.rs:325-334` | `Label` 节点定义 |
| `src/api/definitions.rs:1913-1963` | `CreateThread`/`CreateListener`/`CreateListenerLocal` 定义 |
| `src/api/definitions.rs:1964-1972` | `DestroyListener` 定义（归类在 Objects） |
| `src/api/definitions.rs:1982-1990` | `WaitForThread` 定义（归类在 Objects） |
| `src/graph/validation.rs:142-201` | Flow 边 DAG 检测 |
| `src/graph/validation.rs:222-239` | 多 Start 节点警告 |
| `src/graph/validation.rs:331-363` | 菱形可达性警告 |
| `src/project.rs:478-498` | 新建工程默认图创建 |


---

## 附录 C: 参考来源

1. **Custom Missions 2 官方文档**
   - `docs/code_api_reference.md`
   - `docs/documentation.html`
   - `docs/documentation_zh.html`

2. **当前项目代码**
   - `src/graph/*.rs`
   - `src/code_gen/generator.rs`
   - `src/api/definitions.rs`
   - `src/project.rs`
   - `tests/fixtures/*.code`

3. **外部编辑器参考**
   - Unreal Engine 5 Blueprints 文档
   - Unity Bolt / Visual Scripting 文档
   - Godot VisualScript 文档
   - RPG Maker MV/MZ 事件系统帮助
   - Warcraft III World Editor GUI 触发器教程
   - Yarn Spinner / Twine 文档

---

*本文档为架构 redesign 的讨论基础，具体实现需进一步细化数据结构和 UI 交互。*
