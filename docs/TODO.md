# CM2Editer 项目 TODO（新架构）

> **版本**: 0.3.0-architecture  
> **日期**: 2026-07-13  
> **目标**: 以 `.code` DSL 语法结构为中心重构编辑器核心图模型：Thread / Label / Listener 容器化，`Flow` 边仅限容器内部，节点按语言概念分类。  
> **旧版已归档**: `docs/archive/TODO_20260713_v8.md`

---

## 当前状态

| 文档/模块 | 旧版 | 新版 | 状态 |
|----------|------|------|------|
| TODO 清单 | `docs/archive/TODO_20260713_v8.md` | 本文档 | 已更新 |
| 节点手册 | `docs/archive/node_types_20260713_v1.md` | `docs/node_types.md` | 已完成 |
| JSON Schema | `docs/json_schema.md` | `docs/json_schema.md` | 已完成 |
| 系统提示词 | `docs/agent_prompt.md` | `AGENTS.md`（项目根目录） | 已完成 |
| 实战教程 | `docs/tutorial_make_code.md` | `docs/tutorial_make_code.md` | 已完成 |
| 迁移指南 | 无 | `docs/migration_guide.md` | 已完成 |
| 架构评估 | 无 | `docs/architecture_evaluation.md` | 已完成 ✅ |

---

## 待办队列（新架构阶段）

### P0 — 核心图模型重构

- [x] 设计并实现 `ThreadContainer` / `LabelContainer` / `ListenerContainer` 数据结构。
- [x] 重写 JSON 序列化，版本升级为 `2.0`，顶层结构改为 `threads: [...]`；不再兼容 v1.0。
- [x] 重写 `src/code_gen/generator.rs`：基于容器生成 `.code`，不再依赖 BFS 推断子标签。
- [x] 从 `NodeType` 中移除 `Start` / `Label`；`NodeType` 变体数从 168 调整为 166。
- [x] 限制 `Flow` 边仅在 `LabelContainer` / `ListenerContainer` 内部表示顺序；禁止跨容器 `Flow` 边。
- [x] 重写 `src/graph/validation.rs`：移除 `Flow` DAG 约束和菱形警告，新增标签名唯一性、容器内边检查。
- [x] 更新 `src/project.rs`：新建工程默认生成 `main` 线程容器，而不是 `Start` 节点。

### P1 — 节点分类与语义修正

- [x] 按 `.code` 语言概念重新分类全部 168 个节点：
  - Threading & Concurrency
  - Control Flow
  - Variables & Globals
  - Literals
  - Math & Logic
  - Conditions & Queries
  - Game API（按子系统分组）
  - Objects
  - String / File / List
  - Editor-only
- [x] 修正代码生成：
  - `Goto` 必须显式指定目标线程或默认 `_this`。
  - `CreateThread` 不再为每个标签自动生成顶层线程；只生成用户明确创建的线程。
  - 移除 `Return` 自动追加 `_result = null` 的噪音，仅在显式 Return 时生成 `_result`。
- [x] 引入通用变量节点（`Set Variable` / `Variable`）以支持自定义作用域变量。
- [x] 验证所有现有节点在新模型下生成正确 `.code`。

### P2 — UI 与编辑器重构

- [x] 左侧工程树显示 `ThreadContainer` / `LabelContainer` 层级。
- [x] 画布切换为“当前选中标签的内部流图”。
- [x] 提供线程概览图（状态机视图），显示标签间 `Goto` / `CreateThread` / `CreateListener` 关系。
- [x] 移除画布上的 `Start` / `Label` 节点；用容器入口钉替代。

### P3 — 测试与预览版发布

#### P3.1 全量回归测试与 clippy

- [x] 跑全量 `cargo test` / `cargo clippy`，修复 P2 回归。

#### P3.2 补充 UI 回归测试

- [x] 容器切换：验证 `SelectedContainer` 在不同线程/标签/监听器间切换后定位正确。
- [x] 入口钉：验证无 Flow 入边节点的最左上入口规则。
- [x] 概览图：验证多标签/监听器、Goto 关系能正确构建布局。
- [x] 工程保存/导出：验证项目创建、重命名、保存、导出 `.code` 完整流程。

#### P3.3 手动冒烟测试

