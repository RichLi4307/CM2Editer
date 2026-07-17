# CM2Editer JSON Schema（新架构 v2.0）

> 本文档定义新架构下 `.code.json` 文件的序列化格式。
> 新架构以 **容器（ThreadContainer / LabelContainer / ListenerContainer）** 为骨架，替代旧版的扁平 `nodes + edges + labels` 模型。
> 旧版 v1.x 已归档：`docs/archive/json_schema.md`（将在新架构实现后补充迁移说明）。

---

## 版本声明

```json
{
  "version": "2.0"
}
```

- v2.0 与 v1.x **不兼容**。
- 旧版工程需通过迁移脚本转换到 v2.0。

---

## 顶层结构

```json
{
  "version": "2.0",
  "meta": {
    "name": "mission",
    "created_at": "2026-07-13T00:00:00Z",
    "author": "..."
  },
  "threads": [
    {
      "id": "thread-1",
      "name": "main",
      "variable_name": "var_main_thread",
      "auto_start": true,
      "labels": [...],
      "listeners": [],
      "position": {"x": 0, "y": 0}
    }
  ],
  "viewport": {
    "x": 0,
    "y": 0,
    "zoom": 1.0
  }
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `version` | string | `"2.0"` |
| `meta` | object | 工程元信息 |
| `threads` | array | 所有线程容器 |
| `viewport` | object | 画布视口状态 |

---

## ThreadContainer（线程容器）

```json
{
  "id": "thread-1",
  "name": "main",
  "variable_name": "var_main_thread",
  "auto_start": true,
  "entry_pin": {"x": 0, "y": 0},
  "labels": [
    {
      "id": "label-1",
      "name": "main",
      "params": [],
      "nodes": [...],
      "edges": [...],
      "entry_pin": {"x": 0, "y": 0},
      "position": {"x": 0, "y": 0}
    }
  ],
  "listeners": [],
  "position": {"x": 0, "y": 0}
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | string | 唯一标识符 |
| `name` | string | 线程名称，用于生成和显示 |
| `variable_name` | string | 生成的 `.code` 变量名，如 `var_main_thread` |
| `auto_start` | boolean | 是否在模块加载时生成 `CreateThread("main")` |
| `entry_pin` | object | 线程入口钉坐标（仅用于线程概览图） |
| `labels` | array | 该线程拥有的标签容器 |
| `listeners` | array | 该线程拥有的监听器容器 |
| `position` | object | 线程概览图中的位置 |

### 代码生成规则

- `auto_start: true` 的线程生成顶层语句：`variable_name = CreateThread("first_label_name")`。
- `auto_start: false` 的线程不生成顶层语句，仅由 `CreateThread` 节点运行时创建。
- 线程内所有标签共享同一个 `_this` 作用域。

---

## LabelContainer（标签容器）

```json
{
  "id": "label-1",
  "name": "main",
  "params": [
    {"name": "duration", "type": "Number", "default": 0}
  ],
  "nodes": [
    {
      "id": "node-1",
      "type": "Log",
      "position": {"x": 100, "y": 100},
      "params": {"message": "hello"},
      "ports": [...]
    }
  ],
  "edges": [
    {
      "id": "edge-1",
      "source": {"node": "node-1", "port": "out_flow"},
      "target": {"node": "node-2", "port": "in_flow"},
      "kind": "Flow"
    }
  ],
  "entry_pin": {"x": 0, "y": 0},
  "position": {"x": 0, "y": 0}
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | string | 唯一标识符 |
| `name` | string | 标签名，用于 `Goto` / `CreateThread` / `CreateListener` 引用 |
| `params` | array | 标签参数签名（进入标签时传入的命名参数） |
| `nodes` | array | 容器内节点 |
| `edges` | array | 容器内边 |
| `entry_pin` | object | 标签入口钉坐标 |
| `position` | object | 线程内部标签画布的位置 |

### 重要约束

- `Flow` 边只能连接同一 `LabelContainer` 内的节点。
- `Data` 边可以连接同一 `ThreadContainer` 内不同 `LabelContainer` 的节点（用于标签参数和返回值引用）。
- 标签名在同一 `ThreadContainer` 内必须唯一。
- 标签结束时如果没有显式 `Return`，自动生成 `_result = null`。

### 标签参数签名

```json
[
  {"name": "duration", "type": "Number", "default": 0},
  {"name": "message", "type": "String", "default": ""}
]
```

调用时生成：`labelname(duration=3, message="hi")`。

---

## ListenerContainer（监听器容器）

```json
{
  "id": "listener-1",
  "name": "check_status",
  "kind": "listener",
  "variable_name": "var_check_status_listener",
  "params": [],
  "nodes": [...],
  "edges": [...],
  "entry_pin": {"x": 0, "y": 0},
  "position": {"x": 0, "y": 200}
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | string | 唯一标识符 |
| `name` | string | 监听器回调标签名 |
| `kind` | string | `"listener"` 或 `"local_listener"` |
| `variable_name` | string | 生成的 `.code` 变量名 |
| `params` | array | 同 LabelContainer |
| `nodes` / `edges` | array | 同 LabelContainer |
| `entry_pin` | object | 入口钉坐标 |
| `position` | object | 位置 |

### 代码生成规则

- `listener`: `var_check_status_listener = CreateListener("check_status")`
- `local_listener`: `var_check_status_listener = CreateListenerLocal("check_status")`
- 监听器回调体与标签体结构相同，每帧/每秒被调用。

---

## Node（节点）

```json
{
  "id": "node-1",
  "type": "Log",
  "position": {"x": 100, "y": 100},
  "size": {"x": 180, "y": 120},
  "collapsed": false,
  "params": {"message": "hello"},
  "inputs": [
    {"id": "in_flow", "type": "Flow", "label": "执行", "required": true}
  ],
  "outputs": [
    {"id": "out_flow", "type": "Flow", "label": "下一步", "required": true}
  ],
  "dynamic_ports": {},
  "category": "General"
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | string | 节点唯一标识符 |
| `type` | string | `NodeType` 的 PascalCase 名称，如 `"Log"` |
| `position` | object | 节点在画布上的位置 `{x, y}` |
| `size` | object | 节点在画布上的尺寸 `{x, y}` |
| `collapsed` | boolean | 是否折叠 |
| `params` | object | 节点参数表，键为 API 参数名，值为字面量或引用 |
| `inputs` | array | 输入端口列表 |
| `outputs` | array | 输出端口列表 |
| `dynamic_ports` | object | 动态端口/参数分组状态，键为组 ID，值为端口/参数 ID 数组（可选，默认空） |
| `category` | string | 节点分类，用于 UI 着色 |

### 动态端口

`dynamic_ports` 记录运行时由用户扩展的端口/参数。每个键对应 `NodeDefinition.dynamic_ports` 中的一个组，值为该组当前实际存在的成员 ID。这些 ID 必须同时存在于 `inputs`、`outputs` 或 `params` 中。

```json
"dynamic_ports": {
  "elseif_branches": ["elseif_0", "elseif_1"],
  "format_args": ["arg_0", "arg_1"]
}
```

### 移除的节点

v2.0 中不再包含以下节点：

| 旧节点 | 替代方式 |
|--------|---------|
| `Start` | 由 `ThreadContainer` / `LabelContainer` 的 `entry_pin` 替代 |
| `Label` | 由 `LabelContainer.name` 替代 |

### 端口字段

`inputs` 与 `outputs` 数组中的每个端口对象包含以下字段：

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | string | 端口唯一标识（节点内） |
| `type` | string | 端口类型，如 `"Flow"`、`"Number"`、`"String"`、`"Boolean"`、`"List"`、`"Object"` |
| `label` | string | UI 显示文本 |
| `required` | boolean | 是否为必填端口 |

特殊端口 `in_flow` / `out_flow` 类型为 `"Flow"`，表示执行顺序；其他类型端口用于数据传递。

---

## Edge（边）

```json
{
  "id": "edge-1",
  "source": {"node": "node-1", "port": "out_flow"},
  "target": {"node": "node-2", "port": "in_flow"},
  "kind": "Flow",
  "waypoints": []
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | string | 唯一标识符 |
| `source` | object | 源节点和端口 |
| `target` | object | 目标节点和端口 |
| `kind` | string | `"Flow"` 或 `"Data"` |
| `waypoints` | array | 路径控制点 |

### Flow 边约束

- 仅在同一 `LabelContainer` / `ListenerContainer` 内有效。
- 不允许形成环（因为对应 `.code` 的顺序执行，循环应使用 `While` 节点）。
- 一个节点可以有多个出 `Flow`（分支），但通常一个入 `Flow`（`If` 等分支节点除外）。

### Data 边约束

- 可以跨 `LabelContainer` 连接（用于引用其他标签的输出）。
- 一个 Data 输入端口只能有一条入边。
- 一个 Data 输出端口可以有多条出边。

---

## v1.x → v2.0 映射

| 旧结构 | 新结构 |
|--------|--------|
| `graph.nodes` | 分散到各 `ThreadContainer` / `LabelContainer` / `ListenerContainer` 的 `nodes` |
| `graph.edges` | 分散到各 `LabelContainer` / `ListenerContainer` 的 `edges` |
| `graph.labels: HashMap<String, Vec<String>>` | 由 `LabelContainer.name` 和 `ThreadContainer.labels` 替代 |
| `Start` 节点 | `ThreadContainer` 的第一个 `LabelContainer` |
| `Label` 节点 | 同名的 `LabelContainer` |
| 跨标签 `Flow` 边 | 删除；改用 `Goto` / `CreateThread` / `CreateListener` 的名称参数 |
| 同标签 `Flow` 边 | 保留在对应 `LabelContainer.edges` 中 |

---

## 示例：最小 v2.0 工程

```json
{
  "version": "2.0",
  "meta": {
    "name": "test_project",
    "created_at": "2026-07-13T00:00:00Z"
  },
  "threads": [
    {
      "id": "thread-main",
      "name": "main",
      "variable_name": "var_main_thread",
      "auto_start": true,
      "labels": [
        {
          "id": "label-main",
          "name": "main",
          "params": [],
          "nodes": [
            {
              "id": "node-log",
              "type": "Log",
              "position": {"x": 100, "y": 100},
              "params": {"message": "Hello from main"}
            }
          ],
          "edges": []
        }
      ],
      "listeners": []
    }
  ],
  "viewport": {"x": 0, "y": 0, "zoom": 1.0}
}
```

对应 `.code`：

```code
var_main_thread = CreateThread("main")

main:
    thread = _this
    Log("Hello from main")
    _result = null
```

---

## 未实现事项

- v1.x → v2.0 迁移脚本（规划中）。
- 新 UI 的线程树和标签画布（P2）。
- 序列化文件的实际 Rust 结构体（待实现）。
