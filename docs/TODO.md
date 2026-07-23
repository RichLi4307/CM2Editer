# CM2Editer 项目 TODO

> **版本**: 0.4.0
> **日期**: 2026-07-23
> **目标**: v0.3.0-alpha 已发布，当前进入 v0.4.0 开发；P3.4 UI 设计规范落地阶段 A/B/C 已完成，吸附式窗口方案已移出 P3.4。
> **旧版已归档**: `docs/archive/TODO_20260720_v11.md`（Phase2 完成详情）、`docs/archive/TODO_20260720_v12.md`（纳入自动化测试归纳的上版）

---

## 当前状态

- v0.3.0-alpha 已发布，当前进入 **v0.4.0** 开发。
- P3.4 UI 设计规范落地：**阶段 A（令牌化）、B（TokenButton/TokenComboBox）、C（i18n 守卫）**已完成；吸附式窗口方案已移出 P3.4，作为 v0.4.0 分支/RFC 试点。
- P3.1 手动冒烟测试：因当前 UX 仍在重构中，暂不继续执行，待 B/C/D 阶段完成后再复测。
- 自动化测试：`cargo test --lib` **198 项通过**；`cargo test` 完整套件（198 lib + 9 integration）全部通过。
- `cargo clippy --lib` **0 warnings**。

---

## 待办队列

### P3 — 发布准备

- [x] **P3.1 手动冒烟测试（部分完成）**：按 `docs/test_checklist.md` 完成 1-5 章节，共 39 JM / 4 DN / 90 NT。详见「手动测试汇总」与「已知缺陷 / 改进项」。
- [ ] **P3.1 手动冒烟测试（剩余）**：继续执行 6-10 章节（节点参数编辑、底部面板、代码生成、i18n、验证与错误处理）。
- [x] **P3.2 构建与打包**：Release 构建，打包字体、命名空间、README、AGENTS.md、LICENSE。
- [x] **P3.3 发布预览版**：GitHub Release `v0.3.0-alpha`，附已知限制说明。
- [x] **P3.4 UI 设计规范落地**：阶段 A/B/C 已完成（`tokens` 模块扩展、全库尺寸/颜色替换、`TokenButton`/`TokenComboBox` 封装替换、颜色守卫范围扩展、`i18n` 三语守卫）；吸附式窗口方案已移出 P3.4，作为 v0.4.0 分支/RFC 试点。

### P3.1 中发现的待修复/改进项（未排序）

> 来自 `docs/test_checklist.md` 的 DN 与「手动测试备注」。已由本 session 全部修复，待下一轮手动复测确认。

- [x] **1.3 默认线程策略**：移除新建工程自动创建主线程，改为 `ContainerGraph::default_empty()`；新增项目树线程/标签/监听器创建与删除按钮，保留 `default_main()` 仅用于测试与旧工程兼容。
- [x] **2.1.3 搜索大小写敏感**：节点库与 Space 搜索模糊匹配改为大小写不敏感。
- [x] **2.1.7 描述悬停**：节点库面板、搜索窗口、画布节点悬停均显示 `node.{NodeType}.description` Tooltip。
- [x] **2.2.3 文件树底部控件溢出**：将新建 `.code`/保存/导出按钮移入 `ScrollArea`，与代码文件内容一起滚动，避免超出屏幕；同时消除底部控件与文件树下降速度不一致。
- [x] **节点分类审计**：新增 `test_boolean_output_nodes_are_in_conditions`，强制所有 Boolean 输出节点归入 `scene.conditions.state_check`；已补充 `FunctionExists`、`GetStageChanged`、`CollectItem`、`FileExists`、`ListContains`、`NPCIsAlive`、`NPCSeesPlayer`、`NPCSeesFlashing`。
- [x] **坐标标签显示**：中文环境下坐标面板使用「场景中文名 + 名称」；新增 `I18n.stage_name` 与 `stage.*` 中文翻译，其他语言保持原样。
- [x] **属性面板坐标/面向输入**：Vector/Quaternion 参数在空数组时提供文本输入（JSON 数组），不再只能打开弹窗；Quaternion 不再打开坐标选择器。
- [x] **NPCIsAlive 下拉选择**：Object 类型 `npc` 参数新增 NPC 输出端口专用下拉框，列出图中所有 `out_npc` / 标签为 "NPC" 的输出端口。
- [x] **连线时端口灰化**：拖拽连线时，`NodeRenderer` 根据源端口类型将不兼容输入端口变灰（透明度 0.25）。
- [ ] **MissionPanel / MissionMenuItem 节点化**：按 `AGENTS.md` 第10章设计决策落地，修复方法注册表、统一 `CallMethod` 参数名、放宽 `Global`/`Local` 类型、新增专用方法节点并更新节点库与 `docs/node_types.md`。测试版旧工程不兼容。
  1. [ ] 修正 `src/api/method_registry.rs` 中 MissionPanel / MissionMenuItem 方法。
  2. [ ] 将 `CallMethod` 参数 `thread` 改名为 `object`，同步 `definitions.rs` / `generator.rs` / `properties.rs` / 测试。
  3. [ ] 将 `Global` / `Local` 的 `value` 参数和 `out_value` 端口类型从 `List` 改为 `Object`。
  4. [ ] 在 `src/graph/types.rs` 新增专用方法 `NodeType` 变体。
  5. [ ] 在 `src/api/definitions.rs` 新增对应 `NodeDefinition`（A 类：in_flow + object + 方法参数 → out_flow）。
  6. [ ] 在 `src/code_gen/generator.rs` 为每个新增节点写 A 类生成分支。
  7. [ ] 在 `src/ui/panels/node_library/catalog.rs` 更新节点库分类。
  8. [ ] 同步更新 `docs/node_types.md`：新增 A 类节点表格、更新分类、说明生命周期用法。
  9. [ ] 同步更新 `docs/analysis_mission_panel_workflows.md`：标记已修复项。
  10. [ ] `cargo test` 全绿后提交。

