# CustomMissions2 流编辑器 — 节点类型清单

> 版本：v1.0  
> 来源：基于 CustomMissions2 API 文档全面梳理  
> 用途：定义编辑器必须支持的全部节点类型，作为开发契约  
> 相关文档：
>
> - JSON 格式：[json_schema.md](json_schema.md)
> - 项目架构：[rust_project_skeleton.md](rust_project_skeleton.md)
> - 开发约束：[agent_prompt.md](agent_prompt.md)

---

## 一、约定

- 每个节点对应文档中的一个**函数调用**、**控制结构**或**对象构造/方法**。
- 节点类型命名采用 `PascalCase`，与文档中的函数名保持一致；对象方法使用 `Object.Method` 形式（如 `Area.Inside`）。
- 端口分为两类：
  - **执行流（Flow）**：白色，控制执行顺序，对应 `.code` 中的逐行执行
  - **数据端口（Data）**：彩色，传递 Number/String/Boolean/List/Object，用于参数动态化
- 节点按功能分组，对应编辑器左侧面板的分类树。
- 参数类型中 `Any` 表示接受任意类型；`Object` 通常表示游戏对象引用（Thread/Area/NPC 等）。

---

## 二、基础控制（Control）

| 节点 | 输入端口 | 输出端口 | 参数 | 说明 |
| ------ | --------- | --------- | ------ | ------ |
| `Start` | — | `Flow` | — | 任务入口，每张图必须有且仅有一个 |
| `Label` | `Flow` | `Flow` | `name: String` | 标签定义，可作为 Goto 目标 |
| `Goto` | `Flow` | `Flow` | `label: String`, `params: Object` | 跳转到指定标签 |
| `If` | `Flow` | `True: Flow`, `False: Flow` | `condition: Boolean` | 条件分支 |
| `While` | `Flow` | `Flow`, `Break: Flow` | `condition: Boolean` | 循环，支持内部 Break |
| `For` | `Flow` | `Flow`, `Break: Flow` | `iterable: List` | 遍历列表，变量名 `i` |
| `Break` | `Flow` | — | — | 提前退出循环 |
| `Return` | `Flow` | — | `value: Any` | 函数返回，设置 `_result` |
| `Wait` | `Flow` | `Flow` | `seconds: Number` | 延迟等待（基于 `_timediff`，单位为秒） |
| `WaitForEvent` | `Flow` | `Flow` | `eventName: String` | 阻塞当前线程，直到 `SetEvent` 触发指定事件 |

---

## 三、通用函数（General Functions）

| 节点 | 输入端口 | 输出端口 | 参数 | 返回类型 | 说明 |
| ------ | --------- | --------- | ------ | --------- | ------ |
| `Log` | `Flow` | `Flow` | `output: Any` | — | 控制台输出，用于调试 |
| `Global` | `Flow` | `Flow` | `name: String`, `value: Any` | `Any` | 读写全局变量；`value` 非空时写入，否则读取 |
| `Local` | `Flow` | `Flow` | `name: String`, `value: Any` | `Any` | 读写局部变量；作用域为当前线程/标签 |
| `GetType` | `Flow` | `Flow` | `value: Any` | `String` | 获取值类型名（如 `"Number"`、`"String"`） |
| `GetLanguage` | `Flow` | `Flow` | — | `String` | 获取当前语言代码（如 `"En"`、`"Ja"`） |
| `DumpVariables` | `Flow` | `Flow` | `recursion: Number` | — | 打印所有变量到日志 |
| `DumpVariable` | `Flow` | `Flow` | `var: Any`, `recursion: Number` | — | 打印单个变量到日志 |
| `CallFunction` | `Flow` | `Flow` | `function: String`, `params: List` | `Any` | 动态调用函数名 |
| `CallMethod` | `Flow` | `Flow` | `thread: Object`, `method: String`, `params: List` | `Any` | 动态调用对象方法 |
| `Color` | `Flow` | `Flow` | `r: Number`, `g: Number`, `b: Number`, `a: Number` | `List` | 创建颜色列表 `[r, g, b, a]` |
| `Range` | `Flow` | `Flow` | `start: Number`, `stop: Number`, `step: Number` | `List` | 生成数字范围 `[start, start+step, ...]` |
| `SetEvent` | `Flow` | `Flow` | `name: String`, `value: Any` | — | 设置跨线程/跨帧事件数据 |
| `GetEvent` | `Flow` | `Flow` | `name: String` | `List` | 获取事件数据；若无事件返回空列表 |

