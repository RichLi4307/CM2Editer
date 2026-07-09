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

> 目标：如果快感值 >= 90，输出"高"，否则输出"低"

If 节点需要接在 Flow 链上——用它替换第二步的 Log，Data 管线并行输入：

```text
Start ──Flow──→ [If] ──Flow(true)──→ Log("高")
                  │   ──Flow(false)─→ Log("低")
                  │ (Data 输入)
                  └── CompareNumbers.out_result
```

| 步骤 | 操作 | 需要的节点 |
|------|------|-----------|
| 1 | 删除或断开 Step 2 的 Log | |
| 2 | 从 Control 拖 **If** 到画布 | `If` |
| 3 | 连接 Start.`out_flow` → If.`in_flow` | Flow 边 |
| 4 | 从 General Functions 拖两个新 **Log**，分别填 `"高"` 和 `"低"` | `Log` ×2 |
| 5 | 连接 If.`out_true` → Log("高").`in_flow` | Flow 边 |
| 6 | 连接 If.`out_false` → Log("低").`in_flow` | Flow 边 |
| 7 | 从 Game Functions: Player 拖 **GetStateNumber** 到画布，选 `Ecstasy` | `GetStateNumber` |
| 8 | 从 Math 拖 **NumberConstant** 到画布，设值 90 | `NumberConstant` |
| 9 | 从 Math 拖 **CompareNumbers** 到画布，`operator` 设 `>=` | `CompareNumbers` |
| 10 | 连线：`GetStateNumber.out_value` → `CompareNumbers.a` | Data 边 |
| 11 | 连线：`NumberConstant.out_value` → `CompareNumbers.b` | Data 边 |
| 12 | 连线：`CompareNumbers.out_result` → `If.condition` | Data 边 |

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

> 目标：每帧都检查状态，而不是只检查一次

监听器的回调标签是一个**独立的标签体**（不需要 Start），需要用一个 **Label 节点**作为标签体的入口。

| 步骤 | 操作 | 需要的节点 |
|------|------|-----------|
| 1 | 从 Objects 拖 **CreateListener** 到画布 | `CreateListener` |
| 2 | 属性面板 `labelName` 填 `check_loop` → 回车 | |
| 3 | 将 Start.`out_flow` → CreateListener.`in_flow`（main 标签创建监听器） | |
| 4 | 从 Control 拖 **Label** 到画布，`name` 填 `check_loop` → 回车 | `Label` |
| 5 | 左栏标签管理器 → 展开"标签" → `check_loop` 现在显示 1 个节点（Label） | |
| 6 | 把第三步的 If + CompareNumbers + GetStateNumber + Log 连在 Label 后面：Label.`out_flow` → If.`in_flow` → ... | |
| 7 | If 后面不再连回 CreateListener——监听器每帧从头执行，自然循环 | |

工作方式：

- `main:` 标签 → CreateListener 启动 `check_loop`
- `check_loop:` 标签 → Label 节点（入口）→ If → Log → `_result = null`
- 每帧 CM2 调用 `check_loop:` 一次，执行整个标签体

---

## 第五步：状态机（Goto 切换）

> 目标：条件触发后跳转到下一步，不再轮询

| 步骤 | 操作 | 需要的节点 |
|------|------|-----------|
| 1 | 从 Control 拖 **Goto** 到画布，属性面板 `label` 填 `step2` → 回车 | `Goto` |
| 2 | 连接 If.`out_true` → Goto.`in_flow`（替换原来连到 Log("高") 的 Flow 边） | |
| 3 | `step2` 标签自动出现在左栏标签管理器中 | |

Goto 之后当前线程结束执行。`step2` 标签体可通过添加 Label 节点并设置 `name="step2"` 来定义内容，和第四步的 `check_loop` 标签用法一致。

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
