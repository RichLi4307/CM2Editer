# 节点详细说明

本文档提供每个节点的中文详细介绍，比官方文档更适合编辑器属性面板展示。

## 目录

- **A**
  - [Abs](#abs)
  - [Acos](#acos)
  - [AddCurrentEarnRP](#addcurrentearnrp)
  - [AddCurrentRP](#addcurrentrp)
  - [AddEcstasy](#addecstasy)
  - [AddItemCount](#additemcount)
  - [AddMoisture](#addmoisture)
  - [AddStamina](#addstamina)
  - [Asin](#asin)
  - [Atan](#atan)

- **B**
  - [Boolean](#boolean)
  - [Break](#break)
  - [BreakVector](#breakvector)

- **C**
  - [CallFunction](#callfunction)
  - [CallMethod](#callmethod)
  - [CanGameOver](#cangameover)
  - [Ceil](#ceil)
  - [CheckCondition](#checkcondition)
  - [CheckCosplay](#checkcosplay)
  - [CheckEquipment](#checkequipment)
  - [CollectItem](#collectitem)
  - [Color](#color)
  - [Comment](#comment)
  - [CompareNumbers](#comparenumbers)
  - [Copy](#copy)
  - [Cos](#cos)
  - [CreateArea](#createarea)
  - [CreateAudio](#createaudio)
  - [CreateCondition](#createcondition)
  - [CreateGallery](#creategallery)
  - [CreateInput](#createinput)
  - [CreateInteractArea](#createinteractarea)
  - [CreateItemCondition](#createitemcondition)
  - [CreateList](#createlist)
  - [CreateListFromJson](#createlistfromjson)
  - [CreateListener](#createlistener)
  - [CreateListenerLocal](#createlistenerlocal)
  - [CreateMessengerChat](#createmessengerchat)
  - [CreateMissionMenuItem](#createmissionmenuitem)
  - [CreateMissionPanel](#createmissionpanel)
  - [CreateNPC](#createnpc)
  - [CreateSnapshot](#createsnapshot)
  - [CreateText](#createtext)
  - [CreateThread](#createthread)
  - [CreateZone](#createzone)

- **D**
  - [DeactivateSex](#deactivatesex)
  - [DeleteSnapshot](#deletesnapshot)
  - [DestroyListener](#destroylistener)
  - [DropItem](#dropitem)
  - [DumpVariable](#dumpvariable)
  - [DumpVariables](#dumpvariables)

- **E**
  - [EquipAdultToy](#equipadulttoy)
  - [EquipCosplay](#equipcosplay)

- **F**
  - [FileExists](#fileexists)
  - [Find](#find)
  - [Floor](#floor)
  - [For](#for)
  - [ForeachNode](#foreachnode)
  - [Format](#format)

- **G**
  - [GetAllSnapshots](#getallsnapshots)
  - [GetAllWaypoints](#getallwaypoints)
  - [GetCurrentEarnRP](#getcurrentearnrp)
  - [GetCurrentRP](#getcurrentrp)
  - [GetCurrentThread](#getcurrentthread)
  - [GetEcstasy](#getecstasy)
  - [GetEvent](#getevent)
  - [GetFileExtension](#getfileextension)
  - [GetFiles](#getfiles)
  - [GetGraphicsOption](#getgraphicsoption)
  - [GetImageReference](#getimagereference)
  - [GetItemCount](#getitemcount)
  - [GetLanguage](#getlanguage)
  - [GetMod](#getmod)
  - [GetMods](#getmods)
  - [GetMoisture](#getmoisture)
  - [GetPosition](#getposition)
  - [GetRandomPosition](#getrandomposition)
  - [GetSave](#getsave)
  - [GetSettings](#getsettings)
  - [GetSkillShortcut](#getskillshortcut)
  - [GetSnapshotData](#getsnapshotdata)
  - [GetStageRankLimit](#getstageranklimit)
  - [GetStamina](#getstamina)
  - [GetStateBool](#getstatebool)
  - [GetStateNumber](#getstatenumber)
  - [GetTime](#gettime)
  - [GetTimeDiff](#gettimediff)
  - [GetType](#gettype)
  - [Global](#global)
  - [Goto](#goto)
  - [Group](#group)

- **I**
  - [If](#if)

- **L**
  - [Length](#length)
  - [Local](#local)
  - [LockHandcuffs](#lockhandcuffs)
  - [Log](#log)
  - [Log10](#log10)
  - [Log2](#log2)
  - [LogN](#logn)
  - [LogicAnd](#logicand)
  - [LogicNot](#logicnot)
  - [LogicOr](#logicor)
  - [Lower](#lower)

- **M**
  - [MakeVector](#makevector)
  - [Max](#max)
  - [Meta](#meta)
  - [Min](#min)

- **N**
  - [NumberConstant](#numberconstant)

- **O**
  - [OwnCosplay](#owncosplay)

- **P**
  - [PlaySoundEffect](#playsoundeffect)

- **Q**
  - [Quaternion](#quaternion)

- **R**
  - [Random](#random)
  - [RandomInt](#randomint)
  - [Range](#range)
  - [Return](#return)
  - [Round](#round)

- **S**
  - [SetAction](#setaction)
  - [SetCamera](#setcamera)
  - [SetCurrentEarnRP](#setcurrentearnrp)
  - [SetCurrentRP](#setcurrentrp)
  - [SetEcstasy](#setecstasy)
  - [SetEvent](#setevent)
  - [SetFutanari](#setfutanari)
  - [SetGraphicsOption](#setgraphicsoption)
  - [SetItemCount](#setitemcount)
  - [SetMoisture](#setmoisture)
  - [SetPiston](#setpiston)
  - [SetPlayerData](#setplayerdata)
  - [SetPlayerPosition](#setplayerposition)
  - [SetPortalEnabled](#setportalenabled)
  - [SetSexMenu](#setsexmenu)
  - [SetSexPosition](#setsexposition)
  - [SetSkill](#setskill)
  - [SetSkillShortcut](#setskillshortcut)
  - [SetStage](#setstage)
  - [SetStageRankLimit](#setstageranklimit)
  - [SetStamina](#setstamina)
  - [SetVariable](#setvariable)
  - [SetVibrator](#setvibrator)
  - [ShowBlackscreen](#showblackscreen)
  - [Sign](#sign)
  - [Sin](#sin)
  - [StringConstant](#stringconstant)
  - [SubString](#substring)

- **T**
  - [Tan](#tan)
  - [ToNumber](#tonumber)
  - [TriggerGameOver](#triggergameover)
  - [Trunc](#trunc)

- **U**
  - [UnequipAdultToy](#unequipadulttoy)
  - [UnequipAllCosplay](#unequipallcosplay)
  - [UnequipCosplay](#unequipcosplay)
  - [UnlockHandcuffs](#unlockhandcuffs)
  - [Upper](#upper)

- **V**
  - [Variable](#variable)
  - [Vector](#vector)
  - [Vector3Add](#vector3add)
  - [Vector3Cross](#vector3cross)
  - [Vector3Distance](#vector3distance)
  - [Vector3Dot](#vector3dot)
  - [Vector3Length](#vector3length)
  - [Vector3Rotate](#vector3rotate)
  - [Vector3Scale](#vector3scale)
  - [Vector3SqrLength](#vector3sqrlength)
  - [Vector3Sub](#vector3sub)

- **W**
  - [Wait](#wait)
  - [WaitForEvent](#waitforevent)
  - [WaitForThread](#waitforthread)
  - [While](#while)

## Goto

- **中文名**：跳转
- **官方 API**：`thread.Goto("label"[, data=myObj][, index=0][, nextstep="end"])`
- **返回值类型**：null
- **作用**：将当前线程的执行跳转到同一线程内的指定标签。跳转时可携带额外数据作为目标标签的局部变量，实现带参数的标签跳转。
- **参数说明**：
  - `label`（必填，String）— 目标标签名
  - `params`（可选，Object）— 传递给目标标签的额外参数，会作为局部变量在目标标签中可用
- **输出端口**：
  - `out_label`（Data，String）— 输出的标签名
- **`.code` 示例**：

```code
main:
    thread = _this
    Log(output="开始任务")
    thread.Goto("step1")

step1:
    thread = _this
    Log(output="执行步骤1")
    thread.Goto("step2", data="额外信息")

step2:
    thread = _this
    Log(output=data)
```

- **常见使用场景**：
  - 在线程内实现多阶段任务流程
  - 跳转回循环开始位置实现重复
  - 带数据传递的状态机迁移
- **相关节点**：If, While, CallFunction, CreateThread, WaitForThread

---

## Break

- **中文名**：跳出
- **官方 API**：`break`
- **返回值类型**：null
- **作用**：提前退出 `while` 或 `for` 循环体，继续执行循环之后的代码。不能用于循环之外。
- **参数说明**：无
- **`.code` 示例**：

```code
main:
    thread = _this
    i = 10
    while i >= 0
        if i == 5
            break
        Log(output=i)
        i = i - 1
    Log(output="循环结束")
```

- **常见使用场景**：
  - 搜索到目标后提前结束遍历
  - 满足退出条件时跳出无限循环
  - 配合 While 实现 do-while 模式
- **相关节点**：While, For, Return, If

---

## Return

- **中文名**：返回
- **官方 API**：`return [expression]` 或 `_result = value`
- **返回值类型**：由 `value` 参数决定
- **作用**：从当前标签（函数）提前返回，可设置返回值。在 `.code` 中表现为设置 `_result` 变量；如果 `return` 后跟表达式，还会隐式设置 `_result` 为该表达式的值。
- **参数说明**：
  - `value`（可选，List）— 返回给调用者的值，默认为 `null`
- **`.code` 示例**：

```code
main:
    thread = _this
    result = myFunc(a=2, b=3)
    Log(output=result)

myFunc:
    _result = a + b
```

或者使用 return 语句：

```code
myFunc:
    return a + b
```

- **常见使用场景**：
  - 标签/函数中返回计算结果
  - 条件判断后提前退出
  - 返回错误状态给调用者
- **相关节点**：CallFunction, CallMethod, If, Goto, ForeachNode

---

## Wait

- **中文名**：等待
- **官方 API**：`Wait(seconds=Number)`
- **返回值类型**：null
- **作用**：暂停当前线程的执行，等待指定的秒数后再继续执行后续节点。常用于创建时间间隔或延迟效果。
- **参数说明**：
  - `seconds`（必填，Number）— 等待的秒数（可以为小数）
- **`.code` 示例**：

```code
main:
    thread = _this
    Log(output="开始")
    Wait(seconds=3.0)
    Log(output="3秒后")
```

- **常见使用场景**：
  - NPC 对话之间的延迟
  - 阶段性事件的时间控制
  - 等待玩家操作前的短暂停顿
- **相关节点**：WaitForEvent, WaitForThread, Listener, CreateThread

---

## WaitForEvent

- **中文名**：等待事件
- **官方 API**：`WaitForEvent(eventName=String)`
- **返回值类型**：null
- **作用**：阻塞当前线程的执行，直到指定的跨帧事件被设置（通过 `SetEvent`）。常用于线程间的同步和跨帧通信。
- **参数说明**：
  - `eventName`（必填，String）— 要等待的事件名称
- **`.code` 示例**：

```code
main:
    thread = _this
    Log(output="等待事件触发")
    WaitForEvent(eventName="player_arrived")
    Log(output="事件已触发")

m2:
    thread = _this
    SetEvent(name="player_arrived", value=true)
```

- **常见使用场景**：
  - 等待另一个线程完成特定操作
  - 等待玩家到达指定位置
  - 同步多个线程的执行顺序
- **相关节点**：SetEvent, GetEvent, Wait, WaitForThread, CreateThread

---

## Log

- **中文名**：日志
- **官方 API**：`Log([Output])`
- **返回值类型**：null
- **作用**：向游戏控制台输出一条消息，用于调试和日志记录。是开发和调试任务时最常用的工具之一。
- **参数说明**：
  - `output`（必填，String）— 要输出的文本内容
- **`.code` 示例**：

```code
main:
    thread = _this
    Log(output="任务已启动")
    value = 42
    Log(output="value = " + value)
```

- **常见使用场景**：
  - 调试时输出变量值
  - 追踪任务执行流程
  - 验证条件分支是否被执行
- **相关节点**：Warning, Error, DumpVariables, DumpVariable, GetType

---

## Global

- **中文名**：全局变量
- **官方 API**：`Global(VariableName[, Value])`
- **返回值类型**：value of global variable
- **作用**：创建或访问一个全局变量。即使在函数或线程内部，也可以用它访问被局部变量遮蔽的全局变量。全局变量在所有线程和标签之间共享。
- **参数说明**：
  - `name`（必填，String）— 全局变量名
  - `value`（可选，List）— 要设置的值；不提供时仅读取
- **输出端口**：
  - `out_value`（Data，List）— 读取到的变量值
- **`.code` 示例**：

```code
main:
    thread = _this
    Global(name="g_count", value=0)
    myFunc()
    count = Global(name="g_count")
    Log(output=count)

myFunc:
    Global(name="g_count", value=Global("g_count") + 1)
```

- **常见使用场景**：
  - 跨线程/标签共享计数器
  - 存储全局配置状态
  - 突破局部作用域限制访问变量
- **相关节点**：Local, Variable, SetVariable, GetSave, _state 全局变量

---

## Local

- **中文名**：局部变量
- **官方 API**：`Local(VariableName[, Value])`
- **返回值类型**：value of local variable
- **作用**：在当前作用域中创建或访问一个局部变量，即使外层作用域有同名变量也会在当前层新建。主要用于在函数内创建与外层同名的变量而不影响外层。
- **参数说明**：
  - `name`（必填，String）— 局部变量名
  - `value`（可选，List）— 要设置的值；不提供时仅读取
- **输出端口**：
  - `out_value`（Data，List）— 读取到的变量值
- **`.code` 示例**：

```code
name = "global"
main:
    thread = _this
    Local(name="name", value="local")
    Log(output=name)  ; 输出 "local"
testFunc:
    Log(output=name)  ; 输出 "global"（外层不受影响）
```

- **常见使用场景**：
  - 在函数内创建与外层同名的临时变量
  - 隔离作用域避免变量污染
  - 测试和调试作用域规则
- **相关节点**：Global, Variable, SetVariable, DumpVariables

---

## GetType

- **中文名**：获取类型
- **官方 API**：`GetType(Value)`
- **返回值类型**：String
- **作用**：返回给定值的类型名称字符串。返回值可能是 `"Number"`、`"String"`、`"Boolean"`、`"List"`、`"Object"` 或 `"null"`。
- **参数说明**：
  - `value`（必填，List）— 要检查类型的值
- **输出端口**：
  - `out_type`（Data，String）— 类型名称
- **`.code` 示例**：

```code
main:
    thread = _this
    type = GetType(42)
    Log(output=type)  ; 输出 "Number"
    type2 = GetType("hello")
    Log(output=type2) ; 输出 "String"
```

- **常见使用场景**：
  - 验证变量类型后再进行操作
  - 处理动态类型输入时的防御性编程
  - 调试时确认值的实际类型
- **相关节点**：Log, DumpVariables, GetLanguage, Variable

---

## GetLanguage

- **中文名**：获取语言
- **官方 API**：`GetLanguage()`
- **返回值类型**：String
- **作用**：返回当前游戏语言设置。返回值可能是 `"En"`、`"Ja"`、`"Ko"`、`"Sc"` 或 `"Tc"`，分别对应英文、日文、韩文、简体中文和繁体中文。
- **参数说明**：无
- **输出端口**：
  - `out_language`（Data，String）— 语言代码
- **`.code` 示例**：

```code
main:
    thread = _this
    lang = GetLanguage()
    if lang == "En"
        Log(output="Hello!")
    elseif lang == "Ja"
        Log(output="こんにちは！")
```

- **常见使用场景**：
  - 根据语言调整对话或 UI 文本
  - 仅在特定语言下触发的行为分支
  - 配合 Translate 实现多语言支持
- **相关节点**：GetType, Log, GetSettings, Translate

---

## DumpVariables

- **中文名**：打印所有变量
- **官方 API**：`DumpVariables([RecursionCount])`
- **返回值类型**：null
- **作用**：将当前作用域及其父作用域中的所有变量打印到游戏控制台，是调试时查看变量状态的利器。
- **参数说明**：
  - `recursion`（可选，Number）— 递归深度，用于展开列表内部的内容
- **`.code` 示例**：

```code
main:
    thread = _this
    a = 1
    b = "test"
    myList = CreateList(x=10, y=20)
    DumpVariables(recursion=2)
```

- **常见使用场景**：
  - 调试时快速查看所有变量状态
  - 检查列表/对象的内部结构
  - 排查变量未定义或值不符合预期的问题
- **相关节点**：DumpVariable, Log, GetType, GetSave

---

## DumpVariable

- **中文名**：打印变量
- **官方 API**：`DumpVariable(Variable[, RecursionCount])`
- **返回值类型**：null
- **作用**：将指定的单个变量打印到游戏控制台。对于列表变量可以指定递归深度来查看内部内容，比 DumpVariables 更聚焦。
- **参数说明**：
  - `var`（必填，List）— 要打印的变量
  - `recursion`（可选，Number）— 递归深度，用于展开列表内部的内容
- **`.code` 示例**：

```code
main:
    thread = _this
    complexList = CreateList(a=CreateList(x=1, y=2), b="text")
    DumpVariable(var=complexList, recursion=3)
```

- **常见使用场景**：
  - 调试特定变量的值
  - 查看复杂嵌套列表的结构
  - 检查函数返回值的内容
- **相关节点**：DumpVariables, Log, GetType, GetSave

---

## CallFunction

- **中文名**：调用函数
- **官方 API**：`CallFunction(function=FunctionName[, parameters=Parameters])` 或 `CallFunction(FunctionName[, Parameters])` 或 `CallFunction(FunctionReference)`
- **返回值类型**：Value
- **作用**：通过函数名（字符串）动态调用函数。支持传递参数对象，也支持使用包含函数名和参数列表的引用列表。可以调用引擎定义函数或通过标签自定义的函数。
- **参数说明**：
  - `function`（必填，String）— 要调用的函数名
  - `params`（可选，Object）— 参数列表，键值对形式传递给目标函数
- **输出端口**：
  - `out_result`（Data，List）— 函数返回值
- **`.code` 示例**：

```code
main:
    thread = _this
    ; 直接传参
    result = CallFunction(function="fn_add", params=CreateList(a=2, b=3))
    Log(output=result)  ; 输出 5

    ; 动态函数名
    funcName = "fn_mult"
    result2 = CallFunction(function=funcName, params=CreateList(a=4, b=5))
    Log(output=result2)  ; 输出 20

fn_add:
    _result = a + b

fn_mult:
    _result = a * b
```

- **常见使用场景**：
  - 根据条件动态选择要调用的函数
  - 在列表中存储函数名统一调用
  - 跨 mod 调用其他 mod 注入的函数
- **相关节点**：CallMethod, Return, FunctionExists, ForeachNode, CreateThread

---

## CallMethod

- **中文名**：调用方法
- **官方 API**：`CallMethod(thread=Thread, method=MethodName[, parameters=Parameters])` 或 `CallMethod(Thread, MethodName[, Parameters])` 或 `CallMethod(MethodReference)`
- **返回值类型**：Value
- **作用**：在线程对象上动态调用方法。通过传入线程引用和方法名（字符串）来调用该线程上的方法，支持传递参数。
- **参数说明**：
  - `thread`（必填，Object）— 线程对象引用
  - `method`（必填，String）— 要调用的方法名
  - `params`（可选，Object）— 参数列表，键值对形式传递给方法
- **输出端口**：
  - `out_result`（Data，List）— 方法返回值
- **`.code` 示例**：

```code
main:
    thread = _this
    ; 调用线程的 Goto 方法
    CallMethod(thread=thread, method="Goto", params=CreateList(label="step1"))

    ; 调用等待线程完成
    child = CreateThread(labelName="sub")
    CallMethod(thread=child, method="WaitForFinish")
```

- **常见使用场景**：
  - 动态调用线程方法（Goto、WaitForFinish 等）
  - 通过方法引用统一处理不同对象
  - 调用存储在变量中的方法名
- **相关节点**：CallFunction, Goto, CreateThread, WaitForThread

---

## Color

- **中文名**：颜色
- **官方 API**：`Color(Red, Green, Blue[, Alpha])`
- **返回值类型**：List
- **作用**：创建一个包含 RGBA 四个颜色分量的列表。每个分量为 0-1 范围的数值，分别代表红、绿、蓝和透明度。可用于 UI 相关函数或需要颜色值的场景。
- **参数说明**：
  - `r`（必填，Number）— 红色分量（0-1）
  - `g`（必填，Number）— 绿色分量（0-1）
  - `b`（必填，Number）— 蓝色分量（0-1）
  - `a`（必填，Number）— 透明度分量（0-1）
- **输出端口**：
  - `out_color`（Data，List）— 颜色列表 `[r, g, b, a]`
- **`.code` 示例**：

```code
main:
    thread = _this
    red = Color(r=1.0, g=0.0, b=0.0, a=1.0)
    semiRed = Color(r=1.0, g=0.0, b=0.0, a=0.5)
```

- **常见使用场景**：
  - 设置 UI 文本或面板颜色
  - 传递给黑屏渐变等视觉效果函数
  - 创建颜色常量供后续使用
- **相关节点**：Range, GetTime, NumberConstant, MakeVector, StringConstant

---

## Range

- **中文名**：范围
- **官方 API**：`Range(Stop)`、`Range(Start, Stop[, Step])`
- **返回值类型**：List
- **作用**：生成一个数字序列列表。从 Start 开始，以 Step 为步长递增，直到不超过 Stop（**Stop 值不包含在序列中**）。常用于 for 循环中生成遍历范围。
- **参数说明**：
  - `start`（必填，Number）— 起始值（包含）
  - `stop`（必填，Number）— 结束值（不包含）
  - `step`（可选，Number）— 步长，默认为 1
- **输出端口**：
  - `out_list`（Data，List）— 生成的数字序列列表
- **`.code` 示例**：

```code
main:
    thread = _this
    ; for 循环中遍历范围
    for i in Range(start=0, stop=5)
        Log(output=i)
    ; 输出 0, 1, 2, 3, 4

    ; 自定义步长
    for i in Range(start=0, stop=10, step=2)
        Log(output=i)
    ; 输出 0, 2, 4, 6, 8
```

- **常见使用场景**：
  - 在 for 循环中遍历数值范围
  - 生成索引序列用于列表访问
  - 创建等差序列数据
- **相关节点**：For, Color, ForeachNode, Random, NumberConstant

---

## GetSave

- **中文名**：读取存档
- **官方 API**：`_save.key`
- **返回值类型**：List（任意存储的值）
- **作用**：读取跨会话持久存储中的值。通过 `_save` 全局列表保存的数据在游戏关闭后仍会保留，可用于实现持久化进度系统。注意不能存储 Object 类型。
- **参数说明**：
  - `key`（必填，String）— 存储键名
- **输出端口**：
  - `out_value`（Data，Any）— 存储的值
- **`.code` 示例**：

```code
main:
    thread = _this
    ; 写入存档
    _save.TotalScore = 100
    _save.PlayerName = "Hero"

    ; 读取存档
    score = GetSave(key="TotalScore")
    name = GetSave(key="PlayerName")
    Log(output="Score: " + score + ", Name: " + name)
```

- **常见使用场景**：
  - 保存玩家的任务进度
  - 记录解锁的成就和奖励
  - 跨游戏会话保留设置偏好
- **相关节点**：GetTime, GetSettings, GetMod, Variable, Global

---

## GetTime

- **中文名**：读取时间
- **官方 API**：`_time`
- **返回值类型**：Number
- **作用**：读取游戏累计运行时间（以秒为单位）。该时间会扣除暂停时间，考虑慢速效果，是游戏内实际流逝的时间。
- **参数说明**：无
- **输出端口**：
  - `out_value`（Data，Number）— 累计时间（秒）
- **`.code` 示例**：

```code
main:
    thread = _this
    startTime = GetTime()
    Wait(seconds=5.0)
    endTime = GetTime()
    elapsed = endTime - startTime
    Log(output="经过了 " + elapsed + " 秒")
```

- **常见使用场景**：
  - 计算经过的时间间隔
  - 实现倒计时逻辑
  - 配合 GetTimeDiff 做动画插值
- **相关节点**：GetTimeDiff, Wait, GetSave, GetSettings

---

## GetTimeDiff

- **中文名**：读取时间差
- **官方 API**：`_timediff`
- **返回值类型**：Number
- **作用**：读取上一帧到当前帧的游戏内时间差（秒）。该值考虑了慢速和暂停效果，适合用于帧率无关的动画和物理更新。
- **参数说明**：无
- **输出端口**：
  - `out_value`（Data，Number）— 帧时间差（秒）
- **`.code` 示例**：

```code
main:
    thread = _this
    ; 在监听器中每帧使用时间差更新进度
    listener = CreateListener(labelName="update")

update:
    thread = _this
    dt = GetTimeDiff()
    progress = progress + dt * 2.0
    Log(output="Progress: " + progress)
```

- **常见使用场景**：
  - 每帧更新进度条或动画
  - 实现与帧率无关的计时器
  - 在监听器中做连续物理或逻辑更新
- **相关节点**：GetTime, Wait, Listener, GetSettings

---

## GetSettings

- **中文名**：读取设置
- **官方 API**：`_settings`
- **返回值类型**：Object
- **作用**：读取 `meta.json` 中定义的用户设置菜单的值。设置值在游戏内通过手机菜单调整，可在代码中以 `_settings.key` 的形式读取。
- **参数说明**：无
- **输出端口**：
  - `out_value`（Data，Object）— 设置对象
- **`.code` 示例**：

```code
main:
    thread = _this
    ; 读取 meta.json 中定义的设置
    difficulty = GetSettings()
    actualDiff = difficulty.range  ; 假设 settings 中有 name="range" 的 Integer 设置
    if actualDiff >= 50
        Log(output="高难度模式")
```

- **常见使用场景**：
  - 实现难度选择、音量控制等用户配置
  - 根据玩家偏好调整任务行为
  - 读取 meta.json 中定义的任何自定义设置
- **相关节点**：GetSave, GetMod, GetMods, GetLanguage, Variable

---

## GetMod

- **中文名**：读取 Mod 数据
- **官方 API**：`_mod`
- **返回值类型**：List
- **作用**：读取当前 mod 的项目内共享数据列表。其他激活的 mod 可以通过 `_mods[folderName]` 访问这些数据。Object 类型（除 List 外）不允许存储，会被自动移除。
- **参数说明**：无
- **输出端口**：
  - `out_value`（Data，List）— 当前 mod 的共享数据
- **`.code` 示例**：

```code
main:
    thread = _this
    ; 写入共享数据
    modData = GetMod()
    modData.SharedFlag = true
    modData.Counter = 42

    ; 从其他 mod 读取（在其他项目文件中）
    otherData = _mods.OtherProjectFolder
```

- **常见使用场景**：
  - 在不同任务 mod 之间共享状态
  - 实现跨 mod 的联动机制
  - 发布公共 API 数据给其他 mod 读取
- **相关节点**：GetMods, GetSave, GetSettings, GetLanguage, GetTime

---

## GetMods

- **中文名**：读取所有 Mod 数据
- **官方 API**：`_mods`
- **返回值类型**：Object（List）
- **作用**：读取所有已激活 mod 的项目数据。数字索引包含项目文件夹名（字符串），项目名作为索引时可访问对应 mod 的 `_mod` 数据内容。Object 类型（除 List 外）不能共享。
- **参数说明**：无
- **输出端口**：
  - `out_value`（Data，Object）— 所有 mod 数据的集合
- **`.code` 示例**：

```code
main:
    thread = _this
    allMods = GetMods()

    ; 遍历所有 mod
    i = 0
    while i < allMods.Count()
        modName = allMods[i]
        modData = allMods[modName]
        Log(output="Mod: " + modName)
        i = i + 1

    ; 直接访问特定 mod
    otherData = _mods.SomeOtherProject
```

- **常见使用场景**：
  - 发现并列出所有正在运行的 mod
  - 读取其他 mod 的公开数据
  - 检查是否有特定 mod 处于激活状态
- **相关节点**：GetMod, GetSave, GetSettings, GetLanguage, GetName

---

## Variable

- **中文名**：变量
- **官方 API**：`VariableName`（按名称读取当前作用域中的变量）
- **返回值类型**：由变量本身决定
- **作用**：读取当前作用域中指定名称的变量值。可用于读取当前线程/标签中已定义的局部变量或上一层作用域的变量。
- **参数说明**：
  - `name`（必填，String）— 要读取的变量名
- **输出端口**：
  - `out_value`（Data，Any）— 变量的当前值
- **`.code` 示例**：

```code
main:
    thread = _this
    playerName = "Alice"
    score = 100
    ; 通过 Variable 节点读取已定义的变量
    name = playerName
    Log(output=name)  ; 输出 "Alice"
```

- **常见使用场景**：
  - 作为数据节点提供变量的当前值
  - 在节点图中建立变量引用连线
  - 连接其他节点实现动态数据流
- **相关节点**：SetVariable, Global, Local, GetType, Log

---

## SetVariable

- **中文名**：设置变量
- **官方 API**：`VariableName = Expression`
- **返回值类型**：null
- **作用**：将计算后的值赋给当前作用域中的指定变量。如果变量不存在则创建。支持所有基本类型（Number、String、Boolean、List、Object）的赋值。
- **参数说明**：
  - `name`（必填，String）— 要设置的变量名
- **输入端口**：
  - `value`（Data，Any，必填）— 要赋的值
- **`.code` 示例**：

```code
main:
    thread = _this
    ; 简单赋值
    counter = 0
    ; 运算后赋值
    counter = counter + 1
    message = "计数: " + counter
    Log(output=message)
```

- **常见使用场景**：
  - 存储中间计算结果
  - 更新循环计数器
  - 保存函数返回值供后续使用
- **相关节点**：Variable, Global, Local, CallFunction, SetVariable

---

## ForeachNode

- **中文名**：Foreach
- **官方 API**：`i = Foreach(list, threadVar)` → 目标标签内可访问 `value`（当前元素）；返回 `_result = false` 时停止遍历
- **返回值类型**：Number（遍历索引）
- **作用**：遍历列表中的每个元素，并在指定的线程/标签中对每个元素执行处理逻辑。遍历过程中可通过设置 `_result = false` 提前停止遍历。当前元素通过 `value` 变量在目标标签内访问。
- **参数说明**：
  - `list`（必填，List）— 要遍历的列表
  - `threadVar`（必填，Object）— 处理每个元素的目标线程/标签引用
- **`.code` 示例**：

```code
main:
    thread = _this
    items = CreateList("A", "B", "C", "D")
    index = Foreach(items, thread)

processItem:
    thread = _this
    Log(output="处理: " + value)
    ; 当处理到 "C" 时停止
    if value == "C"
        _result = false
```

- **常见使用场景**：
  - 遍历列表并对每个元素执行相同逻辑
  - 搜索列表中满足条件的元素
  - 批量处理 NPC、物品或任务数据
- **相关节点**：For, CallFunction, While, For, Range

---

## Boolean

| 属性 | 值 |
|------|-----|
| **分类** | Literals |
| **代码生成类别** | C（纯数据节点） |
| **颜色** | 青绿色 `[0, 150, 136, 255]` |

### 1. 中文名

布尔值

### 2. 官方 API 签名

```code
true
false
```

内置常量，无函数调用。

### 3. 返回值类型

`Boolean` — `true` 或 `false`

### 4. 作用

输出一个布尔常量值。这是 .code 中最基础的数据节点之一，用于向 `If`、`While`、条件组合节点等提供固定布尔值。当需要硬编码启用/禁用开关时使用。

### 5. 参数说明

| 参数名 | 显示名 | 必填 | 类型 | 含义 |
|--------|--------|------|------|------|
| `value` | 值 | 是 | 枚举 | 选择 `true` 或 `false` |

### 6. `.code` 使用案例

```code
// 直接输出 true
isActive = true

// 用于 If 条件
if true
    Log("Always runs")
```

### 7. 常见使用场景

- 给 `If` 节点提供硬编码条件（调试分支）
- 作为 `SetPlayerData` 等函数的布尔参数输入
- 与 `LogicNot` 组合生成 `false` 变体
- 作为 Flag 变量的初始值

### 8. 相关节点

- `If` — 消耗布尔条件的控制流节点
- `LogicNot` — 对布尔值取反
- `LogicAnd` / `LogicOr` — 组合多个布尔值
- `GetStateBool` — 从游戏状态读取动态布尔值

---

## GetStateBool

| 属性 | 值 |
|------|-----|
| **分类** | Conditions & Queries |
| **代码生成类别** | C（纯数据节点） |
| **颜色** | 紫粉色 `[171, 71, 188, 255]` |

### 1. 中文名

读取布尔状态

### 2. 官方 API 签名

`_state` 全局变量的一部分：

```code
_state.Futanari
_state.Sitting
_state.IsDayTime
// …
```

详见 `docs/kb/documentation_part_006.md` 的 `_state` 章节。

### 3. 返回值类型

`Boolean`

### 4. 作用

从只读全局状态 `_state` 中读取一个布尔型字段。这些字段表示玩家当前的各种状态，如是否扶她、是否坐着、是否被看到等。返回值可用于 `If` / `While` 的条件或 `Logic` 系列节点的输入。

### 5. 参数说明

| 参数名 | 显示名 | 必填 | 类型 | 含义 |
|--------|--------|------|------|------|
| `stateKey` | 状态键 | 是 | 枚举 | 要读取的 `_state` 布尔字段名 |

可选值：

| 键 | 含义 |
|----|------|
| `Futanari` | 玩家是否处于扶她状态 |
| `Sitting` | 玩家是否坐着 |
| `Orgasm` | 玩家是否高潮 |
| `Moving` | 玩家是否在移动 |
| `Crouching` | 玩家是否蹲着 |
| `Peeing` | 玩家是否在排尿 |
| `Dashing` | 玩家是否在冲刺 |
| `InLight` | 玩家是否在光亮处 |
| `NearNPC` | 玩家是否靠近 NPC |
| `Watched` | 玩家是否被注视 |
| `ShowingOff` | 玩家是否在展示裸露部位 |
| `Bukkake` | 玩家身上是否有精液 |
| `Blindfolded` | 玩家是否蒙眼 |
| `Invisible` | 玩家是否隐身 |
| `InOpenToilet` | 玩家是否在开放厕所 |
| `FPCamera` | 是否为第一人称视角 |
| `IsDayTime` | 当前是否为白天 |
| `GameOver` | 游戏是否结束 |

### 6. `.code` 使用案例

```code
// 检查玩家是否在白天且裸露
if (_state.Futanari) && (_state.IsDayTime)
    Log("Futanari in daylight!")
```

### 7. 常见使用场景

- 条件分支的输入：如"如果在白天则……"
- 状态检测：玩家位置、姿势、隐蔽状态
- 与 `LogicAnd` / `LogicOr` / `LogicNot` 组合复杂条件
- 创建 `CreateCondition` 的条件表达式

### 8. 相关节点

- `GetStateNumber` — 读取数值型状态
- `Boolean` — 布尔常量
- `If` — 消耗布尔条件的控制流节点
- `CheckEquipment` / `CheckCosplay` — 检查装备/服装状态

---

## GetStateNumber

| 属性 | 值 |
|------|-----|
| **分类** | Conditions & Queries |
| **代码生成类别** | C（纯数据节点） |
| **颜色** | 紫粉色 `[171, 71, 188, 255]` |

### 1. 中文名

读取数值状态

### 2. 官方 API 签名

`_state` 全局变量的一部分：

```code
_state.Ecstasy
_state.Detection
_state.Stamina
// …
```

### 3. 返回值类型

`Number`

### 4. 作用

从只读全局状态 `_state` 中读取一个数值型字段。这些字段表示可量化的游戏数据，如快感值、察觉度、等级、体力等。输出值可直接用于 `CompareNumbers` 比较或数学计算。

### 5. 参数说明

| 参数名 | 显示名 | 必填 | 类型 | 含义 |
|--------|--------|------|------|------|
| `stateKey` | 状态键 | 是 | 枚举 | 要读取的 `_state` 数值字段名 |

可选值：

| 键 | 含义 |
|----|------|
| `Ecstasy` | 当前快感值 |
| `Detection` | 当前被察觉度 |
| `Rank` | 当前闪玩等级 |
| `HeartRate` | 当前心率 |
| `Stamina` | 当前体力 |
| `StaminaMax` | 最大体力值 |
| `Moisture` | 膀胱湿润度 |
| `Bodypaint` | 体绘剩余量 |

### 6. `.code` 使用案例

```code
// 检测快感是否达到阈值
if _state.Ecstasy >= 90
    Log("About to climax!")

// 警告低体力
if _state.Stamina < 20
    Warning("Stamina is low")
```

### 7. 常见使用场景

- 与 `CompareNumbers` 配合构建阈值条件
- 监控玩家状态（体力、快感、察觉度）
- Dump 调试：`Log("Ecstasy: " + _state.Ecstasy)`
- 根据 `_state.Rank` 控制场景解锁逻辑

### 8. 相关节点

- `GetStateBool` — 读取布尔型状态
- `CompareNumbers` — 数值比较，输出布尔值
- `NumberConstant` — 数值常量输入源
- `Random` / `RandomInt` — 随机数值生成

---

## CompareNumbers

| 属性 | 值 |
|------|-----|
| **分类** | Math & Logic |
| **代码生成类别** | C（纯数据节点） |
| **颜色** | 灰色 `[96, 125, 139, 255]` |

### 1. 中文名

数值比较

### 2. 官方 API 签名

```code
// 直接使用 .code 的比较运算符
{valueA} >= {valueB}
{valueA} == {valueB}
{valueA} != {valueB}
```

运算符定义参见 `docs/kb/documentation_part_002.md` > Operators。

### 3. 返回值类型

`Boolean`

### 4. 作用

比较两个数值并输出布尔结果。支持 6 种比较操作符：`>=`、`==`、`!=`、`>`、`<`、`<=`。该节点是条件逻辑的基础构件，常将 `GetStateNumber` 或其它数值节点的输出接入后，连接 `If` 的条件端口。

### 5. 参数说明

| 参数名 | 显示名 | 必填 | 类型 | 含义 |
|--------|--------|------|------|------|
| `a` | 数值A | 是 | Number | 比较左操作数 |
| `b` | 数值B | 是 | Number | 比较右操作数 |
| `operator` | 操作符 | 是 | 枚举 | 比较关系：`>=` / `==` / `!=` / `>` / `<` / `<=` |

### 6. `.code` 使用案例

```code
// 等级检查
if _state.Rank >= 3
    Log("Rank 3 or higher!")

// 相等检查
if _state.Stamina == 0
    TriggerGameOver()

// 不等检查
if _state.Stamina != _state.StaminaMax
    Warning("Not fully rested")
```

### 7. 常见使用场景

- 等级门槛判断：`_state.Rank >= 5`
- 资源阈值告警：`_state.Stamina < 10`
- 状态归零检测：`_state.Ecstasy == 0`
- 与 `GetStateNumber` 串联构成条件链

### 8. 相关节点

- `GetStateNumber` — 提供左侧数值来源
- `NumberConstant` — 提供右侧常量数值
- `If` — 消费布尔结果的控制流节点
- `LogicAnd` / `LogicOr` — 多条件组合
- `Min` / `Max` — 数值边界处理

---

## LogicAnd

| 属性 | 值 |
|------|-----|
| **分类** | Math & Logic |
| **代码生成类别** | C（纯数据节点） |
| **颜色** | 灰色 `[96, 125, 139, 255]` |

### 1. 中文名

逻辑与

### 2. 官方 API 签名

```code
{valueA} && {valueB}
```

参见 `docs/kb/documentation_part_002.md` > Operators：`&&` 运算符，短路求值。

### 3. 返回值类型

`Boolean`

### 4. 作用

对两个布尔输入执行逻辑与（AND）运算，输出 `true` 当且仅当两个输入均为 `true`。使用 `.code` 的 `&&` 短路运算符，当左操作数为 `false` 时不再求值右操作数。输出可连接 `If` / `While` 的条件端口，或继续接入其它逻辑节点。

### 5. 参数说明

| 参数名 | 显示名 | 必填 | 类型 | 含义 |
|--------|--------|------|------|------|
| `a` | 输入A | 是 | Boolean（数据端口） | 左操作数 |
| `b` | 输入B | 是 | Boolean（数据端口） | 右操作数 |

与 `CompareNumbers` 等参数节点不同，`LogicAnd` 的输入是数据端口，而非属性面板参数。这意味着输入值必须通过 Data 边来自其他节点。

### 6. `.code` 使用案例

```code
// 白天且裸露 = 双倍风险
if (_state.IsDayTime) && (_state.ShowingOff)
    Warning("Double risk during daytime flashing!")

// 同时满足多个条件
if (_state.Rank >= 3) && (_state.Futanari) && (_state.InLight)
    Log("Rank 3+ Futanari in light")
```

### 7. 常见使用场景

- 多条件组合：等级 + 状态 + 位置同时满足
- 安全检测：`(NearNPC) && (ShowingOff)` 时触发警告
- 条件式解锁：要求同时满足多个游戏条件
- 长条件链中与 `LogicOr` 混合构建复杂布尔表达式

### 8. 相关节点

- `LogicOr` — 逻辑或组合
- `LogicNot` — 对结果取反
- `If` — 消费最终条件的控制流节点
- `Boolean` — 提供固定 `true` / `false` 输入
- `GetStateBool` — 动态布尔值输入源

---

## LogicOr

| 属性 | 值 |
|------|-----|
| **分类** | Math & Logic |
| **代码生成类别** | C（纯数据节点） |
| **颜色** | 灰色 `[96, 125, 139, 255]` |

### 1. 中文名

逻辑或

### 2. 官方 API 签名

```code
{valueA} || {valueB}
```

参见运算符文档：`||` 短路运算符。

### 3. 返回值类型

`Boolean`

### 4. 作用

对两个布尔输入执行逻辑或（OR）运算，输出 `true` 当至少一个输入为 `true`。使用 `.code` 的 `||` 短路运算符，当左操作数为 `true` 时不再求值右操作数。

### 5. 参数说明

| 参数名 | 显示名 | 必填 | 类型 | 含义 |
|--------|--------|------|------|------|
| `a` | 输入A | 是 | Boolean（数据端口） | 左操作数 |
| `b` | 输入B | 是 | Boolean（数据端口） | 右操作数 |

### 6. `.code` 使用案例

```code
// 任意一种姿势触发
if (_state.Sitting) || (_state.Crouching)
    Log("Player is sitting or crouching")

// 任意危险条件触发警报
if (_state.Detection > 50) || (_state.NearNPC)
    Warning("Danger!")
```

### 7. 常见使用场景

- 备选条件：多个等价条件中任一成立即触发
- 宽松判定："坐或蹲" 视为同组姿势
- 容错逻辑：多个输入源中有一个有效即可
- 与 `LogicAnd` 混合：`(A || B) && C` 形式

### 8. 相关节点

- `LogicAnd` — 逻辑与组合
- `LogicNot` — 对结果取反
- `If` — 消费最终条件的控制流节点
- `CompareNumbers` — 提供比较结果

---

## LogicNot

| 属性 | 值 |
|------|-----|
| **分类** | Math & Logic |
| **代码生成类别** | C（纯数据节点） |
| **颜色** | 灰色 `[96, 125, 139, 255]` |

### 1. 中文名

逻辑非

### 2. 官方 API 签名

```code
!{value}
```

参见运算符文档：`!` 逻辑非运算符。

### 3. 返回值类型

`Boolean`

### 4. 作用

对输入的布尔值执行逻辑非（NOT）运算，将 `true` 变为 `false`，`false` 变为 `true`。这是构建负条件最直接的节点，常用于反转条件判断或在 `CreateCondition` 表达式中使用 `!` 前缀。

### 5. 参数说明

| 参数名 | 显示名 | 必填 | 类型 | 含义 |
|--------|--------|------|------|------|
| `a` | 输入 | 是 | Boolean（数据端口） | 要取反的布尔值 |

### 6. `.code` 使用案例

```code
// 不在白天
if !(_state.IsDayTime)
    Log("It's night time")

// 没有装备手铐
if !(_state.AdultToys.Handcuff != null)
    Log("No handcuffs equipped")

// 组合：不在白天 且 没有裸露
(!(_state.IsDayTime)) && (!(_state.ShowingOff))
```

### 7. 常见使用场景

- 反转 `GetStateBool` 输出："是否不在移动"
- 构建 "非 A 且 B" 的条件
- 安全检查：`!(_state.GameOver)` 游戏未结束时执行逻辑
- 与 `LogicAnd` / `LogicOr` 组合成 De Morgan 律等价形式

### 8. 相关节点

- `LogicAnd` / `LogicOr` — 其它逻辑运算
- `Boolean` — 提供固定 `true` / `false` 输入
- `GetStateBool` — 要取反的状态值
- `CheckEquipment` — 检查装备状态（可反转表示"未装备"）

---

## CheckEquipment

| 属性 | 值 |
|------|-----|
| **分类** | Conditions & Queries |
| **代码生成类别** | C（纯数据节点） |
| **颜色** | 紫粉色 `[171, 71, 188, 255]` |

### 1. 中文名

检查装备

### 2. 官方 API 签名

```code
_state.AdultToys.{equipType} != null
```

使用 `_state.AdultToys` 子列表，参见 `docs/kb/documentation_part_006.md` > `_state` > `AdultToys`。

### 3. 返回值类型

`Boolean`

### 4. 作用

检测玩家当前是否装备了指定的成人玩具类型。内部通过检查 `_state.AdultToys.{type} != null` 实现。支持所有成人玩具类型，包括手铐系列、跳蛋、蒙眼布、活塞等。多用于任务条件判定和游戏逻辑分支。

### 5. 参数说明

| 参数名 | 显示名 | 必填 | 类型 | 含义 |
|--------|--------|------|------|------|
| `equipType` | 装备类型 | 是 | 枚举 | 要检查的成人玩具类型 |

可选值：

| 值 | 含义 |
|----|------|
| `Handcuff` | 普通手铐 |
| `KeyHandcuff` | 钥匙手铐 |
| `TimerHandcuff` | 定时手铐 |
| `Vibrator` | 跳蛋 |
| `EyeMask` | 蒙眼布 |
| `TitRotor` | 乳头旋转器 |
| `KuriRotor` | 阴蒂旋转器 |
| `PistonAnal` | 肛用活塞 |
| `PistonPussy` | 阴道用活塞 |
| `AnalPlug` | 肛塞 |

### 6. `.code` 使用案例

```code
// 检查是否戴着手铐
if _state.AdultToys.Handcuff != null
    Log("Cuffed!")

// 组合：蒙眼且戴跳蛋 = 高难度模式
if (_state.AdultToys.EyeMask != null)
    if (_state.AdultToys.Vibrator != null)
        Log("Hard mode: blindfolded + vibrator")
```

### 7. 常见使用场景

- 任务条件判定：需要/禁止装备某玩具
- 难度调节：根据装备组合调整游戏行为
- 教程引导：检测玩家是否已装备指定物品
- 与 `LogicNot` 组合：玩家未装备时给予提示

### 8. 相关节点

- `CheckCosplay` — 检查服装状态
- `GetStateBool` — 读取其它布尔状态
- `LogicNot` — 取反可达"未装备"判定
- `EquipAdultToy` / `UnequipAdultToy` — 装备/卸下成人玩具
- `LockHandcuffs` — 主动装备手铐

---

## CheckCosplay

| 属性 | 值 |
|------|-----|
| **分类** | Conditions & Queries |
| **代码生成类别** | C（纯数据节点） |
| **颜色** | 紫粉色 `[171, 71, 188, 255]` |

### 1. 中文名

检查服装

### 2. 官方 API 签名

```code
// 单件
Cosplay_{cosplayKey}

// 多件（编辑器生成 && 连接）
Cosplay_A && Cosplay_B
```

服装键列表参见 `docs/kb/documentation_part_003.md` 的 `m_cosplay_*` 常量。

### 3. 返回值类型

`Boolean`

### 4. 作用

检测玩家是否穿着指定的角色扮演服装部件。支持同时检查多件服装：当指定多件服装键时，内部使用逻辑与（`&&`）连接并加括号，方便与 `LogicAnd` / `LogicOr` 等节点组合。该节点直接读取引擎内置的 `Cosplay_` 条件变量，而非 `_state`。

### 5. 参数说明

| 参数名 | 显示名 | 必填 | 类型 | 含义 |
|--------|--------|------|------|------|
| `cosplayKeys` | 服装键 | 是 | List | 要检查的服装键列表（允许多个） |

服装键例如 `bunny_ear`、`sister_veil`、`maid_apron` 等，完整的键名以游戏中的 CosplayID 为准。

### 6. `.code` 使用案例

```code
// 单件：是否戴着猫耳
if Cosplay_kemono_ear_cat
    Log("Wearing cat ears!")

// 多件：女仆套装完整穿着
if (Cosplay_maid_apron && Cosplay_maid_onepiece && Cosplay_maid_choker)
    Log("Full maid outfit!")
```

### 7. 常见使用场景

- 任务条件：要求玩家穿着/不穿某服装
- Cosplay 组合奖励：检测完整套装给予加成
- 剧情分支：根据服装改变 NPC 对话
- 模式切换：检测特殊服装开启新行为

### 8. 相关节点

- `CheckEquipment` — 检查玩具装备
- `EquipCosplay` / `UnequipCosplay` — 装备/卸下服装
- `OwnCosplay` — 设置服装拥有状态
- `LogicAnd` / `LogicOr` — 组合其它条件
- `GetStateBool` — 读取其它状态值

## DropItem

- **中文名**：掉落物品
- **官方 API 签名**：
  ```code
  DropItem(itemtype = DropItemType, stage = StageName, x = PositionX, y = PositionY, z = PositionZ[, rx = RotationX, ry = RotationY, rz = RotationZ, rw = RotationW][, compass = CompassIconVisible])
  DropItem(itemtype = DropItemType, position = Position[, compass = CompassIconVisible])
  ```
- **返回值类型**：`String | Number | null`（编辑器 Data 输出端口名 `out_item`，类型 Object）
- **作用**：在指定场景的特定位置掉落一件物品。掉落物可以被玩家或 NPC 拾取。对于假阳具等可多次掉落的物品，返回值可能不是字符串而是编号，用于区分不同实例。
- **参数说明**：

  | 参数 | 必填 | 类型 | 含义 |
  |------|------|------|------|
  | `itemtype` | 是 | 枚举 | 掉落物品类型：None、Coat、Hoodie、Basket、Pants、Bra、HandcuffKey、VibeRemocon、DildoFloor、DildoWall |
  | `stage` | 是 | 枚举 | 目标场景：None、Apart、Convenience、FashionShop、Residence、ShoppingMall、StationFront、Park、Mansion、TokyoStreet、Suburbs、Street、City、BarberShop、Laundry、Underpass |
  | `position` | 是 | Vector | 掉落位置（可用 Vector 代替单独的 x/y/z） |
  | `x` / `y` / `z` | 是 | Number | 掉落坐标分量（与 position 二选一） |
  | `rotation` | 否 | Quaternion | 物品旋转（可用四元数代替 rx/ry/rz/rw） |
  | `rx` / `ry` / `rz` / `rw` | 否 | Number | 四元数旋转分量 |
  | `compass` | 否 | Boolean | 是否显示指南针标记指向掉落物 |

- **`.code` 示例**：
  ```code
  var_n1_out_item = DropItem(itemtype="Coat", stage="Residence", x=-26.6, y=-0.1, z=-120)
  ```

- **常见使用场景**：
  - 任务中让玩家在特定位置找到关键物品（外套、钥匙等）
  - 散落衣物作为环境叙事的一部分
  - 配合 CollectItem 实现"寻找并拾取"的任务链

- **相关节点**：CollectItem、SetItemCount、AddItemCount、GetItemCount、CreateArea

---

## CollectItem

- **中文名**：拾取物品
- **官方 API 签名**：
  ```code
  CollectItem(itemtype = DropItemType[, stage = StageName, x = PositionX, y = PositionY, z = PositionZ])
  CollectItem(itemtype = DropItemType[, position = Position])
  CollectItem(itemtype = DropItemType)
  ```
- **返回值类型**：`Boolean`（编辑器 Data 输出端口名 `out_collected`）
- **作用**：拾取指定类型的掉落物品。如果场景中有多个同类型物品，可以通过指定位置选择具体拾取哪个实例；如果不指定位置，则使用 DropItem 返回的实例编号。
- **参数说明**：

  | 参数 | 必填 | 类型 | 含义 |
  |------|------|------|------|
  | `itemtype` | 是 | 枚举 | 要拾取的物品类型，同 DropItemType |
  | `stage` | 否 | 枚举 | 物品所在场景，不填则自动匹配 |
  | `position` | 否 | Vector | 物品位置，用于区分同类型多实例 |
  | `x` / `y` / `z` | 否 | Number | 位置坐标分量 |

- **`.code` 示例**：
  ```code
  var_n2_out_collected = CollectItem(itemtype="Coat", stage="Residence", x=-26.6, y=-0.1, z=-120)
  ```

- **常见使用场景**：
  - 玩家完成"寻找物品"任务后收回物品
  - 清理场景中多余的掉落物
  - 配合 DropItem 的返回值精确拾取指定实例

- **相关节点**：DropItem、SetItemCount、AddItemCount、GetItemCount、SetEvent

---

## SetVibrator

- **中文名**：设置跳蛋
- **官方 API 签名**：
  ```code
  SetVibrator(VibratorStrength)
  ```
- **返回值类型**：`null`（无 Data 输出端口）
- **作用**：控制玩家身上跳蛋的振动强度。所有参数均有下拉枚举选择。
- **参数说明**：

  | 参数 | 必填 | 类型 | 含义 |
  |------|------|------|------|
  | `strength` | 是 | 枚举 | 振动强度：Off（关闭）、Low（低）、High（高）、Random（随机） |

- **`.code` 示例**：
  ```code
  SetVibrator(strength="High")
  ```

- **常见使用场景**：
  - 根据任务进度改变跳蛋强度作为动态反馈
  - 在特定事件中触发随机模式增加不确定性
  - 任务达成后关闭跳蛋作为奖励

- **相关节点**：SetPiston、EquipAdultToy、UnequipAdultToy、LockHandcuffs、UnlockHandcuffs

---

## SetPiston

- **中文名**：设置活塞
- **官方 API 签名**：
  ```code
  SetPiston(PistonStrength)
  ```
- **返回值类型**：`null`（无 Data 输出端口）
- **作用**：控制玩家身上活塞玩具的运动强度。所有参数均有下拉枚举选择。
- **参数说明**：

  | 参数 | 必填 | 类型 | 含义 |
  |------|------|------|------|
  | `strength` | 是 | 枚举 | 活塞强度：Off（关闭）、Low（低）、Medium（中）、High（高）、Random（随机） |

- **`.code` 示例**：
  ```code
  SetPiston(strength="Medium")
  ```

- **常见使用场景**：
  - 根据任务事件动态调节活塞强度
  - 随机模式用于惩罚/奖励机制
  - 配合成人玩具装备节点实现完整玩具控制链

- **相关节点**：SetVibrator、EquipAdultToy、UnequipAdultToy、LockHandcuffs、UnlockHandcuffs

---

## LockHandcuffs

- **中文名**：锁手铐
- **官方 API 签名**：
  ```code
  LockHandcuffs(handcuffstype = HandcuffsType[, attachtoobject = AttachToObject][, duration = DurationInSeconds])
  ```
- **返回值类型**：`null`（无 Data 输出端口）
- **作用**：给玩家锁上手铐。支持普通手铐、钥匙手铐和定时手铐三种类型，可选择是否固定在物体上以及设置定时时长。
- **参数说明**：

  | 参数 | 必填 | 类型 | 含义 |
  |------|------|------|------|
  | `handcuffstype` | 是 | 枚举 | 手铐类型：Handcuff（普通）、KeyHandcuff（钥匙手铐）、TimerHandcuff（定时手铐） |
  | `attachtoobject` | 否 | Boolean | 是否将手铐绑定到场景物体上（如床、椅子） |
  | `duration` | 否 | Number | 定时手铐的倒计时时长（秒），仅 TimerHandcuff 有效 |

- **`.code` 示例**：
  ```code
  LockHandcuffs(handcuffstype="TimerHandcuff", duration=120)
  ```

- **常见使用场景**：
  - 任务中被 NPC 抓住后锁上手铐限制行动
  - 定时手铐制造紧迫感，玩家必须在倒计时内完成任务
  - 钥匙手铐需要玩家先找到钥匙才能解锁

- **相关节点**：UnlockHandcuffs、EquipAdultToy、UnequipAdultToy、DropItem、HandcuffKey

---

## UnlockHandcuffs

- **中文名**：解锁手铐
- **官方 API 签名**：
  ```code
  UnlockHandcuffs()
  ```
- **返回值类型**：`null`（无 Data 输出端口）
- **作用**：解锁玩家当前佩戴的所有手铐。无参数，调用即解锁。
- **参数说明**：无参数。
- **`.code` 示例**：
  ```code
  UnlockHandcuffs()
  ```

- **常见使用场景**：
  - 玩家完成任务条件后解除手铐限制
  - 定时手铐到期后自动调用的后续动作
  - 找到手铐钥匙后执行解锁

- **相关节点**：LockHandcuffs、EquipAdultToy、UnequipAdultToy、DropItem（掉落 HandcuffKey）

---

## EquipCosplay

- **中文名**：装备 Cosplay
- **官方 API 签名**：
  ```code
  EquipCosplay(CosplayNameKey1[, CosplayNameKey2]...)
  EquipCosplay(ListOfCosplayNameKeys1[, ListOfCosplayNameKeys2]...)
  ```
- **返回值类型**：`null`（无 Data 输出端口）
- **作用**：为玩家装备一套或多套角色扮演服装。支持传入多个服装键值，也可以传入列表批量装备。编辑器参数为 `cosplayKeys`（List 类型），可选中多个服装。
- **参数说明**：

  | 参数 | 必填 | 类型 | 含义 |
  |------|------|------|------|
  | `cosplayKeys` | 是 | List | 服装键名列表，例如 ["Maid", "Bunny"]，支持多选 |

- **`.code` 示例**：
  ```code
  EquipCosplay(cosplayKeys=["Maid", "Bunny"])
  ```

- **常见使用场景**：
  - 任务中要求玩家换上特定服装
  - 进入特定场景时自动换装
  - 作为任务奖励/惩罚更换玩家外观

- **相关节点**：UnequipCosplay、UnequipAllCosplay、OwnCosplay、EquipAdultToy

---

## UnequipCosplay

- **中文名**：卸下 Cosplay
- **官方 API 签名**：
  ```code
  UnequipCosplay(CosplayNameKey1[, CosplayNameKey2]...)
  UnequipCosplay(ListOfCosplayNameKeys1[, ListOfCosplayNameKeys2]...)
  ```
- **返回值类型**：`null`（无 Data 输出端口）
- **作用**：卸下玩家当前穿着的指定角色扮演服装。支持传入多个服装键或列表批量卸下。
- **参数说明**：

  | 参数 | 必填 | 类型 | 含义 |
  |------|------|------|------|
  | `cosplayKeys` | 是 | List | 要卸下的服装键名列表，例如 ["Maid"] |

- **`.code` 示例**：
  ```code
  UnequipCosplay(cosplayKeys=["Maid"])
  ```

- **常见使用场景**：
  - 离开特定场景后恢复玩家默认服装
  - 任务阶段变换需要更换外观
  - 与 EquipCosplay 配对使用实现换装逻辑

- **相关节点**：EquipCosplay、UnequipAllCosplay、OwnCosplay、EquipAdultToy

---

## UnequipAllCosplay

- **中文名**：卸下全部 Cosplay
- **官方 API 签名**：
  ```code
  UnequipAllCosplay()
  ```
- **返回值类型**：`null`（无 Data 输出端口）
- **作用**：卸下玩家当前穿着的所有角色扮演服装，一键恢复默认外观。
- **参数说明**：无参数。
- **`.code` 示例**：
  ```code
  UnequipAllCosplay()
  ```

- **常见使用场景**：
  - 任务结束时一键清除所有服装状态
  - 强制玩家恢复默认外观进入下一个任务阶段
  - 出错恢复逻辑中快速重置玩家外观

- **相关节点**：EquipCosplay、UnequipCosplay、OwnCosplay、EquipAdultToy、UnequipAdultToy

---

## OwnCosplay

- **中文名**：拥有 Cosplay
- **官方 API 签名**：
  ```code
  OwnCosplay(owns = NewOwn, CosplayNameKey1[, CosplayNameKey2]...)
  OwnCosplay(owns = NewOwn, ListOfCosplayNameKeys1[, ListOfCosplayNameKeys2]...)
  ```
- **返回值类型**：`null`（无 Data 输出端口）
- **作用**：设置玩家是否拥有指定的角色扮演服装。与 EquipCosplay 不同，此节点控制的是"拥有"状态而非"穿着"状态。可以批量设置多件服装的拥有权。
- **参数说明**：

  | 参数 | 必填 | 类型 | 含义 |
  |------|------|------|------|
  | `owns` | 是 | Boolean | true = 授予服装，false = 剥夺服装 |
  | `cosplayKeys` | 是 | List | 服装键名列表，例如 ["Maid", "Bunny"] |

- **`.code` 示例**：
  ```code
  OwnCosplay(owns=true, cosplayKeys=["Maid"])
  ```

- **常见使用场景**：
  - 玩家完成任务后获得服装作为奖励
  - 在服装商店购买/解锁新服装
  - 限制玩家只能穿着已拥有的服装

- **相关节点**：EquipCosplay、UnequipCosplay、UnequipAllCosplay、EquipAdultToy

---

## EquipAdultToy

- **中文名**：装备成人玩具
- **官方 API 签名**：
  ```code
  EquipAdultToy(AdultToyName1[, AdultToyName2]...)
  EquipAdultToy(ListOfAdultToyNames1[, ListOfAdultToyNames2]...)
  ```
- **返回值类型**：`null`（无 Data 输出端口）
- **作用**：为玩家装备一个或多个成人玩具。玩具名称可传入多个字符串或列表参数。已装备的玩具随后可通过 SetVibrator / SetPiston 等节点控制。
- **参数说明**：

  | 参数 | 必填 | 类型 | 含义 |
  |------|------|------|------|
  | `toyNames` | 是 | List | 玩具名称列表：AnalPlug、Vibrator、EyeMask、Handcuff、KeyHandcuff、TimerHandcuff、TitRotor、KuriRotor、PistonFuta、PistonAnal、PistonPussy |

- **`.code` 示例**：
  ```code
  EquipAdultToy(toyNames=["Vibrator", "AnalPlug"])
  ```

- **常见使用场景**：
  - 任务中强制玩家装备特定玩具
  - 配合 SetVibrator / SetPiston 实现完整的玩具控制链
  - 作为任务惩罚或特定事件的前置条件

- **相关节点**：UnequipAdultToy、SetVibrator、SetPiston、EquipCosplay、LockHandcuffs

---

## UnequipAdultToy

- **中文名**：卸下成人玩具
- **官方 API 签名**：
  ```code
  UnequipAdultToy(AdultToyName1[, AdultToyName2]...)
  UnequipAdultToy(ListOfAdultToyNames1[, ListOfAdultToyNames2]...)
  ```
- **返回值类型**：`null`（无 Data 输出端口）
- **作用**：卸下玩家身上的一个或多个成人玩具。与 EquipAdultToy 相对，支持字符串和列表两种传参方式。
- **参数说明**：

  | 参数 | 必填 | 类型 | 含义 |
  |------|------|------|------|
  | `toyNames` | 是 | List | 要卸下的玩具名称列表，类型同 EquipAdultToy |

- **`.code` 示例**：
  ```code
  UnequipAdultToy(toyNames=["Vibrator"])
  ```

- **常见使用场景**：
  - 任务完成后移除玩家身上的玩具
  - 特定场景要求玩家不能携带玩具
  - 与 EquipAdultToy 配对实现动态玩具管理

- **相关节点**：EquipAdultToy、SetVibrator、SetPiston、UnequipAllCosplay、LockHandcuffs

---

## SetItemCount

- **中文名**：设置物品数量
- **官方 API 签名**：
  ```code
  SetItemCount(Item, newCount)
  ```
- **返回值类型**：`Number`（编辑器 Data 输出端口名 `out_count`）
- **作用**：将指定物品的数量设置为给定值，覆盖原有数量。返回设置后的新数量。物品来源包括普通消耗品和任务物品。
- **参数说明**：

  | 参数 | 必填 | 类型 | 含义 |
  |------|------|------|------|
  | `item` | 是 | 枚举 | 物品名称：None、Water、Dildo、InvisiblePotion、FutanariPotion、FutanariInversePotion、BodyPaint、InvisibleInversePotion、InvisiblePotionReusable、BodyPaintReusable、BodyPaintWasher、HandcuffKey、VibeRemocon、DroneController、DebugEarnRp 等 |
  | `count` | 是 | Number | 要设置的新数量 |

- **`.code` 示例**：
  ```code
  var_n1_out_count = SetItemCount(item="Water", count=5)
  ```

- **常见使用场景**：
  - 任务开始时给玩家初始物品
  - 重置物品数量到特定值
  - 在商店购买/出售后直接设置库存

- **相关节点**：AddItemCount、GetItemCount、DropItem、CollectItem

---

## AddItemCount

- **中文名**：增加物品数量
- **官方 API 签名**：
  ```code
  AddItemCount(Item, addCount)
  ```
- **返回值类型**：`Number`（编辑器 Data 输出端口名 `out_count`）
- **作用**：增加（或减少，传负数）指定物品的数量。返回增加后的新数量。适合实现渐变式物品增减。
- **参数说明**：

  | 参数 | 必填 | 类型 | 含义 |
  |------|------|------|------|
  | `item` | 是 | 枚举 | 物品名称，同 SetItemCount |
  | `count` | 是 | Number | 要增加的数量（负数表示减少） |

- **`.code` 示例**：
  ```code
  var_n1_out_count = AddItemCount(item="Water", count=1)
  ```

- **常见使用场景**：
  - 玩家喝水后减少 Water 数量
  - 拾取道具后增加对应物品数量
  - 每帧检查任务条件时动态增减物品

- **相关节点**：SetItemCount、GetItemCount、DropItem、CollectItem

---

## GetItemCount

- **中文名**：获取物品数量
- **官方 API 签名**：
  ```code
  GetItemCount(Item)
  ```
- **返回值类型**：`Number`（编辑器 Data 输出端口名 `out_count`）
- **作用**：查询玩家当前持有指定物品的数量。用于条件判断或 UI 显示。
- **参数说明**：

  | 参数 | 必填 | 类型 | 含义 |
  |------|------|------|------|
  | `item` | 是 | String（编辑器为枚举选择） | 要查询数量的物品名称 |

- **`.code` 示例**：
  ```code
  var_n1_out_count = GetItemCount(item="Water")
  ```

- **常见使用场景**：
  - 条件判断：玩家是否有足够数量的物品
  - 更新 UI 显示玩家当前物品数量
  - 配合 If 节点实现"物品数量不足"的分支逻辑

- **相关节点**：SetItemCount、AddItemCount、DropItem、CollectItem、If

> 本文档涵盖与玩家位置、场景、相机、动作、技能、数值统计、游戏控制、性爱相关的全部节点。
>
> 对应源文件：`src/api/definitions.rs`（Game API: Player State / Stats / Game Control）、`docs/kb/documentation_part_003.md`（函数定义）。
>
> 代码生成方式：B 类（通用 `generate_node_call`），参数按 `param.name=value` 格式拼接。

---

## SetPlayerPosition

| 属性 | 值 |
|------|-----|
| **中文名** | 设置玩家位置 |
| **API 签名** | `SetPlayerPosition([x = PositionX, y = PositionY, z = PositionZ][, rx = RotationX, ry = RotationY, rz = RotationZ, rw = RotationW])` / `SetPlayerPosition([position = Position][, rotation = Rotation])` |
| **返回值** | `null` |
| **分类** | Game API: Player State |
| **类别** | B 类 |

### 作用

将玩家传送到指定坐标位置，并可选的设置旋转四元数。支持两种调用形式：分别传入 x/y/z 分量，或传入 Vector 对象。

### 参数说明

| 参数名 | 必填 | 类型 | 说明 |
|--------|------|------|------|
| `position` | 是 | Vector | 目标位置的三维坐标 (x, y, z) |
| `rotation` | 否 | Quaternion | 旋转四元数 (rx, ry, rz, rw)，缺省不改变旋转 |

### 示例

```code
SetPlayerPosition(position={x=-26.6, y=-0.1, z=-120}, rotation=null)
```

### 常见场景

- 任务开始将玩家放到特定地点
- 触发事件后传送玩家到屋内/屋外
- 多个阶段切换时的位置复位

### 相关节点

SetCamera, SetStage, SetAction, GetRandomPosition

---

## SetStage

| 属性 | 值 |
|------|-----|
| **中文名** | 切换场景 |
| **API 签名** | `SetStage([StageType][, daytime = DayTime])` |
| **返回值** | `null` |
| **分类** | Game API: Player State |
| **类别** | B 类 |

### 作用

将玩家切换到指定场景（StageType），并可选择指定是否为白天。切换场景会重新加载该场景的所有物件、NPC 和传送门状态。

### 参数说明

| 参数名 | 必填 | 类型 | 说明 |
|--------|------|------|------|
| `stage` | 是 | 枚举 | 场景类型，可选值：None, Apart, Convenience, FashionShop, Residence, ShoppingMall, StationFront, Park, Mansion, TokyoStreet, Suburbs, Street, City, BarberShop, Laundry, Underpass |
| `daytime` | 否 | Boolean | `true`=白天，`false`=夜晚。缺省保持当前时间 |

### 示例

```code
SetStage(stage="Residence", daytime=true)
```

### 常见场景

- 任务开始时从家传送到商业街
- 夜间行动切换到夜晚模式
- 任务完成后返回住宅

### 相关节点

SetPlayerPosition, SetPortalEnabled, SetStageRankLimit, GetStageRankLimit

---

## SetCamera

| 属性 | 值 |
|------|-----|
| **中文名** | 设置相机 |
| **API 签名** | `SetCamera([pitch = Pitch][, yaw = Yaw][, lock = LockCamera])` |
| **返回值** | `null` |
| **分类** | Game API: Player State |
| **类别** | B 类 |

### 作用

设置当前相机（第一人称或第三人称）的方向。俯仰和偏航值以度为单位。第一人称相机的偏航与玩家模型绑定，因此不生效。设置 `lock=true` 可防止玩家旋转视角，必须在适当时机解锁（`lock=false`）。

### 参数说明

| 参数名 | 必填 | 类型 | 说明 |
|--------|------|------|------|
| `pitch` | 否 | Number | 俯仰角度（度），缺省不改变 |
| `yaw` | 否 | Number | 偏航角度（度），缺省不改变 |
| `lock` | 否 | Boolean | `true`=锁定相机旋转，`false`=解锁。游戏可能在特定事件中自行改变锁定状态 |

### 示例

```code
SetCamera(pitch=-30, lock=true)
```

### 常见场景

- 强制视角看向某个事件目标
- 过场动画期间锁定玩家视角
- 对话/互动后恢复视角控制

### 相关节点

SetPlayerPosition, SetAction, ShowBlackscreen, SetGraphicsOption

---

## SetAction

| 属性 | 值 |
|------|-----|
| **中文名** | 设置动作 |
| **API 签名** | `SetAction(Action)` |
| **返回值** | `null` |
| **分类** | Game API: Player State |
| **类别** | B 类 |

### 作用

强制玩家播放指定动作。动作枚举涵盖了站立、行走、互动、性爱、排泄等多种动画。动作会自动播放至结束或被下一个动作覆盖。

### 参数说明

| 参数名 | 必填 | 类型 | 说明 |
|--------|------|------|------|
| `action` | 是 | 枚举 | 动作名称，可选值：None, OldOnaniNormal, OldGanimataWalk, Pinpon, ConbiniTakeGoods, CrouchCry, EatMedicine, SadHandcuffAtMap, SwitchTimeStop, SwitchPistonMachine, PickingCoat, Pick, Drop, ChangeClothes, DroppingClothes, HandOver, InsertAnalPlug, ExtractAnalPlug, CommonEquip, IntoWasher, TakeFromWasher, UseBuyMachine, DrinkWater, PeeNormal, TakeOffPants, TakeOnPants, TakeOffBra, TakeOnBra, Sad, AttachHandcuffs, PutHandcuffsOnMap, HandcuffsAtMap, UnlockHandcuffsAtMap, AttachEyeMask, SwitchVibrator, PickUpItem, SitDown, StandUp, PutDildoFloor, PutDildoWall, 等 |

### 示例

```code
SetAction(action="SitDown")
```

### 常见场景

- 让玩家坐在椅子上
- 强制触发喝水/排泄动画
- 任务演出中的指定动作

### 相关节点

SetPlayerPosition, SetCamera, SetFutanari, SetSkillShortcut, DeactivateSex

---

## SetFutanari

| 属性 | 值 |
|------|-----|
| **中文名** | 设置双性状态 |
| **API 签名** | `SetFutanari(FutanariActive)` |
| **返回值** | `null` |
| **分类** | Game API: Player State |
| **类别** | B 类 |

### 作用

启用或禁用玩家的扶她（双性）状态。启用后玩家角色会长出阴茎，可在性爱中使用对应的动作。

### 参数说明

| 参数名 | 必填 | 类型 | 说明 |
|--------|------|------|------|
| `active` | 是 | Boolean | `true`=启用扶她状态，`false`=禁用 |

### 示例

```code
SetFutanari(active=true)
```

### 常见场景

- 任务中玩家获得扶她药水后的效果
- 特定性爱场景前开启
- 任务完成后关闭

### 相关节点

SetAction, SetSkill, SetSexPosition, SetSexMenu, DeactivateSex

---

## SetSkill

| 属性 | 值 |
|------|-----|
| **中文名** | 设置技能 |
| **API 签名** | `SetSkill(Skill, Enabled)` |
| **返回值** | `null` |
| **分类** | Game API: Player State |
| **类别** | B 类 |

### 作用

启用或禁用指定技能。技能控制玩家的各种能力和限制，如 Mental（精神力）、Stamina（体力上限）、TimeStop（时停）、Sneak（潜行）等。

### 参数说明

| 参数名 | 必填 | 类型 | 说明 |
|--------|------|------|------|
| `skill` | 是 | 枚举 | 技能名称，可选值：None, Mental, Stamina, CoatLevel, Flasher, Raper, ContinueMission, Slow, NpcDirect, Sneak, AutoSlow, FixFps, MaxAccessoryNum, AutoBaretaSlow, HideStrangeUi, NoFastTravel, Perspective, MyPace, TimeStop, CantDash, DisableHideCostume, FixTps, Exhibitionism, Sex, AutoAddMoisture, NoReinforceEffect, GanimataWalk, 等 |
| `enabled` | 是 | Boolean | `true`=启用，`false`=禁用 |

### 示例

```code
SetSkill(skill="Sneak", enabled=true)
```

### 常见场景

- 任务中临时禁用玩家的时停能力
- 解锁潜行模式
- 装备/卸除特定技能影响玩法

### 相关节点

SetFutanari, SetPlayerData, SetAction, SetStamina

---

## SetPlayerData

| 属性 | 值 |
|------|-----|
| **中文名** | 设置玩家数据 |
| **API 签名** | `SetPlayerData(DataName, DataValue)` / `SetPlayerData("BodyPaintTypeDict", BodyPaintType, Active)` / `SetPlayerData("HairCustomizeDataDict", HairType, DataName, DataValue)` |
| **返回值** | `null` |
| **分类** | Game API: Player State |
| **类别** | B 类 |

### 作用

设置玩家的任意数据字段。通用形式为 `SetPlayerData("键名", 值)`，也支持对 `BodyPaintTypeDict`（身体彩绘）和 `HairCustomizeDataDict`（发型自定义）的特殊重载形式。

### 参数说明

| 参数名 | 必填 | 类型 | 说明 |
|--------|------|------|------|
| `dataName` | 是 | String | 要修改的数据字段名（如 `"BodyPaintTypeDict"`、`"HairCustomizeDataDict"` 等） |
| `value` | 是 | List | 要设置的值，以 List 形式传入。对于通用形式为单值，对于特殊字典为复合数据 |

### 示例

```code
SetPlayerData(dataName="BodyPaintTypeDict", value={key="Flower", active=true})
```

### 常见场景

- 改变玩家身体彩绘
- 自定义玩家发型/颜色
- 修改玩家存档中的任意数据字段

### 相关节点

SetSkill, SetFutanari, SetAction, GetStateBool, GetStateNumber

---

## SetSkillShortcut

| 属性 | 值 |
|------|-----|
| **中文名** | 设置技能快捷栏 |
| **API 签名** | `SetSkillShortcut(Slot, ActionIndex)` |
| **返回值** | `null` |
| **分类** | Game API: Player State |
| **类别** | B 类 |

### 作用

将指定的动作索引绑定到快捷栏槽位（0-7）。Slot 0-7 对应游戏中快捷轮的 8 个位置。修改后会立即生效。

### 参数说明

| 参数名 | 必填 | 类型 | 说明 |
|--------|------|------|------|
| `slot` | 是 | Number | 快捷栏槽位索引，范围 0-7 |
| `actionIndex` | 是 | Number | 动作索引（见下表的动作映射表） |

**快捷栏动作映射：**

| 索引 | 动作 | 索引 | 动作 |
|------|------|------|------|
| 0 | Auto Run | 16 | Penis |
| 1 | Chase | 17 | Supine One-Leg Raise |
| 2 | Dildo | 18 | Armpit Squat |
| 3 | Vibrator Switch | 19 | High-Leg Pose |
| 4 | Handcuffs | 20 | Hip Thrust |
| 5 | Piston Machine Switch | 21 | Masturbate |
| 6 | Time Stop | 22 | Three-Leg Masturbation |
| 7 | Urination | 23 | Supine Masturbation |
| 8 | Spread-Leg Blowjob | 24 | Doggy Masturbation |
| 9 | I-Split Balance | 25 | Nipple Play |
| 10 | Dogeza | 26 | Clit Play |
| 11 | Butt Wiggle | 27 | Stroking |
| 12 | Hand Bra | 28 | Spread-Leg Urination |
| 13 | Ahegao Double Peace | 29 | Standing Pee |
| 14 | Bowlegged All-Fours | 30 | Dog Pee |
| 15 | Spread-Leg Walk | | |

### 示例

```code
SetSkillShortcut(slot=0, actionIndex=21)
```

### 常见场景

- 自定义玩家的快捷轮布局
- 任务中提供特殊动作快捷键
- 教程中引导玩家使用特定动作

### 相关节点

GetSkillShortcut, SetAction, SetSkill

---

## GetSkillShortcut

| 属性 | 值 |
|------|-----|
| **中文名** | 获取技能快捷栏 |
| **API 签名** | `GetSkillShortcut(Slot)` |
| **返回值** | `Number`（动作索引） |
| **分类** | Game API: Player State |
| **类别** | B 类 |

### 作用

获取指定快捷栏槽位当前绑定的动作索引。返回值可用作条件判断或后续 `SetSkillShortcut` 的输入。

### 参数说明

| 参数名 | 必填 | 类型 | 说明 |
|--------|------|------|------|
| `slot` | 是 | Number | 快捷栏槽位索引，范围 0-7 |

### 输出端口

| 端口名 | 类型 | 说明 |
|--------|------|------|
| `out_index` | Number | 当前槽位绑定的动作索引 |

### 示例

```code
var_node1_out_index = GetSkillShortcut(slot=0)
```

### 常见场景

- 检查快捷栏是否已设置
- 备份和恢复快捷栏配置
- 判断玩家是否装备了特定动作

### 相关节点

SetSkillShortcut, SetAction, SetSkill

---

## GetRandomPosition

| 属性 | 值 |
|------|-----|
| **中文名** | 随机位置 |
| **API 签名** | `GetRandomPosition(minRange)` |
| **返回值** | `List`（位置列表）或 `null` |
| **分类** | Game API: Player State |
| **类别** | B 类 |

### 作用

使用当前场景的 `StageRandomPositionManager` 获取一个随机位置。如果场景中没有可用的随机位置，返回 `null`。`minRange` 限制随机点与玩家的最小距离。

### 参数说明

| 参数名 | 必填 | 类型 | 说明 |
|--------|------|------|------|
| `minRange` | 否 | Number | 与玩家的最小距离范围，缺省为 0 |

### 输出端口

| 端口名 | 类型 | 说明 |
|--------|------|------|
| `out_position` | List | 随机位置数据，包含坐标信息 |

### 示例

```code
var_node1_out_position = GetRandomPosition(minRange=10)
```

### 常见场景

- 随机生成物品掉落位置
- NPC 巡逻路径的随机点
- 玩家被传送后的随机重生点

### 相关节点

SetPlayerPosition, GetAllWaypoints, CreateArea, CreateZone

---

## AddCurrentEarnRP

| 属性 | 值 |
|------|-----|
| **中文名** | 增加本次 RP |
| **API 签名** | `AddCurrentEarnRP(RPValue)` |
| **返回值** | `Number`（新的 RP 值） |
| **分类** | Game API: Stats |
| **类别** | B 类 |

### 作用

增加本次外出赚取的 RP（表现分）数值。返回增加后的新 RP 值。本次 RP 会在每次外出（离开住宅）时重置。

### 参数说明

| 参数名 | 必填 | 类型 | 说明 |
|--------|------|------|------|
| `value` | 是 | Number | 要增加的 RP 数值（可为负数） |

### 输出端口

| 端口名 | 类型 | 说明 |
|--------|------|------|
| `out_value` | Number | 增加后的新 RP 值 |

### 示例

```code
var_node1_out_value = AddCurrentEarnRP(value=100)
```

### 常见场景

- 完成任务目标后给予 RP 奖励
- 触发惩罚事件时扣减 RP
- 阶段性结算界面显示

### 相关节点

SetCurrentEarnRP, GetCurrentEarnRP, AddCurrentRP, SetCurrentRP, GetCurrentRP

---

## SetCurrentEarnRP

| 属性 | 值 |
|------|-----|
| **中文名** | 设置本次 RP |
| **API 签名** | `SetCurrentEarnRP(RPValue)` |
| **返回值** | `Number`（新的 RP 值） |
| **分类** | Game API: Stats |
| **类别** | B 类 |

### 作用

直接设置本次外出赚取的 RP 为指定值。返回设置后的新 RP 值。与 `AddCurrentEarnRP` 的区别是覆盖而非累加。

### 参数说明

| 参数名 | 必填 | 类型 | 说明 |
|--------|------|------|------|
| `value` | 是 | Number | 要设置的 RP 数值 |

### 输出端口

| 端口名 | 类型 | 说明 |
|--------|------|------|
| `out_value` | Number | 设置后的新 RP 值 |

### 示例

```code
var_node1_out_value = SetCurrentEarnRP(value=500)
```

### 常见场景

- 任务开始将本次 RP 归零
- 检查点恢复 RP 到固定值
- 特殊事件强制设定 RP

### 相关节点

AddCurrentEarnRP, GetCurrentEarnRP, AddCurrentRP, SetCurrentRP

---

## GetCurrentEarnRP

| 属性 | 值 |
|------|-----|
| **中文名** | 获取本次 RP |
| **API 签名** | `GetCurrentEarnRP()` |
| **返回值** | `Number`（当前 RP 值） |
| **分类** | Game API: Stats |
| **类别** | B 类 |

### 作用

获取本次外出目前赚取的总 RP 值。此值在每次外出时重置，用于判定玩家本次表现。

### 输出端口

| 端口名 | 类型 | 说明 |
|--------|------|------|
| `out_value` | Number | 当前本次 RP 值 |

### 示例

```code
var_node1_out_value = GetCurrentEarnRP()
```

### 常见场景

- 条件判断是否达到目标 RP
- RP 值显示在 UI 中
- 和 `AddCurrentEarnRP` 配合结算

### 相关节点

AddCurrentEarnRP, SetCurrentEarnRP, AddCurrentRP, GetCurrentRP

---

## AddCurrentRP

| 属性 | 值 |
|------|-----|
| **中文名** | 增加持有 RP |
| **API 签名** | `AddCurrentRP(RPValue)` |
| **返回值** | `Number`（新的 RP 值） |
| **分类** | Game API: Stats |
| **类别** | B 类 |

### 作用

增加玩家当前持有的总 RP 数值。注意不要与 `AddCurrentEarnRP`（本次 RP）混淆。持有 RP 是玩家累积的总资产，跨外出不重置。

### 参数说明

| 参数名 | 必填 | 类型 | 说明 |
|--------|------|------|------|
| `value` | 是 | Number | 要增加的 RP 数值（可为负数） |

### 输出端口

| 端口名 | 类型 | 说明 |
|--------|------|------|
| `out_value` | Number | 增加后的新 RP 值 |

### 示例

```code
var_node1_out_value = AddCurrentRP(value=1000)
```

### 常见场景

- 任务完成时结算奖励到总 RP
- 购买物品时扣减总 RP
- 每日登录奖励

### 相关节点

SetCurrentRP, GetCurrentRP, AddCurrentEarnRP, SetCurrentEarnRP

---

## SetCurrentRP

| 属性 | 值 |
|------|-----|
| **中文名** | 设置持有 RP |
| **API 签名** | `SetCurrentRP(RPValue)` |
| **返回值** | `Number`（新的 RP 值） |
| **分类** | Game API: Stats |
| **类别** | B 类 |

### 作用

直接设置玩家当前持有的总 RP 数值。覆盖而非累加。可用于存档恢复、作弊调试或特殊事件强制调整。

### 参数说明

| 参数名 | 必填 | 类型 | 说明 |
|--------|------|------|------|
| `value` | 是 | Number | 要设置的 RP 数值 |

### 输出端口

| 端口名 | 类型 | 说明 |
|--------|------|------|
| `out_value` | Number | 设置后的新 RP 值 |

### 示例

```code
var_node1_out_value = SetCurrentRP(value=9999)
```

### 常见场景

- 读档后恢复 RP
- 付费 DLC 解锁时赠送 RP
- 管理员/调试指令

### 相关节点

AddCurrentRP, GetCurrentRP, AddCurrentEarnRP, SetCurrentEarnRP

---

## GetCurrentRP

| 属性 | 值 |
|------|-----|
| **中文名** | 获取持有 RP |
| **API 签名** | `GetCurrentRP()` |
| **返回值** | `Number`（当前 RP 值） |
| **分类** | Game API: Stats |
| **类别** | B 类 |

### 作用

获取玩家当前持有的总 RP 数值。此值跨外出不重置，是玩家的永久资产。

### 输出端口

| 端口名 | 类型 | 说明 |
|--------|------|------|
| `out_value` | Number | 当前持有 RP 值 |

### 示例

```code
var_node1_out_value = GetCurrentRP()
```

### 常见场景

- 商店界面显示玩家余额
- 条件判断是否足以购买物品
- 任务前置条件检查

### 相关节点

AddCurrentRP, SetCurrentRP, AddCurrentEarnRP, GetCurrentEarnRP

---

## SetEcstasy

| 属性 | 值 |
|------|-----|
| **中文名** | 设置快感 |
| **API 签名** | `SetEcstasy(Value)` |
| **返回值** | `Number`（新的快感值） |
| **分类** | Game API: Stats |
| **类别** | B 类 |

### 作用

直接设置玩家的快感数值。快感值影响性爱相关行为和反应，达到一定程度会触发高潮。

### 参数说明

| 参数名 | 必填 | 类型 | 说明 |
|--------|------|------|------|
| `value` | 是 | Number | 快感数值（通常 0.0 - 1.0 范围） |

### 输出端口

| 端口名 | 类型 | 说明 |
|--------|------|------|
| `out_value` | Number | 设置后的新快感值 |

### 示例

```code
var_node1_out_value = SetEcstasy(value=0.5)
```

### 常见场景

- 性爱场景中调整刺激进度
- 强制触发高潮事件
- 恢复默认快感值

### 相关节点

AddEcstasy, GetEcstasy, SetStamina, SetMoisture, TriggerSexOrgasm

---

## AddEcstasy

| 属性 | 值 |
|------|-----|
| **中文名** | 增加快感 |
| **API 签名** | `AddEcstasy(Value)` |
| **返回值** | `Number`（新的快感值） |
| **分类** | Game API: Stats |
| **类别** | B 类 |

### 作用

增加或减少玩家的快感数值。正数增加快感，负数减少快感。

### 参数说明

| 参数名 | 必填 | 类型 | 说明 |
|--------|------|------|------|
| `value` | 是 | Number | 要增减的快感数值（可为负数） |

### 输出端口

| 端口名 | 类型 | 说明 |
|--------|------|------|
| `out_value` | Number | 增减后的新快感值 |

### 示例

```code
var_node1_out_value = AddEcstasy(value=0.2)
```

### 常见场景

- 性爱互动中逐步累积快感
- 使用道具增加快感
- 冷却期间减少快感

### 相关节点

SetEcstasy, GetEcstasy, SetStamina, AddStamina, SetMoisture

---

## GetEcstasy

| 属性 | 值 |
|------|-----|
| **中文名** | 获取快感 |
| **API 签名** | `GetEcstasy()` |
| **返回值** | `Number`（当前快感值） |
| **分类** | Game API: Stats |
| **类别** | B 类 |

### 作用

获取玩家当前的快感数值。

### 输出端口

| 端口名 | 类型 | 说明 |
|--------|------|------|
| `out_value` | Number | 当前快感值 |

### 示例

```code
var_node1_out_value = GetEcstasy()
```

### 常见场景

- 条件判断快感是否达到高潮阈值
- 快感值显示在 UI 中
- 状态恢复后检查

### 相关节点

SetEcstasy, AddEcstasy, GetStamina, GetMoisture, CanGameOver

---

## SetStamina

| 属性 | 值 |
|------|-----|
| **中文名** | 设置体力 |
| **API 签名** | `SetStamina(Value)` |
| **返回值** | `Number`（新的体力值） |
| **分类** | Game API: Stats |
| **类别** | B 类 |

### 作用

直接设置玩家的当前体力值。可通过 `_state.StaminaMax` 获取体力上限。体力影响玩家奔跑、动作的持续时间。

### 参数说明

| 参数名 | 必填 | 类型 | 说明 |
|--------|------|------|------|
| `value` | 是 | Number | 体力数值 |

### 输出端口

| 端口名 | 类型 | 说明 |
|--------|------|------|
| `out_value` | Number | 设置后的新体力值 |

### 示例

```code
var_node1_out_value = SetStamina(value=100)
```

### 常见场景

- 任务开始时恢复满体力
- 特殊事件将体力设为特定值
- 使用道具后恢复体力

### 相关节点

AddStamina, GetStamina, SetEcstasy, SetMoisture, AddCurrentRP

---

## AddStamina

| 属性 | 值 |
|------|-----|
| **中文名** | 增加体力 |
| **API 签名** | `AddStamina(Value)` |
| **返回值** | `Number`（新的体力值） |
| **分类** | Game API: Stats |
| **类别** | B 类 |

### 作用

增减玩家的当前体力值。正数恢复体力，负数消耗体力。

### 参数说明

| 参数名 | 必填 | 类型 | 说明 |
|--------|------|------|------|
| `value` | 是 | Number | 要增减的体力值（可为负数） |

### 输出端口

| 端口名 | 类型 | 说明 |
|--------|------|------|
| `out_value` | Number | 增减后的新体力值 |

### 示例

```code
var_node1_out_value = AddStamina(value=-20)
```

### 常见场景

- 奔跑/被追逐时扣减体力
- 喝水或休息后恢复体力
- 技能消耗体力

### 相关节点

SetStamina, GetStamina, AddEcstasy, AddMoisture

---

## GetStamina

| 属性 | 值 |
|------|-----|
| **中文名** | 获取体力 |
| **API 签名** | `GetStamina()` |
| **返回值** | `Number`（当前体力值） |
| **分类** | Game API: Stats |
| **类别** | B 类 |

### 作用

获取玩家当前的体力数值。体力最大值可通过 `_state.StaminaMax` 获取。

### 输出端口

| 端口名 | 类型 | 说明 |
|--------|------|------|
| `out_value` | Number | 当前体力值 |

### 示例

```code
var_node1_out_value = GetStamina()
```

### 常见场景

- 条件判断体力是否耗尽
- UI 显示体力条
- 决定是否可执行消耗体力的动作

### 相关节点

SetStamina, AddStamina, GetEcstasy, GetMoisture, GetCurrentRP

---

## SetMoisture

| 属性 | 值 |
|------|-----|
| **中文名** | 设置湿润度 |
| **API 签名** | `SetMoisture(Value)` |
| **返回值** | `Number`（新的湿润度） |
| **分类** | Game API: Stats |
| **类别** | B 类 |

### 作用

直接设置玩家膀胱的充盈（湿润）程度。此值影响排尿相关事件和自动增加潮湿度的技能。

### 参数说明

| 参数名 | 必填 | 类型 | 说明 |
|--------|------|------|------|
| `value` | 是 | Number | 湿润度数值 |

### 输出端口

| 端口名 | 类型 | 说明 |
|--------|------|------|
| `out_value` | Number | 设置后的新湿润度 |

### 示例

```code
var_node1_out_value = SetMoisture(value=50)
```

### 常见场景

- 喝水后增加膀胱充盈度
- 排尿后归零
- 使用利尿剂后设置特定值

### 相关节点

AddMoisture, GetMoisture, SetStamina, SetEcstasy, SetSkill

---

## AddMoisture

| 属性 | 值 |
|------|-----|
| **中文名** | 增加湿润度 |
| **API 签名** | `AddMoisture(Value)` |
| **返回值** | `Number`（新的湿润度） |
| **分类** | Game API: Stats |
| **类别** | B 类 |

### 作用

增减玩家的膀胱充盈度。正数增加湿润度，负数减少（排尿时扣除）。

### 参数说明

| 参数名 | 必填 | 类型 | 说明 |
|--------|------|------|------|
| `value` | 是 | Number | 要增减的湿润度数值（可为负数） |

### 输出端口

| 端口名 | 类型 | 说明 |
|--------|------|------|
| `out_value` | Number | 增减后的新湿润度 |

### 示例

```code
var_node1_out_value = AddMoisture(value=10)
```

### 常见场景

- 随时间自动增加湿润度
- 喝水后增加
- 排尿时用负数扣除

### 相关节点

SetMoisture, GetMoisture, AddStamina, SetSkill (AutoAddMoisture)

---

## GetMoisture

| 属性 | 值 |
|------|-----|
| **中文名** | 获取湿润度 |
| **API 签名** | `GetMoisture()` |
| **返回值** | `Number`（当前湿润度） |
| **分类** | Game API: Stats |
| **类别** | B 类 |

### 作用

获取玩家当前的膀胱充盈（湿润）度数值。

### 输出端口

| 端口名 | 类型 | 说明 |
|--------|------|------|
| `out_value` | Number | 当前湿润度 |

### 示例

```code
var_node1_out_value = GetMoisture()
```

### 常见场景

- 条件判断是否达到必须排尿的程度
- UI 显示膀胱条
- 检测是否需要找厕所

### 相关节点

SetMoisture, AddMoisture, SetStamina, GetStamina, GetEcstasy

---

## CanGameOver

| 属性 | 值 |
|------|-----|
| **中文名** | 可游戏结束 |
| **API 签名** | `CanGameOver([Value])` |
| **返回值** | `Boolean` |
| **分类** | Game API: Game Control |
| **类别** | B 类 |

### 作用

设置或获取游戏是否允许在 NPC 发现玩家时触发 Game Over。当设置为 `false` 时，NPC 发现玩家不会导致游戏结束。可通过 `_state.FoundNPC` 获取发现玩家的 NPC ID（每 NPC 每帧触发一次，未触发时为 -1）。

### 参数说明

| 参数名 | 必填 | 类型 | 说明 |
|--------|------|------|------|
| `value` | 否 | Boolean | 不传时返回当前状态；传入时设置：`true`=允许 Game Over，`false`=禁止 Game Over |

### 输出端口

| 端口名 | 类型 | 说明 |
|--------|------|------|
| `out_value` | Boolean | 当前可游戏结束状态 |

### 示例

```code
var_node1_out_value = CanGameOver(value=false)
```

### 常见场景

- 潜入任务中暂时禁用 Game Over
- 剧情阶段保护玩家
- 检查当前是否允许 Game Over

### 相关节点

TriggerGameOver, SetStageRankLimit, SetPortalEnabled, SetStage

---

## TriggerGameOver

| 属性 | 值 |
|------|-----|
| **中文名** | 触发游戏结束 |
| **API 签名** | `TriggerGameOver()` |
| **返回值** | `null` |
| **分类** | Game API: Game Control |
| **类别** | B 类 |

### 作用

强制触发游戏结束（Game Over）画面。即使玩家拥有防止被抓的技能也会生效。Game Over 状态和 `_stage.GameOver` 会保持为 `true`，直到玩家再次离开住宅。

### 示例

```code
TriggerGameOver()
```

### 常见场景

- 任务失败条件触发
- 特殊结局/惩罚事件
- 剧情强制重启

### 相关节点

CanGameOver, SetStage, SetPortalEnabled, SetStageRankLimit

---

## SetStageRankLimit

| 属性 | 值 |
|------|-----|
| **中文名** | 设置场景等级限制 |
| **API 签名** | `SetStageRankLimit(StageType, Rank)` |
| **返回值** | `null` |
| **分类** | Game API: Game Control |
| **类别** | B 类 |

### 作用

更改进入指定场景所需的 RP 等级限制。当等级需求被提高时，该场景的所有传送门将被禁用，快速旅行也将不可用。传入小于 0 的 Rank 值可重置为默认限制。

### 参数说明

| 参数名 | 必填 | 类型 | 说明 |
|--------|------|------|------|
| `stage` | 是 | 枚举 | 场景类型（同 SetStage） |
| `rank` | 是 | Number | 等级限制值；小于 0 时恢复默认 |

### 示例

```code
SetStageRankLimit(stage="ShoppingMall", rank=5)
```

### 常见场景

- 根据玩家进度解锁新区域
- 剧情中暂时封锁某个场景
- 后期降低限制方便快速移动

### 相关节点

GetStageRankLimit, SetPortalEnabled, SetStage, CanGameOver

---

## GetStageRankLimit

| 属性 | 值 |
|------|-----|
| **中文名** | 获取场景等级限制 |
| **API 签名** | `GetStageRankLimit(StageType)` |
| **返回值** | `Number`（等级限制） |
| **分类** | Game API: Game Control |
| **类别** | B 类 |

### 作用

获取指定场景当前所需的 RP 等级限制。返回值为 `Number`。

### 参数说明

| 参数名 | 必填 | 类型 | 说明 |
|--------|------|------|------|
| `stage` | 是 | String | 场景类型名称 |

### 输出端口

| 端口名 | 类型 | 说明 |
|--------|------|------|
| `out_rank` | Number | 该场景的当前等级限制 |

### 示例

```code
var_node1_out_rank = GetStageRankLimit(stage="Park")
```

### 常见场景

- 检查玩家是否达到某个场景的进入要求
- 显示在任务面板的场景需求中
- 与 `GetCurrentRP` 配合做条件判断

### 相关节点

SetStageRankLimit, GetCurrentRP, SetPortalEnabled, SetStage

---

## SetPortalEnabled

| 属性 | 值 |
|------|-----|
| **中文名** | 设置传送门 |
| **API 签名** | `SetPortalEnabled(StageType, Enabled)` |
| **返回值** | `null` |
| **分类** | Game API: Game Control |
| **类别** | B 类 |

### 作用

启用或禁用前往指定场景的传送门。注意：禁用传送门不影响快速旅行。传送门状态在玩家前往其他场景后重置。

### 参数说明

| 参数名 | 必填 | 类型 | 说明 |
|--------|------|------|------|
| `stage` | 是 | 枚举 | 场景类型（同 SetStage） |
| `enabled` | 是 | Boolean | `true`=启用传送门，`false`=禁用传送门 |

### 示例

```code
SetPortalEnabled(stage="Mansion", enabled=false)
```

### 常见场景

- 剧情中禁用某区域的入口
- 解锁新区域的传送门
- 限制玩家当前可去范围

### 相关节点

SetStageRankLimit, GetStageRankLimit, SetStage, CanGameOver

---

## GetAllWaypoints

| 属性 | 值 |
|------|-----|
| **中文名** | 获取路径点 |
| **API 签名** | `GetAllWaypoints()` |
| **返回值** | `List`（路径点列表） |
| **分类** | Game API: Game Control |
| **类别** | B 类 |

### 作用

获取当前场景中的所有路径点（Waypoint）列表。每个路径点包含 `_Type`、`_RouteInteractType` 和 `_Position` 字段，可用于识别并与 NPC 对象配合使用。

### 输出端口

| 端口名 | 类型 | 说明 |
|--------|------|------|
| `out_waypoints` | List | 路径点列表，每个元素包含类型、交互类型、位置信息 |

### 示例

```code
var_node1_out_waypoints = GetAllWaypoints()
```

### 常见场景

- 获取当前场景中所有椅子/自动售货机位置
- 导航 NPC 到指定路径点
- 检查场景中可用的交互点

### 相关节点

GetRandomPosition, CreateArea, CreateZone, CreateInteractArea

---

## SetSexPosition

| 属性 | 值 |
|------|-----|
| **中文名** | 设置性爱体位 |
| **API 签名** | `SetSexPosition(SexPosition)` |
| **返回值** | `null` |
| **分类** | Game API: Game Control |
| **类别** | B 类 |

### 作用

设置当前性爱场景的体位。体位值必须为 `"StandBack"`（背后位）或 `"Kijoui"`（骑乘位）。

### 参数说明

| 参数名 | 必填 | 类型 | 说明 |
|--------|------|------|------|
| `position` | 是 | 枚举 | 体位名称：`"StandBack"` 或 `"Kijoui"` |

### 示例

```code
SetSexPosition(position="Kijoui")
```

### 常见场景

- 性爱场景中切换体位
- 根据剧情需要设置特定体位
- 与 `SetSexMenu` 配合控制体位选项

### 相关节点

DeactivateSex, SetSexMenu, SetAction, SetFutanari, TriggerSexOrgasm

---

## DeactivateSex

| 属性 | 值 |
|------|-----|
| **中文名** | 停用性爱 |
| **API 签名** | `DeactivateSex()` |
| **返回值** | `null` |
| **分类** | Game API: Game Control |
| **类别** | B 类 |

### 作用

退出当前的性爱场景。调用后玩家恢复常态，性爱相关的 UI 和动作全部停止。

### 示例

```code
DeactivateSex()
```

### 常见场景

- 性爱完成后退出
- 外部事件中断性爱
- NPC 发现时强制退出

### 相关节点

SetSexPosition, SetSexMenu, SetAction, SetFutanari, TriggerGameOver

---

## SetSexMenu

| 属性 | 值 |
|------|-----|
| **中文名** | 设置性爱菜单 |
| **API 签名** | `SetSexMenu([canfinish = CanFinish][, canposition = CanChangePosition])` / `SetSexMenu([canfinish = CanFinish][, canposition = CanChangeToPosition])` |
| **返回值** | `null` |
| **分类** | Game API: Game Control |
| **类别** | B 类 |

### 作用

配置性爱 UI 菜单的选项按钮。`canfinish` 控制是否显示"完成"按钮。`canposition` 可以传布尔值（启用/禁用所有体位按钮），或传 List（键为体位索引的整数字符串，值为布尔值，用于单独控制每个体位按钮）。

### 参数说明

| 参数名 | 必填 | 类型 | 说明 |
|--------|------|------|------|
| `canfinish` | 否 | Boolean | `true`=显示"完成"按钮，`false`=隐藏 |
| `canposition` | 否 | List/Boolean | 布尔值时：`true`=启用所有体位按钮，`false`=禁用所有；List 时：键为体位 0-based 索引，值为布尔值 |

### 示例

```code
SetSexMenu(canfinish=false, canposition={0=true, 1=false})
```

### 常见场景

- 强制玩家不能提前结束性爱
- 只允许特定体位
- 剧情中隐藏完整菜单选项

### 相关节点

SetSexPosition, DeactivateSex, SetAction, SetFutanari, TriggerSexOrgasm

> 本文档涵盖对象构造类节点和视觉/图形功能节点。
> 分类依据：`src/api/definitions.rs` 的 `category` 字段与 `docs/node_types.md` 的场景分类。
> 代码生成方式：除特殊说明外均为 **B 类**（通用 `generate_node_call`），自动生成 `var_{id}_{port} = FunctionName(params...)`。

---

## CreateMissionPanel

- **中文名**：任务面板
- **类别**：Objects（对象）
- **代码生成类别**：B
- **官方 API 签名**：`CreateMissionPanel()`
- **返回值类型**：Object（MissionPanel 对象）
- **作用**：创建一个任务面板，用于在 HUD 上显示任务标题、RP 奖励、进度条等信息。面板可通过返回的对象方法设置文本、可见性、进度条等。

### 参数说明

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| — | — | — | 无参数 |

### 输出端口

| 端口 ID | 类型 | 标签 |
|---------|------|------|
| `out_flow` | Flow | 下一步 |
| `out_panel` | Object | 面板 |

### `.code` 示例

```code
panel1 = CreateMissionPanel()
panel1.SetText("Collect 5 Coins")
panel1.SetRPText("500 RP")
panel1.SetGaugeVisible(true)
panel1.SetGaugeProgress(0.5)
panel1.SetVisible(true)
```

### 常见使用场景

- 任务追踪面板：显示当前目标文本
- RP 奖励展示：在任务面板中设置 RP 文本
- 进度条显示：配合收集类任务显示完成比例

### 相关节点

- `CreateMissionMenuItem` — 任务菜单项
- `CreateText` — 文本对象
- `CreateNPC` — NPC 对象
- `CreateInput` — 输入检测

---

## CreateMissionMenuItem

- **中文名**：任务菜单项
- **类别**：Objects（对象）
- **代码生成类别**：B
- **官方 API 签名**：`CreateMissionMenuItem()`
- **返回值类型**：Object（MissionMenuItem 对象）
- **作用**：创建任务菜单项，显示在 Tab 菜单的任务列表中。支持文本、RP 值、完成标记、最大 RP、背景颜色等属性。

### 参数说明

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| — | — | — | 无参数 |

### 输出端口

| 端口 ID | 类型 | 标签 |
|---------|------|------|
| `out_flow` | Flow | 下一步 |
| `out_item` | Object | 菜单项 |

### `.code` 示例

```code
missionItem = CreateMissionMenuItem()
missionItem.SetText("Side Quest: Park Cleanup")
missionItem.SetRPText("300RP")
missionItem.SetMaxRP(300)
missionItem.SetCleared(true)
missionItem.AutoColor(true)
missionItem.SetBackgroundColor(Color(0.495, 1.000, 0.521, 1.000), Color(0.000, 0.434, 0.024, 1.000))
```

### 常见使用场景

- 任务列表登记：在主菜单中注册子任务
- 任务完成状态标记：使用 `SetCleared` 标记已完成
- 自定义颜色风格：使用 `AutoColor` 或 `SetBackgroundColor` 美化

### 相关节点

- `CreateMissionPanel` — 任务面板
- `CreateArea` — 区域
- `CreateCondition` — 条件对象
- `CreateText` — 文本对象

---

## CreateArea

- **中文名**：创建区域
- **类别**：Objects（对象）
- **代码生成类别**：B
- **官方 API 签名**：`CreateArea(type = "sphere", stage, x, y, z, r[, outline][, compass])`  
  或 `CreateArea(type = "cylinder", stage, x, y, z, r, h[, outline][, compass])`  
  或 `CreateArea(type = "cuboid", stage, x1, y1, z1, x2, y2, z2, w, h[, outline][, compass])`
- **返回值类型**：Object（Area 对象）
- **作用**：在指定场景中创建一个检测区域（球体、圆柱体或长方体），用于检测玩家位置、触发事件或作为视觉引导。支持设置颜色、轮廓线和指南针图标。

### 参数说明

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `type` | 是 | Enum | 区域形状：`"sphere"`、`"cylinder"` 或 `"cuboid"` |
| `stage` | 是 | Enum (StageType) | 所属场景 |
| `position` | 是 | Vector | 中心位置（x, y, z）。球体/圆柱体用 `position`，长方体用 `position` 作为 Start（x1, y1, z1） |
| `r` | 否 | Number | 半径（球体/圆柱体） |
| `h` | 否 | Number | 高度（圆柱体/长方体） |
| `position2` | 否 | Vector | 长方体结束角（x2, y2, z2） |
| `w` | 否 | Number | 长方体宽度 |
| `outline` | 否 | Boolean | 是否显示轮廓线 |
| `compass` | 否 | String/Boolean | 指南针图标（True=默认图标，String=自定义图片路径） |

### 输出端口

| 端口 ID | 类型 | 标签 |
|---------|------|------|
| `out_flow` | Flow | 下一步 |
| `out_area` | Object | 区域 |

### `.code` 示例

```code
myArea = CreateArea(type="sphere", stage="Residence", x=0.0, y=0.0, z=-120.0, r=5.0, outline=true)
myArea.SetColor(1.0, 0.0, 0.0, 0.5)
```

### 常见使用场景

- 触发区域：玩家进入时触发对话或事件
- 危险区域标记：显示红色轮廓提示
- 导航引导：配合指南针图标引导玩家

### 相关节点

- `CreateZone` — 组合多个区域
- `CreateInteractArea` — 交互区域
- `CreateCondition` — 条件对象（配合 `Inside()` 检测）
- `CreateNPC` — NPC 对象

---

## CreateZone

- **中文名**：创建地带
- **类别**：Objects（对象）
- **代码生成类别**：B
- **官方 API 签名**：`CreateZone([Area1][, Area2]...)` 或 `CreateZone([ListOfAreas])`
- **返回值类型**：Object（Zone 对象）
- **作用**：将多个 Area 组合成一个 Zone，用于检测玩家是否在组合区域的范围内，或计算与最近区域的距离。

### 参数说明

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `areas` | 是 | List | Area 对象列表 |

### 输出端口

| 端口 ID | 类型 | 标签 |
|---------|------|------|
| `out_flow` | Flow | 下一步 |
| `out_zone` | Object | 地带 |

### `.code` 示例

```code
area1 = CreateArea(type="sphere", stage="Residence", x=0.0, y=0.0, z=-120.0, r=3.0)
area2 = CreateArea(type="sphere", stage="Residence", x=5.0, y=0.0, z=-120.0, r=3.0)
zone1 = CreateZone(area1, area2)
```

### 常见使用场景

- 复合检测区域：合并多个不相连的区域
- 巡逻路线检测：NPC 巡逻路径点的组合
- 距离计算：使用 `DistanceToNearest()` 获取玩家到最近区域的距离

### 相关节点

- `CreateArea` — 单个区域
- `CreateInteractArea` — 交互区域
- `CreateNPC` — NPC 对象
- `CreateCondition` — 条件对象（`CreateItemCondition` 的 `zone` 参数）

---

## CreateInteractArea

- **中文名**：创建交互区域
- **类别**：Objects（对象）
- **代码生成类别**：B
- **官方 API 签名**：`CreateInteractArea(stage, x, y, z, r, text[, options])`
- **返回值类型**：Object（InteractArea 对象）
- **作用**：创建一个可交互的球体区域，玩家进入区域时会显示交互提示文本。支持多选项交互，返回玩家选择的选项索引。

### 参数说明

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `stage` | 是 | Enum (StageType) | 所属场景 |
| `position` | 是 | Vector | 中心位置（x, y, z） |
| `r` | 是 | Number | 交互半径 |
| `text` | 是 | String | 交互提示文本（显示在屏幕上的操作提示） |
| `options` | 是 | List | 选项文本列表（索引从 0 开始）；为空时 `Check()` 返回 Boolean |

### 输出端口

| 端口 ID | 类型 | 标签 |
|---------|------|------|
| `out_flow` | Flow | 下一步 |
| `out_area` | Object | 区域 |

### `.code` 示例

```code
interact = CreateInteractArea(stage="Residence", x=0.0, y=0.0, z=-120.0, r=2.0, text="Open the door?", options=["Yes", "No"])
chosen = interact.Check()
if chosen == 0
    Log("Player chose Yes!")
else
    Log("Player chose No!")
```

### 常见使用场景

- 门的交互：提示玩家开门
- 物品交互：拾取或检查特定物品
- 对话选择：与 NPC 或物体的多选项交互

### 相关节点

- `CreateArea` — 检测区域
- `CreateInput` — 输入检测
- `CreateText` — 文本对象
- `If` / `While` — 条件分支

---

## CreateText

- **中文名**：创建文本
- **类别**：Objects（对象）
- **代码生成类别**：B
- **官方 API 签名**：`CreateText()`
- **返回值类型**：Object（Text 对象）
- **作用**：创建一个文本显示对象，用于在屏幕上显示对话、描述或提示文字。支持丰富的排版样式（字体描边、阴影、对齐、锚点等），并可通过 `Add()` 方法排队显示多条文本。

### 参数说明

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| — | — | — | 无参数 |

### 输出端口

| 端口 ID | 类型 | 标签 |
|---------|------|------|
| `out_flow` | Flow | 下一步 |
| `out_text` | Object | 文本 |

### `.code` 示例

```code
dialog = CreateText()
dialog.SetFace(color=Color(1.0, 1.0, 1.0, 1.0), dilate=0.0)
dialog.SetOutline(color=Color(0.0, 0.0, 0.0, 1.0), width=1.0)
dialog.SetAlignment("Center")
dialog.Add(text="Hello, world!", delay=0.0, fadein=0.5, duration=3.0, fadeout=0.5)
```

### 常见使用场景

- 角色对话：显示 NPC 对话文本
- 任务描述：展示任务目标和提示
- 剧情叙事：配合延迟/淡入淡出实现过场效果

### 相关节点

- `CreateMissionPanel` — 任务面板
- `CreateSnapshot` — 快照（截图展示）
- `ShowBlackscreen` — 黑屏过渡
- `CreateGallery` — 图库

---

## CreateMessengerChat

- **中文名**：创建聊天
- **类别**：Objects（对象）
- **代码生成类别**：B
- **官方 API 签名**：`CreateMessengerChat(Title[, icontext][, icontextcolor][, iconcolor][, iconfilename])`
- **返回值类型**：Object（MessengerChat 对象）
- **作用**：创建一个手机即时通讯聊天界面。可以添加左右排列的消息、设置按钮选择、检测按钮点击等，模拟手机的聊天应用交互。

### 参数说明

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `title` | 是 | String | 聊天标题（联系人名称） |
| `iconText` | 否 | String | 联系人图标文本（缩写），如 `"JD"` |
| `iconTextColor` | 否 | Color | 图标文本颜色，默认白色 |
| `iconColor` | 否 | Color | 图标/图标边框颜色，默认蓝色 |
| `iconFilename` | 否 | String | 自定义图标图片路径（替代文本图标） |

### 输出端口

| 端口 ID | 类型 | 标签 |
|---------|------|------|
| `out_flow` | Flow | 下一步 |
| `out_chat` | Object | 聊天 |

### `.code` 示例

```code
chat = CreateMessengerChat("Mysterious Stranger", icontext="MS", iconcolor=Color(0.5, 0.0, 0.5, 1.0))
chat.Add("Are you ready?", "Right")
chat.Add("Yes, let's go!", "Left")
chat.SetButtons("Option A", "Option B", ids=CreateList(1, 2))
clicked = chat.Clicked()
```

### 常见使用场景

- 剧情对话：模拟角色间的聊天记录
- 任务派发：通过聊天接收任务指令
- 选择分支：使用 `SetButtons` 和 `Clicked()` 实现分支剧情

### 相关节点

- `CreateText` — 文本对象
- `CreateGallery` — 图库（手机相册）
- `CreateSnapshot` — 快照相机
- `CreateInput` — 输入检测

---

## CreateAudio

- **中文名**：创建音频
- **类别**：Objects（对象）
- **代码生成类别**：B
- **官方 API 签名**：`CreateAudio(FilePath)`
- **返回值类型**：Object（Audio 对象）
- **作用**：创建一个音频源对象，用于播放背景音乐或音效。支持 3D 空间音频，可获取音频时长和播放实例 ID。

### 参数说明

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `filePath` | 是 | String | 音频文件路径（相对于项目文件夹），支持 WAV、OGG、MP3 |

### 输出端口

| 端口 ID | 类型 | 标签 |
|---------|------|------|
| `out_flow` | Flow | 下一步 |
| `out_audio` | Object | 音频 |

### `.code` 示例

```code
bgm = CreateAudio("Audio/my_background_music.ogg")
bgm.Play(volume=0.8)
```

### 常见使用场景

- 背景音乐：播放场景 BGM
- 音效触发：脚步声、开门声等
- 空间音频：在特定位置播放 3D 音效

### 相关节点

- `PlaySoundEffect` — 播放音效
- `CreateText` — 文本对象
- `ShowBlackscreen` — 黑屏过渡
- `CreateSnapshot` — 快照相机

---

## CreateGallery

- **中文名**：创建图库
- **类别**：Objects（对象）
- **代码生成类别**：B
- **官方 API 签名**：`CreateGallery([CallbackFunction][, Condition][, Area][, Zone])`
- **返回值类型**：Object（Gallery 对象）
- **作用**：创建一个图库界面，用于展示筛选后的游戏截图。支持通过回调函数自定义过滤条件，也可通过 Condition/Area/Zone 筛选。通常从手机聊天界面的按钮触发。

### 参数说明

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `callback` | 是 | String | 回调函数名，每张图片调用一次；设置 `_result = true` 则显示该图片 |
| `condition` | 否 | Object | Condition 对象，必须满足才显示 |
| `area` | 否 | Object | Area 对象，必须处于该区域才显示 |
| `zone` | 否 | Object | Zone 对象，必须处于该地带才显示 |

### 输出端口

| 端口 ID | 类型 | 标签 |
|---------|------|------|
| `out_flow` | Flow | 下一步 |
| `out_gallery` | Object | 图库 |

### `.code` 示例

```code
gallery = CreateGallery(CallbackFunction="filterImages")
gallery.Show(multiselect=false)
if gallery.Confirmed()
    sel = gallery.GetSelection()
```

### 常见使用场景

- 相册浏览：玩家查看拍摄的照片
- 证据收集：在侦探任务中筛选特定照片
- 图库选择：从多张照片中选择一张作为线索

### 相关节点

- `CreateSnapshot` — 快照相机
- `CreateMessengerChat` — 聊天（触发图库）
- `CreateCondition` — 条件对象
- `CreateArea` / `CreateZone` — 区域/地带

---

## CreateSnapshot

- **中文名**：创建快照相机
- **类别**：Objects（对象）
- **代码生成类别**：B
- **官方 API 签名**：`CreateSnapshot(position, direction[, width, height][, fov])`
- **返回值类型**：Object（Snapshot 对象）
- **作用**：创建一个快照相机，在指定位置和方向拍摄场景截图。需要等待一帧让游戏渲染后调用 `Save()`。拍摄后应销毁对象以释放性能。

### 参数说明

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `position` | 是 | Vector | 相机位置（x, y, z） |
| `direction` | 是 | Vector | 相机方向（x, y, z） |
| `width` | 是 | Number | 图像宽度（像素） |
| `height` | 是 | Number | 图像高度（像素） |
| `fov` | 是 | Number | 视野角度（Field of View） |

### 输出端口

| 端口 ID | 类型 | 标签 |
|---------|------|------|
| `out_flow` | Flow | 下一步 |
| `out_snapshot` | Object | 快照相机 |

### `.code` 示例

```code
cam = CreateSnapshot(position=CreateList(0.0, 1.5, -120.0), direction=CreateList(0.0, 0.0, 1.0), width=1920, height=1080, fov=60.0)
# Wait one frame for rendering
Wait(0.0)
ref = cam.Save(hidden=false)
```

### 常见使用场景

- 任务截图：自动拍摄场景作为任务证据
- 相册照片：生成玩家可以查看的照片
- 监视摄像头：放置固定位置的相机拍摄场景

### 相关节点

- `CreateGallery` — 图库（展示快照）
- `GetSnapshotData` — 获取快照数据
- `GetAllSnapshots` — 获取所有快照
- `DeleteSnapshot` — 删除快照
- `GetImageReference` — 获取图像引用

---

## CreateNPC

- **中文名**：创建 NPC
- **类别**：Objects（对象）
- **代码生成类别**：B
- **官方 API 签名**：`CreateNPC(AvatarType, position[, rotation][, body][, hair][, face][, size])`  
  或 `CreateNPC(ID)`（连接到已有 NPC）
- **返回值类型**：Object（NPC 对象）
- **作用**：创建一个 NPC 角色或连接到已有 NPC。支持设置外观（体型、发型、面容）、路径、行为（巡逻、跟踪、性爱等）。切换场景时 NPC 会被移除但对象保留，可重新生成。

### 参数说明

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `avatarType` | 是 | String | 外观类型：`"NewFemale"`、`"NewMale"`、`"PreviousFemale"`、`"PreviousMale"`、`"NewOba"`、`"NewOji"` |
| `position` | 是 | Vector | 生成位置（x, y, z） |
| `rotation` | 否 | Quaternion | 初始旋转 |
| `body` | 否 | Number | 身体/服装索引（因 AvatarType 而异） |
| `hair` | 否 | Number | 发型索引 |
| `face` | 否 | Number | 面容索引 |
| `size` | 否 | Number | 大小比例（1.0 为正常） |
| `id` | 否 | Number | 游戏内 NPC ID，用于连接到已有 NPC |

### 输出端口

| 端口 ID | 类型 | 标签 |
|---------|------|------|
| `out_flow` | Flow | 下一步 |
| `out_npc` | Object | NPC |

### `.code` 示例

```code
npc1 = CreateNPC(avatarType="NewFemale", position=CreateList(2.0, 0.0, -120.0), body=3, hair=5, face=2)
npc1.AddWaypoint(CreateList(5.0, 0.0, -120.0))
npc1.AddWaypoint(CreateList(-5.0, 0.0, -120.0), last=true)
```

### 常见使用场景

- 路人 NPC：创建场景中的巡逻路人
- 任务 NPC：玩家可以交互的任务角色
- 监视目标：需要跟踪或隐藏的目标 NPC

### 相关节点

- `CreateArea` / `CreateZone` — 区域/地带
- `CreateInteractArea` — 交互区域
- `SetSexPosition` — 设置性爱体位
- `GetAllWaypoints` — 获取路径点

---

## CreateInput

- **中文名**：创建输入
- **类别**：Objects（对象）
- **代码生成类别**：B
- **官方 API 签名**：`CreateInput(Button[, modifier][, interaction])`  
  或 `CreateInput(Button[, modifier1, modifier2][, interaction])`
- **返回值类型**：Object（Input 对象）
- **作用**：创建一个输入检测对象，用于检测玩家按键操作。支持组合键（如 Shift+F）和交互模式（如长按）。通常在 Listener 中每帧检测。

### 参数说明

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `button` | 是 | String | Unity InputSystem 格式的按键路径，如 `"<Keyboard>/space"` |
| `modifier` | 否 | String | 修饰键，如 `"<Keyboard>/shift"` |
| `interaction` | 否 | String | 交互模式，如 `"hold(duration=0.5)"` |

### 输出端口

| 端口 ID | 类型 | 标签 |
|---------|------|------|
| `out_flow` | Flow | 下一步 |
| `out_input` | Object | 输入 |

### `.code` 示例

```code
input1 = CreateInput("<Keyboard>/space", interaction="hold(duration=0.5)")
input2 = CreateInput("<Keyboard>/F", modifier="<Keyboard>/shift")
listener = CreateListener("waitforinput")

waitforinput:
    if input1.WasPerformed()
        Log("Long press [Space]")
    if input2.WasPerformed()
        Log("Pressed [Shift]+[F]")
```

### 常见使用场景

- 自定义操作：为任务绑定特殊按键
- 长按交互：按住空格键持续执行某动作
- 组合键菜单：Shift+F 打开自定义菜单

### 相关节点

- `CreateListener` — 创建监听器（配合每帧检测）
- `CreateInteractArea` — 交互区域
- `CreateMessengerChat` — 聊天界面
- `If` — 条件分支

---

## ShowBlackscreen

- **中文名**：黑屏过渡
- **类别**：Game API（图形与杂项）
- **代码生成类别**：B
- **官方 API 签名**：`ShowBlackscreen([color][, delay][, fadein][, duration][, fadeout])`
- **返回值类型**：null
- **作用**：显示全屏颜色覆盖层，支持淡入、持续、淡出动画。常用于场景切换、过场动画的开始/结束等视觉过渡效果。

### 参数说明

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `color` | 是 | Color | 覆盖层颜色 RGBA，黑色为 `[0, 0, 0, 1]` |
| `delay` | 否 | Number | 显示前延迟（秒） |
| `fadein` | 否 | Number | 淡入时长（秒） |
| `duration` | 否 | Number | 保持时长（秒） |
| `fadeout` | 否 | Number | 淡出时长（秒） |

### 输出端口

| 端口 ID | 类型 | 标签 |
|---------|------|------|
| `out_flow` | Flow | 下一步 |

### `.code` 示例

```code
# Fade to black over 1 second, wait 2 seconds, fade back in over 1 second
ShowBlackscreen(color=Color(0.0, 0.0, 0.0, 1.0), fadein=1.0, duration=2.0, fadeout=1.0)
```

### 常见使用场景

- 场景切换：切换场景前的黑屏过渡
- 过场动画：剧情片段开始/结束的淡入淡出
- 视觉强调：短暂的黑屏强调某个事件

### 相关节点

- `SetStage` — 切换场景
- `CreateText` — 文本对象（配合过场文字）
- `CreateAudio` — 音频对象（配合音效）
- `CreateSnapshot` — 快照相机

---

## GetSnapshotData

- **中文名**：获取快照数据
- **类别**：Game API（图形与杂项）
- **代码生成类别**：B
- **官方 API 签名**：`GetSnapshotData(ImageReference)`
- **返回值类型**：List
- **作用**：获取指定快照的元数据，包含拍摄时场景的状态信息（位置、灯光、NPC 状态等）。数据结构类似于 `_state` 全局变量，但不包含任务状态和 cosplay 拥有信息。

### 参数说明

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `imageRef` | 是 | String | 快照的图像引用（来自 `Snapshot.Save()` 返回值） |

### 输出端口

| 端口 ID | 类型 | 标签 |
|---------|------|------|
| `out_flow` | Flow | 下一步 |
| `out_data` | List | 数据 |

### `.code` 示例

```code
data = GetSnapshotData(ref)
stageName = data.Stage
```

### 常见使用场景

- 读取照片信息：获取照片拍摄的场景和时间
- 任务验证：检查照片中是否包含特定内容
- 图库展示：在 UI 中显示照片的元数据

### 相关节点

- `CreateSnapshot` — 快照相机
- `GetAllSnapshots` — 获取所有快照
- `DeleteSnapshot` — 删除快照
- `CreateGallery` — 图库

---

## GetAllSnapshots

- **中文名**：获取所有快照
- **类别**：Game API（图形与杂项）
- **代码生成类别**：B
- **官方 API 签名**：`GetAllSnapshots([deleted][, hidden])`
- **返回值类型**：List
- **作用**：获取所有快照的图像引用列表。默认排除已删除和隐藏的快照。

### 参数说明

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `deleted` | 否 | Boolean | 是否包含已标记删除的快照 |
| `hidden` | 否 | Boolean | 是否包含隐藏的快照 |

### 输出端口

| 端口 ID | 类型 | 标签 |
|---------|------|------|
| `out_flow` | Flow | 下一步 |
| `out_list` | List | 快照列表 |

### `.code` 示例

```code
allPhotos = GetAllSnapshots()
for ref in allPhotos
    data = GetSnapshotData(ref)
```

### 常见使用场景

- 遍历所有快照：批量获取照片数据
- 删除清理：获取所有快照后逐一检查删除
- 相册管理：列出所有照片供玩家选择

### 相关节点

- `GetSnapshotData` — 获取快照数据
- `DeleteSnapshot` — 删除快照
- `CreateSnapshot` — 快照相机
- `CreateGallery` — 图库

---

## DeleteSnapshot

- **中文名**：删除快照
- **类别**：Game API（图形与杂项）
- **代码生成类别**：B
- **官方 API 签名**：`DeleteSnapshot(ImageReference)`
- **返回值类型**：null
- **作用**：将指定快照标记为已删除。被标记的快照会在下次游戏启动时被删除（前提是不被任何存档引用）。

### 参数说明

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `imageRef` | 是 | String | 要删除的快照的图像引用 |

### 输出端口

| 端口 ID | 类型 | 标签 |
|---------|------|------|
| `out_flow` | Flow | 下一步 |

### `.code` 示例

```code
DeleteSnapshot(imageRef)
```

### 常见使用场景

- 清理旧照片：删除不需要的快照
- 任务完成清理：完成任务后删除证据照片
- 空间管理：删除无用的快照释放存储

### 相关节点

- `CreateSnapshot` — 快照相机
- `GetAllSnapshots` — 获取所有快照
- `GetSnapshotData` — 获取快照数据
- `GetImageReference` — 获取图像引用

---

## GetImageReference

- **中文名**：获取图像引用
- **类别**：Game API（图形与杂项）
- **代码生成类别**：B
- **官方 API 签名**：`GetImageReference(FilePath)`
- **返回值类型**：String（图像引用，可传递给其他快照/图库函数）
- **作用**：根据文件路径获取图像引用。路径相对于项目文件夹，可用于在 Messenger 或其他功能中引用外部图片。

### 参数说明

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `filePath` | 是 | String | 图片文件路径（相对项目文件夹） |

### 输出端口

| 端口 ID | 类型 | 标签 |
|---------|------|------|
| `out_flow` | Flow | 下一步 |
| `out_ref` | String | 引用 |

### `.code` 示例

```code
customImageRef = GetImageReference("Images/my_custom_photo.png")
messenger.SetImage(customImageRef)
```

### 常见使用场景

- 引用外部图片：在聊天中使用自定义图片
- 任务物品图标：从项目文件夹加载图标
- 相册补充：将外部图片加入游戏相册

### 相关节点

- `CreateSnapshot` — 快照相机
- `GetAllSnapshots` — 获取所有快照
- `DeleteSnapshot` — 删除快照
- `CreateMessengerChat` — 聊天（发送图片）

---

## SetGraphicsOption

- **中文名**：设置图形选项
- **类别**：Game API（图形与杂项）
- **代码生成类别**：B
- **官方 API 签名**：`SetGraphicsOption(Option, Value)`
- **返回值类型**：null
- **作用**：动态调整游戏的图形设置，如动态模糊、亮度、对比度、色温、泛光强度等。值会被取整为整数（除 MotionBlur 为布尔值）。

### 参数说明

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `option` | 是 | Enum | 图形选项名：`MotionBlur`、`Brightness`、`Contrast`、`ColorTemperature`、`BloomStrength`、`BloomBiasDark`、`BloomBiasLight`、`CameraAngle1st`、`CameraAngle3rd` |
| `value` | 是 | List | 值：Number 类型（`MotionBlur` 除外为 Boolean） |

### 输出端口

| 端口 ID | 类型 | 标签 |
|---------|------|------|
| `out_flow` | Flow | 下一步 |

### `.code` 示例

```code
SetGraphicsOption(option="Brightness", value=120)
SetGraphicsOption(option="MotionBlur", value=true)
SetGraphicsOption(option="BloomStrength", value=5)
```

### 常见使用场景

- 动态天气：根据场景调整亮度和对比度
- 电影效果：调整泛光和色温营造氛围
- 视角切换：切换第一人称/第三人称视角

### 相关节点

- `GetGraphicsOption` — 获取图形选项
- `SetCamera` — 设置相机
- `ShowBlackscreen` — 黑屏过渡
- `SetStage` — 切换场景

---

## GetGraphicsOption

- **中文名**：获取图形选项
- **类别**：Game API（图形与杂项）
- **代码生成类别**：B
- **官方 API 签名**：`GetGraphicsOption(Option)`
- **返回值类型**：Number 或 Boolean
- **作用**：获取指定图形选项的当前值。`MotionBlur` 返回 Boolean，其余返回 Number。

### 参数说明

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `option` | 是 | Enum | 图形选项名（同 `SetGraphicsOption`） |

### 输出端口

| 端口 ID | 类型 | 标签 |
|---------|------|------|
| `out_flow` | Flow | 下一步 |
| `out_value` | List | 值 |

### `.code` 示例

```code
currentBrightness = GetGraphicsOption(option="Brightness")
if currentBrightness < 100
    SetGraphicsOption(option="Brightness", value=100)
```

### 常见使用场景

- 检查当前设置：根据玩家的图形设置调整任务体验
- 保存/恢复：在修改前保存设置，任务完成后恢复
- 自适应调整：根据时间或场景自动调整亮度

### 相关节点

- `SetGraphicsOption` — 设置图形选项
- `SetCamera` — 设置相机
- `GetSettings` — 读取 meta 设置
- `If` — 条件判断

> 本文档涵盖数学与向量相关的所有节点。
> 分类依据：`docs/node_types.md` — 所有数学/向量节点均为 C 类（纯 Data 节点），可在 Flow 链中用于数据计算。
> 代码生成：B 类流程节点通过 `generate_node_call` 自动生成 `var_{id}_{port} = FuncName(params)`；C 类通过 `evaluate_data_output` 直接返回表达式。

---

## Random

- **中文名**：随机数
- **官方 API 签名**：`Random([[minInclusive], maxExclusive])` → `Number`
- **返回值类型**：`Number`
- **作用**：生成一个随机浮点数。无参数时返回 `[0, 1)` 区间内的值。可指定最小值和最大值范围。
- **参数说明**：
  - `min`（必填，`Number`）：随机数范围的最小值（包含）
  - `max`（必填，`Number`）：随机数范围的最大值（不包含）
- **`.code` 示例**：
  ```code
  var_n1_out_value = Random(min=0.0, max=100.0)
  ```
- **常见使用场景**：随机掉落物品数量、随机 NPC 行为选择、随机位置偏移
- **相关节点**：`RandomInt`, `GetRandomPosition`, `Floor`, `Abs`

---

## RandomInt

- **中文名**：随机整数
- **官方 API 签名**：`RandomInt([[minInclusive], maxExclusive])` → `integer Number`
- **返回值类型**：`Number`（整数）
- **作用**：生成一个随机整数。无参数时返回 0 或 1。适合需要离散随机值的场景。
- **参数说明**：
  - `min`（必填，`Number`）：随机数范围的最小值（包含）
  - `max`（必填，`Number`）：随机数范围的最大值（不包含）
- **`.code` 示例**：
  ```code
  var_n1_out_value = RandomInt(min=1, max=5)
  ```
- **常见使用场景**：从列表中选择一个随机索引、随机抽奖、随机生成物品数量
- **相关节点**：`Random`, `Floor`, `Ceil`, `Round`

---

## Sin

- **中文名**：正弦
- **官方 API 签名**：`Sin(Angle)` → `Number`
- **返回值类型**：`Number`
- **作用**：计算角度（度）的正弦值。使用角度制而非弧度制。
- **参数说明**：
  - `angle`（必填，`Number`）：角度值（度）
- **`.code` 示例**：
  ```code
  var_n1_out_value = Sin(angle=90.0)
  ```
- **常见使用场景**：波形运动、周期性动画、角度计算
- **相关节点**：`Cos`, `Tan`, `Asin`, `Acos`, `Vector3Rotate`

---

## Cos

- **中文名**：余弦
- **官方 API 签名**：`Cos(Angle)` → `Number`
- **返回值类型**：`Number`
- **作用**：计算角度（度）的余弦值。使用角度制而非弧度制。
- **参数说明**：
  - `angle`（必填，`Number`）：角度值（度）
- **`.code` 示例**：
  ```code
  var_n1_out_value = Cos(angle=45.0)
  ```
- **常见使用场景**：圆周运动定位、方向向量分解、光照计算
- **相关节点**：`Sin`, `Tan`, `Acos`, `Vector3Rotate`

---

## Tan

- **中文名**：正切
- **官方 API 签名**：`Tan(Angle)` → `Number`
- **返回值类型**：`Number`
- **作用**：计算角度（度）的正切值。使用角度制。
- **参数说明**：
  - `angle`（必填，`Number`）：角度值（度）
- **`.code` 示例**：
  ```code
  var_n1_out_value = Tan(angle=30.0)
  ```
- **常见使用场景**：斜率计算、角度到直线变换、三角测量
- **相关节点**：`Sin`, `Cos`, `Atan`, `Atan2`

---

## Asin

- **中文名**：反正弦
- **官方 API 签名**：`Asin(Value)` → `Number`
- **返回值类型**：`Number`
- **作用**：计算数值的反正弦值，返回角度（度）。输入值应在 `[-1, 1]` 范围内。
- **参数说明**：
  - `value`（必填，`Number`）：正弦值，范围 `[-1, 1]`
- **`.code` 示例**：
  ```code
  var_n1_out_value = Asin(value=0.5)
  ```
- **常见使用场景**：从比例反推角度、向量夹角计算、动画插值
- **相关节点**：`Sin`, `Acos`, `Atan`, `Vector3Dot`

---

## Acos

- **中文名**：反余弦
- **官方 API 签名**：`Acos(Value)` → `Number`
- **返回值类型**：`Number`
- **作用**：计算数值的反余弦值，返回角度（度）。输入值应在 `[-1, 1]` 范围内。
- **参数说明**：
  - `value`（必填，`Number`）：余弦值，范围 `[-1, 1]`
- **`.code` 示例**：
  ```code
  var_n1_out_value = Acos(value=0.0)
  ```
- **常见使用场景**：通过点积计算两个向量间的夹角、三角形求解
- **相关节点**：`Cos`, `Asin`, `Atan`, `Vector3Dot`

---

## Atan

- **中文名**：反正切
- **官方 API 签名**：`Atan(Value)` → `Number`
- **返回值类型**：`Number`
- **作用**：计算数值的反正切值，返回角度（度）。适合从比率反推角度。
- **参数说明**：
  - `value`（必填，`Number`）：正切值
- **`.code` 示例**：
  ```code
  var_n1_out_value = Atan(value=1.0)
  ```
- **常见使用场景**：从斜率计算倾角、方向角计算、向量归一化
- **相关节点**：`Tan`, `Asin`, `Acos`, `Vector3Rotate`

---

## Floor

- **中文名**：向下取整
- **官方 API 签名**：`Floor(Value)` → `Number`
- **返回值类型**：`Number`
- **作用**：向下取整，返回小于或等于给定值的最大整数。
- **参数说明**：
  - `value`（必填，`Number`）：要取整的数值
- **`.code` 示例**：
  ```code
  var_n1_out_value = Floor(value=3.7)
  ; 结果: 3.0
  ```
- **常见使用场景**：坐标格点化、整数索引计算、价格打折取整
- **相关节点**：`Ceil`, `Round`, `Trunc`, `RandomInt`

---

## Ceil

- **中文名**：向上取整
- **官方 API 签名**：`Ceil(Value)` → `Number`
- **返回值类型**：`Number`
- **作用**：向上取整，返回大于或等于给定值的最小整数。
- **参数说明**：
  - `value`（必填，`Number`）：要取整的数值
- **`.code` 示例**：
  ```code
  var_n1_out_value = Ceil(value=3.2)
  ; 结果: 4.0
  ```
- **常见使用场景**：分页计算、资源分配上取整、计时器秒数计算
- **相关节点**：`Floor`, `Round`, `Trunc`, `RandomInt`

---

## Round

- **中文名**：四舍五入
- **官方 API 签名**：`Round(Value)` → `Number`
- **返回值类型**：`Number`
- **作用**：对数值进行四舍五入到最接近的整数。
- **参数说明**：
  - `value`（必填，`Number`）：要舍入的数值
- **`.code` 示例**：
  ```code
  var_n1_out_value = Round(value=3.5)
  ; 结果: 4.0
  ```
- **常见使用场景**：UI 显示美化、统计结果舍入、坐标精度控制
- **相关节点**：`Floor`, `Ceil`, `Trunc`, `Sign`

---

## Trunc

- **中文名**：截断
- **官方 API 签名**：`Trunc(Value)` → `Number`
- **返回值类型**：`Number`
- **作用**：截断小数部分，仅保留整数部分。与 `Floor` 不同，对负数不会向下取整（例如 `Trunc(-3.7)` → `-3`）。
- **参数说明**：
  - `value`（必填，`Number`）：要截断的数值
- **`.code` 示例**：
  ```code
  var_n1_out_value = Trunc(value=-3.7)
  ; 结果: -3.0
  ```
- **常见使用场景**：类型转换截断、坐标离散化、去除小数精度
- **相关节点**：`Floor`, `Ceil`, `Round`, `Sign`

---

## Sign

- **中文名**：符号
- **官方 API 签名**：`Sign(Value)` → `Number`
- **返回值类型**：`Number`
- **作用**：返回数值的符号：正数返回 `1`，负数返回 `-1`，零返回 `0`。
- **参数说明**：
  - `value`（必填，`Number`）：要判断符号的数值
- **`.code` 示例**：
  ```code
  var_n1_out_value = Sign(value=-42.0)
  ; 结果: -1.0
  ```
- **常见使用场景**：方向判断、移动方向控制、符号归一化
- **相关节点**：`Abs`, `Floor`, `Trunc`, `CompareNumbers`

---

## Abs

- **中文名**：绝对值
- **官方 API 签名**：`Abs(Value)` → `Number`
- **返回值类型**：`Number`
- **作用**：返回数值的绝对值，即非负值。
- **参数说明**：
  - `value`（必填，`Number`）：要取绝对值的数值
- **`.code` 示例**：
  ```code
  var_n1_out_value = Abs(value=-10.0)
  ; 结果: 10.0
  ```
- **常见使用场景**：距离计算、误差绝对值、安全距离判断
- **相关节点**：`Sign`, `Floor`, `Vector3Length`, `Vector3Distance`

---

## LogN

- **中文名**：自然对数
- **官方 API 签名**：`LogN(Value)` → `Number`
- **返回值类型**：`Number`
- **作用**：计算数值的自然对数（以 e 为底）。输入值必须大于 0。
- **参数说明**：
  - `value`（必填，`Number`）：要计算对数的值（必须 > 0）
- **`.code` 示例**：
  ```code
  var_n1_out_value = LogN(value=2.71828)
  ```
- **常见使用场景**：指数增长/衰减模拟、数据缩放、音高频率转换
- **相关节点**：`Log2`, `Log10`, `Abs`, `Exp`

---

## Log2

- **中文名**：以 2 为底的对数
- **官方 API 签名**：`Log2(Value)` → `Number`
- **返回值类型**：`Number`
- **作用**：计算数值的以 2 为底的对数。输入值必须大于 0。
- **参数说明**：
  - `value`（必填，`Number`）：要计算对数的值（必须 > 0）
- **`.code` 示例**：
  ```code
  var_n1_out_value = Log2(value=8.0)
  ; 结果: 3.0
  ```
- **常见使用场景**：二分查找深度计算、信息熵计算、节拍映射
- **相关节点**：`LogN`, `Log10`, `Abs`, `Min`

---

## Log10

- **中文名**：以 10 为底的对数
- **官方 API 签名**：`Log10(Value)` → `Number`
- **返回值类型**：`Number`
- **作用**：计算数值的常用对数（以 10 为底）。输入值必须大于 0。
- **参数说明**：
  - `value`（必填，`Number`）：要计算对数的值（必须 > 0）
- **`.code` 示例**：
  ```code
  var_n1_out_value = Log10(value=100.0)
  ; 结果: 2.0
  ```
- **常见使用场景**：数量级估算、分贝计算、数据标准化
- **相关节点**：`LogN`, `Log2`, `Abs`, `Max`

---

## Min

- **中文名**：最小值
- **官方 API 签名**：`Min(Value1[, Value2]...)` → `Number`
- **返回值类型**：`Number`
- **作用**：从一组数值中找出最小值。支持两个或多个值。
- **参数说明**：
  - `values`（必填，`List`）：包含多个数值的列表
- **`.code` 示例**：
  ```code
  var_n1_out_value = Min(values=[10, 20, 5, 30])
  ; 结果: 5.0
  ```
- **常见使用场景**：钳制数值上限、取最短距离、资源最少限制
- **相关节点**：`Max`, `Abs`, `CompareNumbers`, `Vector3Length`

---

## Max

- **中文名**：最大值
- **官方 API 签名**：`Max(Value1[, Value2]...)` → `Number`
- **返回值类型**：`Number`
- **作用**：从一组数值中找出最大值。支持两个或多个值。
- **参数说明**：
  - `values`（必填，`List`）：包含多个数值的列表
- **`.code` 示例**：
  ```code
  var_n1_out_value = Max(values=[10, 20, 5, 30])
  ; 结果: 30.0
  ```
- **常见使用场景**：钳制数值下限、取最远距离、最高分判定
- **相关节点**：`Min`, `Abs`, `CompareNumbers`, `Vector3Length`

---

## Vector

- **中文名**：向量
- **官方 API 签名**：`Vector(X, Y[, Z])` → `List`
- **返回值类型**：`List`（格式 `[x, y, z]`）
- **作用**：创建一个 3D 向量（列表格式）。Z 可选，省略时默认为 0。
- **参数说明**：
  - `x`（必填，`Number`）：X 分量
  - `y`（必填，`Number`）：Y 分量
  - `z`（必填，`Number`）：Z 分量
- **`.code` 示例**：
  ```code
  var_n1_out_vector = Vector(x=1.0, y=2.0, z=3.0)
  ```
- **常见使用场景**：构造坐标位置、方向向量、速度向量
- **相关节点**：`Vector3Add`, `Vector3Scale`, `Vector3Length`, `MakeVector`, `BreakVector`

---

## Quaternion

- **中文名**：四元数
- **官方 API 签名**：`Quaternion(RX, RY, RZ, RW)` → `List`
- **返回值类型**：`List`（格式 `[rx, ry, rz, rw]`）
- **作用**：创建一个四元数，用于表示 3D 旋转。
- **参数说明**：
  - `rx`（必填，`Number`）：四元数 X 分量
  - `ry`（必填，`Number`）：四元数 Y 分量
  - `rz`（必填，`Number`）：四元数 Z 分量
  - `rw`（必填，`Number`）：四元数 W 分量
- **`.code` 示例**：
  ```code
  var_n1_out_quaternion = Quaternion(rx=0.0, ry=0.707, rz=0.0, rw=0.707)
  ```
- **常见使用场景**：3D 旋转表示、角色朝向控制、`Vector3Rotate` 的旋转参数
- **相关节点**：`Vector3Rotate`, `Vector`, `Vector3Cross`, `SetCamera`

---

## Vector3Length

- **中文名**：向量长度
- **官方 API 签名**：`Vector3Length(Vector)` → `Number`
- **返回值类型**：`Number`
- **作用**：计算 3D 向量的欧几里得长度（模长），即 `sqrt(x² + y² + z²)`。
- **参数说明**：
  - `v`（必填，`List`）：3D 向量 `[x, y, z]`
- **`.code` 示例**：
  ```code
  var_n1_out_length = Vector3Length(v=[3.0, 4.0, 0.0])
  ; 结果: 5.0
  ```
- **常见使用场景**：速度大小计算、距离阈值判断、向量归一化分母
- **相关节点**：`Vector3SqrLength`, `Vector3Distance`, `Vector3Scale`, `Abs`

---

## Vector3SqrLength

- **中文名**：向量长度平方
- **官方 API 签名**：`Vector3SqrLength(Vector)` → `Number`
- **返回值类型**：`Number`
- **作用**：计算 3D 向量长度的平方（`x² + y² + z²`）。免去开平方开销，适合距离比较。
- **参数说明**：
  - `v`（必填，`List`）：3D 向量 `[x, y, z]`
- **`.code` 示例**：
  ```code
  var_n1_out_length = Vector3SqrLength(v=[3.0, 4.0, 0.0])
  ; 结果: 25.0
  ```
- **常见使用场景**：优化距离比较（避免 `sqrt`）、碰撞检测范围判断、面积计算
- **相关节点**：`Vector3Length`, `Vector3Distance`, `Vector3Dot`, `Vector3Scale`

---

## Vector3Add

- **中文名**：向量加法
- **官方 API 签名**：`Vector3Add(Vector1, Vector2)` → `List`
- **返回值类型**：`List`（`[x, y, z]`）
- **作用**：两个 3D 向量相加，返回分量和。
- **参数说明**：
  - `v1`（必填，`List`）：第一个向量
  - `v2`（必填，`List`）：第二个向量
- **`.code` 示例**：
  ```code
  var_n1_out_vector = Vector3Add(v1=[1.0, 0.0, 0.0], v2=[0.0, 2.0, 0.0])
  ; 结果: [1.0, 2.0, 0.0]
  ```
- **常见使用场景**：位置偏移、力合成、路径插值
- **相关节点**：`Vector3Sub`, `Vector3Scale`, `Vector`, `SetPlayerPosition`

---

## Vector3Sub

- **中文名**：向量减法
- **官方 API 签名**：`Vector3Sub(Vector1, Vector2)` → `List`
- **返回值类型**：`List`（`[x, y, z]`）
- **作用**：两个 3D 向量相减（`v1 - v2`），返回分量差。
- **参数说明**：
  - `v1`（必填，`List`）：被减向量
  - `v2`（必填，`List`）：减向量
- **`.code` 示例**：
  ```code
  var_n1_out_vector = Vector3Sub(v1=[5.0, 5.0, 0.0], v2=[1.0, 1.0, 0.0])
  ; 结果: [4.0, 4.0, 0.0]
  ```
- **常见使用场景**：计算两点相对位移、方向向量、追赶路径差
- **相关节点**：`Vector3Add`, `Vector3Scale`, `Vector3Distance`, `Vector3Length`

---

## Vector3Scale

- **中文名**：向量缩放
- **官方 API 签名**：`Vector3Scale(Vector, Scalar)` 或 `Vector3Scale(Scalar, Vector)` → `List`
- **返回值类型**：`List`（`[x, y, z]`）
- **作用**：将向量乘以标量，参数顺序可互换。
- **参数说明**：
  - `v`（必填，`List`）：要缩放的向量
  - `scalar`（必填，`Number`）：缩放因子
- **`.code` 示例**：
  ```code
  var_n1_out_vector = Vector3Scale(v=[1.0, 2.0, 3.0], scalar=2.0)
  ; 结果: [2.0, 4.0, 6.0]
  ```
- **常见使用场景**：速度乘以时间得到位移、归一化向量乘以长度、方向加权
- **相关节点**：`Vector3Add`, `Vector3Length`, `Vector3Dot`, `Vector`

---

## Vector3Dot

- **中文名**：向量点积
- **官方 API 签名**：`Vector3Dot(Vector1, Vector2)` → `Number`
- **返回值类型**：`Number`
- **作用**：计算两个 3D 向量的点积（标量积）。可用于计算夹角余弦或投影长度。
- **参数说明**：
  - `v1`（必填，`List`）：第一个向量
  - `v2`（必填，`List`）：第二个向量
- **`.code` 示例**：
  ```code
  var_n1_out_value = Vector3Dot(v1=[1.0, 0.0, 0.0], v2=[0.0, 1.0, 0.0])
  ; 结果: 0.0（正交）
  ```
- **常见使用场景**：判断向量方向（正/负/垂直）、光照漫反射计算、`Acos` 配合求夹角
- **相关节点**：`Vector3Cross`, `Vector3Length`, `Acos`, `Vector3Scale`

---

## Vector3Cross

- **中文名**：向量叉积
- **官方 API 签名**：`Vector3Cross(Vector1, Vector2)` → `List`
- **返回值类型**：`List`（`[x, y, z]`）
- **作用**：计算两个 3D 向量的叉积，结果向量垂直于原两个向量。常用于计算法线。
- **参数说明**：
  - `v1`（必填，`List`）：第一个向量
  - `v2`（必填，`List`）：第二个向量
- **`.code` 示例**：
  ```code
  var_n1_out_vector = Vector3Cross(v1=[1.0, 0.0, 0.0], v2=[0.0, 1.0, 0.0])
  ; 结果: [0.0, 0.0, 1.0]
  ```
- **常见使用场景**：计算平面法线、获取垂直向量、三维旋转轴生成
- **相关节点**：`Vector3Dot`, `Vector3Rotate`, `Quaternion`, `Vector3Length`

---

## Vector3Rotate

- **中文名**：旋转向量
- **官方 API 签名**：`Vector3Rotate(Quaternion, Vector)` → `List`
- **返回值类型**：`List`（`[x, y, z]`）
- **作用**：使用四元数旋转 3D 向量，返回旋转后的新向量。
- **参数说明**：
  - `q`（必填，`List`）：四元数 `[rx, ry, rz, rw]`
  - `v`（必填，`List`）：要旋转的向量
- **`.code` 示例**：
  ```code
  var_n1_out_vector = Vector3Rotate(q=[0.0, 0.707, 0.0, 0.707], v=[1.0, 0.0, 0.0])
  ```
- **常见使用场景**：角色朝向旋转、相机方向计算、物体绕轴转动
- **相关节点**：`Quaternion`, `Vector3Cross`, `Vector3Dot`, `Vector3Length`

---

## Vector3Distance

- **中文名**：向量距离
- **官方 API 签名**：`Vector3Distance(Vector1, Vector2)` → `Number`
- **返回值类型**：`Number`
- **作用**：计算两个位置向量之间的欧几里得距离。
- **参数说明**：
  - `v1`（必填，`List`）：第一个位置向量
  - `v2`（必填，`List`）：第二个位置向量
- **`.code` 示例**：
  ```code
  var_n1_out_distance = Vector3Distance(v1=[0.0, 0.0, 0.0], v2=[3.0, 4.0, 0.0])
  ; 结果: 5.0
  ```
- **常见使用场景**：玩家与 NPC 距离检测、物品拾取范围、触发区域判断
- **相关节点**：`Vector3Length`, `Vector3SqrLength`, `Vector3Sub`, `Abs`

---

## GetPosition

- **中文名**：坐标预设
- **官方 API 签名**：`GetPosition()`（NPC 方法）→ `List` or `null`
- **返回值类型**：`List`（`[x, y, z]`）+ `String`（场景名）
- **作用**：从预设坐标库中选取一个位置。输出位置向量和所属场景名。带有坐标 ID 标识，可从编辑器的坐标选择器选取预设。
- **参数说明**：
  - `coord_id`（必填，`String`）：坐标预设的唯一标识符
  - `stage`（可选，`String`）：场景名称，从预设场景类型中选择
  - `x`（必填，`Number`）：位置的 X 坐标
  - `y`（必填，`Number`）：位置的 Y 坐标
  - `z`（必填，`Number`）：位置的 Z 坐标
- **`.code` 示例**：
  ```code
  ; out_position → [x, y, z]
  ; out_stage → "Residence"
  ```
- **常见使用场景**：存储和复用常用的地图坐标点、任务事件触发位置的统一管理
- **相关节点**：`Vector`, `SetPlayerPosition`, `SetStage`, `Vector3Distance`

---

## MakeVector

- **中文名**：构造向量
- **官方 API 签名**：`Vector(X, Y[, Z])` → `List`
- **返回值类型**：`List`（`[x, y, z]`）
- **作用**：将三个独立的数值分量打包为一个向量列表。C 类节点，无 Flow 端口，直接通过数据边输出。
- **参数说明**：
  - `x`（必填，`Number`）：X 分量
  - `y`（必填，`Number`）：Y 分量
  - `z`（必填，`Number`）：Z 分量
- **`.code` 示例**：
  ```code
  var_n1_out_vec = [x_value, y_value, z_value]
  ```
- **常见使用场景**：将分散的坐标分量组装为向量传递给其它节点、动态构造位置
- **相关节点**：`BreakVector`, `Vector`, `Vector3Add`, `SetPlayerPosition`

---

## BreakVector

- **中文名**：拆分向量
- **官方 API 签名**：无独立 API，使用列表索引 `vec[0]` / `vec[1]` / `vec[2]`
- **返回值类型**：`Number`（三个独立输出：X、Y、Z）
- **作用**：将一个 `[x, y, z]` 向量拆分为三个独立的数值输出。C 类节点，无 Flow 端口，直接通过数据边输出各分量。
- **参数说明**：
  - `in_vec`（必填，`List`）：要拆分的 3D 向量
- **`.code` 示例**：
  ```code
  ; x → {in_vec}[0]
  ; y → {in_vec}[1]
  ; z → {in_vec}[2]
  ```
- **常见使用场景**：从位置向量中提取单个坐标值、分别操作向量的各分量
- **相关节点**：`MakeVector`, `Vector`, `Vector3Length`, `Vector3Scale`

---

## NumberConstant

- **中文名**：数值常量
- **官方 API 签名**：无独立 API，直接使用数值字面量
- **返回值类型**：`Number`
- **作用**：输出一个固定的数值。相当于 `.code` 中的数值字面量，供其它节点的 Data 端口使用。
- **参数说明**：
  - `value`（必填，`Number`）：常量值（如 0, 1, 90）
- **`.code` 示例**：
  ```code
  ; 直接生成数值字面量，如 90.0
  ```
- **常见使用场景**：提供固定数字给其它节点作为参数、测试时注入特定值
- **相关节点**：`StringConstant`, `Boolean`, `Random`, `RandomInt`

## Length

- **中文名**：字符串长度
- **API 签名**：`Length(String)`
- **返回值类型**：`Number`
- **作用**：返回字符串的字符数。常用于验证用户输入长度或动态计算字符串尺寸以配合 `SubString` 截取。
- **参数说明**：

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `s` | 是 | String | 要计算长度的字符串 |

- **`.code` 示例**：

```code
s = "Hello World"
len = Length(s)
Log(len) // 输出 11
```

- **常见使用场景**：验证输入长度、判断字符串是否为空、与 `SubString` 配合实现动态截取。
- **相关节点**：`SubString`、`Find`、`Format`、`Lower`、`Upper`

---

## Lower

- **中文名**：转小写
- **API 签名**：`Lower(String)`
- **返回值类型**：`String`
- **作用**：将字符串中所有英文字母转换为小写。非字母字符保持不变，用于不区分大小写的比较或规范化存储。
- **参数说明**：

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `s` | 是 | String | 要转换的字符串 |

- **`.code` 示例**：

```code
s = "Hello World"
low = Lower(s)
Log(low) // 输出 "hello world"
```

- **常见使用场景**：实现大小写不敏感的用户输入匹配、规范化字符串存储、与 `Find` 组合实现不区分大小写的搜索。
- **相关节点**：`Upper`、`Find`、`Length`、`Format`

---

## Upper

- **中文名**：转大写
- **API 签名**：`Upper(String)`
- **返回值类型**：`String`
- **作用**：将字符串中所有英文字母转换为大写。用于标题格式化或统一字符串比较格式。
- **参数说明**：

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `s` | 是 | String | 要转换的字符串 |

- **`.code` 示例**：

```code
s = "Hello World"
up = Upper(s)
Log(up) // 输出 "HELLO WORLD"
```

- **常见使用场景**：统一格式化显示、生成大写标识符、与 `Find` 组合实现不区分大小写的搜索。
- **相关节点**：`Lower`、`Find`、`Length`、`Format`

---

## Find

- **中文名**：查找
- **API 签名**：`Find(SubString, String)`
- **返回值类型**：`Number`
- **作用**：在字符串中搜索指定子串，返回首次出现的起始索引（从 0 开始）。如果未找到子串则返回 -1。
- **参数说明**：

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `sub` | 是 | String | 要查找的子串 |
| `s` | 是 | String | 被搜索的源字符串 |

- **`.code` 示例**：

```code
s = "Hello World"
pos = Find("World", s)
Log(pos) // 输出 6
```

- **常见使用场景**：检查字符串是否包含特定关键词、定位子串位置以配合 `SubString` 截取、实现关键字搜索逻辑。
- **相关节点**：`SubString`、`Length`、`Lower`、`Upper`、`Format`

---

## SubString

- **中文名**：截取
- **API 签名**：`SubString(String, start = Start[, length = Length])` / `SubString(String, start = Start, end = End)` / `SubString(String, end = End[, length = Length])`
- **返回值类型**：`String`
- **作用**：从字符串中提取子串。索引从 0 开始，结果包含 `end` 位置的字符。支持多种参数组合：起始+长度、起始+结束、仅结束等。
- **参数说明**：

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `s` | 是 | String | 源字符串 |
| `start` | 否 | Number | 起始索引（从 0 开始），默认 0 |
| `end` | 否 | Number | 结束索引（包含该位置） |
| `length` | 否 | Number | 截取长度 |

- **`.code` 示例**：

```code
s = "Hello World"
Log(SubString(s, start=4))        // "o World"
Log(SubString(s, start=4, length=5)) // "o Wor"
Log(SubString(s, start=4, end=9))    // "o Worl"
Log(SubString(s, end=9))             // "Hello Worl"
Log(SubString(s, end=3, length=3))   // "ell"
```

- **常见使用场景**：提取文件名、分割对话文本、从格式化字符串中截取特定部分、实现文本省略号。
- **相关节点**：`Length`、`Find`、`Format`、`Lower`、`Upper`

---

## Format

- **中文名**：格式化
- **API 签名**：`Format(FormatString[, UnnamedParameter1]...)` / `Format(FormatString[, ListOfUnnamedParameters])`
- **返回值类型**：`String`
- **作用**：使用占位符 `{0}`、`{1}` 等替换模板字符串中的参数。支持 C# 风格的格式化说明符（如 `{0:F3}` 控制小数位数）。参数可以是独立的命名/未命名参数，也可以是一个包含所有值的 List。
- **参数说明**：

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `fmt` | 是 | String | 格式化模板，包含 `{0}`、`{1}` 等占位符 |
| `params` | 否 | Object | 参数列表或键值对，替换占位符 |

- **`.code` 示例**：

```code
// 使用独立参数
Log(Format("Number with 3 decimals: {0:F3}", 1/7))
// 输出 "Number with 3 decimals: 0.143"

// 使用列表参数
list = CreateList("cow", 4, "legs")
Log(Format("A {0} has {1} {2}.", list))
// 输出 "A cow has 4 legs."
```

- **常见使用场景**：动态拼接对话文本、格式化数值显示、构建包含变量的日志消息、多语言字符串模板。
- **相关节点**：`SubString`、`Length`、`Find`、`ToNumber`、`CreateList`

---

## ToNumber

- **中文名**：转数字
- **API 签名**：`ToNumber(S)`
- **返回值类型**：`Number` 或 `null`
- **作用**：将字符串转换为数值。如果字符串无法转换为合法数字则返回 `null`。常见于解析用户输入或文本文件中的数字。
- **参数说明**：

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `s` | 是 | String | 要转换的字符串 |

- **`.code` 示例**：

```code
val = ToNumber("123.45")
Log(val) // 输出 123.45

bad = ToNumber("abc")
Log(bad) // 输出 null
```

- **常见使用场景**：解析配置文件中的数字、转换用户输入、处理从文件读取的数值字符串。
- **相关节点**：`Format`、`Find`、`Length`、`StringConstant`

---

## FileExists

- **中文名**：文件存在
- **API 签名**：`FileExists(Path)`
- **返回值类型**：`Boolean`
- **作用**：检查指定路径的文件是否存在。路径相对于项目根目录。常用于前置条件判断，避免在文件不存在时执行读取操作。
- **参数说明**：

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `path` | 是 | String | 文件路径（相对项目目录） |

- **`.code` 示例**：

```code
if FileExists("data/settings.json")
    Log("Settings file found")
else
    Log("Settings file missing")
```

- **常见使用场景**：检查配置文件是否存在、图片/音效资源前置校验、条件分支加载逻辑。
- **相关节点**：`GetFiles`、`GetFileExtension`、`CreateListFromJson`、`If`

---

## GetFiles

- **中文名**：获取文件
- **API 签名**：`GetFiles([Path][, subfolders = SearchSubfolders])`
- **返回值类型**：`List`
- **作用**：获取指定目录下的文件列表。可以递归搜索子文件夹。如果不指定路径则默认扫描项目根目录。返回的列表包含相对路径字符串。
- **参数说明**：

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `path` | 是 | String | 目录路径（相对项目目录） |
| `subfolders` | 否 | Boolean | 是否包含子文件夹中的文件 |

- **`.code` 示例**：

```code
files = GetFiles(path="Images", subfolders=true)
i = 0
while i < files.Count()
    Log(files[i])
    i += 1
```

- **常见使用场景**：动态查找项目中所有图片/音频资源、批量处理文件、资产浏览器功能。
- **相关节点**：`FileExists`、`GetFileExtension`、`CreateList`、`For`

---

## GetFileExtension

- **中文名**：获取扩展名
- **API 签名**：`GetFileExtension(Path)`
- **返回值类型**：`String`
- **作用**：从文件路径中提取扩展名（包含句点）。如果文件没有扩展名则返回空字符串。用于按文件类型分类或验证。
- **参数说明**：

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `path` | 是 | String | 文件路径 |

- **`.code` 示例**：

```code
ext = GetFileExtension("image.png")
Log(ext) // 输出 ".png"

noext = GetFileExtension("README")
Log(noext) // 输出 ""
```

- **常见使用场景**：按扩展名筛选文件列表、验证输入路径是否为指定类型、与 `GetFiles` 组合实现资产分类。
- **相关节点**：`FileExists`、`GetFiles`、`SubString`、`Find`

---

## CreateList

- **中文名**：创建列表
- **API 签名**：`CreateList([Name1 = Value1][, Name2 = Value2]...)` / `CreateList([Value1][, Value2]...)` / 命名和未命名参数混合
- **返回值类型**：`List`
- **作用**：创建一个新的 List 对象。支持命名参数（键值对）和未命名参数（自动生成字符串索引）。所有索引在内部被转换为字符串。List 是 `.code` 中最核心的数据结构，兼具数组和字典功能。
- **参数说明**：

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `keyValues` | 否 | Object | 键值对，作为列表的初始条目 |

- **`.code` 示例**：

```code
// 空列表
list1 = CreateList()

// 未命名参数（数字索引）
list2 = CreateList("apple", "banana", "cherry")
Log(list2[0]) // "apple"

// 命名参数（字符串索引）
list3 = CreateList(name = "Player", level = 5, class = "Mage")
Log(list3.name) // "Player"

// 混合使用
list4 = CreateList("first", key = "value", 3)
```

- **常见使用场景**：构建函数参数包、存储配置数据、作为键值对字典使用、构造多值返回值。
- **相关节点**：`Copy`、`CreateListFromJson`、`Format`、`For`

---

## Copy

- **中文名**：复制列表
- **API 签名**：`Copy(ListToCopy[, deepCopy])`
- **返回值类型**：`List`
- **作用**：复制一个 List。默认执行浅拷贝，新列表中的嵌套对象仍然引用原对象。可选 `deepCopy` 参数可执行深拷贝，递归复制所有嵌套内容。
- **参数说明**：

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `list` | 是 | List | 要复制的源列表 |
| `deepCopy` | 否 | Boolean | 是否深拷贝（默认 false） |

- **`.code` 示例**：

```code
original = CreateList(a = 1, nested = CreateList(x = 10))

// 浅拷贝
shallow = Copy(original)
// 修改嵌套对象会影响原列表
shallow.nested.x = 99
Log(original.nested.x) // 99

// 深拷贝
deep = Copy(original, deepCopy = true)
deep.nested.x = 42
Log(original.nested.x) // 不改变（仍为 99）
```

- **常见使用场景**：在修改前保留原始数据快照、传递列表副本以避免副作用、序列化前深拷贝复杂结构。
- **相关节点**：`CreateList`、`CreateListFromJson`、`Global`、`Local`

---

## CreateListFromJson

- **中文名**：从 JSON 创建列表
- **API 签名**：`CreateListFromJson(file = FilePath)`
- **返回值类型**：`List`
- **作用**：从 JSON 文件加载内容并解析为一个 List 对象。文件路径相对于项目根目录。JSON 中的对象和数组会被递归转换为 List 结构。
- **参数说明**：

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `file` | 是 | String | JSON 文件路径（相对项目目录） |

- **`.code` 示例**：

```code
data = CreateListFromJson(file = "data/missions.json")
Log(data.Count())
// 遍历 JSON 中的每个条目
i = 0
while i < data.Count()
    Log(Format("Mission: {0}", data[i].title))
    i += 1
```

- **常见使用场景**：加载关卡/任务配置数据、读取 NPC 对话脚本、导入外部数据驱动的游戏内容。
- **相关节点**：`CreateList`、`Copy`、`FileExists`、`GetFiles`

---

## StringConstant

- **中文名**：字符串常量
- **API 签名**：（纯数据节点，无 Flow 端口，直接输出常量值）
- **返回值类型**：`String`
- **作用**：输出一个固定的字符串值，供其他节点的 Data 端口连接使用。编辑器属性面板中输入文本，代码生成时直接作为字符串字面量使用。
- **参数说明**：

| 参数 | 必填 | 类型 | 含义 |
|------|------|------|------|
| `value` | 是 | String | 字符串常量值 |

- **`.code` 示例**：

```code
// 假设 StringConstant 节点 id 为 "sc1"，value 为 "Hello"
// 在代码中直接作为字符串字面量使用：
Log("Hello")
```

- **常见使用场景**：为其他节点提供固定字符串输入、作为 `Format` 的模板字符串、作为 `Find`/`SubString` 的搜索目标。
- **相关节点**：`NumberConstant`、`Boolean`、`Format`、`Find`

> 这些节点在 `.code` 中没有对应物，代码生成器遇到它们时直接跳过（`follow_flow` 贯通执行链），不产生任何脚本输出。它们只服务于编辑器的可视化组织和元数据管理。

---

## Meta

| 属性 | 值 |
|------|-----|
| **中文名** | 元数据 |
| **分类** | Editor-only |
| **类别** | A 类（自定义代码生成） |
| **颜色** | `#757575`（暗灰色，`SPECIAL_COLOR`） |
| **返回值** | 无 |
| **端口** | `in_flow`（执行输入）+ `out_flow`（下一步输出） |

### 官方 API 签名

编辑器专用节点，无对应官方 `.code` API。其内容映射到导出时的 `meta.json` 文件（任务元数据）。

### 作用

- 在节点图中嵌入任务级元数据（标题、描述、设置菜单配置）
- 导出工程时，与该节点关联的数据被写入 `meta.json`，构成任务的全局元信息
- 允许在可视化画布上直接编辑任务的 `title`、`description` 和 `settings`，无需手动编辑 JSON

### 参数说明

| 参数名 | 必填 | 类型 | 含义 |
|--------|------|------|------|
| `title` | 否 | `Object` | 多语言标题字典，如 `{"en": "My Mission", "zh": "我的任务"}`。映射到 `meta.json` 的 `title` 字段 |
| `description` | 否 | `Object` | 多语言描述字典，如 `{"en": "A cool mission", "zh": "一个酷任务"}`。映射到 `meta.json` 的 `description` 字段 |
| `settings` | 否 | `List` | 设置菜单项列表。每个元素定义一个设置（类型、默认值、多语言标签等）。映射到 `meta.json` 的 `settings` 字段 |

### 使用案例

```code
Meta 节点参数示例：
  title: { "en": "Escape from Prison", "zh": "越狱" }
  description: { "en": "Break out of the high-security facility", "zh": "从高安全级别的监狱中逃出" }
```

### 常见使用场景

- 每个工程中放置一个 `Meta` 节点，集中管理任务标题和描述的多语言翻译
- 定义任务开始前的设置菜单（如难度选择、初始装备），用户可通过 `_settings` 全局变量在 `.code` 中读取
- 作为图的"元数据锚点"，方便导出流程自动收集工程信息

### 相关节点

- `Comment` — 注释文本，辅助文档而非结构化元数据
- `Group` — 视觉分组，组织画布布局
- `StringConstant` / `ListConstant` — 构造参数值的数据源节点

---

## Comment

| 属性 | 值 |
|------|-----|
| **中文名** | 注释 |
| **分类** | Editor-only |
| **类别** | A 类（自定义代码生成） |
| **颜色** | `#757575`（暗灰色，`SPECIAL_COLOR`） |
| **返回值** | 无 |
| **端口** | `in_flow`（执行输入）+ `out_flow`（下一步输出） |

### 官方 API 签名

编辑器专用节点，无对应官方 `.code` API。

### 作用

- 在节点图中添加纯文本注释，帮助开发者或团队成员理解流程图逻辑
- 注释内容仅在编辑器中可见，不参与代码生成或导出
- 可以嵌入执行链中任意位置，不影响执行流程（代码生成器直接贯通 Flow 边）

### 参数说明

| 参数名 | 必填 | 类型 | 含义 |
|--------|------|------|------|
| `text` | 否 | `String` | 注释文本内容。支持多行，编辑器内显示为可编辑文本区域 |

### 使用案例

```code
Comment 节点文本：
  "此处检查玩家是否拥有钥匙。
   如果拥有，走 True 分支开门；
   否则触发寻找钥匙的子线程。"
```

### 常见使用场景

- 标注复杂分支逻辑的判断条件
- 记录待办事项或 TODO 提醒
- 为团队协作提供流程说明（如"此段由 XX 负责"）
- 解释非直观的参数取值理由

### 相关节点

- `Meta` — 结构化元数据，导出到 `meta.json`
- `Group` — 视觉分组，框选多个节点
- `Log` — 运行时日志输出（调试时会生成代码）

---

## Group

| 属性 | 值 |
|------|-----|
| **中文名** | 分组 |
| **分类** | Editor-only |
| **类别** | A 类（自定义代码生成） |
| **颜色** | `#757575`（暗灰色，`SPECIAL_COLOR`） |
| **返回值** | 无 |
| **端口** | `in_flow`（执行输入）+ `out_flow`（下一步输出） |

### 官方 API 签名

编辑器专用节点，无对应官方 `.code` API。

### 作用

- 在画布上创建一个带标题的可视化分组框，用于框选和组织相关节点
- 分组框具有可自定义的颜色和标题，帮助按功能模块梳理复杂流程图
- 仅影响视觉布局，不改变代码生成逻辑

### 参数说明

| 参数名 | 必填 | 类型 | 含义 |
|--------|------|------|------|
| `title` | 否 | `String` | 分组框的标题文本，显示在框的顶部 |
| `color` | 否 | `Color` | 分组框的边框/标题栏颜色，用于视觉区分不同模块 |

### 使用案例

```code
Group 节点参数示例：
  title: "初始化阶段"
  color: #4CAF50（绿色）
```

### 常见使用场景

- 将流程按阶段归纳（"初始化"、"主循环"、"清理"）
- 区分不同游戏系统的逻辑（"对话系统"、"战斗系统"、"UI 控制"）
- 折叠/展开一组节点以减少画布视觉混乱
- 在多作者协作时标注模块归属

### 相关节点

- `Comment` — 文本注释，补充说明
- `Meta` — 任务元数据
- `Log` — 调试输出

## If

- **中文名**：如果
- **官方 API**：`if {condition}` / `else`
- **返回值类型**：void（控制流结构，不产生数据值）
- **作用**：根据布尔条件值选择两条执行路径之一。条件为真时执行 `out_true` 分支，为假时执行 `out_false` 分支。代码生成器会自动寻找两条分支的汇合节点，在汇合处继续执行后续代码。
- **参数说明**：
  - `condition`（必填，Boolean）：条件表达式，为 true 时走真分支，false 时走假分支。
- **`.code` 使用案例**：
  ```code
  if _state.IsDayTime
      Log("现在是白天")
  else
      Log("现在是夜晚")
  ```
- **常见使用场景**：
  - 根据玩家状态（如位置、装备、时间）决定执行逻辑
  - 与 `CheckCondition` 组合检查复杂条件
  - 和 `LogicAnd`/`LogicOr`/`LogicNot` 组合构建复合条件
- **相关节点**：While, For, CheckCondition, LogicAnd, LogicNot

---

## While

- **中文名**：循环
- **官方 API**：`while {condition}`
- **返回值类型**：void（控制流结构）
- **作用**：在条件为真时重复执行循环体内的代码序列。每次循环前重新评估条件。通过 `out_break` 端口连接 `Break` 节点可以提前退出循环。
- **参数说明**：
  - `condition`（必填，Boolean）：循环继续条件，为 true 时继续执行循环体。
- **`.code` 使用案例**：
  ```code
  i = 0
  while i < 10
      Log(i)
      i = i + 1
  ```
- **常见使用场景**：
  - 遍历数字范围（配合 `Range` 节点或手动计数）
  - 等待某个条件成立（配合 `_state` 读取和 `Wait` 节点）
  - 重复执行定期检查逻辑
- **相关节点**：For, If, Break, Range, CompareNumbers

---

## For

- **中文名**：遍历
- **官方 API**：`for i in {iterable}`
- **返回值类型**：void（控制流结构）
- **作用**：遍历列表中的每个元素，循环变量名为 `i`。每次迭代将当前元素赋值给 `i`，执行完循环体后进入下一个元素。通过 `out_break` 端口连接 `Break` 节点可以提前退出。
- **参数说明**：
  - `iterable`（必填，List）：要遍历的列表，每次迭代将列表中的一个值赋给 `i`。
- **`.code` 使用案例**：
  ```code
  for i in Range(5, 10)
      Log(i)
  ; 输出: 5 6 7 8 9
  ```
- **常见使用场景**：
  - 遍历列表中的所有元素
  - 与 `Range` 组合生成数字范围遍历
  - 遍历 `_state` 中的子列表（如物品列表、MOD 列表）
- **相关节点**：While, For, Range, ListMethods, Break

---

## SetEvent

- **中文名**：设置事件
- **官方 API**：`SetEvent(EventName[, Value])`
- **返回值类型**：null
- **作用**：设置一个跨线程、跨帧的事件数据。事件仅在下一帧有效，可以被同项目或其他项目的 `GetEvent` 读取。不能传递对象（Object）类型。
- **参数说明**：
  - `name`（必填，String）：事件名称，用于标识事件。
  - `value`（必填，List）：要传递的数据，可以是任意值（List/Number/String/Boolean），但不支持 Object。
- **`.code` 使用案例**：
  ```code
  SetEvent("mission_complete", "任务A")
  SetEvent("score_update", CreateList(score = 1000, grade = "S"))
  ```
- **常见使用场景**：
  - 线程间通信（一个线程通知另一个线程某个条件达成）
  - 跨帧状态传递
  - 配合 `CreateEventListener` 触发事件监听器
  - 与 `GetEvent` 搭配实现异步交互
- **相关节点**：GetEvent, CreateEventListener, CreateEventListenerLocal

---

## GetEvent

- **中文名**：获取事件
- **官方 API**：`GetEvent(EventName)`
- **返回值类型**：List 或 null
- **作用**：检查上一帧是否设置了指定事件。如果事件存在，返回一个以数字索引的列表包含 `SetEvent` 传递的数据；如果事件不存在，返回 null。
- **参数说明**：
  - `name`（必填，String）：要检查的事件名称。
- **`.code` 使用案例**：
  ```code
  event_data = GetEvent("mission_complete")
  if event_data != null
      Log(event_data[0]) ; 输出: "任务A"
  ```
- **常见使用场景**：
  - 在主线程中轮询检查子线程是否完成某项任务
  - 接收其他 MOD 发送的事件
  - 实现跨帧的简单状态同步
- **相关节点**：SetEvent, CreateEventListener, CreateEventListenerLocal

---

## PlaySoundEffect

- **中文名**：播放音效
- **官方 API**：`PlaySoundEffect(SoundEffectName[, volume = Volume][, x = PositionX, y = PositionY, z = PositionZ])`
- **返回值类型**：null
- **作用**：在指定位置（可选）播放一个游戏内置音效。音量值范围 0~1。如果不传位置参数，播放 2D 音效；传位置参数则播放 3D 空间音效。
- **参数说明**：
  - `name`（必填，Enum/SoundEffect）：音效名称，来自预定义列表（如 `HeartBeat`、`FootStepHeel`、`RankUp` 等共 86 种）。
  - `volume`（可选，Number）：音量，范围 0~1，默认 1。
  - `position`（可选，Vector）：3D 空间位置 `[x, y, z]`，用于空间音效定位。
- **`.code` 使用案例**：
  ```code
  PlaySoundEffect("RankUp")
  PlaySoundEffect("HeartBeat", volume = 0.5)
  PlaySoundEffect("FootStepHeel", x = -26.6, y = -0.1, z = -120)
  ```
- **常见使用场景**：
  - 任务完成时播放提示音（`RankUp`、`Ok`）
  - 玩家进入特定区域时播放环境音
  - 心跳音效增强紧张感（`HeartBeat`）
- **相关节点**：无直接相关节点；可配合 `CreateAudio` 对象播放自定义音频文件

---

## CreateThread

- **中文名**：创建线程
- **官方 API**：`CreateThread(labelName[, Named_Or_Unnamed_Parameter]...)`
- **返回值类型**：Object（Thread 对象引用）
- **作用**：创建一个新线程，标签内的代码立即在执行帧运行一次。线程拥有独立的作用域。通过 `thread.Goto("label")` 跳转到线程内的其他标签切换执行位置。跨线程通信应使用 `SetEvent`/`GetEvent`。
- **参数说明**：
  - `labelName`（必填，String）：线程入口标签名。
  - `params`（可选，Object）：传递给线程的命名参数键值对，在线程内作为局部变量访问。
- **`.code` 使用案例**：
  ```code
  patrol_thread = CreateThread("patrol", start_x = 0, end_x = 100)
  ; 在 "patrol:" 标签内可以直接使用 start_x 和 end_x 变量
  ```
- **常见使用场景**：
  - 创建 NPC 巡逻行为线程
  - 并行执行独立逻辑（如计时器、条件监控）
  - 与 `WaitForThread` 组合等待子线程完成
- **相关节点**：WaitForThread, GetCurrentThread, CreateListener, SetEvent, Goto

---

## CreateListener

- **中文名**：创建监听器
- **官方 API**：`CreateListener(labelName[, Named_Or_Unnamed_Parameter]...)`
- **返回值类型**：Object（Listener 对象引用）
- **作用**：创建一个每帧都运行的监听器。与线程不同，监听器内的变量会跨帧保持值不变。`CreateListener` 在定义代码的作用域中创建监听器（父作用域）。
- **参数说明**：
  - `labelName`（必填，String）：监听器要运行的标签名。
  - `params`（可选，Object）：传递给监听器的命名参数，作为局部变量访问。
- **`.code` 使用案例**：
  ```code
  timer = CreateListener("countdown", seconds = 10)
  countdown:
      seconds = seconds - _timediff
      if seconds <= 0
          Log("时间到！")
          timer = null ; 销毁监听器
  ```
- **常见使用场景**：
  - 倒计时/定时器
  - 持续监控玩家状态变化（如位置、装备）
  - 延迟执行一段代码后自动销毁
- **相关节点**：CreateListenerLocal, DestroyListener, CreateThread, CreateEventListener

---

## CreateListenerLocal

- **中文名**：创建局部监听器
- **官方 API**：`CreateListenerLocal(labelName[, Named_Or_Unnamed_Parameter]...)`
- **返回值类型**：Object（Listener 对象引用）
- **作用**：与 `CreateListener` 功能相同，但在**调用处的作用域**中创建监听器。这意味着监听器可以访问调用处的局部变量（而非定义处的）。当父作用域销毁时监听器也随之销毁。
- **参数说明**：
  - `labelName`（必填，String）：监听器要运行的标签名。
  - `params`（可选，Object）：传递给监听器的命名参数。
- **`.code` 使用案例**：
  ```code
  main:
      localvar = "Hello"
      ; CreateListener 创建在 main 作用域，能访问 localvar
      listener = CreateListenerLocal("print_var")
  print_var:
      Log(localvar) ; 输出: "Hello"（访问了 main 作用域的变量）
  ```
- **常见使用场景**：
  - 共享代码中需要访问调用者局部变量
  - 与 `CreateListener` 对比使用，理解作用域差异
  - 编写通用的可复用监听器工具函数
- **相关节点**：CreateListener, DestroyListener, CreateThread, CreateEventListenerLocal

---

## DestroyListener

- **中文名**：销毁监听器
- **官方 API**：`listener = null`
- **返回值类型**：null
- **作用**：销毁当前监听器。在代码生成器中翻译为 `listener = null`，将监听器变量置为 null，游戏引擎检测到引用计数归零后自动销毁监听器对象。当前获取的 `_this` 引用仍有效。
- **参数说明**：无（节点仅有 Flow 输入输出端口）
- **`.code` 使用案例**：
  ```code
  ; 在 CreateListener 创建的标签内
  timeout:
      duration = duration + _timediff
      if duration >= max_time
          Log("监听器超时，销毁")
          listener = null ; DestroyListener 节点
  ```
- **常见使用场景**：
  - 监听器完成目标任务后自我销毁
  - 倒计时结束停止监听器
  - 防止监听器无限运行导致性能问题
- **相关节点**：CreateListener, CreateListenerLocal, GetCurrentThread

---

## GetCurrentThread

- **中文名**：当前线程
- **官方 API**：`_this`
- **返回值类型**：Object（Thread 引用）
- **作用**：获取当前正在执行的线程对象的引用。可以用于调用线程方法（如 `Goto`）或将线程引用传递给其他函数。
- **参数说明**：无（纯数据节点，无 Flow 端口）
- **`.code` 使用案例**：
  ```code
  ; 在标签内部（代码生成器自动添加 thread = _this）
  thread.Goto("next_label")
  ; 等价于 CreateThread 返回的引用上调用 Goto
  ```
- **常见使用场景**：
  - 在标签内部获取当前线程引用以便调用 `Goto`
  - 传递线程引用给其他函数或标签
  - 与 `CallMethod` 配合调用线程方法
- **相关节点**：CreateThread, WaitForThread, Goto

---

## WaitForThread

- **中文名**：等待线程结束
- **官方 API**：`{thread}.WaitForFinish()`
- **返回值类型**：null
- **作用**：阻塞当前线程的执行，直到指定的子线程完成（即子线程执行完所有标签或跳转到无后续标签的位置）。常用于主线程等待子任务完成后继续。
- **参数说明**：
  - `thread`（必填，Object）：要等待的线程对象引用。
- **`.code` 使用案例**：
  ```code
  worker = CreateThread("do_work")
  ; 执行其他并行工作...
  worker.WaitForFinish()
  Log("工作线程已完成")
  ```
- **常见使用场景**：
  - 主线程等待 NPC 行动线程完成后处理结果
  - 按顺序执行依赖异步任务的逻辑
  - 等待计时器线程结束后执行下一步
- **相关节点**：CreateThread, GetCurrentThread, SetEvent, Goto

---

## CreateCondition

- **中文名**：创建条件
- **官方 API**：`CreateCondition(Condition[, id = ID])`
- **返回值类型**：Object（Condition 对象）
- **作用**：使用简洁的 DSL 语法创建组合条件对象。条件语法支持方括号（`[ ]`=逻辑与）、圆括号（`( )`=逻辑或）、感叹号（`!`=取反）以及数值比较（`>`, `>=`, `<`, `<=`, `==`, `!=`）。可设置 ID 作为子条件在其他条件中引用。
- **参数说明**：
  - `condition`（必填，String）：条件表达式字符串，如 `"[Exposed_Front,Exposed_Hip,!Exposed_All]"`。
  - `id`（可选，String）：条件对象的标识符，可用于在其他条件中以 `Subcondition` 类型引用。
- **`.code` 使用案例**：
  ```code
  ; 创建条件：必须暴露前身和臀部，但不能完全裸体
  cond1 = CreateCondition("[Exposed_Front,Exposed_Hip,!Exposed_All]")

  ; 创建带 ID 的子条件
  cond2 = CreateCondition("IsDayTime", id = "daycheck")

  ; 在其他条件中引用子条件
  cond3 = CreateCondition("[Subcondition_daycheck,Exposed_None]")
  ```
- **常见使用场景**：
  - 构建复杂的任务完成条件
  - 与 `CheckCondition` 组合判断条件是否成立
  - 在 `CreateListener` 或 `CreateEventListener` 中作为预定义条件传递
- **相关节点**：CheckCondition, CreateItemCondition, GetStateBool, CompareNumbers

---

## CreateItemCondition

- **中文名**：物品条件
- **官方 API**：`CreateItemCondition(itemtype = ItemType, zone = Zone[, id = ID])`  
  `CreateItemCondition(itemtype = ItemType, area = Area[, id = ID])`
- **返回值类型**：Object（ItemCondition 对象）
- **作用**：创建一个特定物品在指定区域内的条件对象。用于检查某个掉落物品是否在给定区域/地带内存在。条件对象可以通过 `Check()` 方法判断条件是否满足。
- **参数说明**：
  - `itemtype`（必填，Enum/DropItemType）：物品类型，如 `Coat`、`HandcuffKey`、`DildoFloor` 等。
  - `zone`（可选，Object）：Zone 对象引用，物品须在地带内。
  - `area`（可选，Object）：Area 对象引用，物品须在区域内。`zone` 和 `area` 二选一。
  - `id`（可选，String）：条件标识，用于在其他条件中以子条件引用。
- **`.code` 使用案例**：
  ```code
  coat_zone = CreateZone(CreateArea("sphere", stage = "Residence", x = 0, y = 0, z = 0, r = 10))
  coat_cond = CreateItemCondition(itemtype = "Coat", zone = coat_zone)
  if coat_cond.Check()
      Log("外套在区域内")
  ```
- **常见使用场景**：
  - 检查玩家是否将特定物品放在指定位置
  - 与 `CheckCondition` 组合作为任务完成条件之一
  - 配合 `DropItem` 和 `CollectItem` 实现物品放置/收集任务
- **相关节点**：CreateCondition, CheckCondition, CreateZone, CreateArea, DropItem

---

## CheckCondition

- **中文名**：检查条件
- **官方 API**：`{condition}.Check()`
- **返回值类型**：Boolean
- **作用**：对 `CreateCondition` 或 `CreateItemCondition` 创建的条件对象执行检查，返回布尔值表示条件是否满足。这是一个纯数据节点，无 Flow 端口，直接输出结果给其他节点（如 `If` 的 condition 参数）。
- **参数说明**：无参数；通过 Data 输入端口接收条件对象。
  - `cond`（必填，Object Data 端口）：要检查的 Condition 或 ItemCondition 对象。
- **`.code` 使用案例**：
  ```code
  my_cond = CreateCondition("[Exposed_Front,IsDayTime]")
  result = my_cond.Check()
  if result
      Log("白天且暴露前身")
  ```
- **常见使用场景**：
  - 将 `CreateCondition` 的条件转化为布尔值供 `If` 节点使用
  - 在循环中每帧检查条件是否成立
  - 与 `LogicAnd`/`LogicOr` 组合多个条件的检查结果
- **相关节点**：CreateCondition, CreateItemCondition, If, LogicAnd, LogicOr
