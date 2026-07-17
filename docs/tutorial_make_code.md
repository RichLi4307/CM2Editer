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
- 纯数据节点（如 `GetStateNumber`）没有 Flow 端口，只通过 Data 虚线给其它节点喂值。
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

## 第五步：读取角色状态（例如 Rank 等级）

> 目标：读取当前角色经验等级 `_state.Rank` 并输出

| 步骤 | 操作 |
|------|------|
| 1 | 拖 **GetStateNumber** 数据节点到画布 |
| 2 | 属性面板 → `stateKey` 选择 `Rank`（角色等级/经验） |
| 3 | 拖一个 **Log** |
| 4 | 用 Data 边连接 `GetStateNumber.out_value` → `Log.output` |

生成结果：

```code
main:
    thread = _this
    Log(output=_state.Rank)
```

类似来源：

- `_state.Rank` / `_state.Ecstasy` / `_state.Detection` / `_state.HeartRate` → **GetStateNumber**（下拉框可选）
- `_state.Futanari` / `_state.Sitting` / `_state.Orgasm` → **GetStateBool**（下拉框可选）
- 持有货币 RP → **GetCurrentRP** / **AddCurrentRP** / **SetCurrentRP**
- 本次外出赚取 RP → **GetCurrentEarnRP** / **AddCurrentEarnRP** / **SetCurrentEarnRP**
- `_time` → **GetTime**
- `_timediff` → **GetTimeDiff**

> 注意：
> - `GetSave` / `GetSettings` / `GetMod` / `GetMods` 返回对象/列表，需要指定键或字段拆分。`GetSave` 现在支持 `key` 参数生成 `_save.key`，但存档不是实时写入的，不适合读取当前分数或经验。
> - 角色经验/等级没有专门的“增加经验”节点，一般通过 `_state.Rank` 读取等级，或用 `SetPlayerData(dataName="Rank", value=...)` 尝试设置（具体是否生效取决于游戏内部实现）。

---

## 第六步：保存与导出

- **保存工程**（`Ctrl+S` 或按钮）：保存项目结构和 JSON 图到工程目录。
- **导出工程**：弹出文件夹选择，选择你的 `CustomMissions2` 目录后，生成 `.code` 文件到该目录。

导出后的 `.code` 文件即可被游戏加载。

---

## 第七步：多条件组合判断

> 目标：当 **完全暴露** 且 **穿着指定 Cosplay** 且 **快感值 >= 90** 时，才执行后续操作。

### 7.1 检查“完全暴露”

1. 拖 **创建条件**（`CreateCondition`）到画布。
2. 属性面板 → `condition` 选择 `Exposed_All`。
3. 拖 **检查条件**（`CheckCondition`）。
4. 用 Data 边连接 `CreateCondition.out_condition` → `CheckCondition.cond`。
5. 把 `CreateCondition` 接入 Flow 链（它有 `in_flow` / `out_flow`）。

### 7.2 检查“穿着某件 Cosplay”

1. 拖 **检查服装**（`CheckCosplay`）。
2. 属性面板 → `cosplayKey` 输入服装键，例如 `Bunny`。

### 7.3 检查“快感值 >= 90”

1. 拖 **GetStateNumber**，`stateKey` 选 `Ecstasy`。
2. 拖 **CompareNumbers**，`b` = `90.0`，`operator` = `>=`。
3. 用 Data 边连接 `GetStateNumber.out_value` → `CompareNumbers.a`。

### 7.4 把三个条件合并成一个布尔

1. 拖两个 **逻辑与**（`LogicAnd`）。
2. 连接：
   - `CheckCondition.out_result` → `LogicAnd1.a`
   - `CheckCosplay.out_value` → `LogicAnd1.b`
   - `LogicAnd1.out_result` → `LogicAnd2.a`
   - `CompareNumbers.out_result` → `LogicAnd2.b`
3. 拖一个 **If**，把 `LogicAnd2.out_result` → `If.condition`。

生成结果（示意）：

```code
main:
    thread = _this
    Log(output="开始")
    var_cond = CreateCondition(condition="Exposed_All")
    if (var_cond.Check()) && (Cosplay_Bunny) && (_state.Ecstasy >= 90.0)
        ...
    else
        ...
```

> 注意：
> - `CreateCondition` 是 Flow 节点，必须放在 Flow 链里才会生成变量。
> - `CheckCondition` 是 Data 节点，把条件对象转成布尔值，才能接入 `LogicAnd` / `If`。
> - 实际变量名（如 `var_cond`）由节点 ID 自动生成，不必手动写。

---

## 第八步：修改游戏数据（增加 RP）

> 目标：满足上述三个条件时，增加 **10 点持有 RP**。