---

## 四、游戏函数（Game Functions）

### 4.1 物品与装备

| 节点 | 参数 | 返回 | 说明 |
| ------ | ------ | ------ | ------ |
| `DropItem` | `itemtype: String`, `stage: String`, `position: List`, `rotation: List`, `compass: Boolean` | `String/Number` | 在指定场景掉落物品，返回物品引用或 ID |
| `CollectItem` | `itemtype: String`, `position: List` | `Boolean` | 捡起指定类型物品 |
| `SetVibrator` | `strength: Number/String` | — | 设置跳蛋强度；`String` 用于模式名 |
| `SetPiston` | `strength: Number/String` | — | 设置活塞强度；`String` 用于模式名 |
| `LockHandcuffs` | `handcuffstype: String`, `attachtoobject: Boolean`, `duration: Number` | — | 上锁手铐 |
| `UnlockHandcuffs` | — | — | 解锁手铐 |
| `EquipCosplay` | `cosplayKeys: List` | — | 装备角色扮演服装 |
| `UnequipCosplay` | `cosplayKeys: List` | — | 卸下角色扮演服装 |
| `UnequipAllCosplay` | — | — | 卸下全部角色扮演服装 |
| `OwnCosplay` | `owns: Boolean`, `cosplayKeys: List` | — | 设置服装拥有状态 |
| `EquipAdultToy` | `toyNames: List` | — | 装备成人玩具 |
| `UnequipAdultToy` | `toyNames: List` | — | 卸下成人玩具 |

### 4.2 玩家状态

| 节点 | 参数 | 返回 | 说明 |
| ------ | ------ | ------ | ------ |
| `SetPlayerPosition` | `position: List`, `rotation: List` | — | 设置玩家位置 |
| `SetStage` | `stage: String`, `daytime: Boolean` | — | 切换场景 |
| `SetCamera` | `pitch: Number`, `yaw: Number`, `lock: Boolean` | — | 设置摄像机 |
| `SetAction` | `action: String` | — | 设置玩家动作 |
| `SetFutanari` | `active: Boolean` | — | 设置双性状态 |
| `SetSkill` | `skill: String`, `enabled: Boolean` | — | 启用/禁用技能 |
| `SetPlayerData` | `dataName: String`, `value: Any` | — | 设置玩家数据 |
| `SetSkillShortcut` | `slot: Number`, `actionIndex: Number` | — | 设置技能快捷栏 |
| `GetSkillShortcut` | `slot: Number` | `Number` | 获取快捷栏 |
| `GetRandomPosition` | `minRange: Number` | `List` | 获取随机位置 |

### 4.3 数值操作（RP/体力/快感等）

| 节点 | 参数 | 返回 | 说明 |
| ------ | ------ | ------ | ------ |
| `AddCurrentEarnRP` | `value: Number` | `Number` | 增加本次外出赚取RP |
| `SetCurrentEarnRP` | `value: Number` | `Number` | 设置本次外出赚取RP |
| `GetCurrentEarnRP` | — | `Number` | 获取本次外出赚取RP |
| `AddCurrentRP` | `value: Number` | `Number` | 增加持有RP |
| `SetCurrentRP` | `value: Number` | `Number` | 设置持有RP |
| `GetCurrentRP` | — | `Number` | 获取持有RP |
| `SetEcstasy` | `value: Number` | `Number` | 设置快感值 |
| `AddEcstasy` | `value: Number` | `Number` | 增加快感值 |
| `GetEcstasy` | — | `Number` | 获取快感值 |
| `SetStamina` | `value: Number` | `Number` | 设置体力 |
| `AddStamina` | `value: Number` | `Number` | 增加体力 |
| `GetStamina` | — | `Number` | 获取体力 |
| `SetMoisture` | `value: Number` | `Number` | 设置膀胱/湿润度 |
| `AddMoisture` | `value: Number` | `Number` | 增加膀胱/湿润度 |
| `GetMoisture` | — | `Number` | 获取膀胱/湿润度 |
| `SetItemCount` | `item: String`, `count: Number` | `Number` | 设置物品数量 |
| `AddItemCount` | `item: String`, `count: Number` | `Number` | 增加物品数量 |
| `GetItemCount` | `item: String` | `Number` | 获取物品数量 |

