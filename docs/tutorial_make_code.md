# CM2Editer 实战教程：当前 UI 下制作可用的 .code 模组

> 本教程匹配当前 P2 UI 版本。阅读前建议先了解 `docs/node_types.md`（节点手册）和 `docs/json_schema.md`（序列化格式）。

---

## 新架构核心概念（先读这段）

`.code` 运行模型（当前版本）：

```text
模块加载时（顶层，所有 .code 合并）：
  var_main_thread = CreateThread("main")     ← 自动启动 main 线程

main 线程的 main 标签：
    thread = _this
    Log("hello")                                ← Flow 边依次执行
    if _state.Ecstasy >= 90
        Log("高")
    else
        Log("低")
```

要点：

- **工程** = 若干 `.code` 文件。
- **.code 文件** = 一个 `GraphDocument`，包含若干 `ThreadContainer`（线程容器）。
- **线程容器** = 一组 `LabelContainer`（标签） + `ListenerContainer`（监听器）。新建工程会自动生成一个 `main` 线程。
- **标签容器** = 画布上当前编辑的代码块。默认工程有一个 `main` 标签。
- **监听器容器** = 每帧/每秒被调用的回调式代码块（高级，当前 UI 暂不支持新建）。
- `Flow` 边**只在同一个容器内部**表示执行顺序；跨容器用 `Goto` / `CreateThread` / `CreateListener` 等节点表达。
- 想引用当前线程，使用 **GetCurrentThread** 数据节点，输出 `_this`。
- 纯数据节点（如 `GetSave`）没有 Flow 端口，只通过 Data 虚线给其它节点喂值。
- **入口钉**（画布左侧的小圆点）不是节点，只是视觉标记，指向当前容器的**入口节点**。

---

## 界面布局

- **左栏 Project 标签**：工程文件树 + 节点库。
- **中栏**：画布。只显示当前选中的标签/监听器容器。
- **右栏**：属性面板。选中节点时显示参数。
- **底部**：
  - 左侧：`.code` 实时预览。
  - 中间：JSON 预览。
  - 右侧：DataFlow 面板（列出当前容器的数据节点）。

---

## 第一步：新建工程

1. 启动 `CM2Editer`。
2. 点击左栏上方 **新建工程** 按钮，选择父文件夹并输入工程名称，确定。
3. 创建后，工程树出现：

```text
[main].code
  └── main 线程
       └── main 标签
```

4. 点击 `main` 标签，画布显示一个入口钉，文字为 `入口: main`。
5. 底部 `.code` 预览会显示 `main` 线程框架（目前只有 `thread = _this`）。

---

## 第二步：Hello World

> 目标：生成 `main.code` 输出 `Log("hello")`

| 步骤 | 操作 |
|------|------|
| 1 | 确保左侧工程树选中了 `main` 标签 |
| 2 | 左栏 Project 标签 → 节点库 → 展开 `Game API` → 拖 **Log** 到画布 |
| 3 | 点击 Log 节点，右栏属性面板 → `output` 参数 → 输入 `hello` → 按回车 |
| 4 | 按 `Ctrl+S` 保存，或点击 **保存工程** |

生成结果（底部 `.code` 预览）：

```code
var_main_thread = CreateThread("main")

main:
    thread = _this
    Log(output="hello")
```

> 注意：
> - `output` 是 String 类型，直接写 `hello`，不需要加引号，编辑器会生成正确引号。
> - **入口钉不是节点，不能从它拖线**。只要 Log 是当前唯一“没有 Flow 入边”的节点，入口钉会自动指向它。

---

## 第三步：连接多个节点

> 目标：先输出 `hello`，再输出 `world`

1. 再拖一个 **Log** 到画布，`output` 设为 `world`。
2. 从第一个 Log 的 `out_flow` 端口拖一条 **Flow 边** 到第二个 Log 的 `in_flow` 端口。

画布逻辑：

```text
入口钉 → Log("hello") → Log("world")
```

生成结果：

```code
main:
    thread = _this
    Log(output="hello")
    Log(output="world")
```

> 入口节点规则：同一容器内，**最靠左上、且没有 Flow 边连入 `in_flow` 的节点**会被选为入口。所以第一个 Log 仍保持入口地位。

---

## 第四步：添加条件判断

> 目标：如果 `_state.Ecstasy >= 90`，输出“高”，否则输出“低”

