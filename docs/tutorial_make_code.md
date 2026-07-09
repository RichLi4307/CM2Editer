# CM2Editer 实战教程：从零制作可用的 .code 文件

> 配合 `docs/code_api_reference.md`（DSL 语法）和 `docs/node_types.md`（节点手册）阅读。

---

## 前置知识

**.code 运行模型**：

```text
模块加载时（顶层，所有 .code 合并）：
  var_main = CreateThread("main")     ← 触发 main: 标签执行一次

main:                                  ← 标签，不是自动入口
    thread = _this
    ...                               ← Flow 边依次执行的节点
    thread.Goto("step1")              ← 跳到下一个状态

    step1:                             ← 子标签（同缩进层级）
        listener = CreateListener("check")

        check:                         ← 监听器标签（每帧执行）
            if _state.Futanari
                thread.Goto("step2")
            _result = null
```

---

## 第一步：新建工程

1. 启动 CM2Editer → 欢迎页点击 `New 新建工程`
2. 填写工程名称 → 确定
3. 画布上会出现一个 **Start** 节点（`main:` 标签的入口）

---

## 第二步：搭建最简单流程（Start → Log）

> 目标：生成 `main.code` 输出 `Log("hello")`

| 步骤 | 操作 |
|------|------|
| 1 | 左栏工程标签 → 展开"General Functions" → 拖 **Log** 到画布 |
| 2 | 连接 Start 的 `out_flow` → Log 的 `in_flow` |
| 3 | 右侧属性面板 → `output` 参数 → 输入 `"hello"` → 按回车 |
| 4 | `Ctrl+S` 保存 |

生成结果：
```code
var_main_thread = CreateThread("main")

main:
    Log(output="hello")
    _result = null
```

---

## 第三步：加条件判断

> 目标：根据快感值判断是否触发

| 步骤 | 操作 | 需要的节点 |
|------|------|-----------|
| 1 | 从 Control 拖 **If** 到画布 | `If` |
| 2 | 从 Game Functions: Player 拖 **GetStateNumber** 到画布 | `GetStateNumber` |
| 3 | 从 Math 拖 **NumberConstant** 到画布，设值为 90 | `NumberConstant` |
| 4 | 从 Math 拖 **CompareNumbers** 到画布 | `CompareNumbers` |
| 5 | 连线：`GetStateNumber.out_value` → `CompareNumbers.a` | Data 边 |
| 6 | 连线：`NumberConstant.out_value` → `CompareNumbers.b` | Data 边 |
| 7 | 选择 CompareNumbers → 属性面板 `operator` 设为 `>=` | |
| 8 | 连线：`CompareNumbers.out_result` → `If.condition` | Data 边 |
| 9 | 从 General Functions 拖两个 **Log** 到画布（一个输出"高"，一个输出"低"）| |
| 10 | 连接 If.`out_true` → Log("高").`in_flow` | Flow 边 |
| 11 | 连接 If.`out_false` → Log("低").`in_flow` | Flow 边 |

生成结果：
```code
var_main_thread = CreateThread("main")

main:
    if _state.Ecstasy >= 90
        Log(output="高")
    else
        Log(output="低")
    _result = null
```

---

## 第四步：加监听器（每帧检查）

> 目标：每帧检查状态，而不是只检查一次

| 步骤 | 操作 | 需要的节点 |
|------|------|-----------|
| 1 | 从 Objects 拖 **CreateListener** 到画布 | `CreateListener` |
| 2 | 属性面板 `labelName` 填 `check_loop` → 回车 | |
| 3 | 连接 Start.`out_flow` → CreateListener.`in_flow` | |
| 4 | CreateListener.`out_flow` → Log("started").`in_flow` | |
| 5 | 在 Log("started") 之后串一个 **Goto** 节点（标靶设为空字符串或不要 Goto）| |

> 监听器 label `check_loop` 会自动注册到标签管理器（左栏可见）。
> 监听器内容是 If+CompareNumbers 组合——每帧跑一遍条件判断。

**简化的正确模式**：

```code
main: 标签
  CreateListener("check")  → 启动每帧轮询
  _result = null

check: 标签（自动创建）
  [GetStateNumber] → [CompareNumbers] → [If]
  _result = null
```

---

## 第五步：状态机（Goto 切换）

> 目标：条件触发后跳转到下一步，不再轮询

```code
check: 标签
    if _state.Ecstasy >= 90        ← 条件满足
        thread.Goto("step2")       ← 跳到 step2

    step2: 标签（自动注册）
        Log("triggered!")
        _result = null
```

| 步骤 | 操作 | 需要的节点 |
|------|------|-----------|
| 1 | 从 Flow 拖 **Goto** 到画布 | `Goto` |
| 2 | 属性面板 `label` 填 `step2` → 回车 | |
| 3 | 连接 If.`out_true` → Goto.`in_flow` | |
| 4 | `step2` 标签自动出现在左栏标签管理器中 | |
| 5 | 在画布上拖一个 **Log** → 填 `"triggered!"` | |
| 6 | **无需连 Flow 边**——Goto 是终点，step2 是独立入口 | |

---

## 第六步：条件对象判定

> 目标：用 CreateCondition 检查暴露状态

| 步骤 | 操作 | 需要的节点 |
|------|------|-----------|
| 1 | 从 Objects 拖 **CreateCondition** 到画布 | `CreateCondition` |
| 2 | 属性面板 `condition` 填 `[Exposed_All]` | |
| 3 | 从 Math 拖 **CheckCondition** 到画布 | `CheckCondition` |
| 4 | 连线：`CreateCondition.out_condition` → `CheckCondition.cond` | |
| 5 | 连线：`CheckCondition.out_result` → `If.condition` | |

生成结果：
```code
var_node_cond_out_condition = CreateCondition(condition="[Exposed_All]")
...
if var_node_cond_out_condition.Check()
    ...
```

---

## 常用组合速查

| 目的 | 节点组合 |
|------|---------|
| 恒定真/假 | `Boolean(true/false)` → `If.condition` |
| 检查状态 | `GetStateBool(Futanari)` → `If.condition` |
| 数值比较 | `GetStateNumber(Ecstasy)` + `NumberConstant(90)` → `CompareNumbers` → `If` |
| 两条件都满足 | `GetStateBool(A)` + `GetStateBool(B)` → `LogicAnd` → `If` |
| 任一条件满足 | `GetStateBool(A)` + `GetStateBool(B)` → `LogicOr` → `If` |
| 取反 | `GetStateBool(X)` → `LogicNot` → `If` |
| 条件对象 | `CreateCondition` → `CheckCondition` → `If` |
| 检查装备 | `CheckEquipment(Vibrator)` → `If` |
| 检查服装 | `CheckCosplay` → 命名空间选 cosplay → `If` |
| 坐标输出 | `GetPosition` → `MakeVector` / `BreakVector` |
| 每帧轮询 | `CreateListener("label")` → label 内放 If |
| 状态切换 | `Goto("next")` → next 标签自动创建 |
| 多线程 | `CreateThread("child")` → child 标签自动创建 |

---

## 保存后的目录结构

```text
MyMission/
  ├── meta.json          ← 任务元信息
  ├── main.code          ← 生成的 .code 文件（可用文本编辑器查看）
  └── .cm2editor/
      └── main.code.json ← 编辑器内部图文件（下次可重新打开编辑）
```