- [ ] 手动验证一个示例任务从新建到导出 `.code` 的完整流程。

#### P3.4 构建与打包

- [ ] 构建 Release 版本并打包字体、命名空间、README、AGENTS.md、LICENSE。

#### P3.5 发布预览版

- [ ] 发布 GitHub Release `v0.3.0-alpha`，附已知限制说明。

---

## Agent 交付规则

1. **更新 `CHANGELOG.md`** — 每次功能交付后追加条目。
2. **更新 `docs/TODO.md`** — 标记已完成任务 ✅，追加工作日志条目。
3. **`cargo test` 全过再 commit** — 108 项全部通过为提交门槛。
4. **任何任务完成后必须提交一次 commit** — 不要留下未提交改动。
5. **commit message 用中文前缀** — 格式 `<类型>: <简要描述>`，例如：`重构: 容器化图模型`、`文档: 更新节点分类`。
6. **重大文档变更需归档** — 将旧版按 `{文件名}_{YYYYMMDD}_v{序号}.md` 放入 `docs/archive/`。

---

## 用户备注区

- 新架构的核心原则是：编辑器为 `.code` 语法结构服务，而不是让 `.code` 迁就流程图直觉。
- `main` 只是一个约定俗成的顶层线程标签，不是特殊入口。
- Listener 是每帧/每秒调用标签的循环；局部监听器捕获创建处作用域。
- 标签间关系应通过名称引用或 Data 端口表达，不能画 `Flow` 边。
- `app` 和 `ui` 模块已重新在 `src/lib.rs` 中启用，并迁移到容器化模型。

---

## Agent 工作日志

| 日期 | 任务编号 | 说明 | 状态 |
|------|----------|------|------|
| 2026-07-13 | 文档-归档 | 将旧版 `TODO.md` / `node_types.md` 归档到 `docs/archive/` | 已完成 |
| 2026-07-13 | 文档-新架构 | 完成新架构核心文档：TODO.md、node_types.md、json_schema.md、agent_prompt.md、tutorial_make_code.md、migration_guide.md | 已完成 ✅ |
| 2026-07-13 | 实现-P2 | 完成 UI 与编辑器重构：`src/app.rs` 迁移到 `ContainerGraph`；工程树显示 Thread/Label/Listener 层级；画布切换为当前容器内部流图；新增入口钉渲染；新增线程概览图面板 | 已完成 ✅ |
| 2026-07-13 | 修复-P2 | 统一入口节点判定：`LabelContainer::entry_node_id()` 按最左上的无 Flow 入边节点稳定选择入口；入口钉渲染与代码生成器共用同一逻辑；修复 `main.rs` 启动 UI | 已完成 |
| 2026-07-13 | 文档-教程 | 重写 `docs/tutorial_make_code.md`，对齐当前 UI 工作流程，注明多容器创建暂不支持 | 已完成 |
| 2026-07-13 | 文档-教程 | 在实战教程中新增多条件判断（Exposed_All + Cosplay + Ecstasy）、RP 奖励、只执行一次守卫等进阶示例 | 已完成 |
| 2026-07-13 | 测试 | 新增 `graph::container::tests::test_entry_node_id_prefers_top_left_no_incoming_flow` | 已完成 |
| 2026-07-13 | 修复-节点 | 修复 `GetSave` 节点：新增 `key` 参数，输出改为 `Any`，生成器输出 `_save.key`，与文档一致 | 已完成 |
| 2026-07-13 | 文档-教程 | 将实战教程第五步改为 `GetStateNumber(Rank)`，区分 RP、`_state` 状态与 `_save` 存档读取；补充 RP 与角色经验键名说明 | 已完成 |
| 2026-07-13 | 架构-评估 | 完成 `docs/architecture_evaluation.md` | 已完成 ✅ |
| 2026-07-13 | 文档-重构 | 合并 `docs/agent_prompt.md` 为项目根目录 `AGENTS.md`，按 Kilo 约定添加文档置信上下级与查阅指南 | 已完成 |
| 2026-07-13 | 规划-P3 | 将 P3 重新定义为「测试与预览版发布」，并拆分为 P3.1–P3.5 | 已完成 |
| 2026-07-13 | 测试-P3.1 | `cargo test` 103 项通过，`cargo clippy` 18 个 pre-existing 警告、0 个 error | 已完成 |
| 2026-07-13 | 测试-P3.2 | 补充 UI 回归测试：容器切换、入口钉、概览图 CreateListener、工程保存/导出 | 已完成 |
| 2026-07-13 | 修复-P3.1 | 修复 `src/graph/validation.rs` 与 `src/ui/panels/properties.rs` 中的 `unwrap()`，使 clippy 通过 | 已完成 |
| 2026-07-14 | 修复-UI | 服装命名空间选择器 (`cosplay`) 属性页弹窗分类视图改为多选 checkbox 列表并加窗口标题提示；左栏资产管理面板新增“多选”开关支持批量勾选复制；`CheckCosplay` 参数改为 `cosplayKeys` (List) 并生成 `(Cosplay_A && Cosplay_B)` 便于与 LogicAnd/LogicOr 组合；新增对应生成器测试 | 已完成 |
| 2026-07-15 | 文档-i18n | 创建 `docs/i18n.md`，设计编辑器国际化方案：首批中英、日语预留、键命名规范、迁移阶段、与节点面板场景分类的关系 | 待审阅 |
| 2026-07-15 | 修复-i18n | 完成 i18n 初审：修复语言选择器显示代码、命名空间卡片硬编码 zh、`label.position` 占位符拼写错误；迁移属性面板部分文本；`cargo test` 105 项通过 | 已完成 |