### 4.4 游戏控制

| 节点 | 参数 | 返回 | 说明 |
| ------ | ------ | ------ | ------ |
| `CanGameOver` | `value: Boolean` | `Boolean` | 设置/获取是否可游戏结束 |
| `TriggerGameOver` | — | — | 强制触发游戏结束 |
| `PlaySoundEffect` | `name: String`, `volume: Number`, `position: List` | — | 播放音效 |
| `SetStageRankLimit` | `stage: String`, `rank: Number` | — | 设置场景等级限制 |
| `GetStageRankLimit` | `stage: String` | `Number` | 获取场景等级限制 |
| `SetPortalEnabled` | `stage: String`, `enabled: Boolean` | — | 启用/禁用传送门 |
| `GetAllWaypoints` | — | `List` | 获取所有路径点 |
| `SetSexPosition` | `position: String` | — | 设置性爱体位 |
| `DeactivateSex` | — | — | 停用性爱 |
| `SetSexMenu` | `canfinish: Boolean`, `canposition: Boolean/List` | — | 设置性爱菜单 |

---

## 五、附加游戏函数（Additional Game Functions）

| 节点 | 参数 | 返回 | 说明 |
| ------ | ------ | ------ | ------ |
| `ShowBlackscreen` | `color: List`, `delay: Number`, `fadein: Number`, `duration: Number`, `fadeout: Number` | — | 显示黑屏/过渡 |
| `GetSnapshotData` | `imageRef: String` | `List` | 获取快照元数据 |
| `GetAllSnapshots` | `deleted: Boolean`, `hidden: Boolean` | `List` | 获取所有快照引用 |
| `DeleteSnapshot` | `imageRef: String` | — | 标记删除快照 |
| `GetImageReference` | `filePath: String` | `String` | 获取图像引用 |

---

## 六、图形函数（Graphics）

| 节点 | 参数 | 返回 | 说明 |
|------|------|------|------|
| `SetGraphicsOption` | `option: String`, `value: Number/Boolean` | — | 设置图形选项 |
| `GetGraphicsOption` | `option: String` | `Number/Boolean` | 获取图形选项 |

---

## 七、数学函数（Math）

### 7.1 标准数学

| 节点 | 参数 | 返回 | 说明 |
| ------ | ------ | ------ | ------ |
| `Random` | `min: Number`, `max: Number` | `Number` | 随机浮点数 |
| `RandomInt` | `min: Number`, `max: Number` | `Number` | 随机整数 |
| `Sin` / `Cos` / `Tan` | `angle: Number` | `Number` | 三角函数 |
| `Asin` / `Acos` / `Atan` | `value: Number` | `Number` | 反三角函数 |
| `Floor` / `Ceil` / `Round` / `Trunc` | `value: Number` | `Number` | 取整 |
| `Sign` / `Abs` | `value: Number` | `Number` | 符号/绝对值 |
| `LogN` / `Log2` / `Log10` | `value: Number` | `Number` | 对数 |
| `Min` / `Max` | `values: List` | `Number` | 最小/最大值 |

### 7.2 向量

| 节点 | 参数 | 返回 | 说明 |
| ------ | ------ | ------ | ------ |
| `Vector` | `x: Number`, `y: Number`, `z: Number` | `List` | 创建向量 |
| `Quaternion` | `rx, ry, rz, rw: Number` | `List` | 创建四元数 |
| `Vector3Length` | `v: List` | `Number` | 向量长度 |
| `Vector3SqrLength` | `v: List` | `Number` | 向量长度平方 |
| `Vector3Add` / `Sub` | `v1: List`, `v2: List` | `List` | 向量加减 |
| `Vector3Scale` | `v: List`, `scalar: Number` | `List` | 向量缩放 |
| `Vector3Dot` / `Cross` | `v1: List`, `v2: List` | `Number/List` | 点乘/叉乘 |
| `Vector3Rotate` | `q: List`, `v: List` | `List` | 旋转向量 |
| `Vector3Distance` | `v1: List`, `v2: List` | `Number` | 向量距离 |

---

## 八、字符串函数（String）