---

## 已完成测试归纳

### 自动化测试

- **库单元测试**：`cargo test --lib` **192 项全部通过**。新增核心覆盖包括：
  - 图序列化/反序列化、项目生命周期（新建/保存/打开/导出/重命名/删除）。
  - 画布、节点渲染、交互、DataFlow 边命中。
  - 节点库搜索、收藏、场景分类覆盖。
  - `_state` 探针选择器、条件表达式校验、命名空间/坐标增删。
  - 代码生成器：A 类节点（Goto/If/While/For/Break/Return/CallFunction/CallMethod/ForeachNode/DestroyListener/Wait/WaitForThread/WaitForEvent）、CreateNPC、Translate、FunctionExists/GetModVersion、NPC 方法、List 方法、Log 级别等。
- **集成测试**：`cargo test` 9 项全部通过：
  - `tests/code_gen.rs`：生成代码到文件、语义元素保留、容器图生成。
  - `tests/examples_verify.rs`：示例工程验证占位。
  - `tests/json_roundtrip.rs`：图/路点/参数引用/meta 的 JSON 往返。
- **代码质量**：`cargo clippy --lib` 0 warnings；`cargo clippy --all-targets` 仅 3 个既有测试 warning。

### 手动测试（P3.1 部分结果）

> 测试范围：v0.3.0 检查单 1-5 章节（欢迎页、节点库、工程树、命名空间/坐标、画布操作、连线）。

- **JM（已测试/通过）**：39 项
  - 1.x：启动、打开工程、窗口最大化、保存、再打开。
  - 2.1：场景分类显示、二级折叠、跨场景节点、拖出节点、节点收藏。
  - 2.2：文件树显示、分隔条。
  - 2.3：命名空间窗口、cosplay 二级分类、复制 key、增删按钮。
  - 2.4：坐标场景分组、展开、增删按钮。
  - 3.x：节点拖拽、缩放、平移、框选、选中、Space 搜索、删除、撤销/重做、复制粘贴、容器拖入。
  - 4.1-4.8：Flow/Data 连线、Data 虚线显示与选中、删除虚线、If/CompareNumbers/CreateCondition.id 数据连接。
- **DN（已测试/不通过）**：4 项，已由本 session 修复，待下一轮手动复测确认。
  - 1.3：新建工程自动创建主线程（用户要求不自动创建）。
  - 2.1.3：搜索框不支持大写字母。
  - 2.1.7：节点描述仅在属性面板显示，工程树/画布悬停无描述。
  - 2.2.3：文件树底部控件仍可溢出屏幕，且底部下降速度不一致。
- **NT（未测试）**：90 项，对应检查单 4.9-4.10、5.x、6.x-10.x 及代码生成章节。

---

## Agent 交付规则

