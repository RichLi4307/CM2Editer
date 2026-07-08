# If 条件 / 布尔值输出节点设计方案

## 问题

If 节点需要布尔值输入，但**没有节点输出 Boolean**。条件表达式只能手写字符串（`_state.Futanari`、`true`），无法用 DataFlow 连线复用。

.code 本质是一种脚本语言，条件判断必须"包装"成表达式。这决定了实现路线：

| 路线 | 适用场景 | 工作量 |
|------|---------|--------|
| A. 条件模板下拉框 | 常用判断，快速填入 | 小（改 `properties.rs`） |
| B. Boolean 常量节点 | DataFlow 连 true/false | 中（加 `NodeType` + `definitions` + `code_gen`） |
| C. 预制菜节点族 | 复杂条件组合复用 | 大（需新节点类型 + code_gen 分支） |
| D. 内置到 If 节点 | 一站式免连线 | 大（重构 If 面板） |

> **建议**：优先 A+B（最小改动获取最大收益），C/D 作为 v0.2 扩展。

---

## 路线 A：条件模板下拉框（Phase 1）

在 If 属性面板中为 `condition` 参数添加模板下拉，用户选中即填入表达式。

### 交互规则

1. 若 condition 的 Data 端口已被连线 → 显示 `🔗` 只读，模板不出现
2. 否则：`ComboBox` 选模板 → 填入条件字段 → 用户可继续手动编辑
3. 不影响 If 节点现有的手写表达式能力

### 模板分类

#### 1. 字面量

**标签** / **值**

`✅ true` → `true`
`❌ false` → `false`

#### 2. 角色状态 boolean

**标签** | **值**
---|---
Futanari（扶她） | `_state.Futanari`
Sitting（坐姿）| `_state.Sitting`
Orgasm（高潮）| `_state.Orgasm`
Moving（移动中）| `_state.Moving`
Crouching（蹲伏）| `_state.Crouching`
Peeing（排泄中）| `_state.Peeing`
Dashing（奔跑）| `_state.Dashing`
InLight（光照区）| `_state.InLight`
NearNPC（附近有人）| `_state.NearNPC`
Watched（被注视）| `_state.Watched`
ShowingOff（露出）| `_state.ShowingOff`
Bukkake（射精）| `_state.Bukkake`
Blindfolded（蒙眼）| `_state.Blindfolded`
Invisible（隐身）| `_state.Invisible`
InOpenToilet（开放厕所）| `_state.InOpenToilet`
Bodypaint（身体涂鸦）| `_state.Bodypaint`
FPCamera（第一人称）| `_state.FPCamera`
IsDayTime（白天）| `_state.IsDayTime`
GameOver（结束）| `_state.GameOver`

#### 3. 装备/拘束

**标签** | **值**
---|---
无手铐 | `Handcuffs.Type == nil`
手铐 · 普通 | `Handcuffs.Type == \"Handcuff\"`
手铐 · 钥匙 | `Handcuffs.Type == \"KeyHandcuff\"`
手铐 · 计时 | `Handcuffs.Type == \"TimerHandcuff\"`
跳蛋 · 关闭 | `Vibrator == \"Off\"`
跳蛋 · 低档 | `Vibrator == \"Low\"`
跳蛋 · 高档 | `Vibrator == \"High\"`
活塞 · 关闭 | `Piston == \"Off\"`
活塞 · 低档 | `Piston == \"Low\"`
活塞 · 中档 | `Piston == \"Medium\"`
活塞 · 高档 | `Piston == \"High\"`

#### 4. 数值比较（末尾可补数字）

**标签** | **值**
---|---
快感 ≥ | `_state.Ecstasy >= `
侦测 ≥ | `_state.Detection >= `
体力 ≥ | `_state.Stamina >= `
最大体力 ≥ | `_state.StaminaMax >= `
湿润度 ≥ | `_state.Moisture >= `
心率 ≥ | `_state.HeartRate >= `
等级 ≥ | `_state.Rank >= `

> 数值比较模板末尾留空格，用户续写数字（如 `_state.Ecstasy >= 50`）

### 文件变动