| 节点 | 参数 | 返回 | 说明 |
| ------ | ------ | ------ | ------ |
| `Length` | `s: String` | `Number` | 字符串长度 |
| `Lower` / `Upper` | `s: String` | `String` | 大小写转换 |
| `Find` | `sub: String`, `s: String` | `Number` | 查找子串索引 |
| `SubString` | `s: String`, `start: Number`, `end: Number`, `length: Number` | `String` | 截取子串 |
| `Format` | `fmt: String`, `params: List` | `String` | 格式化字符串 |
| `ToNumber` | `s: String` | `Number` | 字符串转数字 |

---

## 九、文件函数（File）

| 节点 | 参数 | 返回 | 说明 |
| ------ | ------ | ------ | ------ |
| `FileExists` | `path: String` | `Boolean` | 文件是否存在 |
| `GetFiles` | `path: String`, `subfolders: Boolean` | `List` | 获取文件列表 |
| `GetFileExtension` | `path: String` | `String` | 获取文件扩展名 |

---

## 十、对象构造与方法（Objects）

### 10.1 List（列表）

| 节点 | 参数 | 返回 | 说明 |
| ------ | ------ | ------ | ------ |
| `CreateList` | `keyValues: Object` | `List` | 创建列表 |
| `Copy` | `list: List`, `deepCopy: Boolean` | `List` | 复制列表 |
| `CreateListFromJson` | `file: String` | `List` | 从 JSON 文件创建列表 |
| `List.Insert` | `index: Number`, `values: List` | — | 插入元素 |
| `List.Remove` | `index: Number`, `count: Number` | — | 移除元素 |
| `List.Count` | — | `Number` | 获取元素数量 |
| `List.Contains` | `value: Any` | `Boolean` | 是否包含 |
| `List.IndexOf` | `value: Any` | `String` | 查找索引，未找到返回 null |
| `List.Keys` | `includeAll: Boolean` | `List` | 获取所有键 |

### 10.2 Thread（线程）

| 节点 | 参数 | 返回 | 说明 |
| ------ | ------ | ------ | ------ |
| `CreateThread` | `labelName: String`, `params: Any` | `Object` | 创建线程 |
| `Thread.Goto` | `labelName: String`, `params: Any` | — | 跳转标签 |
| `Thread.GetLabel` | — | `String` | 获取当前标签 |
| `Thread.CallCustom` | `methodName: String` | `Any` | 调用线程内自定义方法 |

### 10.3 Listener（监听器）

| 节点 | 参数 | 返回 | 说明 |
|------|------|------|------|
| `CreateListener` | `labelName: String`, `params: Any` | `Object` | 创建监听器（父作用域） |
| `CreateListenerLocal` | `labelName: String`, `params: Any` | `Object` | 创建监听器（当前作用域） |

### 10.4 MissionPanel（任务面板）

| 节点 | 参数 | 返回 | 说明 |
| ------ | ------ | ------ | ------ |
| `CreateMissionPanel` | — | `Object` | 创建任务面板 |
| `MissionPanel.SetText` | `text: String` | — | 设置文本 |
| `MissionPanel.SetRPText` | `text: String` | — | 设置RP文本 |
| `MissionPanel.SetVisible` | `visible: Boolean` | — | 设置可见性 |
| `MissionPanel.SetGaugeVisible` | `visible: Boolean` | — | 设置进度条可见 |
| `MissionPanel.SetGaugeProgress` | `progress: Number` | — | 设置进度 |
| `MissionPanel.GetText` / `GetRPText` / `GetVisible` / `GetGaugeVisible` / `GetGaugeProgress` | — | 对应类型 | 获取状态 |

### 10.5 MissionMenuItem（任务菜单项）

| 节点 | 参数 | 返回 | 说明 |
| ------ | ------ | ------ | ------ |
| `CreateMissionMenuItem` | — | `Object` | 创建菜单项 |
| `MissionMenuItem.SetText` / `GetText` | `text: String` | `String` | 文本 |
| `MissionMenuItem.SetRPText` / `GetRPText` | `text: String` | `String` | RP文本 |
| `MissionMenuItem.SetCleared` / `GetCleared` | `cleared: Boolean` | `Boolean` | 通关标记 |
| `MissionMenuItem.SetMark` / `GetMark` | `mark: Boolean` | `Boolean` | 标记 |
| `MissionMenuItem.SetClears` / `GetClears` | `clears: String` | `String` | 通关条件 |
| `MissionMenuItem.SetMaxRP` / `GetMaxRP` | `maxRP: Number/String` | `String` | 最大RP |
| `MissionMenuItem.AutoColor` | `value: Boolean` | `Boolean` | 自动配色 |
| `MissionMenuItem.SetBackgroundColor` | `color1: List`, `color2: List` | — | 背景渐变色 |
| `MissionMenuItem.SetMaxRPColor` | `color1: List`, `color2: List` | — | RP渐变色 |
| `MissionMenuItem.SetStages` | `stages: List` | — | 设置适用场景 |

