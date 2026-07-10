# Custom Missions 2 `.code` DSL 权威参考

> **来源**：`docs/documentation.html`（官方英文 API 文档，Crisp2002）+
> `docs/examples/` 中 80+ 个前辈手搓 `.code` 的实战验证。
> 中文版 `documentation_zh.html` 为社区翻译，部分章节缺失，以英文原版为准。

---

## 1. 程序结构

### 入口标签

```code
main:
    thread = _this
    ...

step1:
    ...
```

- `main:` — 每个 `.code` 文件的入口标签
- 标签名后必须有 `:`，缩进用 **Tab**
- If/While/for 语句**不跟冒号**
- 多 `.code` 文件按文件名排序合并

### 线程（Thread）

```js
thread = _this                                  // 当前线程引用
child = CreateThread(labelName="childLabel")     // 创建并跳转
child.WaitForFinish()                            // 等待子线程结束
```

`_this` = 当前 `.code` 文件主线程的特殊引用。

### 监听器（Listener）

```js
listener = CreateListener("label")
listener = CreateListener("label", duration=3.0)
listener = CreateListenerLocal("label")           // 局部作用域
listener = null                                   // 销毁
```

`CreateListener` 每秒调用一次 `label`。设 `duration` 后，标签内可访问 `time` + `duration`。

### 跳转

```js
thread.Goto("step1")
thread.Goto("chatoutput", data=myObj, index=0, nextstep="end")
```

`Goto()` 可传递额外参数作为目标标签的局部变量。

---

## 2. 控制流

### if / elseif / else

```js
if value == 0                     // 无括号 = 更快（官方推荐）
    Log("zero")
elseif value < 0
    Log("negative")
else
    Log("positive")

// 有括号也可，但稍慢
if (value == 0)
    Log("zero")
```

- **`if` 后不跟冒号**（不同于 Python）
- 块通过 Tab 缩进区分
- 关键字 `elseif`（无空格）

### while

```js
while i >= 0
    Log(i)
    i = i - 1

// 可用 break 提前退出
while true
    if done
        break
```

### for…in

```js
for i in Range(5, 10)
    Log(i)

for item in myList
    Log(item)
```

- `Range(start, end)` 生成整数序列
- 可 `break`

### Foreach（遍历 + 条件停止）

```js
i = Foreach(myList, thread)
i = Foreach(items, roller)
```

`thread` 内可访问 `value`（当前元素）。当 `thread` 返回 `_result = false` 时停止遍历。

---

## 3. 操作符（完整表）

| 操作 | 符号 | 说明 |
|------|------|------|
| 指数 | `**` | |
| 逻辑非 | `!` | |
| 一元 ± | `+`, `-` | |
| 乘/除/整除/取模 | `*`, `/`, `//`, `%` | |
| 加/减 | `+`, `-` | |
| IN | `in` | |
| 比较 | `<`, `<=`, `>`, `>=` | |
| 等/不等 | `==`, `!=` | |
| 位与（全求值） | `&` | 两端都求值，null 访问会崩溃 |
| 异或 | `^` | |
| 位或（全求值） | `\|` | 两端都求值 |
| 逻辑与（短路） | `&&` | 左 false 时不求右 |
| 逻辑或（短路） | `\|\|` | 左 true 时不求右 |
| 字符串拼接 | `+` | `"prefix_" + key` |
| 赋值 | `=`, `+=`, `-=`, `*=`, `/=` | |

### `&` vs `&&` 的关键区别（来自官方文档 §Tips）

```js
if (list != null) & (list[0] == 1)     // 崩溃！& 求值两端，list==null 时 list[0] 报错
if (list != null) && (list[0] == 1)    // 正确：&& 短路，list==null 时不求右
if list != null                        // 最推荐：无括号 + 嵌套 if
    if list[0] == 1
        Log("works")
```

> **结论**：优先用 `&&` / `||`，仅在确定两边都安全时用 `&` / `|`。

---

## 4. 全局变量

### 内置全局

