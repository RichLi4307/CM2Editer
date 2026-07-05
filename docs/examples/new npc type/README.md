# 示例任务：新 NPC 类型（New NPC Type）

## 文件说明

| 文件 | 作用 |
|------|------|
| [main.code](main.code) | 任务脚本源码，包含三个并发线程：主线程、行走线程、全局陌生感控制线程 |
| [meta.json](meta.json) | 任务元数据，供游戏加载器读取，用于任务选择界面和设置面板 |
| [README.md](README.md) | 本文档，解释示例结构与 [meta.json](meta.json) 字段含义 |

---

## [meta.json](meta.json) 字段详解

```json
{
	"title": {
		"En": "新NPC类型"
	},
	"description": {
		"En": "settings"
	},
	"settings": [
		{
			"name": "blind",
			"title": "NPC陌生感",
			"type": "Boolean",
			"default": false
		},
		{
			"type": "Label",
			"title": "Deactivate \"strangeness\" for all NPCs including those created by the game"
		}
	],
	"defaultactive": false
}
```

### 顶层字段

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `title` | Object | 是 | 多语言任务标题，键为语言代码（`En`、`Ja` 等），值为显示文本 |
| `description` | Object | 是 | 多语言任务描述，结构与 `title` 相同 |
| `settings` | Array | 否 | 玩家可在游戏内调整的设置项列表 |
| `defaultactive` | Boolean | 否 | 任务默认是否在游戏内激活；`false` 表示需要玩家手动开启 |

### `settings` 数组项

`settings` 支持两种条目类型：

#### 1. 可配置参数（`Boolean` / `Integer` / `Number` / `String` 等）

| 字段 | 类型 | 说明 |
|------|------|------|
| `name` | String | 参数在脚本中的访问名，如 `_settings.blind` |
| `title` | String | 显示给玩家的设置名称 |
| `type` | String | 参数类型，如 `Boolean`、`Integer`、`Number`、`String` |
| `default` | 任意 | 默认值，类型需与 `type` 一致 |
| `minvalue` | Number | 数值类型时的最小值（可选） |
| `maxvalue` | Number | 数值类型时的最大值（可选） |

#### 2. 说明标签（`Label`）

| 字段 | 类型 | 说明 |
|------|------|------|
| `type` | String | 固定为 `Label` |
| `title` | String | 在设置面板中显示的说明文本 |

---

## 脚本执行流程

```text
启动
 │
 ├── CreateThread("main")  ─────────┐
 │                                  │
 ├── CreateThread("walk")  ─────────┤── 三个线程并发运行
 │                                  │
 └── CreateThread("blind") ─────────┘
```

### `main` 线程

1. 定义 6 种 NPC 类型（新女性 / 新男性 / 旧女性 / 旧男性 / 新大妈 / 新大爷）及其外观参数。
2. 在 Residence 场景创建 `Spawn NPCs` 交互区，选项为上述 6 种类型。
3. 创建 3 个独立的装备切换交互区（智能手机 / 耳机 / 眼镜）。
4. 每帧监听交互：
   - 玩家选择类型后，生成对应数量的 NPC；
   - 玩家触发装备切换后，遍历所有已生成 NPC 取反对应装备状态。

### `walk` 线程

1. 在场景另一侧创建 `Spawn Walking NPC` 交互区。
2. 玩家交互后生成一名 `NewMale` NPC 并为其设置路径点。
3. 每帧根据玩家与 NPC 的距离动态控制 NPC 的停止/行走状态。
4. 当 NPC 死亡时自动重生并重新设置路径点。

### `blind` 线程

1. 监听 `_settings.blind` 设置。
2. 当开启时，为所有 `_state.NPCs` 中 ID 小于 1,000,000 的 NPC 创建引用并禁用陌生感。
3. 当切换场景时清空已跟踪的 NPC 列表。
4. 当设置值变化时，同步更新所有已跟踪存活 NPC 的陌生感状态。

---

## 关键 API 回顾

| API | 作用 |
|-----|------|
| `CreateThread("label")` | 创建并发线程，入口为指定标签 |
| `CreateListener("label", ...)` | 每帧循环执行标签内代码 |
| `CreateList()` | 创建空列表（数组/字典） |
| `List.Insert(value)` | 在列表末尾插入元素 |
| `List.Count()` | 返回列表元素数量 |
| `CreateNPC(type, position, rotation, ...)` | 创建 NPC |
| `CreateInteractArea(...)` | 创建球形交互区域 |
| `InteractArea.Check()` | 检查交互状态，返回选中索引或布尔值 |
| `NPC.SkipStrangeness(bool)` | 禁用/启用 NPC 的陌生感系统 |
| `NPC.AddWaypoint(...)` | 为 NPC 添加路径点 |
| `NPC.Stopped(bool)` | 控制 NPC 是否停止行走 |
| `_state.Position` | 玩家当前位置 |
| `_state.NPCs` | 当前场景中所有 NPC 列表 |
| `_settings.xxx` | 读取 [meta.json](meta.json) 中定义的设置项 |
| `_stagechanged` | 布尔值，表示当前帧是否切换了场景 |

---

## 注意事项

1. **线程隔离**：三个线程拥有独立作用域，同名变量（如 `npcs`）在不同线程中互不干扰。
2. **监听器语义**：`CreateListener` 会每帧调用标签内代码，相当于一个 `while(true)` 循环，因此内部逻辑应尽量轻量。
3. **NPC 引用**：`CreateNPC(id)` 可通过已有 NPC 的 ID 创建引用对象，从而调用其方法。
4. **ID 过滤**：`id < 1000000` 是一种约定，用于区分游戏生成的基础 NPC 与玩家/任务创建的特殊 NPC。
5. **场景切换**：`_stagechanged` 为 `true` 时务必清理场景相关的对象引用，避免跨场景访问已销毁对象。
