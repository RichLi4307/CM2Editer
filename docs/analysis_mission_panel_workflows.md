# MissionPanel / MissionMenuItem 工作流程与 UX 问题分析

> 来源：对 `docs/examples/失控的代价/` 真实 `.code` 文件的逆向分析
> 日期：2026-07-23
> 范围：MissionPanel 构造函数、配置方法、生命周期、数据流，及与当前编辑器节点设计的差距

---

## 1. 官方 API 语义（依据 `docs/kb/documentation_part_004.md`）

两个对象均由**无参数构造函数**创建，返回 Object，后续通过方法调用配置。

| 对象 | 构造函数 | 方法 |
|------|----------|------|
| **MissionPanel** | `CreateMissionPanel()` | `SetText(Text)`, `SetRPText(Text)`, `SetVisible(Visible)`, `SetGaugeVisible(Visible)`, `SetGaugeProgress(Progress)`, `GetText()`, `GetRPText()`, `GetVisible()`, `GetGaugeVisible()`, `GetGaugeProgress()` |
| **MissionMenuItem** | `CreateMissionMenuItem()` | `SetText(Text)`, `GetText()`, `SetRPText(Text)`, `GetRPText()`, `SetCleared(ClearedMarker)`, `GetCleared()`, `SetMark(Mark)`, `GetMark()`, `SetClears(Clears)`, `GetClears()`, `SetMaxRP(MaxRP)`, `GetMaxRP()`, `AutoColor(Value)`, `SetBackgroundColor(Color1, Color2)`, `SetMaxRPColor(Color1, Color2)`, `SetStages(Stages)` |

**结论**：构造函数本身没有入口参数。面板创建后必须通过返回的对象再调用方法配置。

---

## 2. 关键代码样本（从 `失控的代价` 逆向）

### 2.1 模式 A：系统全局面板（通知系统）

`listener_noice.code` 第 3-6 行、第 47-48 行、第 57 行：

```code
tempPanel = CreateMissionPanel()
tempPanel.SetGaugeVisible(false)
tempPanel.SetVisible(false)
Global("notice_panel", tempPanel)
```

```code
notice_panel.SetText(currentText)
notice_panel.SetVisible(true)
```

```code
notice_panel.SetVisible(false)
```

- 创建 1 个面板，隐藏，无计量器，存入全局变量。
- 通过专用 `listener_notice` 每帧检查通知数据，触发时更新文本并显示。
- 超时后隐藏。无 `Destroy`。

### 2.2 模式 B：监听器局部持久面板（多 HUD 面板）

`common.code` 第 308-327 行：

```code
detectionPanel = CreateMissionPanel()
detectionPanel.SetText(text_about_to_be_exposed)
detectionPanel.SetVisible(false)
detectionPanel.SetGaugeVisible(false)

enableCamoPanel = CreateMissionPanel()
enableCamoPanel.SetText(text_about_to_be_camo)
enableCamoPanel.SetVisible(false)
enableCamoPanel.SetGaugeVisible(false)

camoValuePanel = CreateMissionPanel()
camoValuePanel.SetText(text_about_to_be_camo_value)
camoValuePanel.SetVisible(false)
camoValuePanel.SetGaugeVisible(false)

cooldownPanel = CreateMissionPanel()
cooldownPanel.SetText(text_about_to_be_camo_cool_down)
cooldownPanel.SetVisible(false)
cooldownPanel.SetGaugeVisible(false)
```

同一监听器中创建 4 个独立面板，作为 HUD 元素：
- 探测面板、伪装启动面板、伪装数值面板、冷却面板。
- 均在 `isInit` 中创建，在监听器生命周期内复用。

### 2.3 模式 C：标签局部一次性面板（剧情提示）

`main.code` 第 84-87 行：

```code
panel = CreateMissionPanel()
panel.SetVisible(true)
panel.SetGaugeVisible(false)
panel.SetText(text_notice_sub_warring)
```

- 在 `prologue_cp2_threat_failed` 标签中创建。
- 创建后立即显示，设置文本，**不调用 `SetVisible(false)`**。
- 当线程 `Goto` 跳转到其他标签后，局部变量自然离开作用域。

### 2.4 模式 D：带进度条的动态面板

`common.code` 第 238 行：

```code
panel.SetGaugeProgress(totalDistance / requiredDistance)
```

`listener_prologue_cp2_punish` 第 1043-1052 行：