| 变量 | 类型 | 说明 |
|------|------|------|
| `_state` | Object | 玩家状态（只读，用函数修改） |
| `_stagechanged` | Boolean | 当前帧场景是否切换 |
| `_timediff` | Number | 上一帧到当前帧时间差（考虑慢速/暂停） |
| `_time` | Number | 累计时间（扣除暂停） |
| `_save` | Object | 跨会话持久存储（不能存 Object，仅基本类型+List） |
| `_settings` | Object | meta.json 设置菜单中的值 |
| `_mod` | List | 共享给其他 mod 的数据（仅 List） |
| `_mods` | Object | 所有已激活 mod 的 `_mod` 数据 |
| `_name` | String | 当前项目文件夹名 |

### `_state` 完整结构

```markdown
_state
├── DateTime              Boolean
├── Blindfolded           Boolean
├── Peeing               Boolean
├── Moving               Boolean
├── Dashing              Boolean
├── Crouching            Boolean
├── Sitting              Boolean
├── InLight              Boolean
├── Orgasm               Boolean
├── Bukkake              Boolean
├── NearNPC              Boolean
├── ShowingOff           Boolean
├── Watched              Boolean
├── Action               String
├── Futanari             Boolean
├── Invisible            Boolean
├── InOpenToilet         Boolean
├── Bodypaint            Number (0 = none)
├── DayTime              Boolean
├── NPCArea              String (or null)
├── FirstPerson          Boolean
├── Ecstasy              Number
├── Detection            Number
├── Rank                 Number
├── Vibrator             String (Off/Low/High/Random)
├── Piston               String (Off/Low/Medium/High/Random)
├── RpBonus              Number
├── HeartRate            Number
├── Stamina              Number
├── StaminaMax           Number
├── Moisture             Number
├── FoundNPC             Number (发现玩家的 NPC ID，-1 = 无)
├── GameOver             Boolean
├── Handcuffs
│   ├── State            Boolean
│   └── Type             String (nil/Handcuff/KeyHandcuff/TimerHandcuff)
├── Exposed
│   ├── None             Boolean
│   ├── Front            Boolean
│   ├── Upper            Boolean
│   ├── HipCrouch        Boolean
│   ├── Hip              Boolean
│   └── All              Boolean
├── Position
│   ├── stage            String
│   ├── laststage        String
│   ├── x, y, z          Number
│   └── rx, ry, rz, rw   Number (rotation quaternion)
├── Camera
│   ├── x, y, z          Number
│   ├── rx, ry, rz, rw   Number
│   ├── pitch            Number
│   └── yaw              Number
├── CameraTarget
│   ├── Face  {x, y, z}
│   ├── Body  {x, y, z}
│   └── Crotch{x, y, z}
├── Cosplay
│   ├── [cosplayName]    Boolean (true=正穿着)
│   └── [0], [1], ...    String (当前穿戴的 cosplay 名)
├── Skills
│   └── [skillName]      Boolean (true=已启用)
├── DroppedItems [0..n]
│   ├── Type             String
│   └── Position {...}
├── AdultToys
│   └── [toyName]        Boolean (存在 = 已装备)
├── Items
│   └── [itemName]       Number (物品数量)
├── Missions
│   ├── Completed        Number
│   ├── Count            Number
│   ├── [MissionID]      Number (通关次数)
│   └── [stage].Completed, .Count
├── CurrMissions
│   ├── Completed, Count
│   └── [MissionID]      Number (0-1 进度)
├── Coat
│   ├── Dropped          Boolean
│   ├── Front            String (Closed/Open1/Open2/None)
│   └── Back             String (Closed/Open/None)
└── NPCs [0..n]
    ├── ID               Number
    ├── Position {...}
    ├── SeesPlayer       Boolean
    ├── SeesFlashing     Boolean
    ├── Headset          Boolean
    ├── Glasses          Boolean
    ├── Smartphone       Boolean
    ├── AvatarType       String
    ├── FixedType        String
    ├── Sitting          Boolean
    └── State            String
```

---

## 5. Condition 对象（`CreateCondition`）

官方 `MakeCond` 微型 DSL：

