# CustomMissions2 流编辑器 — JSON 序列化格式规范

> 版本：v1.0  
> 用途：定义编辑器画布 ↔ JSON 文件的双向映射契约  
> 阅读对象：编辑器前端、Rust 后端、自定义任务加载器  
> 相关文档：
>
> - 节点清单：[node_types.md](node_types.md)
> - 项目骨架与 Rust 模块：[rust_project_skeleton.md](rust_project_skeleton.md)
> - Agent 开发约束：[agent_prompt.md](agent_prompt.md)

---

## 一、顶层结构

```json
{
  "version": "1.0",
  "meta": { ... },
  "nodes": [ ... ],
  "edges": [ ... ],
  "labels": { ... },
  "threads": [ ... ],
  "viewport": { ... }
}
```

| 字段 | 类型 | 必填 | 默认值 | 说明 |
| ------ | ------ | ------ | -------- | ------ |
| `version` | String | 是 | — | 格式版本，用于迁移兼容 |
| `meta` | Object | 否 | `{}` | 任务元数据（对应 [meta.json](meta.json)） |
| `nodes` | Array | 是 | — | 所有节点列表 |
| `edges` | Array | 是 | `[]` | 所有连线列表 |
| `labels` | Object | 否 | `{}` | 标签到节点 ID 列表的映射 |
| `threads` | Array | 否 | `[{main}]` | 线程定义（并发分支） |
| `viewport` | Object | 否 | 见下文 | 画布视口状态（视图层） |

> 注意：
>
> - `meta` 中的内容**不参与代码生成**，由加载器直接读取。
> - 当 `threads` 缺失时，加载器应自动推断为单线程 `main`。

## 二、Meta 对象

与文档中的 [meta.json](meta.json) 一一对应，编辑器直接透传。

```json
{
  "meta": {
    "title": {
      "En": "Test Mission",
      "Ja": "テストミッション"
    },
    "description": {
      "En": "This is a description"
    },
    "settings": [
      {
        "name": "range",
        "title": "Range Integer",
        "type": "Integer",
        "minvalue": 0,
        "maxvalue": 100,
        "default": 50
      },
      {
        "type": "Label",
        "title": "A descriptive label for the settings panel"
      }
    ],
    "defaultactive": true
  }
}
```

### 字段说明

| 字段 | 类型 | 必填 | 说明 |
| ------ | ------ | ------ | ------ |
| `title` | Object | 是 | 多语言标题，键为语言代码（如 `En`、`Ja`） |
| `description` | Object | 否 | 多语言描述 |
| `settings` | Array | 否 | 玩家可调设置项，见 [meta.json](meta.json) 示例 |
| `defaultactive` | Boolean | 否 | 任务是否默认激活；默认 `true` |

> 规则：`meta` 中的内容**不参与代码生成**，由加载器直接读取。

---

## 三、Node 对象

```json
{
  "id": "node_001",
  "type": "DropItem",
  "category": "Game Functions",
  "position": { "x": 200.0, "y": 150.0 },
  "size": { "width": 180.0, "height": 120.0 },
  "collapsed": false,
  "params": {
    "itemtype": "Coat",
    "stage": "Residence",
    "x": -26.60,
    "y": -0.10,
    "z": -120.0
  },
  "ports": {
    "inputs": [
      { "id": "in_flow", "type": "Flow", "label": "执行" }
    ],
    "outputs": [
      { "id": "out_flow", "type": "Flow", "label": "下一步" },
      { "id": "out_result", "type": "String", "label": "返回值" }
    ]
  }
}
```

### 字段说明

| 字段 | 类型 | 必填 | 默认值 | 说明 |
| ------ | ------ | ------ | -------- | ------ |
| `id` | String | 是 | — | 全局唯一标识，格式 `node_{uuid}` 或 `node_{index}` |
| `type` | String | 是 | — | 节点类型名，必须与 [node_types.md](node_types.md) 中的名称一致 |
| `category` | String | 否 | 从类型推导 | 分类，用于导入时归类 |
| `position` | Object | 是 | — | 画布坐标 `{x, y}`，单位像素 |
| `size` | Object | 否 | 按类型默认 | 节点尺寸 `{width, height}`，用于恢复折叠状态 |
| `collapsed` | Boolean | 否 | `false` | 是否折叠（宏节点/子图） |
| `params` | Object | 否 | `{}` | 节点参数键值对，键对应 API 参数名 |
| `ports` | Object | 否 | 由 `type` 推导 | 端口定义；可由节点类型注册表推导，导出时可选 |

### 参数值类型映射