`src/ui/panels/properties.rs`：
- 定义 `IF_CONDITION_TEMPLATES` 常量（按类别 struct）
- 在 `param_editor` 中拦截 `NodeType::If` / `key == "condition"`，渲染 `ComboBox`

不涉及其他文件。

---

## 路线 B：Boolean 常量节点（Phase 2）

### 新节点

**名称**：布尔值
**分类**：Math
**颜色**：`MATH_COLOR`（与 Random、Sin 同类）
**端口**：
- 无 Flow 端口（纯数据输出）
- 输出：`out_value`（`PortType::Boolean`）
**参数**：`value`（`Enum`，`["true", "false"]`，默认 `"true"`）

### 代码生成

Boolean 节点本身不产生代码（无 Flow 路径不执行）。当 If 的 condition Data 端口连到 Boolean 的 `out_value` 时，`require_param("condition")` 沿 Data 边回溯到 Boolean 节点，取其 `value` 参数值 → `"true"` / `"false"` → 写入 `If(true)` / `If(false)`。

### 文件变动

| 文件 | 改动 |
|------|------|
| `src/graph/types.rs` | `NodeType` 加 `Boolean` 变体 |
| `src/api/definitions.rs` | 新 `NodeDefinition`，加 `all_definitions` |
| `src/code_gen/generator.rs` | 无需改动——Data 引用已支持 |
| 测试 | `test_all_variants_have_definition` 计数 143 → 144 |

---

## 路线 C：预制菜节点族（Phase 3 / v0.2）

在 Boolean 常量之后，逐步增加**逻辑判断节点**，均输出 `PortType::Boolean`：

| 节点 | 参数 | 输出值 | code_gen |
|------|------|--------|----------|
| **CompareNumbers** | a: Number, b: Number, op: Enum(=,≠,<,>,≤,≥) | 比较结果 | `{a} {op} {b}` |
| **GetStateBool** | key: Enum（见模板表） | 状态名表达式 | `_state.{key}` |
| **CheckHandcuffs** | type: Enum | 手铐比较表达式 | `Handcuffs.Type == "{type}"` |
| **CheckVibrator** | level: Enum | 跳蛋比较表达式 | `Vibrator == "{level}"` |
| **HasItem** | item: Enum（ITEMS） | 物品数量表达式 | `Items["{item}"] > 0` |
| **SkillEnabled** | skill: Enum（SKILLS） | 技能启用表达式 | `Skills["{skill}"]` |
| **LogicNot** | input: Boolean (Data) | 逻辑非 | `not ({input})` |
| **LogicAnd** | a: Boolean (Data), b: Boolean (Data) | 逻辑与 | `({a}) and ({b})` |
| **LogicOr** | a: Boolean (Data), b: Boolean (Data) | 逻辑或 | `({a}) or ({b})` |

> 这些节点都**无 Flow 端口**，仅通过 Data 连线传递表达式字符串给 If/While。

---

## 实施顺序

```
Phase 1（当前）  → 路线 A：条件模板下拉框
                  → 路线 B：Boolean 常量节点
                  → 测试 + 提交

Phase 2（后续）  → 从路线 C 中选择 2-3 个最常用的预制菜节点
                  → CompareNumbers + GetStateBool + CheckHandcuffs

Phase 3（v0.2+）  → 补齐路线 C 其余节点
                  → 逻辑组合（Not/And/Or）
                  → 条件表达式高亮/语法检查
```

---

## 附录：.code If 条件语法速查

### 布尔状态
`_state.{Futanari|Sitting|Orgasm|Moving|Crouching|Peeing|Dashing|InLight|NearNPC|Watched|ShowingOff|Bukkake|Blindfolded|Invisible|InOpenToilet|Bodypaint|FPCamera|IsDayTime|GameOver}`

### 数值状态
`_state.{Ecstasy|Detection|Rank|HeartRate|Stamina|StaminaMax|Moisture}`

### 装备/拘束
`Handcuffs.Type`、`Handcuffs.State`、`Vibrator`、`Piston`

### 类别条件
`Action_<name>`、`Cosplay_<id>`、`OwnsCosplay_<id>`、`AdultToy_<name>`、`OwnsAdultToy_<name>`、`Item_<name>`、`MissionCompleted_<id>`、`Skill_<name>`、`Exposed_<part>`
