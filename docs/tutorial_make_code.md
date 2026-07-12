# CM2Editer 实战教程：从零制作可用的 .code 文件

> 配合 `docs/code_api_reference.md`（DSL 语法）和 `docs/node_types.md`（节点手册）阅读。
> 本教程基于 CM2Editer v0.2.2 + P1 低难度节点实际实现。

---

## 前置知识

**.code 运行模型**：

```text
模块加载时（顶层，所有 .code 合并）：
  var_main_thread = CreateThread("main")     ← 触发 main: 标签执行一次

main:                                          ← 标签，由 Start 节点表示入口
    var_node_listener_out_listener = CreateListener("check_loop")
    ...                                      ← Flow 边依次执行的节点
    thread.Goto("step1")                       ← 跳到下一个状态

    step1:                                     ← 子标签（同缩进层级）
        ...
        thread.Goto("step2")

    check_loop:                                ← 监听器标签（每帧执行）
        if _state.Futanari
            thread.Goto("step2")
        _result = null
```

要点：

- `Start` 节点 = 标签入口，不是普通函数调用。
- 顶层 `CreateThread` 的变量名是 `var_{label}_thread`，不是 `var_{label}`。
- 节点内创建的 `CreateThread` / `CreateListener` 会生成 `var_node_xxx_out_thread` / `var_node_xxx_out_listener` 这样的变量名。
- `Goto` 节点生成 `thread.Goto("label")`，其中 `thread` 是脚本约定变量名；当前代码生成器不会自动定义 `thread`，你可以把它理解为“当前线程对象”。
- 想要显式引用当前线程，用 **GetCurrentThread** 数据节点，输出 `_this`。
- 想要销毁监听器，用 **DestroyListener** 节点，生成 `listener = null`（这里的 `listener` 是脚本约定变量名，对应你之前创建的监听器）。
- 纯数据节点（Data-only）无 Flow 端口，只通过 Data 虚线给其他节点喂值；选中相关节点时才会显示 Data 虚线。

---

## 第一步：新建工程

1. 启动 CM2Editer → 欢迎页点击 `New 新建工程`
2. 选择父文件夹 → 填写工程名称 → 确定
3. 画布上会出现一个 **Start** 节点（`main:` 标签的入口）
4. 右栏/底部会同步生成预览代码

---

## 第二步：搭建最简单流程（Start → Log）

> 目标：生成 `main.code` 输出 `Log("hello")`

| 步骤 | 操作 |
|------|------|
| 1 | 左栏工程标签 → 展开 `General Functions` → 拖 **Log** 到画布 |
| 2 | 连接 Start 的 `out_flow` → Log 的 `in_flow` |
| 3 | 右侧属性面板 → `output` 参数 → 输入 `hello` → 按回车 |
| 4 | `Ctrl+S` 保存 |

生成结果：

```code
var_main_thread = CreateThread("main")

main:
    Log(output="hello")
    _result = null
```

> 注意：`output` 是 String 类型，直接写 `hello` 即可，不需要再加引号。

---

## 第三步：加条件判断

> 目标：如果快感值 >= 90，输出"高"，否则输出"低"

If 节点需要接在 Flow 链上；Data 管线并行输入，用 Data 虚线连接：

```text
Start ──Flow──→ [If] ──Flow(true)──→ Log("高")
                  │   ──Flow(false)─→ Log("低")
                  │ (Data 输入)
                  └── CompareNumbers.out_result
                        ├── GetStateNumber.out_value
                        └── NumberConstant(90)
```