```code
CreateCondition("单个条件")                    // 无括号 = 单条件
CreateCondition("[A, B, C]")                  // [...] = AND
CreateCondition("(A, B, C)")                  // (...) = OR
CreateCondition("[A, (B, C), !D]")            // 嵌套 + !NOT
```

条件关键词完整表 → 见 `docs/if_condition_design.md` 附录。

### 条件对象用法

```js
cond = CreateCondition("[Exposed_Front, !Crouching, IsDayTime]")
cond.Check()          // → Boolean

area = CreateArea(type="sphere", stage="Residence", x=0, y=0, z=0, r=1000)
gallery = CreateGallery(condition=cond, area=area)
```

---

## 6. 代码生成对照

### 生成格式

| 节点 | 当前 CM2Editer 生成 | 正确 DSL 语法 | 对齐状态 |
|------|--------------------|--------------|---------|
| If | `if {condition}` (小写 if + 缩进) | `if condition` | ✅ 已对齐 |
| While | `while {condition}` (小写 + 缩进) | `while condition` | ✅ 已对齐 |
| For | `for i in {iterable}` | `for i in Range(a, b)` | ⚠️ 仅支持列表遍历，Range 未实现 |
| Break | `break` | `break` | ✅ 已对齐 |
| Data 变量 | `var_{node_id}_{port}` | `var_{node_id}_{port}` | ✅ 已对齐 |
| Data 值引用 | 递归解析：`evaluate_data_output` | 内联表达式 | ✅ 已对齐 |
| 标签 | `var_X_thread = CreateThread("X")` (顶层) | `var = CreateThread("X")` | ✅ 已对齐 |
| _result 收尾 | 自动追加，Return 已有时跳过 | `_result = null` | ✅ 已对齐 |

### 表达式生成规则

```code
if condition 中的 condition 值：
  Boolean 字面量 → "true" / "false"
  _state 变量   → "_state.Futanari"
  AdultToys     → "_state.AdultToys.Vibrator != null"
  数值比较      → "_state.Ecstasy >= 50"
  逻辑组合      → "(_state.InLight) && (_state.AdultToys.Handcuff != null)"
  条件对象      → "cond.Check()"
```

> **if 生成格式建议**：`if {condition}` 换行 → Tab → 分支体 → 空行 → `elseif` 或 `else`

---

## 7. 数据结构 API

### 创建

```js
myList = CreateList()                            // 空
myList = CreateList("a", "b", "c")               // 初始化
myDict = CreateThread("StaticDict")              // 静态字典
mySet = CreateThread("RecordSet")                // 去重集合
myQueue = CreateThread("Queue")                  // 队列
myStack = CreateThread("Stack")                  // 栈
```

### 列表

```js
myList[0] = value                        // 索引写
myList.Insert("item")                    // 尾部追加
myList.Remove(key)                       // 删除
myList.Contains("item")                  // 包含 → Boolean
myList.Count()                           // 长度
myList.GetKeys() / GetValues()           // 键/值列表
myList.Clear()                           // 清空
```

### 队列

```js
q.Enqueue(item) / q.Dequeue() / q.GetCount()
```

### 数学

```js
Random(min, max)          // 随机浮点
Trunc(value)              // 截断
Abs(value)                // 绝对值
```

---

## 8. 中英文档差异

中文翻译 `documentation_zh.html` 缺失章节：

| 缺失 | 说明 |
|------|------|
| **`EventListener`** | 事件监听器对象（不同于 Listener） |
| **`Common` 对象方法** | 所有 Object 共用的方法 |
| **`Inter-Mod Tutorial`** | 跨 mod 通信（`_mod`, `_mods`）+ 自定义函数注入 |
| **`Audio` 独立章节** | 全局音频函数 |
| **`for...in` 循环** | 范围/列表遍历（中文版未提） |
| **`break` 关键字** | 循环中断（中文版未提） |
| **操作符 `**`, `//`, `%`, `^`, `in`** | 中文版缺失 |
| **`&` vs `&&` 短路差异** | 官方有专门 Tips 解释 |
| **if 括号性能差异** | 官方 Tips："无括号更快" |
