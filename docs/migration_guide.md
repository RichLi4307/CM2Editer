# 旧架构 → 新架构迁移指南

> 本文档说明如何把 CM2Editer v0.2.2（旧架构）保存的 `.code.json` 工程和节点图迁移到 v0.3.0-architecture（新架构）。
> 新架构核心变化：从扁平 `nodes + edges + labels` 模型转向 `ThreadContainer / LabelContainer / ListenerContainer` 容器化模型。

---

## 1. 核心变化速查

| 旧架构 | 新架构 | 说明 |
|--------|--------|------|
| `version: "1.0"` | `version: "2.0"` | 不兼容，需迁移 |
| `graph.nodes` | `threads[i].labels[j].nodes` | 节点分散到各容器 |
| `graph.edges` | `threads[i].labels[j].edges` | Flow 边只能留在原标签容器内 |
| `graph.labels` | `threads[i].labels` | 标签本身成为容器 |
| `Start` 节点 | `ThreadContainer` 第一个 `LabelContainer` | 入口钉替代 Start 节点 |
| `Label` 节点 | 同名 `LabelContainer` | 标签名即容器名 |
| 跨标签 `Flow` 边 | 删除，改用名称引用 | 用 `Goto` / `CreateThread` / `CreateListener` 参数 |
| 多线程关系 | `threads` 数组 | 每个线程独立容器 |
| 监听器 | `threads[i].listeners` | 监听器也是容器 |

---

## 2. 映射规则

### 2.1 Start 节点 → main 标签入口

旧图：

```json
{
  "type": "Start",
  "id": "node_start"
}
```

新图：

- 该 `Start` 节点所在标签 `main` 变为 `ThreadContainer` 的第一个 `LabelContainer`。
- 标签入口由 `LabelContainer.entry_pin` 表示，不再创建 `Start` 节点。

### 2.2 Label 节点 → 同名 LabelContainer

旧图：

```json
{
  "type": "Label",
  "id": "node_label",
  "params": { "name": "step1" }
}
```

新图：

- 创建 `LabelContainer`，`name` = `"step1"`。
- 该 `Label` 节点本身不进入新容器的 `nodes` 列表。
- 原 `Label` 节点后面的 Flow 链成为该 `LabelContainer` 的 `nodes` 和 `edges`。

### 2.3 同标签 Flow 边 → 保留

旧图中：

```json
{
  "type": "Flow",
  "from": { "node": "node_a", "port": "out_flow" },
  "to": { "node": "node_b", "port": "in_flow" }
}
```

如果 `node_a` 和 `node_b` 都被同一个旧标签（如 `main`）包含，则这条边直接保留在 `main` 的 `LabelContainer.edges` 中。

### 2.4 跨标签 Flow 边 → 删除，改用 Goto

旧图中如果 `node_a` 在 `main` 标签，`node_b` 在 `step1` 标签，且存在 Flow 边 `node_a → node_b`：

- 删除该 `Flow` 边。
- 在 `main` 标签中插入一个 `Goto` 节点，`label` 参数设为 `"step1"`。
- 用 Flow 边连接 `node_a` → `Goto`。

### 2.5 线程定义

旧图：

```json
"threads": [
  { "id": "thread_main", "name": "main", "entry_label": "main", "parent": null, "auto_start": true }
]
```

新图：

```json
"threads": [
  {
    "id": "thread_main",
    "name": "main",
    "variable_name": "var_main_thread",
    "auto_start": true,
    "labels": [ { "name": "main", ... } ],
    "listeners": []
  }
]
```

---

## 3. 迁移步骤

### 步骤 1：备份旧工程

迁移前把整个工程文件夹复制一份，避免数据丢失。

### 步骤 2：运行迁移脚本（待实现）

```bash
cargo run --bin migrate -- path/to/project.cm2editor
```

迁移脚本会做：

1. 读取所有 `.code.json` 文件。
2. 按 `labels` 映射把节点分组到 `LabelContainer`。
3. 处理 `Start` 和 `Label` 节点。
4. 处理跨标签 `Flow` 边，生成 `Goto` 节点。
5. 按旧 `threads` 定义生成 `ThreadContainer`。
6. 写入 `version: "2.0"` 的新 JSON。

### 步骤 3：人工检查无法自动迁移的部分

迁移脚本会生成报告，列出需要人工检查的项目：