### 10.6 Area（区域）

| 节点 | 参数 | 返回 | 说明 |
| ------ | ------ | ------ | ------ |
| `CreateArea` | `type: String`, `stage: String`, `position: List`, `r: Number`, `h: Number`, `outline: Boolean`, `compass: Boolean/String` | `Object` | 创建区域 |
| `Area.SetVisible` | `visible: Boolean` | — | 设置可见 |
| `Area.SetColor` | `color: List` | — | 设置颜色 |
| `Area.SetOutline` | `visible: Boolean` | — | 设置轮廓 |
| `Area.SetCompass` | `visible: Boolean` | — | 设置指南针 |
| `Area.Inside` | `position: List` | `Boolean` | 是否在区域内 |
| `Area.Distance` | `position: List` | `Number` | 到区域距离 |

### 10.7 Zone（地带）

| 节点 | 参数 | 返回 | 说明 |
| ------ | ------ | ------ | ------ |
| `CreateZone` | `areas: List` | `Object` | 创建地带（多区域组合） |
| `Zone.SetVisible` / `SetColor` | 同 Area | — | 同 Area |
| `Zone.Inside` | `position: List` | `Boolean` | 是否在地带内 |
| `Zone.DistanceToLastPosition` | `position: List` | `Number` | 到最近区域距离 |
| `Zone.DistanceToNearest` | `position: List` | `Number` | 到最近区域距离 |

### 10.8 Condition（条件）

| 节点 | 参数 | 返回 | 说明 |
|------|------|------|------|
| `CreateCondition` | `condition: String`, `id: String` | `Object` | 创建条件对象 |
| `Condition.Check` | — | `Boolean` | 检查条件是否满足 |

### 10.9 ItemCondition（物品条件）

| 节点 | 参数 | 返回 | 说明 |
|------|------|------|------|
| `CreateItemCondition` | `itemtype: String`, `zone: Object`, `area: Object`, `id: String` | `Object` | 创建物品条件 |
| `ItemCondition.Check` | — | `Boolean` | 检查条件 |

### 10.10 InteractArea（交互区域）

| 节点 | 参数 | 返回 | 说明 |
|------|------|------|------|
| `CreateInteractArea` | `stage: String`, `position: List`, `r: Number`, `text: String`, `options: List` | `Object` | 创建交互区域 |
| `InteractArea.Check` | — | `Boolean/Number` | 检查交互状态 |

### 10.11 Text（文本）

| 节点 | 参数 | 返回 | 说明 |
| ------ | ------ | ------ | ------ |
| `CreateText` | — | `Object` | 创建文本对象 |
| `Text.SetFace` / `SetOutline` / `SetUnderlay` | 颜色/宽度/偏移等 | — | 设置文本样式 |
| `Text.SetAnchor` | `x: Number`, `y: Number` | — | 设置锚点 |
| `Text.SetAlignment` | `alignment: String` | — | 设置对齐 |
| `Text.SetSize` / `SetWidth` | `size: Number` | — | 设置大小/宽度 |
| `Text.SetFrontLayer` | `front: Boolean` | — | 是否前置层 |
| `Text.Clear` | — | — | 清空队列 |
| `Text.Add` | `text: String`, `delay: Number`, `fadein: Number`, `fadeout: Number`, `duration: Number`, 样式参数... | — | 添加文本到队列 |

### 10.12 Messenger Chat（即时通讯）

| 节点 | 参数 | 返回 | 说明 |
| ------ | ------ | ------ | ------ |
| `CreateMessengerChat` | `title: String`, `iconText: String`, `iconTextColor: List`, `iconColor: List`, `iconFilename: String` | `Object` | 创建聊天 |
| `Messenger.Add` | `text: String`, `orientation: String`, `user: String`, `userColor: List`, `silent: Boolean` | — | 添加消息 |
| `Messenger.Clear` | — | — | 清空 |
| `Messenger.SetButtons` | `captions: List`, `ids: List` | — | 设置按钮 |
| `Messenger.Clicked` | — | `Number/Value` | 获取点击的按钮 |
| `Messenger.Opened` | — | `Boolean` | 是否打开 |