```code
detectionPanel.SetVisible(true)
detectionPanel.SetGaugeProgress(detectionDuration / 2.5)
detectionPanel.SetGaugeVisible(true)
...
detectionPanel.SetVisible(false)
detectionPanel.SetGaugeProgress(0)
```

- 进度条在 Listener 中每帧更新。
- 条件满足时显示并更新进度；条件结束隐藏并清零。

### 2.5 唯一硬编码中文文本

`listener_prologue_cp2_punish` 第 1084-1085 行：

```code
missionPanel.SetVisible(true)
missionPanel.SetText("失控的代价：\n1分钟之内，保持至少一种暴露状态，然后出门！")
```

其余所有文本均使用翻译引用（`text_*`）。

---

## 3. 结构化流程发现

### 3.1 三种生命周期模式

| 模式 | 存储方式 | 作用域 | 清理方式 | 示例 |
|------|----------|--------|----------|------|
| **系统全局通知** | `Global("notice_panel", panel)` | 跨线程/监听器 | `SetVisible(false)` | `listener_noice.code`, `end.code` |
| **监听器局部持久** | 监听器局部变量 | 单个监听器 | `SetVisible(false)` | `common.code`, `listener_prologue.code` |
| **标签局部一次性** | 标签局部变量 | 单个标签 | `Goto` 后自然离开作用域 | `main.code` |

### 3.2 文本来源分布

- 翻译引用：占绝对多数（`text_about_to_be_exposed`, `text_qa_choose`, `text_notice_sub_warring` 等）。
- 硬编码字符串：仅 1 处，用于快速测试/临时提示。
- 变量动态文本：通知系统从 `notice_data.text` 读取。

### 3.3 多面板共存

`common.code` 一个监听器中同时存在 4 个面板，`listener_prologue_cp2_punish` 中同时存在 `missionPanel` + `detectionPanel`。

MissionPanel 不是单一任务追踪器，而是**多个 HUD 信息面板**。

### 3.4 变量命名约定

常见命名：`panel`, `missionPanel`, `detectionPanel`, `enableCamoPanel`, `camoValuePanel`, `cooldownPanel`, `notice_panel`, `ending_notice_panel`, `skipPanel`, `checkPanel`。

---

## 4. 当前节点设计与真实代码的差距

### 4.1 方法注册表错误（`src/api/method_registry.rs`）

#### MissionPanel

| 当前注册表 | 官方 API | 真实示例 | 问题 |
|------------|----------|----------|------|
| `SetTitle` | ❌ 不存在 | 使用 `SetText` | 名称错误，用户找不到正确方法 |
| `SetProgress` | ❌ 不存在 | 使用 `SetGaugeProgress` | 名称错误 |
| `SetVisible` | ✅ | ✅ 常用 | 正确 |
| `Destroy` | ❌ MissionPanel 上不存在 | 从未使用 | 多余，会生成无效代码 |
| 缺失 `SetText` | ✅ | 最常用 | 无法配置面板文本 |
| 缺失 `SetGaugeVisible` | ✅ | 常用 | 无法显示/隐藏进度条 |
| 缺失 `SetGaugeProgress` | ✅ | 常用 | 无法更新进度 |
| 缺失 `SetRPText` | ✅ | 示例未使用 | 可低优先级补齐 |
| 缺失所有 Getter | ✅ | 示例未使用 | 可低优先级补齐 |

#### MissionMenuItem

| 当前注册表 | 官方 API | 问题 |
|------------|----------|------|
| `SetText` | ✅ | 正确 |
| `SetCompleted` | ❌ 应为 `SetCleared` | 名称错误 |
| 缺失 `GetText`, `SetRPText`, `GetRPText`, `SetCleared`, `GetCleared`, `SetMark`, `GetMark`, `SetClears`, `GetClears`, `SetMaxRP`, `GetMaxRP`, `AutoColor`, `SetBackgroundColor`, `SetMaxRPColor`, `SetStages` | ✅ | 全部缺失 |

### 4.2 缺少节点 / 方法入口

当前编辑器只有：
- `CreateMissionPanel`（创建对象）
- `CreateMissionMenuItem`（创建对象）
- `CallMethod`（动态调用）

没有为 MissionPanel 高频方法提供第一公民节点：
- `MissionPanel.SetText`
- `MissionPanel.SetVisible`
- `MissionPanel.SetGaugeVisible`
- `MissionPanel.SetGaugeProgress`

用户创建面板后，必须手动用 `CallMethod` 节点，并知道方法名。这造成"创建面板后白创建了"的 UX 感受。

