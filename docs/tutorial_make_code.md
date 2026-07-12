# CM2Editer 实战教程：从零制作可用的 .code 文件（新架构）

> 配合 `docs/code_api_reference.md`（DSL 语法）和 `docs/node_types.md`（节点手册）阅读。
> 本教程基于 CM2Editer v0.3.0-architecture（新架构），以 `.code` 语法结构为中心。

---

## 前置知识

**.code 运行模型**（新架构）：

```text
模块加载时（顶层，所有 .code 合并）：
  var_main_thread = CreateThread("main")     ← 自动启动 main 线程

main 线程的 main 标签：                          ← 容器，入口是入口钉
    thread = _this
    Log("hello")                                ← Flow 边依次执行
    thread.Goto("step1")                        ← 跳到同线程的另一个标签

step1 标签：
    thread = _this
    ...
    thread.Goto("step2")

check_status 监听器：                           ← 每帧调用 check_status 标签
    if _state.Futanari
        thread.Goto("step2")
    _result = null
```

要点：

- **线程容器** 包含一组标签和监听器。新建工程时自动生成 `main` 线程容器。
- **标签容器** 是命名代码块，类似函数或状态机状态。`main` 只是约定俗成的名称，不是特殊入口。
- **监听器容器** 是每帧/每秒调用标签的循环。它捕获创建时的作用域。
- `Flow` 边**只在同一个标签/监听器容器内部**表示顺序；跨容器关系用名称引用或 `Data` 边表达。
- `Goto` 生成 `thread.Goto("label")`，`thread` 是脚本约定变量名，代表当前线程。
- 想显式引用当前线程，用 **GetCurrentThread** 数据节点，输出 `_this`。
- 纯数据节点（Data-only）无 Flow 端口，只通过 Data 虚线给其他节点喂值。

---

## 第一步：新建工程

1. 启动 CM2Editer → 欢迎页点击 `New 新建工程`。
2. 选择父文件夹 → 填写工程名称 → 确定。
3. 左侧工程树会出现一个 **main 线程**，展开后有一个 **main 标签**。
4. 点击 `main` 标签，画布显示该标签的内部节点图（默认只有一个入口钉）。
5. 右栏/底部同步生成预览代码。

> 新架构中没有 `Start` 节点。入口钉是 `LabelContainer` 的入口，不是节点。

---

## 第二步：在 main 标签里输出日志

> 目标：生成 `main.code` 输出 `Log("hello")`

| 步骤 | 操作 |
|------|------|
| 1 | 确保左侧选中了 `main` 线程 → `main` 标签 |
| 2 | 左栏节点面板 → 展开 `Game API` → 拖 **Log** 到画布 |
| 3 | 连接入口钉的 `out_flow` → Log 的 `in_flow` |
| 4 | 右侧属性面板 → `output` 参数 → 输入 `hello` → 按回车 |
| 5 | `Ctrl+S` 保存 |

生成结果：

```code
var_main_thread = CreateThread("main")

main:
    thread = _this
    Log(output="hello")
    _result = null
```

> 注意：`output` 是 String 类型，直接写 `hello` 即可，不需要再加引号。

---

## 第三步：添加条件判断

> 目标：如果快感值 >= 90，输出“高”，否则输出“低”

### 3.1 添加 If 节点和判断条件

| 步骤 | 操作 |
|------|------|
| 1 | 拖一个 **If** 节点到 `main` 标签画布 |
| 2 | 拖 **CompareNumbers** 到画布（纯 Data 节点） |
| 3 | 拖 **GetStateNumber** 到画布，`stateKey` 选 `Ecstasy` |
| 4 | 用 Data 边连接 `GetStateNumber.out_value` → `CompareNumbers.a` |
| 5 | 在 `CompareNumbers` 属性面板设置 `b` = `90`，`operator` = `>=` |
| 6 | 用 Data 边连接 `CompareNumbers.out_result` → `If.condition` |
| 7 | 用 Flow 边连接入口钉 → If → 两个分支的 Log 节点 |
| 8 | 在两个分支各放一个 Log，分别输出 `高` 和 `低` |

生成结果：

```code
main:
    thread = _this
    if _state.Ecstasy >= 90
        Log(output="高")
    else
        Log(output="低")
    _result = null
```

---

## 第四步：添加子标签并用 Goto 切换状态

> 目标：在 `main` 标签里跳转到 `step1` 标签

### 4.1 创建 step1 标签

| 步骤 | 操作 |
|------|------|
| 1 | 左侧 `main` 线程右键 → `Add Label` → 输入 `step1` |
| 2 | 左侧点击 `step1` 标签，进入它的画布 |
| 3 | 拖一个 Log，输出 `进入 step1` |
| 4 | 返回 `main` 标签画布 |
| 5 | 拖一个 **Goto** 节点，设置 `label` 参数为 `step1` |
| 6 | 用 Flow 边连接 `main` 里的 Log 或 If 分支 → Goto |

生成结果：

```code
var_main_thread = CreateThread("main")

main:
    thread = _this
    Log(output="hello")
    thread.Goto("step1")

step1:
    thread = _this
    Log(output="进入 step1")
    _result = null
```

