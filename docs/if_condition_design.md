# If 条件 / 布尔值输出节点设计方案

> **语法权威参考**：`docs/code_api_reference.md` — 基于 80+ 个前辈手搓 `.code` 反推的 DSL 全集。

---

## 架构总览

```text
┌─────────────┐      Data(Number/String)     ┌─────────────────┐      Data(Boolean)     ┌──────┐
│  监测节点   │ ───────────────────────────→ │  Condition 节点 │ ─────────────────────→ │ If   │
│  (Monitor)  │                              │  (Judge)        │                        │      │
│  读游戏状态 │                              │  评估条件       │                        │ 分支 │
└─────────────┘                              └─────────────────┘                        └──────┘
     ↑                                                                                     │
     │  有些监测节点直接输出 Boolean，可跳过 Condition                                     │
     └─────────────────────────────────────────────────────────────────────────────────────┘
```

**三条 DataFlow 通路**：

1. `Boolean 常量` → If.condition（最简单的真/假分支）
2. `监测节点` → `Condition 节点` → If.condition（结构化条件评估）
3. `监测节点` → If.condition（监测节点输出类型 = Boolean 时直连）

---

## 节点定义

### 一类：监测节点（Monitor）—— 读游戏状态

这些节点**没有 Flow 端口**，纯数据输出。从 `_state.*` 中读取游戏状态并以**强类型端口**暴露。

#### M1. Boolean 常量

| 字段 | 值 |
|------|----|
| NodeType | `Boolean` |
| 分类 | Math |
| Inputs | 无 |
| Outputs | `out_value: Boolean` — `"true"` / `"false"` |
| Params | `value: Enum(["true", "false"])`，默认 `"true"` |
| code_gen 值 | `"true"` / `"false"` |

#### M2. GetStateBool —— 读取单个 boolean 状态

| 字段 | 值 |
|------|----|
| NodeType | `GetStateBool` |
| 分类 | Game Functions: Player |
| Inputs | 无 |
| Outputs | `out_value: Boolean` |
| Params | `stateKey: Enum` — 18 个布尔状态名（从 `_state.*` 列出） |
| code_gen 值 | `_state.{key}` 如 `_state.Futanari` |

**关键设计**：Enum 选项按三级分类呈现（可用命名空间选择器式窗口）：

```text
角色状态    │  装备/拘束        │  环境
────────────┼───────────────────┼────────
Futanari    │  Blindfolded      │  InLight
Sitting     │  Invisible        │  NearNPC
Orgasm      │  Bodypaint        │  Watched
Moving      │  GameOver         │  IsDayTime
Crouching   │                   │  InOpenToilet
Peeing      │                   │  FPCamera
Dashing     │                   │  ShowingOff
            │                   │  Bukkake
```

#### M3. GetStateNumber —— 读取单个数值状态

| 字段 | 值 |
|------|----|
| NodeType | `GetStateNumber` |
| 分类 | Game Functions: Player |
| Inputs | 无 |
| Outputs | `out_value: Number` |
| Params | `stateKey: Enum` — 8 个数值状态名 |

选项：`Ecstasy`, `Detection`, `Rank`, `HeartRate`, `Stamina`, `StaminaMax`, `Moisture`, `Bodypaint`

#### M4. GetPosition —— 读取坐标

| 字段 | 值 |
|------|----|
| NodeType | `GetPosition` |
| 分类 | Game Functions: Player |
| Inputs | 无 |
| Outputs | `out_position: List`, `out_stage: String` |
| code_gen 值 | `[{x}, {y}, {z}]`, `_state.Position.stage` |

#### M5. CheckEquipment —— 检查装备/玩具

| 字段 | 值 |
|------|----|
| NodeType | `CheckEquipment` |
| 分类 | Game Functions: Items |
| Inputs | 无 |
| Outputs | `out_value: Boolean` — `!= null` 判断 |
| Params | `equipType: Enum` — `Handcuff`, `KeyHandcuff`, `TimerHandcuff`, `Vibrator`, `EyeMask`, `TitRotor`, `KuriRotor`, `PistonAnal`, `PistonPussy`, `AnalPlug` |
| code_gen 值 | `_state.AdultToys.{type} != null` |

#### M6. CheckCosplay —— 检查服装

| 字段 | 值 |
|------|----|
| NodeType | `CheckCosplay` |
| 分类 | Game Functions: Items |
| Inputs | 无 |
| Outputs | `out_value: Boolean` |
| Params | `cosplayKey: Namespace(cosplay)` — 用命名空间选择器 |
| code_gen 值 | `OwnsCosplay_{key}` 或 `Cosplay_{key}`（取决于 `checkMode` 参数） |

---

### 二类：Condition 条件节点（Judge）—— 评估

#### C1. CompareNumbers —— 数值比较

| 字段 | 值 |
|------|----|
| NodeType | `CompareNumbers` |
| 分类 | Math |
| Inputs | `a: Number` (Data), `b: Number` (Data) |
| Outputs | `out_result: Boolean` |
| Params | `operator: Enum` — `==`, `!=`, `>`, `>=`, `<`, `<=`，默认 `>=` |
| code_gen 值 | 从 Data 边提取 a 源值和 b 源值，组表达式如 `_state.Ecstasy >= 50` |

