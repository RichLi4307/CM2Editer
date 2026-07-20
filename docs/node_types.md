# CM2Editer 节点手册（新架构 v0.3.0）

> 本文档描述新架构下 CM2Editer 支持的节点类型。
> 新架构以 `.code` 语法结构为中心，节点按 **语言概念** 分类，而非 API 表面。
> 旧版已归档：`docs/archive/node_types_20260713_v1.md`
> 完整参数定义参见 `src/api/definitions.rs`。

---

## 1. 代码生成兼容性（必读）

新增/修改节点时必须先读本文档。节点按代码生成方式分为三类：

### A 类：自定义代码生成（`generate_sequence` 显式 match）

这些节点涉及 `.code` 控制结构或特殊语法，需要手写代码生成逻辑：

| 节点 | 代码生成 |
|------|---------|
| `Goto` | `thread.Goto("label")` |
| `If` | `if {condition}` / `elseif {condition}` / `else` |
| `While` | `while {condition}` |
| `For` | `for i in {iterable}` / `for i in Range(start, stop, step?)` |
| `Break` | `break` |
| `Return` | `_result = {value}` |
| `CallFunction` | `funcName(args)` |
| `CallMethod` | `object.Method(args)` |
| `ForeachNode` | `var = Foreach(list, thread)` |
| `DestroyListener` | `listener = null` |
| `WaitForThread` | `{thread}.WaitForFinish()` |
| `Wait` | `Wait(seconds)` |
| `WaitForEvent` | `WaitForEvent(eventName)` |
| `Meta` / `Comment` / `Group` | 不产生代码，贯通 Flow 链 |

**规则**：向 A 类节点新增 Data 输出端口，必须：

1. 在 match 臂中手动写 `var_{id}_{port} = ...`。
2. 在 `evaluate_data_output` 中添加分支，将端口名映射到实际参数值。

### B 类：通用代码生成（`generate_node_call`）

所有带 Flow 端口、但不属于 A 类的节点。代码生成器自动调用对应 API 函数，并自动为 Data 输出端口生成 `var_{id}_{port} = Func(...)`。

**规则**：新增 Data 输出端口无需改代码生成器。

### C 类：纯 Data 节点（无 Flow 端口）

仅通过 Data 边给其他节点喂值，不在 Flow 链中遍历。处理位置：`evaluate_data_output()`。

**规则**：新增 C 类节点或输出端口需在 `evaluate_data_output` 中添加分支。

---

## 2. 节点分类（按 `.code` 语言概念）

### 2.1 Threading & Concurrency（线程与并发）

> 这些节点直接对应 `.code` 的线程和监听器原语。在新架构中，它们与 `ThreadContainer` / `ListenerContainer` 紧密配合。

| 节点 | 类别 | 说明 | `.code` 示例 |
|------|------|------|--------------|
| `CreateThread` | B | 创建并启动一个线程 | `var = CreateThread(labelName="x")` |
| `CreateListener` | B | 创建全局监听器 | `var = CreateListener(labelName="x")` |
| `CreateListenerLocal` | B | 创建局部监听器 | `var = CreateListenerLocal(labelName="x")` |
| `CreateEventListener` | B | 创建全局事件监听器（`SetEvent` 触发，标签内可用 `__eventdata_` / `__eventname_`） | `var = CreateEventListener("label", "event")` |
| `CreateEventListenerLocal` | B | 创建局部事件监听器（`SetEvent` 触发） | `var = CreateEventListenerLocal("label", "event")` |
| `DestroyListener` | A | 销毁监听器 | `listener = null` |
| `GetCurrentThread` | C | 获取当前线程引用 | `_this` |
| `WaitForThread` | A | 等待线程结束 | `{thread}.WaitForFinish()` |

### 2.2 Control Flow（控制流）

> 这些节点只在 `LabelContainer` 或 `ListenerContainer` 内部有效。`Flow` 边表示同一容器内的执行顺序。

| 节点 | 类别 | 说明 | `.code` 示例 |
|------|------|------|--------------|
| `Goto` | A | 跳转到同一线程的另一个标签 | `thread.Goto("step2")` |
| `If` | A | 条件分支（支持 `if` / `elseif` / `else`） | `if {cond} ... elseif {cond} ... else ...` |
| `While` | A | 循环 | `while {cond} ...` |
| `For` | A | 遍历列表/范围 | `for i in Range(0, 10) ...` |
| `Break` | A | 跳出循环 | `break` |
| `Return` | A | 设置返回值 | `_result = {value}` |
| `Wait` | A | 等待秒数 | `Wait(3.0)` |
| `WaitForEvent` | A | 等待事件触发 | `WaitForEvent("done")` |
| `Start` | — | ⚠️ 已弃用，新架构中不再作为 `NodeType` | 无 |
| `Label` | — | ⚠️ 已弃用，新架构中不再作为 `NodeType` | 无 |

