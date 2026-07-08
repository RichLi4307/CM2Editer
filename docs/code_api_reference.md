# Custom Missions 2 `.code` DSL 速查参考

> 基于 `docs/examples/` 中前辈手搓的 80+ 个 `.code` 文件反推的 DSL 语法全集。
> 本文档是 CM2Editer 代码生成的**权威参考**。

---

## 1. 程序结构

### 入口标签

```
main:
    thread = _this
    ...

其他标签:
    step1:
    __generate:
    listener_callback:
```

- `main:` — 每个 `.code` 文件的入口
- 标签名后必须有 `:`，内容用缩进表示作用域
- 多 `.code` 文件按文件名排序合并执行

### 线程（Thread）

```
thread = _this                    -- 获取当前线程引用
child = CreateThread("label")     -- 创建新线程，跳转到 label
local = CreateThread("label", local=true)  -- 创建本地作用域线程
```

`_this` = 当前 `.code` 文件主线程的特殊引用。

### 监听器（Listener）

```
listener = CreateListener("label")
listener = CreateListener("label", duration=3.0)
listener = CreateListenerLocal("label")           -- 局部作用域
```

- `CreateListener("label")` — 启动监听器，每秒调用 `label` 一次
- 设置 `duration` 后，`label` 内可访问 `time` 和 `duration` 变量
- 设置 `listener = null` 可销毁监听器

### Gotcha / 跳转

```
thread.Goto("label")
thread.Goto("label", data=myObject, index=0, nextstep="nextLabel")
```

- `Goto()` 可传递额外参数作为子标签的局部变量
- `nextstep` = 执行完数据处理后跳转到哪个标签（常见于 `chatoutput` 模式）

---

## 2. 控制流

### if / elseif / else

```lua
if condition
    ...

if _state.Position.stage != "Apart"
    if _state.Position.laststage == "Apart"
        ...

if (_save.Bra != null) | (_save.Pants != null)
    ...
else
    listener_bra = null

if (worn_condition != null) & (drop < 0)
    ...
elseif drop >= 0
    ...
elseif equip >= 0
    ...
```

- `if` 后可直接跟表达式，复杂表达式用 `()` 括起
- `elseif` 和 `else` 是标准关键字
- **没有显式 `end`**——作用域通过缩进和标签跳转确定
- 逻辑与/或用 `&` / `|`（单字符）。部分作者用 `&&` / `||`（双字符），CM2 加载器二者均接受
- 取反用 `!`

### while

```lua
while i < len
    ...
    i += 1
```

### Foreach（遍历）

```lua
i = Foreach(myList, thread)
    -- thread 中的 'value' = 当前元素
    -- 当 thread 返回 _result = false 时停止
```

### 三元 / 内联条件

```lua
_result = condition && trueValue || falseValue
```

---

## 3. 条件系统（CreateCondition）

### MakeCond 语法

```
CreateCondition("单个条件")                    -- 无括号 = 单个条件
CreateCondition("[条件1, 条件2]")              -- 方括号 = AND
CreateCondition("(条件1, 条件2)")              -- 圆括号 = OR
CreateCondition("[条件1, 条件2, !取反条件]")    -- ! 前缀 = NOT
CreateCondition("[条件1, (条件2, 条件3)]")      -- 嵌套
```

**动态构造条件**（字符串拼接）：

```lua
s = ""
while i < items.Count()
    if s != ""
        s = s + ","
    s = s + "Cosplay_" + items[i]
    i = i + 1
condition = CreateCondition("[" + s + "]")       -- AND 所有服装
condition = CreateCondition("(" + s + ")")       -- OR 任一服装
```

### 条件关键词

#### 玩家动作

| 关键词 | 说明 |
|--------|------|
| `Moving` | 移动中 |
| `Crouching` | 蹲伏中 |
| `Peeing` | 排泄中 |
| `Dashing` | 跑动中 |
| `Sitting` | 坐着 |
| `Action_xxx` | 指定动作（如 `Action_UseDildoWallPussy1`） |
| `Orgasm` | 高潮中 |
| `GameOver` | 游戏结束 |