### 10.13 Audio（音频）

| 节点 | 参数 | 返回 | 说明 |
| ------ | ------ | ------ | ------ |
| `CreateAudio` | `filePath: String` | `Object` | 创建音频 |
| `Audio.Play` | `volume: Number`, `position: List` | — | 播放 |
| `Audio.Length` | — | `Number` | 获取长度（秒） |

### 10.14 Gallery（图库）

| 节点 | 参数 | 返回 | 说明 |
| ------ | ------ | ------ | ------ |
| `CreateGallery` | `callback: String`, `condition: Object`, `area: Object`, `zone: Object` | `Object` | 创建图库 |
| `Gallery.Show` | `multiselect: Boolean` | — | 显示图库 |
| `Gallery.Confirmed` | — | `Boolean` | 是否确认选择 |
| `Gallery.GetSelection` | — | `List` | 获取选中项 |

### 10.15 Snapshot（快照）

| 节点 | 参数 | 返回 | 说明 |
|------|------|------|------|
| `CreateSnapshot` | `position: List`, `direction: List`, `width: Number`, `height: Number`, `fov: Number` | `Object` | 创建快照相机 |
| `Snapshot.Save` | `hidden: Boolean` | `String` | 保存并返回引用 |

### 10.16 NPC

| 节点 | 参数 | 返回 | 说明 |
| ------ | ------ | ------ | ------ |
| `CreateNPC` | `avatarType: String`, `position: List`, `rotation: List`, `body: Number`, `hair: Number`, `face: Number`, `size: Number` / `id: Number` | `Object` | 创建/连接NPC |
| `NPC.IsAlive` | — | `Boolean` | 是否存活 |
| `NPC.Respawn` | `position: List`, `rotation: List` | — | 重新生成 |
| `NPC.Warp` | `position: List`, `rotation: List` | — | 瞬移 |
| `NPC.AddWaypoint` | `position: List`, `rotation: List`, `waypointIndex: Number`, `last: Boolean` | — | 添加路径点 |
| `NPC.ClearWaypoints` | — | — | 清除路径点 |
| `NPC.Stopped` | `stopped: Boolean` | `Boolean` | 停止/恢复行走 |
| `NPC.Finished` | — | `Boolean` | 是否到达终点 |
| `NPC.GetID` / `GetType` | — | `Number` / `String` | 获取ID/类型 |
| `NPC.Strangeness` | `value: Number` | `Number` | 警觉度 |
| `NPC.SkipStrangeness` | `skip: Boolean` | `Boolean` | 是否跳过警觉 |
| `NPC.Smartphone` / `Headset` / `Glasses` | `value: Boolean` | `Boolean` | 装备状态 |
| `NPC.Size` / `Speed` | `value: Number` | `Number` | 大小/速度 |
| `NPC.Penis` / `PenisScale` | `state: Number` / `scale: Number` | `Number` | 阴茎状态/缩放 |
| `NPC.SeesPlayer` / `SeesFlashing` | — | `Boolean` | 是否看到玩家 |
| `NPC.GetPosition` | — | `List` | 获取位置 |
| `NPC.GetState` | — | `String` | 获取状态 |
| `NPC.ActivateSex` | — | — | 激活性爱 |
| `NPC.TracePlayer` | `value: Boolean` | `Boolean` | 追踪玩家 |
| `NPC.CanGameOver` | `value: Boolean` | `Boolean` | 是否可游戏结束 |
| `NPC.PlayAction` | `actionName: String` | — | 播放动作 |

### 10.17 Input（输入）

| 节点 | 参数 | 返回 | 说明 |
| ------ | ------ | ------ | ------ |
| `CreateInput` | `button: String`, `modifier: String`, `interaction: String` | `Object` | 创建输入检测 |
| `Input.WasPressed` | — | `Boolean` | 本帧是否按下 |
| `Input.WasReleased` | — | `Boolean` | 本帧是否释放 |
| `Input.WasPerformed` | — | `Boolean` | 本帧是否触发（含interaction） |
| `Input.IsPressed` | — | `Boolean` | 当前是否按住 |

---