### 2.3 Variables & Globals（变量与全局状态）

| 节点 | 类别 | 说明 | `.code` 示例 |
|------|------|------|--------------|
| `Global` | B | 读/写全局变量 | `Global("key", value)` |
| `Local` | B | 读/写局部变量 | 局部变量存取 |
| `GetSave` | C | 读取跨会话保存数据中的指定键 | `_save.SomeKey`（参数 `key`） |
| `GetTime` | C | 读取累计时间 | `_time` |
| `GetTimeDiff` | C | 读取上一帧时间差 | `_timediff` |
| `GetSettings` | C | 读取 meta 设置 | `_settings.Key` |
| `GetMod` | C | 读取 mod 共享数据 | `_mod.Key` |
| `GetMods` | C | 读取所有已激活 mod 数据 | `_mods` |
| `GetStageChanged` | C | 读取本帧是否发生场景切换 | `_stagechanged` |
| `GetProjectName` | C | 读取当前工程文件夹名 | `_name` |
| `SetVariable` | A | 设置当前作用域变量，支持 `=`/`+=`/`-=`/`*=`/`/=` | `i += 1` |
| `Variable` | C | 读取当前作用域变量 | `myVar` |
| `SetEvent` | B | 设置跨项目事件 | `SetEvent("name", data)` |
| `GetEvent` | C | 获取事件数据 | `_event` |
| `DumpVariables` | B | 导出所有变量 | `DumpVariables()` |
| `DumpVariable` | B | 导出单个变量 | `DumpVariable("name")` |
| `GetType` | C | 获取值类型名 | `type(value)` |
| `GetLanguage` | C | 获取当前语言 | `GetLanguage()` |
| `FunctionExists` | C | 检查函数是否存在 | `FunctionExists("name")` |
| `GetModVersion` | C | 获取 Mod 版本 | `GetModVersion()` / `GetModVersion("guid")` |

> **说明**：`SetVariable` / `Variable` 是新架构通用变量节点，已替代 `Global` / `Local` 的歧义用法。

### 2.4 Literals（字面量）

| 节点 | 类别 | 说明 | `.code` 示例 |
|------|------|------|--------------|
| `NumberConstant` | C | 数字常量 | `90` |
| `StringConstant` | C | 字符串常量 | `"hello"` |
| `Boolean` | C | 布尔常量 | `true` / `false` |
| `Color` | C/B | 颜色 `[r, g, b, a]` | `Color(1, 0, 0, 1)` |
| `Range` | C | 数值范围 | `Range(0, 10)` |

### 2.5 Math & Logic（数学与逻辑）

| 节点 | 类别 | 说明 |
|------|------|------|
| `Random` | C | 随机浮点数 |
| `RandomInt` | C | 随机整数 |
| `Sin`, `Cos`, `Tan`, `Asin`, `Acos`, `Atan` | C | 三角函数 |
| `Floor`, `Ceil`, `Round`, `Trunc`, `Sign`, `Abs` | C | 数值处理 |
| `LogN`, `Log2`, `Log10` | C | 对数 |
| `Min`, `Max` | C | 最值 |
| `Vector`, `Quaternion` | C | 构造向量/四元数 |
| `Vector3Length`, `Vector3SqrLength`, `Vector3Add`, `Vector3Sub`, `Vector3Scale`, `Vector3Dot`, `Vector3Cross`, `Vector3Rotate`, `Vector3Distance` | C | 向量运算 |
| `GetPosition` | C | 获取位置 |
| `MakeVector`, `BreakVector` | C | 向量打包/解包 |
| `CompareNumbers` | C | 数字比较 |
| `LogicAnd`, `LogicOr`, `LogicNot` | C | 布尔逻辑 |

### 2.6 Conditions & Queries（条件对象与状态查询）

> 这些节点生成布尔值，通常接入 `If` / `While` 的 `condition` Data 端口。

| 节点 | 类别 | 说明 | `.code` 示例 |
|------|------|------|--------------|
| `CheckCondition` | C | 检查 Condition 对象 | `{cond}.Check()` |
| `CheckEquipment` | C | 检查装备状态 | `_state.AdultToys.X != null` |
| `CheckCosplay` | C | 检查 cosplay 状态（多件用 `&&` 连接） | `Cosplay_{key}` / `(Cosplay_A && Cosplay_B)` |
| `GetStateBool` | C | 读取布尔状态 | `_state.Futanari` |
| `GetStateNumber` | C | 读取数值状态 | `_state.Ecstasy` |