#### 暴露状态

| 关键词 | 说明 |
|--------|------|
| `Exposed_None` | 无暴露 |
| `Exposed_Front` | 正面暴露 |
| `Exposed_Upper` | 上身暴露 |
| `Exposed_HipCrouch` | 蹲伏时臀部暴露 |
| `Exposed_Hip` | 臀部暴露 |
| `Exposed_All` | 完全暴露 |

#### 衣装

| 关键词 | 说明 |
|--------|------|
| `CoatDropped` | 外套脱下 |
| `CoatFrontClosed` | 前面系好 |
| `CoatFrontOpen1` | 前面半开 |
| `CoatFrontOpen2` | 前面打开 |
| `CoatBackClosed` | 后面系好 |
| `CoatBackOpen` | 后面打开 |

#### 拘束 / 装备

| 关键词 | 说明 |
|--------|------|
| `Blindfolded` | 蒙眼 |
| `NoHandcuffs` | 无手铐 |
| `HandcuffsBack` | 手铐反铐 |
| `HandcuffsObject` | 手铐绑物体 |
| `NormalHandcuffs` | 普通手铐 |
| `KeyedHandcuffs` | 钥匙手铐 |
| `TimedHandcuffs` | 计时手铐 |

#### 振动器 / 活塞

| 关键词 | 说明 |
|--------|------|
| `VibrationOff` | 跳蛋关闭 |
| `VibrationLow` | 跳蛋低档 |
| `VibrationHigh` | 跳蛋高档 |
| `VibrationRandom` | 跳蛋随机 |
| `PistonOff` | 活塞关闭 |
| `PistonLow` | 活塞低档 |
| `PistonMedium` | 活塞中档 |
| `PistonHigh` | 活塞高档 |
| `PistonRandom` | 活塞随机 |

#### 玩具装备

| 关键词 | 说明 |
|--------|------|
| `AdultToy_AnalPlug` | 肛塞 |
| `AdultToy_Vibrator` | 跳蛋 |
| `AdultToy_EyeMask` | 眼罩 |
| `AdultToy_Handcuff` | 手铐 |
| `AdultToy_KeyHandcuff` | 钥匙手铐 |
| `AdultToy_TimerHandcuff` | 计时手铐 |
| `AdultToy_TitRotor` | 乳头跳蛋 |
| `AdultToy_KuriRotor` | 阴蒂跳蛋 |
| `AdultToy_PistonFuta` | 扶她活塞 |
| `AdultToy_PistonAnal` | 肛门活塞 |
| `AdultToy_PistonPussy` | 阴道活塞 |

#### 环境

| 关键词 | 说明 |
|--------|------|
| `IsDayTime` | 白天 |
| `InLight` | 在光照区 |
| `InOpenToilet` | 在开放厕所 |
| `NearNPC` | 靠近 NPC |
| `FPCamera` | 第一人称视角 |

#### 时装 / 技能

| 关键词 | 说明 |
|--------|------|
| `Cosplay_xxx` | 穿着指定 Cosplay（如 `Cosplay_m_cosplay_succubus_cosplay_horn`） |
| `OwnsCosplay_xxx` | 拥有指定 Cosplay |
| `Skill_xxx` | 技能已启用 |
| `Item_xxx` | 拥有物品数量 > 0 |

#### 拍照数据

可在 `gallery_callback` 中检查照片元数据，判断条件关键字与上述一致（如 `data.Futanari`、`data.Exposed.Hip`、`data.Vibrator`）。

### 数值条件
```
某些条件可用 >, >=, <, <=, ==, != 与数字比较——CreateCondition 中不常用，主要在 if/while 表达式中直接使用 _state 值比较。
```

---

## 4. 游戏状态访问 (`_state`)

### 布尔状态

