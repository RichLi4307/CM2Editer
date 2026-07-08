# If 条件 / 布尔值输出节点设计方案

## 背景

**If 节点** 的 `condition` 参数类型为 `ParamType::Boolean`（→ `PortType::Boolean` Data 端口），这是目前唯一的 Boolean Data 入口。

**问题**：项目中没有任何节点输出 `PortType::Boolean`。If 的 condition 只能手写字符串表达式（如 `_state.Futanari`、`true`），DataFlow 端口形同虚设。

**目标**：提供"预制菜"式条件构建——用户可以从下拉列表中选取常用条件模板，也可以通过 DataFlow 复用布尔值。

---

## 方案概要

### 方案 A：条件模板下拉框（优先实现）

在 If 节点的 **属性面板**（`PropertiesPanel`）中，针对 `condition` 参数显示一个 `ComboBox`，提供 30+ 条预设 `.code` 表达式。用户选择后自动填入条件字段。

**模板列表**（`src/ui/panels/properties.rs` 中定义）：

| 标签 | 生成的值 |
|------|---------|
| `✅ true` | `true` |
| `❌ false` | `false` |
| `_state.Futanari` | `_state.Futanari` |
| `_state.Sitting` | `_state.Sitting` |
| `_state.Orgasm` | `_state.Orgasm` |
| `_state.Moving` | `_state.Moving` |
| `_state.Crouching` | `_state.Crouching` |
| `_state.Peeing` | `_state.Peeing` |
| `_state.Dashing` | `_state.Dashing` |
| `_state.InLight` | `_state.InLight` |
| `_state.NearNPC` | `_state.NearNPC` |
| `_state.Watched` | `_state.Watched` |
| `_state.ShowingOff` | `_state.ShowingOff` |
| `_state.Bukkake` | `_state.Bukkake` |
| `_state.Blindfolded` | `_state.Blindfolded` |
| `_state.Invisible` | `_state.Invisible` |
| `_state.InOpenToilet` | `_state.InOpenToilet` |
| `_state.Bodypaint` | `_state.Bodypaint` |
| `_state.FPCamera` | `_state.FPCamera` |
| `_state.IsDayTime` | `_state.IsDayTime` |
| `_state.GameOver` | `_state.GameOver` |
| `无手铐` | `Handcuffs.Type == nil` |
| `手铐=普通手铐` | `Handcuffs.Type == \"Handcuff\"` |
| `手铐=钥匙手铐` | `Handcuffs.Type == \"KeyHandcuff\"` |
| `手铐=计时手铐` | `Handcuffs.Type == \"TimerHandcuff\"` |
| `跳蛋=关闭` | `Vibrator == \"Off\"` |
| `跳蛋=低档` | `Vibrator == \"Low\"` |
| `跳蛋=高档` | `Vibrator == \"High\"` |
| `活塞=关闭` | `Piston == \"Off\"` |
| `活塞=开启` | `Piston == \"Low\"` |
| `快感 >=` | `_state.Ecstasy >= ` |
| `侦测 >=` | `_state.Detection >= ` |
| `体力 >=` | `_state.Stamina >= ` |
| `湿润度 >=` | `_state.Moisture >= ` |
| `心率 >=` | `_state.HeartRate >= ` |
| `等级 >=` | `_state.Rank >= ` |

**交互**：
- 模板只是**快速填充**，用户选择后仍可在文本框直接编辑。
- 当 condition Data 端口被连线时（🔗），模板不再显示——Data 优先。
- 不影响现有 DataFlow 机制。

### 方案 B：Boolean 常量节点（后续）

新增 `NodeType::Boolean` 节点：

```rust
NodeDefinition::new(NodeType::Boolean, "Math", "布尔值", "输出布尔常量", MATH_COLOR)
    .with_outputs(vec![out_data("out_value", PortType::Boolean, "布尔值")])
    .with_params(vec![e("value", "值", &["true", "false"])])
```

- 无 Flow 端口，仅做纯数据输出。
- 通过 DataFlow 连入 If 的 condition 端口。
- 代码生成时会被 If 引用为 `true` / `false` 字面量。

后续可扩展：`GetStateBoolean`（读 _state）、`CompareNumbers`（数值比较）等节点，均输出 Boolean。

---

## 实现步骤

### Step 1：If 条件模板

**文件**：`src/ui/panels/properties.rs`

- 定义 `IF_CONDITION_TEMPLATES: &[(&str, &str)]` 常量。
- 在 `param_editor` 中，当 `node.node_type == NodeType::If && key == "condition"` 时：
  1. 若 Data 端口已连线 → 跳过（显示 🔗）
  2. 渲染 `ComboBox`（`id_salt("if_condition_template")`）
  3. 用户选中模板 → 发出 `(key, Literal(json!(expression)))` 参数修改
  4. 下方保留文本框，支持手写

### Step 2：Boolean 节点

**文件**：`src/graph/types.rs` → 添加 `Boolean` 到 `NodeType` 枚举
**文件**：`src/api/definitions.rs` → 添加 Boolean 节点定义
**文件**：`src/code_gen/generator.rs` → 无 Flow 节点不参与代码生成路径，仅在 Data 引用时被动取值
**文件**：`src/api/registry.rs` → 自动注册（`all_node_definitions` 已覆盖）
**测试**：`test_all_variants_have_definition` 测试中 variant 数量需 +1

### Step 3：验证

- `cargo test` 全部通过。
- 手动测试：拖入 If 节点 → 属性面板出现条件下拉 → 选模板填充 → 手改生效。
- 拖入 Boolean 节点 → 输出端口为 Boolean → 连线到 If condition → 代码生成 `If(true)` / `If(false)`。

---

## If 条件语法参考

`.code` 中 `If(...)` 表达式支持的语法（摘录自 `docs/documentation_zh.html`）：

### 布尔状态变量
`_state.{变量名}` — 值为 `true`/`false`
```
_state.Futanari    _state.Sitting    _state.Orgasm
_state.Moving      _state.Crouching  _state.Peeing
_state.Dashing     _state.InLight    _state.NearNPC
_state.Watched     _state.ShowingOff _state.Bukkake
_state.Blindfolded _state.Invisible  _state.InOpenToilet
_state.Bodypaint   _state.FPCamera   _state.IsDayTime
_state.GameOver    _state.FirstPerson
```

### 数值状态变量
可与数字比较（`>=`、`>`、`<`、`<=`、`==`、`!=`）
```
_state.Ecstasy     _state.Detection   _state.Rank
_state.HeartRate   _state.Stamina     _state.StaminaMax
_state.Moisture
```

### 手铐状态
```
Handcuffs.State    — 布尔值（是否已上铐）
Handcuffs.Type     — 字符串："Handcuff" / "KeyHandcuff" / "TimerHandcuff"
```

### 振动器 / 活塞
```
Vibrator  — "Off" / "Low" / "High" / "Random"
Piston    — "Off" / "Low" / "Medium" / "High" / "Random"
```

### Category Conditions（类别条件）
```
Action_<Actionname>          — 当前动作为指定动作
Cosplay_<CosplayID>          — 穿着指定服装
OwnsCosplay_<CosplayID>      — 拥有指定服装
AdultToy_<AdultToy>          — 装备指定成人玩具
OwnsAdultToy_<AdultToy>      — 拥有指定成人玩具
Item_<Itemname>              — 物品数量 > 0
MissionCompleted_<ID>        — 任务已完成
Skill_<Skillname>            — 技能已启用
Exposed_<Bodypart>           — 露出指定部位
```