| 步骤 | 操作 |
|------|------|
| 1 | 拖 **增加持有 RP**（`AddCurrentRP`）到 If 的 True 分支后方 |
| 2 | 属性面板 → `value` 输入 `10` |
| 3 | Flow 边连接 `If.out_true` → `AddCurrentRP.in_flow` |
| 4 | 若需要继续后续逻辑，从 `AddCurrentRP.out_flow` 连向下一个节点 |

生成结果：

```code
    if (var_cond.Check()) && (Cosplay_Bunny) && (_state.Ecstasy >= 90.0)
        AddCurrentRP(value=10)
    else
        Log(output="条件不满足")
```

> 类似节点：
> - **AddCurrentEarnRP** — 增加本次外出赚取的 RP。
> - **SetCurrentRP** / **SetCurrentEarnRP** — 直接设置，而不是增加。

---

## 第九步：只执行一次，不要每帧加分

Flow 链里的节点只会在 Flow 到达它时执行一次。所以上面的 `AddCurrentRP` 在 `main` 标签的 Flow 链里只会加一次分。

**但如果你把它放进监听器（Listener）容器，监听器每帧都会执行，RP 就会每帧都增加。** 这是最常见的新手错误。

当前 UI 暂不支持新建监听器，因此你基本不会遇到这个问题。但如果以后你手动创建监听器，请务必加“只执行一次”守卫：

1. 拖 **Boolean** 节点，设为 `false`。
2. 拖 **SetVariable**，`name` 填 `reward_given`；用 Data 边把 `Boolean.out_value` 连到 `SetVariable.value`。
3. 在加分前，拖 **Variable**，`name` 填 `reward_given`。
4. 拖 **LogicNot**，把 `Variable.out_value` 连到 `LogicNot.a`。
5. 用 **LogicAnd** 把 `LogicNot.out_result` 与条件结果组合，作为 `If.condition` 的输入。
6. 加分后，再拖一个 **Boolean** 节点设为 `true`，连到另一个 **SetVariable(name=reward_given)** 的 `value` 端口，把 `reward_given` 设为 `true`。

守卫逻辑示意：

```text
入口钉 → SetVariable(reward_given=false)
            ↓
         Log("开始")
            ↓
         CreateCondition(Exposed_All)
            ↓
         If (条件 && !reward_given)
        ┌───────────┴───────────┐
   True: AddCurrentRP(value=10)   False: 跳过
            ↓
         SetVariable(reward_given=true)
```

生成 `.code`（示意）：

```code
main:
    thread = _this
    reward_given = false
    Log(output="开始")
    var_node_xxx = CreateCondition(condition="Exposed_All")
    if (var_node_xxx.Check()) && (Cosplay_Bunny) && (_state.Ecstasy >= 90.0) && (!reward_given)
        AddCurrentRP(value=10)
        reward_given = true
```

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
| `CreateCondition` 后直接接 `If` | 需要 `CheckCondition` 把条件对象转成布尔值再连 `If` / `LogicAnd` |
| 在监听器里加分不做守卫 | 监听器每帧执行，必须加布尔变量保证只加一次 |

---

## 完整示例：条件奖励

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
         CreateCondition(Exposed_All)
            ↓
         If ((cond.Check()) && (Cosplay_Bunny) && (_state.Ecstasy >= 90.0))
        ┌──────────────────┴──────────────────┐
   True: AddCurrentRP(value=10)             False: Log("条件不满足")
```

生成 `.code`（变量名示意）：

```code
var_main_thread = CreateThread("main")

main:
    thread = _this
    Log(output="开始")
    var_node_xxx = CreateCondition(condition="Exposed_All")
    if (var_node_xxx.Check()) && (Cosplay_Bunny) && (_state.Ecstasy >= 90.0)
        AddCurrentRP(value=10)
    else
        Log(output="条件不满足")
```

---

## 完整示例：只执行一次的奖励

如果你担心这段逻辑会被反复触发，可以加一个守卫变量：

```text
入口钉 → SetVariable(reward_given=false)
            ↓
         Log("开始")
            ↓
         CreateCondition(Exposed_All)
            ↓
         If (条件 && !reward_given)
        ┌───────────┴───────────┐
   True: AddCurrentRP(value=10)   False: 跳过
            ↓
         SetVariable(reward_given=true)
```

生成 `.code`（示意）：

```code
main:
    thread = _this
    reward_given = false
    Log(output="开始")
    var_node_xxx = CreateCondition(condition="Exposed_All")
    if (var_node_xxx.Check()) && (Cosplay_Bunny) && (_state.Ecstasy >= 90.0) && (!reward_given)
        AddCurrentRP(value=10)
        reward_given = true
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