---

## 创意与待实现（Backlog）

### 按使用场景分类的节点面板（暂缓，待 i18n 完成后实施）

**目标**：节点库不再按 API 分类（如 `Game API`、`Objects`、`Control Flow`）组织，而是按开发者实际使用场景分类，例如：

- 任务 / 流程
  - 线程与监听器
  - 流程控制
  - 任务面板
  - 事件通信
- 条件判定
  - 比较运算
  - 逻辑运算
  - 状态检查
- 数据获取
  - 玩家信息
  - 物品 / 装备
  - 全局变量
- 数据修改
  - 玩家状态
  - 服装操作
  - 成人玩具
  - 物品操作
- 数据处理
  - 数学
  - 字符串
  - 列表
  - 向量 / 颜色
- 视觉 / UI
- 编辑器专用

**关键设计**：
- 节点注册层（`src/api/definitions.rs`）保持 API 分类不变，一个节点只在一个 API 分类中。
- 节点面板层新增独立的分类注册表（如 `src/ui/panels/node_library/catalog.rs`），一个节点可出现在多个场景分类中。
- 支持二级分类：一级分类折叠 → 二级分类折叠 → 节点列表。
- 预留鼠标悬停节点介绍接口。
- 搜索窗口同时支持按节点名和场景分类关键字搜索。

**暂缓原因**：当前 UI 没有多语言接口，若先进行节点面板分类重构，所有新增分类名称、悬停提示等中文文本都会成为后续 i18n 的额外迁移负担。应在多语言接口就位后实施，避免重复劳动。

### UI 多语言接口（i18n）

**现状**：当前编辑器中所有 UI 文本（节点显示名、描述、端口名、参数名、按钮、状态消息、提示等）均硬编码中文，无 i18n 抽象。粗略统计约 989 处硬编码中文字符串。

**目标**：建立运行时多语言机制，首批支持中文（zh）和英文（en），后续可扩展其他语言。

**建议实现方向**：
- 新增 `src/ui/i18n.rs` 或 `src/i18n.rs`，提供 `I18n` 结构体与 `t!(key)` / `i18n.text(key)` 接口。
- 翻译文件使用 JSON 或 YAML，放在 `assets/i18n/` 或 `locales/` 目录下（如 `zh.json`、`en.json`）。
- 在 `App` 中持有 `I18n` 实例，切换语言时重新加载翻译表。
- 替换优先级：
  1. UI 面板标题、按钮、状态消息（用户直接可见）
  2. 节点显示名、描述、端口名、参数名（影响节点库和属性面板）
  3. 条件模板、工具提示、错误信息
- 注意：节点注册层的 `NodeDefinition` 可先保持硬编码，通过 `I18n` 在渲染时动态替换显示文本，避免一次性改动过大。

**下一步**：先设计并实现 `I18n` 基础设施，再逐步迁移硬编码文本。节点面板使用场景分类重构排在此之后。