### 4.3 数据流设计问题

- 配置方法节点必须接收面板对象作为调用对象（`self`）。
- 当前 `CallMethod` 节点有 `thread` 参数（参数名误导，应为通用 `object`）。
- 参数如 `Text`、`Visible`、`Progress` 既应支持属性面板写死，也应支持从 data 端口动态传入。

### 4.4 生成代码不匹配风险

如果用户从当前注册表选 `SetTitle` 或 `SetProgress`，生成的 `.code` 在游戏加载器中会失败：

```code
panel.SetTitle("...")    // 错误：不存在
panel.SetProgress(0.5)   // 错误：不存在
```

---

## 5. UX 问题

### 5.1 创建面板后"什么都做不了"

`CreateMissionPanel` 节点只有 Flow 输入/输出和 Object 输出。用户拖出 Object 端口后，没有明显下一步节点。推荐方法没有出现在节点库中，或出现在错误名称下（`SetTitle`/`SetProgress`）。

### 5.2 方法节点调用对象不明确

`CallMethod` 节点的参数叫 `thread`，但实际上应传入任意对象。MissionPanel 用户必须知道：
1. 要用 `CallMethod`
2. 方法名是 `SetText`（不是 `SetTitle`）
3. 参数对象要连到 `thread` 端口

### 5.3 进度条更新难以表达

真实代码中进度条更新通常发生在 Listener 内：

```code
# 在 listener 中每帧：
panel.SetGaugeProgress(current / max)
```

节点图中需要：Listener 入口 → 读取状态 → 计算 → 调用 `SetGaugeProgress` → 循环。当前没有直观方式表达"每帧更新面板进度"。

### 5.4 多面板管理混乱

一个监听器中可能创建 4-6 个面板。节点图中需要：
- 清晰的变量命名（`detectionPanel`, `camoValuePanel`）
- 将面板存入局部变量或全局变量
- 多个面板状态同步更新

当前缺少局部变量节点和全局变量节点的高可见性入口。

### 5.5 翻译引用与字符串混合

真实代码使用翻译键（`text_*`），而编辑器当前可能让用户直接输入字符串。缺少：
- 翻译键选择器/提示
- 一键在 `text.code` 中注册新翻译键

---

## 6. 建议

### 6.1 立即修复方法注册表

1. MissionPanel：移除 `SetTitle`/`SetProgress`/`Destroy`，添加 `SetText`、`SetGaugeVisible`、`SetGaugeProgress`。
2. MissionMenuItem：将 `SetCompleted` 改为 `SetCleared`，补齐其余官方方法。
3. 补齐 Getter，但优先级低于 Setter。

### 6.2 提供第一公民配置节点

为高频方法添加专用节点（或至少在 `CallMethod` 下拉中正确列出）：
- `MissionPanel.SetText`
- `MissionPanel.SetVisible`
- `MissionPanel.SetGaugeVisible`
- `MissionPanel.SetGaugeProgress`
- `MissionPanel.SetRPText`

### 6.3 统一 `CallMethod` 参数命名

将 `CallMethod` 的 `thread` 参数改名为 `object` 或 `target`，避免用户误以为是线程专用。

### 6.4 支持对象链式调用/变量存储

在节点图中明确支持：
- 创建对象 → 存入局部变量/全局变量 → 后续节点读取变量并调用方法。
- 当前 `Local`/`Global` 节点已存在，但应优化 Object 类型支持。

### 6.5 增强 Listener 模板

提供"MissionPanel 进度条更新"模板：Listener 入口 + 读取状态 + 计算进度 + 调用 `SetGaugeProgress` + 循环，降低用户手动搭建成本。

### 6.6 翻译键集成

在字符串参数输入框中，支持 `text_*` 键名提示或选择器，方便用户遵循社区命名规范。

---

## 7. 附录：关键统计

- `docs/examples/失控的代价/` 中 MissionPanel 创建次数：**21 次**（6 个文件）。
- 全仓库 `MissionMenuItem` 在 `.code` 示例中出现次数：**0 次**。
- `SetText` 使用次数：**~85+ 次**。
- `SetVisible` 使用次数：**~100+ 次**。
- `SetGaugeProgress` 使用次数：**~30+ 次**。
- `SetGaugeVisible` 使用次数：**~35+ 次**。
- `SetRPText` 使用次数：**0 次**。
- `Destroy` 使用次数：**0 次**。

---

*本分析用于指导 MissionPanel / MissionMenuItem 节点与 UI 的下一步重构。*
