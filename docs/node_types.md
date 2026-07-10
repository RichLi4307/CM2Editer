# CM2Editer 节点手册（v0.2.1 实际实现）

> 本文档描述 CM2Editer **实际支持的** 159 个节点类型及其用法。
> 代码生成语法以 `docs/code_api_reference.md` 为准。

---

## 代码生成兼容性 ⚠️ 必读

新增/修改节点时必须对照下表。节点分为三类：

### A 类：自定义代码生成（`generate_sequence` 显式 match 臂）

| 节点 | 代码生成 |
|------|---------|
| `Start`, `Label`, `Comment`, `Meta`, `Group` | 贯通：`follow_flow` 继续 Flow 链 |
| `Goto` | `thread.Goto(label)`，Data 端口需手动生成 var 赋值 |
| `If` | `generate_if()`：分支 Flow + Data condition |
| `While` | `generate_while()`：循环 Flow |
| `For` | `generate_for()`：`for i in list` |
| `Break` | `break` |
| `Return` | `_result = {value}` |
| `CallFunction` | `funcName(args)`，Data 端口需手动生成 |
| `ForeachNode` | `var = Foreach(list, thread)` |

**规则**：向 A 类节点新增 Data 输出端口，必须在 match 臂中手动写 `var_{id}_{port} = ...`。参照 Goto `out_label` 实现。

⚠️ **同时**：A 类 Flow 节点的 Data 输出端口值需被其他节点通过 Data 边引用时，还必须在 `evaluate_data_output` 中添加对应分支，将端口名映射到实际参数值。否则回退到 `var_{id}_{port}`（变量名引用而非变量值）。参见 `Goto.out_label` 和 `CreateListener.out_name` 的实现（`generator.rs:458-473`）。

### B 类：通用代码生成（`generate_node_call`）

| 节点 | 代码生成 |
|------|---------|
| `CreateThread`, `CreateListener`, `CreateListenerLocal` | 显式匹配到 `generate_node_call` |
| 其他所有带 Flow 端口的节点 | `_` 默认走 `generate_node_call` |

`generate_node_call` 自动为**所有 Data 输出端口**生成 `var_{id}_{port} = Func(params)`。**新增 Data 输出端口无需改代码生成器**。

### C 类：纯 Data 节点（无 Flow 端口）

| 节点 |
|------|
| `Boolean`, `NumberConstant`, `StringConstant`, `GetStateBool`, `GetStateNumber`, `CompareNumbers`, `LogicAnd`, `LogicOr`, `LogicNot`, `CheckCondition`, `CheckEquipment`, `CheckCosplay`, `GetPosition`, `MakeVector`, `BreakVector` |

处理位置：`evaluate_data_output()`——仅在其他节点通过 Data 边引用时才被递归解析。**不在 Flow 链中遍历**。新增参数或输出端口需在 `evaluate_data_output` 中添加对应分支。

---

## 约定

| 术语 | 说明 |
|------|------|
| **Flow 端口** | 白色，控制执行顺序，跟随 Flow 边生成代码 |
| **Data 端口** | 彩色，按类型着色。传递 Number/String/Boolean/List/Object |
| **纯 Data 节点** | 无 Flow 端口，仅通过 Data 边给其他节点喂值。代码生成中不直接出现，通过 `evaluate_data_output()` 递归解析 |
| **🔗 连线优先级** | Data 端口有连线时，属性面板参数自动隐藏，值取连线源。断线后恢复 |

---

## 控制流（Flow）

| 节点 | Flow 入/出 | 参数 | `.code` 输出 |
|------|-----------|------|-------------|
| **Start** | 出 Flow | — | 标签入口（不产生代码） |
| **Goto** | 入 Flow | `label`(String), `args`(Object) | `thread.Goto("label")` |
| **If** | 入 Flow, 出 True/False | `condition`(Boolean) | `if {expr}` + `else` |
| **While** | 入 Flow, 出 Loop/Break | `condition`(Boolean) | `while {expr}` |
| **For** | 入 Flow, 出 Loop/Break | `iterable`(List) | `for i in {list}` |
| **Break** | 入 Flow | — | `break` |
| **Return** | 入 Flow | `value`(List) | `_result = {value}` |
| **CallFunction** | 入/出 Flow | `function`(String), `params`(Object) | `funcName(args)` |
| **Foreach** | 入/出 Flow | `list`(String), `threadVar`(String) | `var = Foreach(list, thread)` |

---

## 线程与监听器（Objects）

| 节点 | Flow 入/出 | 参数 | `.code` 输出 |
|------|-----------|------|-------------|
| **CreateThread** | 入/出 Flow | `labelName`(String), `params`(Object) | `var = CreateThread(labelName="x")` |
| **CreateListener** | 入/出 Flow | `labelName`(String), `params`(Object) | `var = CreateListener(labelName="x")` |
| **CreateListenerLocal** | 入/出 Flow | `labelName`(String), `params`(Object) | `var = CreateListenerLocal(labelName="x")` |