| 表达式 | 说明 |
|--------|------|
| `_state.Futanari` | 扶她状态 |
| `_state.Sitting` | 坐姿 |
| `_state.Orgasm` | 高潮中 |
| `_state.Moving` | 移动中 |
| `_state.Crouching` | 蹲伏中 |
| `_state.Peeing` | 排泄中 |
| `_state.Dashing` | 跑动中 |
| `_state.InLight` | 在光照区 |
| `_state.NearNPC` | 附近有人 |
| `_state.Watched` | 被注视 |
| `_state.ShowingOff` | 露 |
| `_state.Bukkake` | 射精 |
| `_state.Blindfolded` | 蒙眼 |
| `_state.Invisible` | 隐身 |
| `_state.InOpenToilet` | 在开放厕所 |
| `_state.Bodypaint` | 身体涂鸦 |
| `_state.FPCamera` | 第一人称视角 |
| `_state.GameOver` | 游戏结束 |
| `_state.FirstPerson` | 第一人称 |

### 数值状态

| 表达式 | 说明 |
|--------|------|
| `_state.Ecstasy` | 快感值 |
| `_state.Detection` | 侦测度 |
| `_state.Rank` | 等级 |
| `_state.HeartRate` | 心率 |
| `_state.Stamina` | 体力 |
| `_state.StaminaMax` | 最大体力 |
| `_state.Moisture` | 湿润度 |

### 复杂状态

| 表达式 | 说明 |
|--------|------|
| `_state.Position.stage` | 当前场景名 |
| `_state.Position.laststage` | 上一帧的场景名 |
| `_state.Position.x/y/z` | 坐标 |
| `_state.Position.rx/ry/rz/rw` | 旋转 |
| `_state.Action` | 当前动作名 |
| `_state.Skills[key]` | 技能是否启用（key = 技能名） |
| `_state.Cosplay[key]` | 指定服装是否已装备（`== true` / `!= true`） |
| `_state.AdultToys[key]` | 成人玩具状态（key = 玩具名，null = 未拥有） |
| `_state.DayTime` | 晴天/白天布尔值（通过 gallery callback 的 data.DayTime 访问） |
| `_stagechanged` | 场景切换标记 |
| `_timediff` | 上一帧到当前帧的时间差（秒，用于计时器累加） |

### 字符串

```lua
s = s + ","
s = s + "Cosplay_" + items[i]
condition = CreateCondition("[" + s + "]")
condition = CreateCondition("(" + s + ")")
```

- 字符串拼接使用 `+` 操作符
- 字符串常量用双引号 `"..."` 括起
- 单行注释用 `#` 开头

---

## 5. 代码生成规则（CM2Editer 兼容）

### 已验证的生成语法

```
main:
    Log(output="hello")
    _result = null

childThread:
    var_thr_out = CreateThread(labelName="child")
```

- `If(true) [` — 当前 CM2Editer 生成的 `If` 语法（大写 I + 方括号），**与前辈手写的 `if ()` 是不同语法**，两者可能共存或为版本差异。为确保兼容，继续使用 `If(true) [` 格式生成，待测试验证后决定是否迁移到 `if () ` 风格。
- 条件表达式的**值**可与手写 DSL 通用：`true`、`false`、`_state.Futanari`、`_state.Ecstasy >= 50`

### 条件表达式值类型

| If condition 参数字面量 | 生成结果 | 是否有效 |
|-------------------------|----------|----------|
| `true` / `false` | `If(true)` / `If(false)` | ✅ 已验证（测试夹具） |
| `_state.Futanari` | `If(_state.Futanari)` | ❓ 待实际游戏测试 |
| `_state.Ecstasy >= 50` | `If(_state.Ecstasy >= 50)` | ❓ 待测试 |
| `_state.Bodypaint > 0 && !_state.Invisible` | `If(_state.Bodypaint > 0 && !_state.Invisible)` | ❓ 待测试 |

---

## 6. 数据结构与 API

### 创建

```lua
myList = CreateList()                           -- 空列表
myList = CreateList("a", "b", "c")              -- 初始化
myDict = CreateThread("StaticDict")             -- 静态字典
mySet = CreateThread("RecordSet")               -- 集合（去重）
myQueue = CreateThread("Queue")                 -- 队列
```