| 编辑器中的值 | JSON 中的表示 | 对应的 Rust 类型 | 示例 |
| ------------- | -------------- | ----------------- | ------ |
| 数字 | Number | `f64` | `50`, `-26.6` |
| 字符串 | String | `String` | `"Coat"`, `"Residence"` |
| 布尔 | Boolean | `bool` | `true`, `false` |
| 列表 | Array | `Vec<ParamValue>` 或 `List` | `[-26.6, -0.1, -120]` |
| 对象 | Object | `HashMap<String, ParamValue>` | `{"r": 1, "g": 0, "b": 0}` |
| 颜色 | Array | `[f32; 4]` | `[1.0, 0.0, 0.0, 1.0]` |
| 向量 | Array | `[f32; 3]` | `[-26.6, -0.1, -120]` |
| 四元数 | Array | `[f32; 4]` | `[0, 0, 0, 1]` |
| 引用（其他节点输出） | Object | `ParamValue::Ref` | `{"ref": "node_002", "port": "out_result"}` |

> 颜色与向量在 JSON 中均使用数组，加载器通过参数元数据中的 `param_type` 区分语义。

---

## 四、Edge 对象

```json
{
  "id": "edge_001",
  "from": { "node": "node_001", "port": "out_flow" },
  "to": { "node": "node_002", "port": "in_flow" },
  "type": "Flow",
  "waypoints": [
    { "x": 300, "y": 200 },
    { "x": 350, "y": 200 }
  ]
}
```

### 字段说明

| 字段 | 类型 | 必填 | 默认值 | 说明 |
| ------ | ------ | ------ | -------- | ------ |
| `id` | String | 是 | — | 唯一标识，建议 `edge_{uuid}` |
| `from` | Object | 是 | — | 源 `{node, port}` |
| `to` | Object | 是 | — | 目标 `{node, port}` |
| `type` | String | 是 | — | `Flow` / `Data` |
| `waypoints` | Array | 否 | `[]` | 连线中间点（用户手动调整路径） |

### 连线规则（验证器检查项）

1. `from` 和 `to` 必须指向存在的节点和端口
2. `type` 为 `Flow` 时，两端端口类型必须均为 `Flow`
3. `type` 为 `Data` 时，两端数据类型必须兼容（同类型或可隐式转换，见 `PortType::is_compatible_with`）
4. 一个输入端口只能有一条入边（`Data` 类型）；`Flow` 类型输入端口可有多条入边
5. 不允许自环（`from.node == to.node`），除非显式使用 Loop 节点
6. 不允许重复边（同一 from 到同一 to）

---

## 五、Labels 对象

标签是代码中的入口点，映射到节点序列。