### 2.7 String / File / List（字符串、文件与列表）

| 节点 | 类别 | 说明 |
|------|------|------|
| `Length`, `Lower`, `Upper`, `Find`, `SubString`, `Format`, `ToNumber` | C | 字符串处理 |
| `Translate` | A | 本地化字符串：`Translate(key[, args...])` 返回 String |
| `FileExists`, `GetFiles`, `GetFileExtension` | C | 文件操作 |
| `CreateList`, `Copy`, `CreateListFromJson` | C | 列表构造 |
| `ListInsert`, `ListRemove` | A | List 方法：插入 / 移除元素 |
| `ListCount`, `ListContains`, `ListIndexOf`, `ListKeys` | C | List 方法：计数 / 包含 / 索引 / 键集 |
| `ForeachNode` | A | 遍历列表（特殊函数调用）`var = Foreach(list, thread)` |

### 2.8 Game API（游戏功能）

> 这些节点对应 CM2 游戏 API，按子系统分组：

| 子系统 | 节点 |
|--------|------|
| 物品与装备 | `DropItem`, `CollectItem`, `SetVibrator`, `SetPiston`, `LockHandcuffs`, `UnlockHandcuffs`, `EquipCosplay`, `UnequipCosplay`, `UnequipAllCosplay`, `OwnCosplay`, `EquipAdultToy`, `UnequipAdultToy` |
| 玩家状态与设置 | `SetPlayerPosition`, `SetStage`, `SetCamera`, `SetAction`, `SetFutanari`, `SetSkill`, `SetPlayerData`, `SetSkillShortcut`, `GetSkillShortcut`, `GetRandomPosition` |
| 数值统计 | `AddCurrentEarnRP`, `SetCurrentEarnRP`, `GetCurrentEarnRP`, `AddCurrentRP`, `SetCurrentRP`, `GetCurrentRP`, `SetEcstasy`, `AddEcstasy`, `GetEcstasy`, `SetStamina`, `AddStamina`, `GetStamina`, `SetMoisture`, `AddMoisture`, `GetMoisture`, `TriggerSexOrgasm`, `SetItemCount`, `AddItemCount`, `GetItemCount` |
| 游戏控制 | `CanGameOver`, `TriggerGameOver`, `PlaySoundEffect`, `SetStageRankLimit`, `GetStageRankLimit`, `SetPortalEnabled`, `GetAllWaypoints`, `SetSexPosition`, `DeactivateSex`, `SetSexMenu` |
| 图形与杂项 | `ShowBlackscreen`, `GetSnapshotData`, `GetAllSnapshots`, `DeleteSnapshot`, `GetImageReference`, `SetGraphicsOption`, `GetGraphicsOption` |
| 音频 | `StopAudio` |
| 日志 | `Log`（支持 `Warning` / `Error` 级别） |

### 2.9 Objects（对象构造与方法）

> 这些节点创建或调用 `.code` 对象。

| 节点 | 类别 | 说明 |
|------|------|------|
| `CreateMissionPanel` | B | 创建任务面板 |
| `CreateMissionMenuItem` | B | 创建任务菜单项 |
| `CreateArea` | B | 创建区域（支持 sphere / cylinder / cuboid） |
| `CreateZone` | B | 创建区域（旧版） |
| `CreateCondition` | B | 创建条件对象 |
| `CreateItemCondition` | B | 创建物品条件对象 |
| `CreateInteractArea` | B | 创建交互区域 |
| `CreateText` | B | 创建文本对象 |
| `CreateMessengerChat` | B | 创建聊天对象 |
| `CreateAudio` | B | 创建音频对象 |
| `CreateGallery` | B | 创建画廊对象 |
| `CreateSnapshot` | B | 创建 snapshot 对象 |
| `CreateNPC` | B | 创建 NPC 对象 |
| `CreateInput` | B | 创建输入对象 |
| `NPCWarp`, `NPCAddWaypoint` | A | NPC 方法：传送 / 添加路径点 |
| `NPCIsAlive`, `NPCSeesPlayer`, `NPCSeesFlashing` | C | NPC 方法：存活 / 看到玩家 / 看到裸露 |
| `CallFunction` | A | 动态调用函数 |
| `CallMethod` | A | 动态调用对象方法 |
| `GetCurrentThread` | C | 获取当前线程对象（也归类在 Threading） |

### 2.10 Editor-only（编辑器专用）

> 这些节点在 `.code` 中没有对应物，只用于辅助编辑器组织。

| 节点 | 类别 | 说明 |
|------|------|------|
| `Meta` | A | 元数据注释，不产生代码 |
| `Comment` | A | 注释节点，不产生代码 |
| `Group` | A | 视觉分组，不产生代码 |