> 注意：Goto 是状态跳转，不是函数调用。新线程才创建新作用域。

---

## 第五步：添加监听器容器

> 目标：每帧检查 `_state.Futanari`，如果为真则跳转到 `step2`

### 5.1 创建监听器容器

| 步骤 | 操作 |
|------|------|
| 1 | 左侧 `main` 线程右键 → `Add Listener` → 输入 `check_status` |
| 2 | 点击 `check_status` 监听器，进入它的画布 |
| 3 | 拖 **If** 节点 |
| 4 | 拖 **GetStateBool**，`stateKey` 选 `Futanari` |
| 5 | Data 边连接 `GetStateBool.out_value` → `If.condition` |
| 6 | If 的 True 分支连接一个 **Goto**，`label` 设为 `step2` |
| 7 | If 的 False 分支直接连到出口（或结束） |

生成结果（在 `main` 线程作用域内）：

```code
main:
    thread = _this
    Log(output="hello")
    var_check_status_listener = CreateListener("check_status")
    thread.Goto("step1")

step1:
    thread = _this
    Log(output="进入 step1")
    _result = null

check_status:
    thread = _this
    if _state.Futanari
        thread.Goto("step2")
    _result = null
```

> 注意：监听器变量名由容器 `variable_name` 决定。监听器回调标签与容器同名。

---

## 第六步：创建新线程并等待结束

> 目标：在 `main` 线程中启动一个 `worker` 线程，并等待它完成

### 6.1 创建 worker 线程和标签

| 步骤 | 操作 |
|------|------|
| 1 | 左侧工程树右键 → `Add Thread` → 输入 `worker` |
| 2 | 在 `worker` 线程中创建 `work` 标签，内部放若干 Log |
| 3 | 返回 `main` 线程的 `main` 标签画布 |
| 4 | 拖 **CreateThread** 节点，`labelName` 设为 `work` |
| 5 | 拖 **WaitForThread** 节点 |
| 6 | 用 Data 边连接 `CreateThread.out_thread` → `WaitForThread.thread` |
| 7 | 用 Flow 边连接 `CreateThread` → `WaitForThread` |

生成结果：

```code
var_main_thread = CreateThread("main")

main:
    thread = _this
    var_node_xxx_out_thread = CreateThread(labelName="work")
    var_node_xxx_out_thread.WaitForFinish()
    _result = null

worker:
    thread = _this
    Log(output="working")
    _result = null
```

> 注意：这里 `work` 标签在 `worker` 线程容器内。新线程的 `auto_start` 应设为 `false`。

---

## 第七步：使用全局变量数据节点

> 目标：读取 `_save.TotalScore` 并输出

| 步骤 | 操作 |
|------|------|
| 1 | 拖 **GetSave** 到 `main` 标签画布 |
| 2 | 设置 `key` 参数为 `TotalScore` |
| 3 | 拖一个 **Log** |
| 4 | 用 Data 边连接 `GetSave.out_value` → `Log.output` |

生成结果：

```code
main:
    thread = _this
    Log(output=_save.TotalScore)
    _result = null
```

类似地，可以读取：

- `_time` → **GetTime**
- `_timediff` → **GetTimeDiff**
- `_settings.Xxx` → **GetSettings**
- `_mod.Xxx` → **GetMod**
- `_mods` → **GetMods**

---

## 常见误区

| 误区 | 正确做法 |
|------|----------|
| 把两个不同标签用 `Flow` 边连起来 | 跨标签跳转用 **Goto** 节点，通过名称引用 |
| 在新工程里找 `Start` 节点 | 新架构用入口钉和线程/标签容器 |
| 在 `main` 标签里监听另一个线程的事件 | 监听器属于某个线程容器，但捕获的是创建时的作用域 |
| 用 `Goto` 跳到另一个线程的标签 | `Goto` 只能跳转同一 `ThreadContainer` 内的标签；跨线程用 `CreateThread` / 事件 |
| 给 `Log.output` 加引号 | String 参数值直接写内容，编辑器会生成正确引号 |
| 把 Data 节点拖到 Flow 链里 | 纯 Data 节点没有 Flow 端口，只能连 Data 边 |

---

## 完整示例：一个简单状态机

工程结构：

- `main` 线程：包含 `main`、`step1`、`step2` 标签
- `main` 监听器：每帧检查 `_state.Futanari`，为真则跳 `step2`

生成 `.code`：

```code
var_main_thread = CreateThread("main")

main:
    thread = _this
    Log(output="开始")
    var_check_status_listener = CreateListener("check_status")
    thread.Goto("step1")

step1:
    thread = _this
    Log(output="在 step1 等待")
    _result = null

step2:
    thread = _this
    Log(output="进入 step2")
    _result = null

check_status:
    thread = _this
    if _state.Futanari
        thread.Goto("step2")
    _result = null
```

在新架构中，线程树显示：

```text
main
├── main
├── step1
├── step2
└── check_status (listener)
```

---

## 下一步

- 阅读 `docs/node_types.md` 了解所有节点分类。
- 阅读 `docs/json_schema.md` 了解新序列化格式。
- 阅读 `docs/migration_guide.md` 了解从旧版工程迁移的方法。