| 步骤 | 操作 | 需要的节点 |
|------|------|-----------|
| 1 | 删除或断开 Step 2 的 Log | |
| 2 | 从 `Control` 拖 **If** 到画布 | `If` |
| 3 | 连接 Start.`out_flow` → If.`in_flow` | Flow 边 |
| 4 | 从 `General Functions` 拖两个新 **Log**，分别填 `高` 和 `低` | `Log` ×2 |
| 5 | 连接 If.`out_true` → Log("高").`in_flow` | Flow 边 |
| 6 | 连接 If.`out_false` → Log("低").`in_flow` | Flow 边 |
| 7 | 从 `Game Functions: Player` 拖 **GetStateNumber** 到画布，`stateKey` 选 `Ecstasy` | `GetStateNumber` |
| 8 | 从 `Math` 拖 **NumberConstant** 到画布，`value` 设 `90` | `NumberConstant` |
| 9 | 从 `Math` 拖 **CompareNumbers** 到画布，`operator` 设 `>=` | `CompareNumbers` |
| 10 | 选中 CompareNumbers → 拖拽 `a` 输入端口 → 连到 GetStateNumber 的 `out_value` | Data 边 |
| 11 | 拖拽 `b` 输入端口 → 连到 NumberConstant 的 `out_value` | Data 边 |
| 12 | 拖拽 If 的 `condition` 输入端口 → 连到 CompareNumbers 的 `out_result` | Data 边 |

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

> 提示：If 的 `condition` 一旦连了 Data 边，属性面板里的条件模板会隐藏并显示 🔗；断开后恢复模板输入。

---

## 第四步：加监听器（每帧检查）

> 目标：每帧都检查状态，而不是只检查一次

监听器的回调标签是**独立的标签体**（不需要 Start），需要用一个 **Label 节点**作为标签体的入口。

| 步骤 | 操作 | 需要的节点 |
|------|------|-----------|
| 1 | 从 `Objects` 拖 **CreateListener** 到画布 | `CreateListener` |
| 2 | 属性面板 `labelName` 填 `check_loop` → 回车，`check_loop` 会自动注册到左栏标签管理器 | |
| 3 | 将 Start.`out_flow` → CreateListener.`in_flow`（main 标签内创建监听器） | |
| 4 | 从 `Control` 拖 **Label** 到画布，`name` 填 `check_loop` → 回车 | `Label` |
| 5 | 左栏标签管理器 → 展开标签 → `check_loop` 现在显示 1 个节点（Label） | |
| 6 | 把第三步的 If + CompareNumbers + GetStateNumber + Log 连在 Label 后面：Label.`out_flow` → If.`in_flow` → ... | |
| 7 | If 后面不再连回 CreateListener——监听器每帧从头执行，自然循环 | |

生成结果（main 与 check_loop 标签）：

```code
var_main_thread = CreateThread("main")

main:
    var_node_listener_out_listener = CreateListener("check_loop")
    _result = null

check_loop:
    if _state.Ecstasy >= 90
        Log(output="高")
    else
        Log(output="低")
    _result = null
```

工作方式：

- `main:` 标签执行一次 → CreateListener 启动 `check_loop`。
- `check_loop:` 标签每帧被 CM2 调用一次 → Label（入口）→ If → Log → `_result = null`。

---

## 第五步：状态机（Goto 切换）

> 目标：条件触发后跳转到下一步，不再轮询

| 步骤 | 操作 | 需要的节点 |
|------|------|-----------|
| 1 | 从 `Control` 拖 **Goto** 到画布，属性面板 `label` 填 `step2` → 回车 | `Goto` |
| 2 | 连接 If.`out_true` → Goto.`in_flow`（替换原来连到 Log("高") 的 Flow 边） | |
| 3 | `step2` 标签自动出现在左栏标签管理器中 | |
| 4 | 从 `Control` 拖 **Label** 到画布，`name` 填 `step2` → 回车 | `Label` |
| 5 | 在 Label 后面接你想要的流程，例如 Log("进入 step2") | |

Goto 跳转到目标标签后，当前线程从 `step2` 标签继续执行；Goto 节点之后的节点不会再执行。`step2` 标签体通过 Label 节点定义内容，和第四步的 `check_loop` 用法一致。

---

## 第六步：条件对象判定

> 目标：用 CreateCondition 检查暴露状态

| 步骤 | 操作 | 需要的节点 |
|------|------|-----------|
| 1 | 从 `Objects` 拖 **CreateCondition** 到画布 | `CreateCondition` |
| 2 | 属性面板 `condition` 选 `[Exposed_All]` | |
| 3 | 从 `Math` 拖 **CheckCondition** 到画布 | `CheckCondition` |
| 4 | 选中 CheckCondition → 拖拽 `cond` 输入端口 → 连到 CreateCondition 的 `out_condition` | Data 边 |
| 5 | 拖拽 If 的 `condition` 输入端口 → 连到 CheckCondition 的 `out_result` | Data 边 |

