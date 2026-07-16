# .code 语法覆盖缺口审计报告

> 日期：2026-07-16
> 方法：子代理对照官方文档（`docs/kb/documentation_part_002/003/004/006.md`）与 `src/api/definitions.rs`、`src/code_gen/generator.rs`、`src/graph/types.rs` 逐项比对。
> 结论：168 个 NodeType 覆盖约 139 个语法要素（约 90%）+ 约 72 个对象方法可通过 CallMethod 间接调用。真正"无法表达"的缺口只有 **EventListener** 与 **StopAudio** 两处。

---

## 概览统计

| 类别 | 官方要素数 | 已覆盖 | 仅 CallMethod | 缺失 |
|------|-----------|--------|--------------|------|
| 控制流 | 8 | 8 | 0 | 0 |
| 全局函数 | 17 | 14 | 0 | 3 |
| 游戏函数 | 45 | 44 | 0 | 1 |
| 音频 | 1 | 0 | 0 | 1 |
| 数学/向量/字符串/文件 | 40 | 40 | 0 | 0 |
| 对象构造函数 | 18 | 18 | 0 | 0 |
| 全局变量 | 9 | 7 | 0 | 2 |
| List 方法 | 6 | 0 | 6 | 0 |
| 对象方法（NPC/MissionPanel/MissionMenuItem/Area/Zone/Text/MessengerChat/Audio/Gallery/Snapshot/Input） | 72 | 0 | 72 | 0 |
| EventListener | 2 | 0 | 0 | 2 |
| Thread.GetLabel | 1 | 0 | 0 | 1 |
| 语法结构（elseif、+= 等） | 7 | 0 | 0 | 7 |

---

## P0 — 无法绕行 / 生成质量受损

| 缺口 | 官方签名 / 来源 | 影响 | 建议 |
|------|----------------|------|------|
| **CreateEventListener / CreateEventListenerLocal** | `CreateEventListener(LabelName, EventName[, params...])`（kb part_004:204） | 事件驱动监听器（SetEvent 触发、注入 `__eventdata_`/`__eventname_`）无法用每帧轮询的 Listener 替代 | 新增 EventListener 节点，eventName 参数 + `__eventdata_`/`__eventname_` 输出端口 |
| **StopAudio** | `StopAudio(InstanceID[, FadeOutTime])`（kb part_003:1764） | 全局函数，CallMethod 无法表达（不是对象方法） | 新增节点 |
| **_stagechanged / _name 全局变量** | kb part_002:118 / :140 | 监听器中检测场景切换的常用手段，当前完全无法表达 | 新增 GetStageChanged / GetProjectName 数据节点 |
| **elseif 关键字** | kb part_003:66 | 多路分支生成嵌套 if，合法但可读性差 | 生成器优化：False 分支首节点为 If 时折叠为 elseif（见 P3-5） |
| **复合赋值 +=/-=/*=//=** | kb part_002:166 | 只能用 `i = i + 1` 模拟 | SetVariable 增加 op 参数 |
| **TriggerSexOrgasm** | kb part_003:1687 | 需组合 SetEcstasy(1)+SetAction 模拟，非原子语义 | 新增简单节点 |

## P1 — 常用 API 缺专用节点（目前靠 CallMethod / CallFunction 手输）

| 缺口 | 说明 | 建议优先级 |
|------|------|-----------|
| **Translate(Key[, params...])** | 本地化核心函数，UI 文本高频使用 | 高 |
| **Warning / Error** | 日志三级别只做了 Log | 高（给 Log 加 level 枚举参数即可） |
| **FunctionExists** | 跨 mod 防御性调用 | 中 |
| **GetModVersion([ModGUID])** | mod 依赖检查 | 中 |
| **Thread.GetLabel()** | 状态机自检当前位置 | 中 |
| **List 方法 ×6** | Insert/Remove/Count/Contains/IndexOf/Keys，List 是核心集合类型 | 高 |
| **NPC 方法 ×22** | Warp/AddWaypoint/IsAlive/SeesPlayer/SeesFlashing 等高频方法全得手输 | 高（先做 5 个高频） |

## P2 — 冷门 API，CallMethod 可接受

MissionPanel（10）、MissionMenuItem（16）、Area（5）、Zone（5）、Text（10）、MessengerChat（4）、Gallery（3）、Snapshot.Save、Audio.Play/Length、Input（4）、InteractArea.Check（动态返回类型难以静态表达）。

---

## P3 — "加轮子"结构性优化点

### P3-1 对象方法生态全靠 CallMethod 兜底（架构取舍）
50+ 个对象方法需用户记忆大小写敏感的方法名与参数顺序，是出错主要来源。**轮子建议**：做"对象方法选择器"——选中 CallMethod 节点后按对象类型弹出方法下拉+参数模板，而不是逐个加节点。

### P3-2 `_state` 嵌套属性探针
GetStateBool/Number 只暴露扁平硬编码键。`_state.Position.x`、`_state.Camera.pitch`、`_state.Handcuffs.Type`、`_state.Exposed.All`、`_state.Cosplay.<name>`、`_state.NPCs[n].*` 等嵌套路径只能靠 Variable 手写字符串。**轮子建议**：`_state` 路径浏览节点（树形选择器，类似命名空间选择器），输出类型安全。

### P3-3 CreateArea 缺 cuboid 参数集
官方支持 sphere/cylinder/cuboid 三种，当前只有 x/y/z/r/h。cuboid 需 x1,y1,z1,x2,y2,z2,w,h。

### P3-4 For 循环体验
For+Range 需要手动连线。**轮子建议**：For 节点直接提供 start/stop/step 参数，无 iterable 连线时自动包装 `Range()`。

### P3-5 elseif 折叠
False 分支首节点是 If 且无其他入度时生成 `elseif` 而非嵌套。非功能性优化，提升生成代码可读性。

### P3-6 用户定义函数语义
当前"函数"= 某线程里的普通 Label + CallFunction 引用。**轮子建议**：LabelContainer 加"函数"标记（参数列表元数据），属性面板可视化定义参数，调用处 CallFunction 参数自动带名。

### P3-7 条件 DSL 已部分解决
CreateCondition 组合编辑器已落地（6.5 节）。后续可加：表达式实时校验（括号配平、token 合法性）、常用条件模板收藏。

### P3-8 GetKeys/GetValues 存疑
`code_api_reference.md` 提到的 `GetValues()` 在官方 Objects 文档中不存在，可能非官方 API，暂不实现。

---

## 优先级路线图（建议）

1. **P0 修复**（小工作量、解除"无法表达"）：EventListener 节点、StopAudio、_stagechanged/_name、TriggerSexOrgasm
2. **P1 高频节点**：Log level 枚举、Translate、List 六方法、NPC 五高频方法
3. **生成器优化**：elseif 折叠、SetVariable 复合赋值、For 自动包装 Range
4. **体验轮子**：_state 探针选择器、CallMethod 对象方法下拉、CreateArea cuboid