### 列表操作

```lua
myList[0] = value                               -- 索引赋值
myList.Insert("item")                           -- 尾部追加
myList.Remove(key)                              -- 删除
myList.Contains("item")                         -- 包含检查 → Boolean
myList.Count()                                  -- 长度 → Number
myList.GetKeys()                                -- 获取所有键
myList.GetValues()                              -- 获取所有值
myList.Clear()                                  -- 清空
```

### 队列操作

```lua
queue.Enqueue(item)                              -- 入队
queue.Dequeue()                                  -- 出队
queue.GetCount()                                 -- 大小
```

### 数学

```lua
Random(min, max)                                 -- 随机浮点
Trunc(value)                                     -- 截断小数
```

### 拍照 API

```lua
gallery = CreateGallery()
gallery = CreateGallery("callback", condition, area)
gallery.Show()                                   -- 打开拍照
gallery.Confirmed()                              -- 用户确认 → Boolean
gallery.GetSelection()                           -- 获取选中照片列表

-- callback 每帧调用：
gallery_callback:
    if data.Futanari                              -- 检查照片元数据
        _result = true                            -- true = 照片符合条件
```

---

## 7. 操作符汇总

| 类别 | 操作符 | 备注 |
|------|--------|------|
| 逻辑 AND | `&` 或 `&&` | 二者均被 CM2 接受 |
| 逻辑 OR | `\|` 或 `\|\|` | 二者均被 CM2 接受 |
| 逻辑 NOT | `!` | |
| 比较 | `==`, `!=`, `>`, `>=`, `<`, `<=` | |
| 算术 | `+`, `-`, `*`, `/`, `+=`, `-=` | |
| 字符串拼接 | `+` | `"Cosplay_" + items[i]` |
| 空检查 | `!= null`, `== null` | |
| 布尔常量 | `true`, `false` | |

---

## 8. 持久存储 `_save`

```lua
_save.Bra = worn_items          -- 保存到存档
if _save.Bra == null             -- 检查是否存在
    ...
_save.Bra = null                 -- 清除
```

- `_save` 是跨游戏会话持久化的全局字典
- 键名自定义（如 `_save.Bra`、`_save.Pants`）
- 退出/重进游戏后数据仍在
- 用于记录掉落物状态、任务进度等

---

## 9. CM2Editer 节点映射速查

### 已实现

| CM2 函数 | CM2Editer 节点 |
|----------|---------------|
| `CreateThread(labelName="x")` | CreateThread |
| `CreateListener("x")` | CreateListener |
| `thread.Goto("x")` | Goto |
| `Log(output="x")` | Log |
| `SetEvent(name="x", value=y)` | SetEvent |
| `GetEvent("x")` | GetEvent |
| `CreateCondition(condition="[x]", id="y")` | CreateCondition |
| `CreateArea(type="x", stage="y", ...)` | CreateArea |
| `CreateGallery("x", condition, area)` | CreateGallery |
| `PlaySoundEffect("x")` | PlaySoundEffect |

### 生成语法待对齐

| 项目 | 当前 CM2Editer 生成 | 前辈 `.code` 实际使用 | 验证状态 |
|------|--------------------|---------------------|---------|
| If 分支 | `If(true) [` | `if condition`（小写 if + 缩进） | ❌ 需要对齐 |
| While 循环 | `While(true) [` | `while i < len`（小写） | ❌ 需要对齐 |
| 逻辑与 | 无（未生成） | `&` 或 `&&` | ⭕ Phase 1 引入 |
| 逻辑或 | 无 | `\|` 或 `\|\|` | ⭕ Phase 1 引入 |
| 字符串拼接 | 无 | `"prefix_" + key` | ⭕ Phase 1 引入 |

> **结论**：CM2Editer 的 `If(true) [` 和 `While(true) [` 语法在 80+ 个前辈项目中**零使用**。所有 `.code` 实例均使用小写 `if` / `while` + 缩进块。代码生成器需要从大写函数式语法迁移到小写语句式语法。详见 `docs/if_condition_design.md`。
