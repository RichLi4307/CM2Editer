# Label 节点与标签管理 plan

> 时间：2026-07-10
> 版本：v0.2.2
> 类型：设计草案，待评审后实施

## 背景

标签机制是 CM2Editer 控制流模型的核心——每个标签体对应一个 `.code` 函数，代码生成器通过 `graph.labels` 将它翻译成标签定义。

但目前标签的创建、绑定和清理分散在多处且没有覆盖所有场景，标签节点可能不被检查器校验，或不触发正确的代码生成。

## 现状分析

| 场景 | 能否创建标签 | 能否绑定节点 | 能否清理 |
|------|------------|------------|--------|
| 左栏 `+ 新建标签` | ✅ | ❌ 创建空的 `graph.labels` entry | ❌ 需手动删除 |
| 拖拽 Label 节点到画布 | ✅ | ✅ 节点挂在画布上 | ✅ 删除节点即可 |
| 通过属性面板改名 | ✅ | ✅ 自动注册到 `graph.labels` | ❌ 旧名字残留 |
| 通过 Data 连线改名字 | ✅ | ❌ 仅代码生成器能感知，`graph.labels` 未更新 | ❌ |
| 删除标签节点 | ✅ | ✅ | ❌ `graph.labels` 残留 |
| Goto / CreateThread 自动绑定 | ✅（空标签） | ❌ 空的 node list | ❌ 标签节点删除后仍保留 |

## 计划

### Phase 1：修复绑定范围
- **删除 Label 节点**时清理 `graph.labels` 对应 entry
- **Label 改名**时从旧名字的 `graph.labels` 中移除，并插入新名字的 entry
- **`+ 新建标签`**时同时创建一个空 Label 节点（或提示用户需要拖拽 Label 到画布）

### Phase 2：完善验证规则
- 未绑定标签的 Label 节点触发**新警告** `FlowError::Warning("Label 节点未被绑定到 graph.labels")`
- BFS 源 3 保持现有规则：只匹配已绑定的 Label

### Phase 3：Data 连线支持
- `CreateListener.out_label` / `Goto.out_name` / `StringConstant.out_value` → Label.name 的 Data 连线时，自动将 Label 绑定到 `graph.labels`

### 风险
- 多个标签节点同名 → 现在会直接替换，后续需检查冲突
- 改名时连线的端口可能已经丢失 → 维护属性面板自身的稳定性

## 需你确认的点

1. 删除 Label 节点时自动删除 `graph.labels` → 同意吗？
2. 未绑定的 Label 是否需要警告 → 确认新警告 `FlowError::Warning("Label 节点未被绑定到 graph.labels")` 是否需要新的错误级别？
3. 是否需要在 `+ 新建标签` 时自动创建 Label 节点（简化流程），还是保持当前手动拖拽？