```json
{
  "labels": {
    "main": ["node_001", "node_002", "node_003"],
    "delayed1": ["node_004"],
    "delayed2": ["node_005"]
  }
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| 键 | String | 标签名（如 `main`, `m1`, `listener_1`） |
| 值 | Array | 该标签按顺序执行的节点 ID 列表 |

> 规则：
>
> - 每个 `.code` 文件至少有一个标签（默认 `main`）
> - 标签内的节点通过 `Flow` 边串联，标签之间通过 `Goto` / `CreateThread` 跳转
> - 如果一个节点不在任何标签中，视为孤立节点，验证器报 Warning

---

## 六、Threads 对象

支持并发线程的显式声明。每个线程对应一个 `CreateThread` 调用。

```json
{
  "threads": [
    {
      "id": "thread_001",
      "name": "main",
      "entry_label": "main",
      "parent": null,
      "auto_start": true
    },
    {
      "id": "thread_002",
      "name": "delay_manager",
      "entry_label": "delaymanager",
      "parent": null,
      "auto_start": false
    },
    {
      "id": "thread_003",
      "name": "listener_1",
      "entry_label": "l",
      "parent": "thread_002",
      "auto_start": false
    }
  ]
}
```

| 字段 | 类型 | 必填 | 默认值 | 说明 |
| ------ | ------ | ------ | -------- | ------ |
| `id` | String | 是 | — | 线程唯一标识 |
| `name` | String | 否 | `""` | 线程显示名称 |
| `entry_label` | String | 是 | — | 入口标签名，对应 `labels` 中的键 |
| `parent` | String/null | 否 | `null` | 父线程 ID，`null` 表示顶层线程 |
| `auto_start` | Boolean | 否 | `true` | 是否任务开始时自动启动 |

> 规则：
>
> - 子线程 `parent` 指向的线程结束时，子线程应随父线程一起清理。
> - 非自动启动线程需由 `CreateThread` 节点显式启动。

---

## 七、Viewport 对象

视图层状态，不影响逻辑，仅恢复编辑体验。

```json
{
  "viewport": {
    "x": 0.0,
    "y": 0.0,
    "zoom": 1.0,
    "grid_size": 20.0,
    "show_grid": true
  }
}
```

| 字段 | 类型 | 必填 | 默认值 | 说明 |
| ------ | ------ | ------ | -------- | ------ |
| `x` | Number | 否 | `0.0` | 画布中心 X 坐标 |
| `y` | Number | 否 | `0.0` | 画布中心 Y 坐标 |
| `zoom` | Number | 否 | `1.0` | 缩放比例，建议范围 `0.1` ~ `4.0` |
| `grid_size` | Number | 否 | `20.0` | 网格间距 |
| `show_grid` | Boolean | 否 | `true` | 是否显示网格 |

---

## 八、完整示例

```json
{
  "version": "1.0",
  "meta": {
    "title": { "En": "Delay Example" }
  },
  "nodes": [
    {
      "id": "node_start",
      "type": "Start",
      "position": { "x": 100, "y": 100 },
      "ports": {
        "outputs": [{ "id": "out_flow", "type": "Flow", "label": "开始" }]
      }
    },
    {
      "id": "node_log_init",
      "type": "Log",
      "position": { "x": 300, "y": 100 },
      "params": { "output": "DelayExample: Init" },
      "ports": {
        "inputs": [{ "id": "in_flow", "type": "Flow" }],
        "outputs": [{ "id": "out_flow", "type": "Flow" }]
      }
    },
    {
      "id": "node_delay_1",
      "type": "Wait",
      "position": { "x": 500, "y": 50 },
      "params": { "seconds": 5 },
      "ports": {
        "inputs": [{ "id": "in_flow", "type": "Flow" }],
        "outputs": [{ "id": "out_flow", "type": "Flow" }]
      }
    },
    {
      "id": "node_log_delayed",
      "type": "Log",
      "position": { "x": 700, "y": 50 },
      "params": { "output": "DelayExample: Delay 1" },
      "ports": {
        "inputs": [{ "id": "in_flow", "type": "Flow" }],
        "outputs": [{ "id": "out_flow", "type": "Flow" }]
      }
    }
  ],
  "edges": [
    {
      "id": "edge_1",
      "from": { "node": "node_start", "port": "out_flow" },
      "to": { "node": "node_log_init", "port": "in_flow" },
      "type": "Flow"
    },
    {
      "id": "edge_2",
      "from": { "node": "node_log_init", "port": "out_flow" },
      "to": { "node": "node_delay_1", "port": "in_flow" },
      "type": "Flow"
    },
    {
      "id": "edge_3",
      "from": { "node": "node_delay_1", "port": "out_flow" },
      "to": { "node": "node_log_delayed", "port": "in_flow" },
      "type": "Flow"
    }
  ],
  "labels": {
    "main": ["node_start", "node_log_init", "node_delay_1", "node_log_delayed"]
  },
  "threads": [
    {
      "id": "thread_main",
      "name": "main",
      "entry_label": "main",
      "parent": null,
      "auto_start": true
    }
  ],
  "viewport": {
    "x": 0, "y": 0, "zoom": 1.0
  }
}
```

---

## 九、版本迁移策略

| 版本 | 变更 | 兼容处理 |
| ------ | ------ | --------- |
| `1.0` | 初始版本 | — |
| `1.1` | 新增 `threads` | 旧文件无 `threads` 时，默认单线程 `main`，`auto_start=true`，`parent=null` |
| `1.2` | 新增 `node.size` | 旧文件无 `size` 时，按节点类型默认尺寸 |
| `1.3` | 新增 `viewport.grid_size` / `show_grid` | 旧文件无这些字段时，使用默认值 |

**规则**：

- 加载时：未知字段忽略，缺失字段使用默认值
- 保存时：始终写入最新版本格式
- 升级时：保留旧版本备份（`mission.json.bak`）
- 迁移逻辑实现于 `src/serializer/migration.rs`

---

## 十、验证器检查清单

加载 JSON 时必须验证；编辑器中也应在保存/导出前运行相同验证。

1. [x] `version` 存在且为支持的版本
2. [x] `nodes` 中每个节点 `id` 唯一
3. [x] `nodes` 中每个节点 `type` 在节点清单中存在
4. [x] `nodes` 中每个节点 `params` 符合对应节点类型的参数定义（类型、必填、选项范围）
5. [x] `edges` 中 `from.node` 和 `to.node` 均存在于 `nodes`
6. [x] `edges` 中 `from.port` 和 `to.port` 在对应节点中存在
7. [x] `edges` 中类型匹配规则满足
8. [x] `labels` 中每个节点 ID 存在于 `nodes`
9. [x] `labels` 中同一节点不重复出现在同一线性序列中
10. [x] `threads` 中 `entry_label` 存在于 `labels` 中
11. [x] 无环检测（`Flow` 边构成的图必须是有向无环图，除非显式 Loop 节点）
12. [x] 必填参数已填写（`params` 中无 `null` 且包含所有必填字段）
13. [x] 孤立节点检测（不在任何 `labels` 与任何 `Flow` 边中的节点报 Warning）