## 十一、特殊节点（编译期/元数据）

特殊节点用于编辑器内部或编译期处理，不直接对应运行时的 API 调用，但对序列化/代码生成有重要影响。

- `Meta`：用于承载节点级别的元数据（仅编辑器使用），例如注释、作者、版本等，序列化时应放入 `meta` 字段。
- `Comment`：纯注释节点，编辑器应在导出时忽略此节点，但在 JSON 中保留以便编辑体验（`comments` 字段或 `params` 中的注释键）。
- `Group`：用于在画布上对节点分组（视觉分组），导出时不影响生成的 `.code`，但应保留位置信息以便还原视图。

实现建议：

- 编辑器在序列化时对特殊节点做显式处理，避免将其误认为可执行节点。
- 验证器在检查节点类型时，应允许特殊节点存在，但不将其纳入拓扑排序或代码生成路径。
- `Comment` 节点可保留在顶层 `comments` 数组中，也可作为 `nodes` 数组中的特殊类型；无论哪种方式，导出 `.code` 时均应忽略。
- `Group` 节点仅影响画布视觉组织，不进入序列化逻辑（可保存为 `nodes` 中的分组元数据，不影响执行语义）。

---

## 附注

本文档作为节点类型的活文档（living document），未来会根据 CustomMissions2 API 的变更进行更新。如需补充或纠正某个节点的参数/端口定义，请提交 Issue 或 PR。

---

## 十二、节点类型颜色编码（建议）

颜色用于编辑器节点标题栏，帮助用户快速识别节点类别。

| 分类 | 颜色 | 颜色值（RGBA） | 节点示例 |
| ------ | ------ | --------------- | --------- |
| 控制流 | 紫色 | `#9C27B0` | Start, If, While, Goto |
| 通用函数 | 蓝色 | `#2196F3` | Log, CallFunction, Range |
| 游戏动作 | 绿色 | `#4CAF50` | DropItem, SetStage, PlaySoundEffect |
| 数值操作 | 橙色 | `#FF9800` | AddCurrentRP, SetEcstasy, GetStamina |
| 对象构造 | 青色 | `#00BCD4` | CreateArea, CreateNPC, CreateThread |
| 对象方法 | 浅蓝 | `#03A9F4` | Area.Inside, NPC.Warp, Text.Add |
| 数学 | 灰色 | `#607D8B` | Random, Sin, Vector3Add |
| 字符串 | 粉色 | `#E91E63` | Format, SubString, ToNumber |
| 文件 | 棕色 | `#795548` | FileExists, GetFiles |
| 等待/事件 | 黄色 | `#FFEB3B` | Wait, WaitForEvent, CreateListener |
| 特殊 | 灰色 | `#757575` | Meta, Comment, Group |

> 颜色定义同时应写入 `src/ui/theme.rs` 或 `assets/themes/default.json`，保持代码与文档一致。`src/ui/theme.rs` 顶部必须包含注释：
> ```rust
> // 颜色来源：docs/node_types.md 第 12 节（节点类型颜色编码表）
> // 若修改本文档颜色，必须同步更新 src/ui/theme.rs 与 assets/themes/default.json
> ```

---

## 十三、版本说明

- 本清单基于 API 文档 v1.0 梳理。
- 新增 API 函数时，按相同格式追加节点定义即可；同时更新 `src/api/definitions.rs` 与 [src/graph/types.rs](../src/graph/types.rs) 中的 `NodeType` 枚举。
- 节点参数中的可选值用 `[]` 标出，编辑器 UI 中应体现为「非必填」。
- 若某节点同时支持读写（如 `Global` / `Local` / `NPC.Stopped`），其参数 `value` 为空时表示读取，非空时表示写入。
- **计数口径**：`NodeType` 枚举当前包含 **143 个变体**，统计范围为可直接实例化的节点类型（控制流、通用函数、游戏函数、数学/字符串/文件函数、对象构造函数、特殊节点）。
- **对象方法映射**：第 10 节中形如 `Area.Inside`、`NPC.Warp`、`Text.Add` 的对象方法不单独映射为 `NodeType` 枚举变体；运行时通过 `(ObjectType, MethodName)` 组合或 `CallMethod` 节点动态调用。若未来需将对象方法提升为独立节点类型，必须同步更新 `NodeType` 枚举、`api/definitions.rs` 注册表与本文档第 10 节。