生成结果：

```code
var_node_cond_out_condition = CreateCondition(condition="[Exposed_All]")
...
if var_node_cond_out_condition.Check()
    ...
```

---

## 第七步：等待子线程结束（新增）

> 目标：创建子线程后，主线程等待它完成

| 步骤 | 操作 | 需要的节点 |
|------|------|-----------|
| 1 | 从 `Objects` 拖 **CreateThread** 到画布，`labelName` 填 `child` | `CreateThread` |
| 2 | 从 `Objects` 拖 **WaitForThread** 到画布 | `WaitForThread` |
| 3 | 将 CreateThread.`out_thread` 连到 WaitForThread.`thread` | Data 边 |
| 4 | Flow 连接：Start → CreateThread → WaitForThread → ... | |

生成结果：

```code
var_node_ct_out_thread = CreateThread("child")
...
var_node_ct_out_thread.WaitForFinish()
```

> WaitForThread 节点本身不会生成变量赋值，而是直接对输入的线程变量调用 `.WaitForFinish()`。

---

## 第八步：遍历数字范围（新增）

> 目标：用 `for i in Range(0, 10)` 循环

| 步骤 | 操作 | 需要的节点 |
|------|------|-----------|
| 1 | 从 `General Functions` 拖 **Range** 到画布，`start=0`，`stop=10` | `Range` |
| 2 | 从 `Control` 拖 **For** 到画布 | `For` |
| 3 | 将 Range.`out_list` 连到 For.`iterable` | Data 边 |
| 4 | For.`out_flow` 连接 Log("loop") 等循环体 | |

生成结果：

```code
for i in Range(0, 10)
    Log(output="loop")
```

---

## 第九步：使用全局变量（新增）

> 目标：读取 `_time`、`_timediff`、`_save`、`_settings` 等全局变量

| 步骤 | 操作 | 需要的节点 |
|------|------|-----------|
| 1 | 从 `General Functions` 拖 **GetTime** / **GetTimeDiff** / **GetSave** / **GetSettings** / **GetMod** / **GetMods** 到画布 | 对应数据节点 |
| 2 | 将它们的 `out_value` 连到需要这些值的参数端口 | Data 边 |

例如把 `GetTime.out_value` 连到 `SetEcstasy.value`：

```code
SetEcstasy(value=_time)
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
| 每帧轮询 | `CreateListener("label")` → Label 内放 If |
| 状态切换 | `Goto("next")` → Label 定义 `next` 标签体 |
| 多线程 | `CreateThread("child")` → Label 定义 `child` 标签体；可用 `WaitForThread` 等待 |
| 销毁监听器 | `DestroyListener` → 生成 `listener = null` |
| 当前线程引用 | `GetCurrentThread` → 输出 `_this` |
| 遍历范围 | `Range` → `For.iterable` |
| 读取全局变量 | `GetTime` / `GetTimeDiff` / `GetSave` / `GetSettings` / `GetMod` / `GetMods` |

---

## 保存后的目录结构

```text
MyMission/
  ├── meta.json          ← 任务元信息
  ├── main.code          ← 生成的 .code 文件（可用文本编辑器查看）
  └── .cm2editor/
      └── main.code.json ← 编辑器内部图文件（下次可重新打开编辑）
```

---

## 常见误区

1. **Data 节点没有 Flow 端口**：`GetStateNumber`、`CompareNumbers`、`Boolean` 等不能接在 Flow 链上，只能用 Data 边喂值给其他节点。
2. **Data 边默认不显示**：选中相关节点或 Data 边本身时才会渲染虚线；不影响代码生成。
3. **Goto 的 `thread` 是约定名**：Goto 节点生成 `thread.Goto("label")`，需保证你的脚本中 `thread` 指向当前线程（通常就是 `_this`）。
4. **DestroyListener 的 `listener` 是约定名**：只有你在前面用 `CreateListener` 把返回值赋给了 `listener` 变量，这句才生效。
5. **子标签必须配 Label 节点**：`CreateListener("check_loop")` 或 `Goto("step2")` 不会自动创建标签体，必须手动放 Label 节点并设置同名 `name`。