> `params` 用于传递额外参数给标签，如 `duration=3.0`。

---

## 条件系统（Condition Objects）

| 节点 | Data 入/出 | 参数 | `.code` 输出 |
|------|-----------|------|-------------|
| **CreateCondition** | 入 Flow, 出 Object | `condition`(String), `id`(String) | `var = CreateCondition(condition="[Exposed_All]", id="x")` |
| **CreateItemCondition** | 入 Flow, 出 Object | `itemtype`(Enum) | 同上 |

→ `Condition` Object 不能直接接 `If`。需通过 **CheckCondition** 转为 Boolean。

---

## Boolean 管道（Phase 6 Data 节点）

> 这些节点**无 Flow 端口**，仅通过 Data 边连入 If/While。

| 节点 | Data 入 | Data 出 | 参数 | `.code` 输出 |
|------|---------|---------|------|-------------|
| **Boolean** | — | `out_value: Boolean` | `value`(Enum: true/false) | `true` / `false` |
| **NumberConstant** | — | `out_value: Number` | `value`(Number) | `90` |
| **GetStateBool** | — | `out_value: Boolean` | `stateKey`(Enum:18项) | `_state.Futanari` |
| **GetStateNumber** | — | `out_value: Number` | `stateKey`(Enum:8项) | `_state.Ecstasy` |
| **CompareNumbers** | `a:Number`, `b:Number` | `out_result: Boolean` | `a`(Number), `b`(Number), `operator`(Enum:>=/==/!=/>/</<=) | `_state.Ecstasy >= 90` |
| **LogicAnd** | `a:Boolean`, `b:Boolean` | `out_result: Boolean` | — | `({a}) && ({b})` |
| **LogicOr** | `a:Boolean`, `b:Boolean` | `out_result: Boolean` | — | `({a}) \|\| ({b})` |
| **LogicNot** | `a:Boolean` | `out_result: Boolean` | — | `!({a})` |
| **CheckCondition** | `cond:Object` | `out_result: Boolean` | — | `{cond}.Check()` |
| **CheckEquipment** | — | `out_value: Boolean` | `equipType`(Enum:10项) | `_state.AdultToys.{type} != null` |
| **CheckCosplay** | — | `out_value: Boolean` | `cosplayKey`(String→命名空间) | `Cosplay_{key}` |

### 数据节点使用模式

```text
[GetStateNumber(Ecstasy)] ──Data──→ [CompareNumbers.a]
[NumberConstant(90)]     ──Data──→ [CompareNumbers.b]
                                        │ Boolean
                                        ▼
                                    [If.condition]
→ 生成: if _state.Ecstasy >= 90
```

---

## 坐标系统（Phase 7）

| 节点 | Data 入 | Data 出 | 参数 |
|------|---------|---------|------|
| **GetPosition** | — | `out_position:List`, `out_stage:String` | `coord_id`, `stage`(Enum), `x`,`y`,`z`(Number) |
| **MakeVector** | `x:Number`, `y:Number`, `z:Number` | `out_vec:List` | `x`,`y`,`z`(Number) |
| **BreakVector** | `in_vec:List` | `x:Number`, `y:Number`, `z:Number` | — |

---

## If 条件模板（属性面板快速填充）

If/While 的 `condition` 参数编辑区提供 ComboBox 30+ 预设模板：

| 类别 | 示例 |
|------|------|
| 字面量 | `true`, `false` |
| 角色状态 | `_state.Futanari`, `_state.Sitting`, `_state.Orgasm`... |
| 环境 | `_state.InLight`, `_state.NearNPC`, `_state.IsDayTime`... |
| 装备/拘束 | `_state.Blindfolded`, `_state.Invisible`, `_state.AdultToys.Handcuff != null`... |
| 数值比较 | `_state.Ecstasy >=`, `_state.Detection >=`, `_state.Stamina >=`... |

> Data 端口连线后模板自动隐藏，只显示 🔗 源引用。

---

## 标签管理（左栏工程标签下）

- 显示所有 `graph.labels` 条目（标签名 + 节点数）
- 点击标签名 → 画布选中对应节点
- `+ 新建标签` 按钮创建空标签
- Goto / CreateThread 的参数更改时自动注册目标标签
- 保存/生成时，所有标签生成顶层 `CreateThread` 和 `label:` 体

---

## 代码生成规则（速查）

| 项目 | 输出 |
|------|------|
| 顶层 | `var_main_thread = CreateThread("main")` |
| 标签体 | `main:` → Tab 缩进 → Flow 序列 → `_result = null` |
| Data 解析 | `evaluate_data_output()` 递归追踪 Data 边链 |
| If 条件 | `if _state.Futanari`（无括号，小写 if） |
| Goto | `thread.Goto("step1")` |
| 标签收尾 | 每个标签体末尾自动追加 `_result = null`（Return 已有时跳过） |