- 菱形 Flow 路径（同一标签内有两个不同来源到达同一节点）。
- 多 `Start` 节点（旧架构允许一个图多个 Start，新架构通过多个标签容器自然支持）。
- 监听器子标签依赖 BFS 推断（需确认是否改为 `ListenerContainer`）。
- 跨文件标签引用（新架构下跨文件仍然允许，但需检查名称一致性）。

### 步骤 4：打开工程并验证

1. 用新架构编辑器打开工程。
2. 检查左侧线程树/标签树是否与预期一致。
3. 检查每个标签的 Flow 链是否完整。
4. 导出 `.code` 并与旧版输出对比语义。

---

## 4. 无法自动迁移的情况

### 4.1 菱形 Flow 路径

旧架构中：

```text
      → B →
    /       \
Start         D
    \       /
      → C →
```

新架构中 `Flow` 边不能形成菱形（因为 `.code` 是顺序执行）。需要手动改为：

```text
入口 → If → B → D
        → C → D
```

或使用 `Goto` 跳转。

### 4.2 多 Start 节点

旧架构允许一个图中存在多个 `Start` 节点。迁移后：

- 每个 `Start` 节点变成一个独立的 `LabelContainer`。
- 如果它们属于同一逻辑线程，则放入同一个 `ThreadContainer`。
- 如果它们属于不同线程，则创建多个 `ThreadContainer`。

### 4.3 监听器子标签

旧代码生成器用 BFS 推断哪些标签属于哪个监听器。迁移时：

- 如果监听器标签明确被 `CreateListener` 引用，则创建 `ListenerContainer`。
- 如果子标签通过 BFS 可达但未被显式引用，需人工确认是否保留在监听器中。

### 4.4 全局 `thread` 变量约定

旧代码生成中 `Goto` 生成 `thread.Goto("label")`，但从未定义 `thread`。新版代码生成器将自动为每个标签容器生成 `thread = _this`。

---

## 5. 向后兼容策略

- 新架构编辑器保留 `Start` / `Label` 节点的反序列化能力，加载旧工程时自动提示迁移。
- 旧节点类型 `NodeType::Start` 和 `NodeType::Label` 在代码中保留为 deprecated，但新图禁止使用。
- 保存新工程时强制使用 `version: "2.0"`。

---

## 6. 示例

### 旧架构 JSON（v1.0）

```json
{
  "version": "1.0",
  "nodes": [
    { "id": "start", "type": "Start" },
    { "id": "log1", "type": "Log", "params": { "output": "hello" } },
    { "id": "label_step1", "type": "Label", "params": { "name": "step1" } },
    { "id": "log2", "type": "Log", "params": { "output": "step1" } },
    { "id": "goto", "type": "Goto", "params": { "label": "step1" } }
  ],
  "edges": [
    { "type": "Flow", "from": { "node": "start", "port": "out_flow" }, "to": { "node": "log1", "port": "in_flow" } },
    { "type": "Flow", "from": { "node": "log1", "port": "out_flow" }, "to": { "node": "goto", "port": "in_flow" } }
  ],
  "labels": {
    "main": ["start", "log1", "goto"],
    "step1": ["label_step1", "log2"]
  },
  "threads": [
    { "id": "thread_main", "name": "main", "entry_label": "main", "parent": null, "auto_start": true }
  ]
}
```

### 新架构 JSON（v2.0）

```json
{
  "version": "2.0",
  "threads": [
    {
      "id": "thread_main",
      "name": "main",
      "variable_name": "var_main_thread",
      "auto_start": true,
      "labels": [
        {
          "id": "label_main",
          "name": "main",
          "params": [],
          "nodes": [
            { "id": "log1", "type": "Log", "params": { "output": "hello" }, "position": { "x": 0, "y": 0 } },
            { "id": "goto", "type": "Goto", "params": { "label": "step1" }, "position": { "x": 0, "y": 100 } }
          ],
          "edges": [
            { "id": "e1", "source": { "node": "log1", "port": "out_flow" }, "target": { "node": "goto", "port": "in_flow" }, "kind": "Flow" }
          ]
        },
        {
          "id": "label_step1",
          "name": "step1",
          "params": [],
          "nodes": [
            { "id": "log2", "type": "Log", "params": { "output": "step1" }, "position": { "x": 0, "y": 0 } }
          ],
          "edges": []
        }
      ],
      "listeners": []
    }
  ],
  "viewport": { "x": 0, "y": 0, "zoom": 1.0 }
}
```

---

## 7. 迁移状态

- [ ] 迁移脚本实现（Rust）
- [ ] 旧工程自动迁移提示 UI
- [ ] 回归测试覆盖旧工程迁移
- [ ] 文档示例验证