---

## 3. 作用域与生命周期

| 概念 | 说明 |
|------|------|
| `_this` | 当前线程引用。在新架构中，应自然绑定到当前 `ThreadContainer` |
| `_result` | 标签返回值。由 `Return` 节点显式设置；容器结束时默认 `null` |
| 线程变量 | `CreateThread` 等返回值应绑定到 `ThreadContainer.variable_name` |
| 监听器变量 | 同线程变量，但属于 `ListenerContainer` |
| 全局变量 | `Global` / `GetSave` / `GetSettings` / `GetMod` / `GetMods` 等跨作用域访问 |
| 局部变量 | `Local` 节点仅在当前线程作用域内有效；新架构将用 `Set Variable` / `Variable` 替代 |

---

## 4. 已弃用节点

| 节点 | 状态 | 说明 |
|------|------|------|
| `Start` | 已弃用 | 新架构中由 `ThreadContainer` / `LabelContainer` 入口钉替代。保留反序列化兼容。 |
| `Label` | 已弃用 | 新架构中由 `LabelContainer` 名称替代。保留反序列化兼容。 |

---

## 5. 特殊参数说明

### 5.1 `CreateCondition.condition` 组合语法

`CreateCondition` 的 `condition` 参数支持组合表达式，游戏加载器按以下规则解析：

| 语法 | 含义 | 示例 |
|------|------|------|
| `[A, B]` | AND：A 与 B 同时成立 | `[Exposed_All, IsDayTime]` |
| `(A, B)` | OR：A 或 B 任一成立 | `(Crouching, Sitting)` |
| `!A` | NOT：A 不成立 | `!Futanari` |
| `SubCondition_<id>` | 复用本标签中已注册的子条件 | `SubCondition_main` |

在编辑器属性面板中点击 **编辑条件...** 即可打开弹窗：

- 文本框可直接编辑组合表达式；
- `AND [ ]` / `OR ( )` / `NOT !` 按钮一键插入逻辑模板；
- 点击下方基础条件标签自动追加到表达式；
- 已有条件 ID 列表显示当前标签内所有 `CreateCondition` / `CreateItemCondition` 节点的非空 `id`，点击生成 `SubCondition_<id>` 复用。

`CreateCondition.id` 与 `CreateItemCondition.id` 同时支持属性面板输入和数据流输入。当左侧 `id` 数据端口有连接时，代码生成器优先使用连接的变量值；无连接时使用属性面板中的常量值。留空表示不注册子条件。

### 5.2 节点库场景分类

节点库的左侧分类不再按 `NodeDefinition::category`（API 原始分类）组织，而是按开发者实际使用场景分类。分类依据：

1. 节点在 `.code` 中的实际生成结果（`src/code_gen/generator.rs`）。
2. 官方 API 文档（`docs/kb/documentation_part_*.md` / `docs/documentation.html`）。
3. 节点在游戏流程中的实际作用（控制流、条件判断、数据查询、状态修改、视觉/UI、数据处理）。

因此，部分节点可能同时出现在多个场景中；少数节点会根据实际语义从错误场景中移出。例如：

- `CreateCondition` / `CreateItemCondition` 是条件对象构造器，归入 **条件判定 > 状态检查**，而非视觉/UI。
- `Log` 是调试日志，归入 **编辑器专用**，而非音效与屏幕。
- `SetCamera` / `GetImageReference` / `GetAllSnapshots` / `DeleteSnapshot` 是视觉资产/显示控制，归入 **视觉 / UI > 视觉元素**。
- `TriggerGameOver` 是流程控制，归入 **任务 / 流程 > 流程控制**。
- `GetItemCount` 是库存查询，归入 **数据获取 > 物品 / 装备**。
- `CreateInteractArea` 是交互检测，归入 **视觉 / UI > 输入与交互**。
- `FileExists` / `GetFiles` 是文件系统操作，归入 **数据处理 > 文件**，而非字符串处理。

具体分类见 `src/ui/panels/node_library/catalog.rs`。

### 5.3 节点详细介绍文档

`docs/node_details.md` 为每个节点提供了中文详细介绍，包含：

- 中文名与官方 API 签名
- 返回值类型
- 实际作用说明
- 每个参数的必填/类型/含义
- `.code` 使用案例
- 常见使用场景
- 相关节点推荐

该文档由子代理基于 `src/api/definitions.rs`、`src/code_gen/generator.rs` 和官方 API 文档（`docs/kb/` / `docs/documentation.html`）生成，旨在比官方文档更结构化、更本地化，并更适合编辑器属性面板参考。属性面板中的节点简介未来将逐步抽取自该文档的关键内容。