1. **更新 `CHANGELOG.md`** — 每次功能交付后追加条目。
2. **更新 `docs/TODO.md`** — 标记已完成任务 ✅，追加工作日志条目。
3. **`cargo test` 全过再 commit** — 191 项全部通过为提交门槛。`cargo clippy` 只允许 pre-existing 警告。
4. **任何任务完成后必须提交一次 commit** — 不要留下未提交改动。
5. **commit message 用中文前缀** — 格式 `<类型>: <简要描述>`，例如：`新增: EventListener 节点`。
6. **存在任何未提交更改时，禁止恢复记录** — 若 `git status --short` 显示未提交改动，不得执行 `git checkout -- <file>`、`git reset --hard`、`git restore` 等覆盖工作区的操作；必须先让用户确认或 commit。
7. **重大文档变更需归档** — 将旧版按 `{文件名}_{YYYYMMDD}_v{序号}.md` 放入 `docs/archive/`。

---

## 用户备注区

- 节点数量已达 188，P2 阶段核心目标是**降低选择困难与搜索噪音**，而非继续堆节点。Phase2 已全部完成。
- 专用节点与通用节点的平衡：高频 API 做专用节点（P1 已覆盖），冷门 API 走 `CallMethod` / `CallFunction`，通用方法下拉（P2.3）是兜底方案。
- 新增节点前先读 `docs/node_types.md` 与 `AGENTS.md` 的节点修改强制规则（A/B/C 类）。
- `docs/syntax_coverage.md` 是 P0–P2 的来源文档，缺口细节以它为准。

---

## Agent 工作日志

| 日期 | 任务编号 | 说明 | 状态 |
|------|----------|------|------|
| 2026-07-23 | 修复 | 工程文件树底部按钮固定，禁止溢出左栏：`project_tree.rs` 将三个按钮移出 `ScrollArea` 并固定底部；`app.rs` 调整左栏高度分配优先级，工程树最小高度 220，节点库可压缩 | 已完成 |
| 2026-07-23 | 修复 | 节点悬停描述 Tooltip 改为仅标题栏悬停且 0.5s 延迟后触发，避免移动鼠标时频繁弹出；`app.rs` 新增 `hovered_node_id` / `hovered_node_start` | 已完成 |
| 2026-07-23 | 文档 | 在 `docs/ui_design_spec.md` 新增第 15 章「吸附式窗口面板方案」，把左/右/底栏从布局面板改为 egui 吸附窗口的候选架构，待架构师审核 | 已完成 |
| 2026-07-23 | 文档 | 在 `docs/ui_design_spec.md` 新增第 14 章「实际落地状态登记表」，列出全部规范项的落地状态与偏差，供架构师审核 | 已完成 |
| 2026-07-23 | 修复 | 修复启动后无欢迎页/开始界面：`update_canvas` 在 `selected_container` 为 `None` 时直接返回，导致欢迎卡片未渲染；改为先绘制画布再显示 `draw_welcome_card`，并提取为独立方法、套用设计令牌 | 已完成 |
| 2026-07-23 | P3.4 | 按 `docs/ui_design_spec.md` 16.7 执行阶段 A/B/C：扩展 `tokens` 布局/尺寸常量，替换 `app.rs` 左/右/底栏尺寸、`node_renderer.rs` 字号、`node_library` 下拉宽度；封装 `TokenButton` 与 `TokenComboBox` 并替换全库调用点；扩展颜色守卫至 `src/app.rs`；新增 i18n 静态 key 三语守卫并补齐 zh/en/ja 缺失翻译；吸附式窗口方案移出 P3.4，第 15 章状态更新为 v0.4.0 RFC 试点；`cargo test --lib` 198 通过 | 已完成 |
| 2026-07-20 22:45 | P3.1 | 修复 P3.1 检查单中 9 项待修复/改进项：默认线程策略、搜索大小写不敏感、描述悬停、文件树底部控件、节点分类审计、坐标标签中文显示、Vector/Quaternion 手动输入、NPC 下拉选择、连线端口灰化；新增 192 项 lib tests | 已完成 |
| 2026-07-20 | P2 | Phase2 全部完成并合并到 `main`：P2.4–P2.11 + checklist UI 缺陷；`cargo test --lib` 191 项通过；`cargo test` 9 项 integration 通过；`cargo clippy --lib` 0 warnings | 已完成 |
| 2026-07-20 | 修复 | 修复 `If` 节点属性面板中条件模板 ComboBox 的 egui duplicate widget ID 警告；`cargo test --lib` 191 项通过 | 已完成 |
| 2026-07-20 | 文档 | 更新 `AGENTS.md`：增加未提交更改时禁止恢复记录的规则 | 已完成 |
| 2026-07-20 | 文档 | 清空 `docs/test_checklist.md` 测试结果并按当前项目状态修正测试项 | 已完成 |
| 2026-07-20 | 文档 | 更新 `CHANGELOG.md`：追加 P2.5/P2.6/P2.7/P2.10/P2.11/P2.9/egui duplicate ID 修复条目 | 已完成 |