| 步骤 | 操作 |
|------|------|
| 1 | 拖一个 **If** 节点到画布 |
| 2 | 拖 **CompareNumbers** 数据节点到画布 |
| 3 | 拖 **GetStateNumber** 数据节点，`stateKey` 选 `Ecstasy` |
| 4 | 用 Data 边连接 `GetStateNumber.out_value` → `CompareNumbers.a` |
| 5 | 在 `CompareNumbers` 属性面板设置 `b` = `90`，`operator` = `>=` |
| 6 | 用 Data 边连接 `CompareNumbers.out_result` → `If.condition` |
| 7 | 用 Flow 边连接第一个 `Log` → `If.in_flow` |
| 8 | 在 `If` 的 `True` / `False` 分支各放一个 `Log`，分别输出 `高` 和 `低` |
| 9 | 从 `If.out_true` / `If.out_false` 分别连 Flow 到对应 Log |

生成结果：

```code
main:
    thread = _this
    Log(output="hello")
    if _state.Ecstasy >= 90
        Log(output="高")
    else
        Log(output="低")
```

---

## 第五步：使用全局变量

> 目标：读取 `_save.TotalScore` 并输出

| 步骤 | 操作 |
|------|------|
| 1 | 拖 **GetSave** 数据节点到画布 |
| 2 | 设置 `key` 参数为 `TotalScore` |
| 3 | 拖一个 **Log** |
| 4 | 用 Data 边连接 `GetSave.out_value` → `Log.output` |

生成结果：

```code
main:
    thread = _this
    Log(output=_save.TotalScore)
```

类似来源：

- `_time` → **GetTime**
- `_timediff` → **GetTimeDiff**
- `_settings.Xxx` → **GetSettings**
- `_mod.Xxx` → **GetMod**
- `_mods` → **GetMods**

> 纯数据节点没有 Flow 端口，只要通过 Data 边被引用，代码生成器会自动把它放在合适位置。

---

## 第六步：保存与导出

- **保存工程**（`Ctrl+S` 或按钮）：保存项目结构和 JSON 图到工程目录。
- **导出工程**：弹出文件夹选择，选择你的 `CustomMissions2` 目录后，生成 `.code` 文件到该目录。

导出后的 `.code` 文件即可被游戏加载。

---

## 常见误区

| 误区 | 正确做法 |
|------|----------|
| 从入口钉拖线 | 入口钉不是节点，不能拖线。入口节点会自动判定为“最靠左上、无 Flow 入边”的节点 |
| 找 `Start` 节点 | 新架构已移除 `Start` 节点，用入口钉 + 容器化模型 |
| 把两个不同标签用 Flow 边连起来 | 跨标签跳转用 **Goto** 节点，通过名称引用 |
| 给 `Log.output` 加引号 | String 参数直接写内容，编辑器会生成正确引号 |
| 把 Data 节点拖到 Flow 链里 | 纯 Data 节点没有 Flow 端口，只能连 Data 边 |
| 入口钉指向了错误节点 | 把正确节点移到最靠左上的位置，或确保其它节点已有 Flow 入边 |

---

## 完整示例：一个简单状态机片段

工程结构：

```text
[main].code
  └── main 线程
       └── main 标签
```

画布逻辑：

```text
入口钉 → Log("开始")
            ↓
         If (_state.Ecstasy >= 90)
       ┌────────┴────────┐
   True: Log("高")   False: Log("低")
```

生成 `.code`：

```code
var_main_thread = CreateThread("main")

main:
    thread = _this
    Log(output="开始")
    if _state.Ecstasy >= 90
        Log(output="高")
    else
        Log(output="低")
```

---

## 关于多标签、多线程、监听器

当前 UI 版本可以在工程树中**查看**线程 / 标签 / 监听器，但**暂不支持**在工程树中直接新建它们。因此本教程只使用默认的 `main` 标签。

如果你需要：

- 创建新标签 → 目前需要手动编辑 JSON 或等待后续 UI 更新。
- 创建新线程 → 目前需要手动编辑 JSON 或等待后续 UI 更新。
- 创建监听器 → 目前需要手动编辑 JSON 或等待后续 UI 更新。

相关格式请阅读 `docs/json_schema.md` 和 `docs/migration_guide.md`。

---

## 下一步

- 阅读 `docs/node_types.md` 了解所有节点分类和用途。
- 阅读 `docs/json_schema.md` 了解手动编辑多容器工程的方法。
- 阅读 `docs/migration_guide.md` 了解从旧版工程迁移到新架构的方法。