**DataFlow 示例**：
```code
GetStateNumber(stateKey=Ecstasy) ──out_value(Number)──→ CompareNumbers.a
Boolean(value=true) ──out_value(Boolean)──→  (或手输数字 50 → CompareNumbers.b)
CompareNumbers ──out_result(Boolean)──→ If.condition
→ 生成：If(_state.Ecstasy >= 50) [
```

#### C2. LogicAnd / LogicOr / LogicNot —— 逻辑组合

| 字段 | LogicAnd / LogicOr | LogicNot |
|------|-------------------|----------|
| NodeType | `LogicAnd` / `LogicOr` | `LogicNot` |
| 分类 | Math | Math |
| Inputs | `a: Boolean`, `b: Boolean` (Data) | `a: Boolean` (Data) |
| Outputs | `out_result: Boolean` | `out_result: Boolean` |
| code_gen 值 | `({a} && {b})` / `({a} \|\| {b})` | `!({a})` |

---

### 三类：条件模板下拉框（If 节点内置）

If 节点 `condition` 参数编辑区域放置一个 `ComboBox`，提供 **30+ 条预设表达式**。选中后填入字段，用户可继续手改。

模板来源：`docs/code_api_reference.md` §3–§4。按分类排列：

```markdown
── 字面量 ──
  ✅ true     → "true"
  ❌ false    → "false"

── 角色状态 (boolean) ──
  Futanari · 扶她       → _state.Futanari
  Sitting · 坐姿         → _state.Sitting
  Orgasm · 高潮          → _state.Orgasm
  Moving · 移动中        → _state.Moving
  ... (全部 19 项)

── 装备/拘束 ──
  有手铐                  → _state.AdultToys.Handcuff != null
  无手铐                  → _state.AdultToys.Handcuff == null
  蒙眼                    → _state.Blindfolded
  隐身                    → _state.Invisible
  身体涂鸦                → _state.Bodypaint > 0

── 数值比较 ──
  快感 ≥ _____           → _state.Ecstasy >= 
  侦测 ≥ _____           → _state.Detection >= 
  体力 ≥ _____           → _state.Stamina >= 
  等级 ≥ _____           → _state.Rank >= 
  湿润度 ≥ _____         → _state.Moisture >= 
  心率 ≥ _____           → _state.HeartRate >= 
```

---

## DataFlow 拼接示例

### 示例 1：最简单的条件分支

```markdown
[Boolean(value=true)] ──out_value(Boolean)──→ [If.condition]
                                                    ├── out_true → Log("是")
                                                    └── out_false → Log("否")
```

### 示例 2：游戏状态驱动分支

```markdown
[GetStateBool(Futanari)] ──out_value(Boolean)──→ [If.condition]
                                                      ├── out_true → Log("扶她")
                                                      └── out_false → Log("不是")
→ 生成：If(_state.Futanari) [
```

### 示例 3：数值比较 + 逻辑组合

```markdown
[GetStateNumber(Ecstasy)] ──out_value──→ [CompareNumbers.a]
                                                      │ CompareNumbers(op=">=")
                                                      │ out_result → [If.condition]
[手输数字 50] ───────────────→ [CompareNumbers.b] ────┘
→ 生成：If(_state.Ecstasy >= 50) [
```

### 示例 4：复杂组合条件

```markdown
[GetStateBool(InLight)] ─────→ [LogicAnd.a]
                                    │ LogicAnd.out_result
[CheckEquipment(Handcuff)] ──→ [LogicAnd.b] ──→ [If.condition]
                                                  ├── out_true → Log("满足条件")
                                                  └── out_false → Log("不满足")
→ 生成：If((_state.InLight) && (_state.AdultToys.Handcuff != null)) [
    Log("满足条件")
] else [
    Log("不满足")
]
```

---

## 实施计划

### Phase 1（当前迭代）

```markdown
文件                                    改动
─────────────────────────────────────────────────────────────────────────────────────
src/graph/types.rs                      加 Boolean, CompareNumbers, GetStateBool,
                                          GetStateNumber, LogicAnd, LogicOr, LogicNot
                                          变体到 NodeType 枚举
src/api/definitions.rs                  加 7 个 NodeDefinition，test 计数 143→150
src/ui/panels/properties.rs             加 IF_CONDITION_TEMPLATES 常量和 ComboBox 渲染
src/code_gen/generator.rs               加 GenerateBoolean/GenerateCompareNumbers/
                                          GenerateGetStateBool 等分支
测试                                    更新 test_all_variants_have_definition 计数
```

### Phase 2（v0.2）

- CheckEquipment、CheckCosplay、GetPosition 节点
- 条件选择窗口（类似命名空间选择器）：左侧三级分类树 → 右侧详情 → 点击生成节点到画布
- `CreateCondition` Data 节点：输出条件 Object，连入 Gallery/Area

### Phase 3（v0.3+）

- Foreach 节点
- While 循环条件 DataFlow 化
- 条件表达式语法高亮
