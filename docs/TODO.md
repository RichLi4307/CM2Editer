# CM2Editer 项目 TODO

> **版本**: 0.3.0
> **日期**: 2026-07-20
> **目标**: Phase2 已全部完成，当前重心 P3 发布准备。
> **旧版已归档**: `docs/archive/TODO_20260720_v11.md`（Phase2 完成详情）

---

## 当前状态

- Phase2 所有任务（P2.4–P2.11 + checklist UI 缺陷）已完成并合并到 `main`。
- 自动化测试：`cargo test --lib` **191 项通过**；`cargo test` 完整套件（191 lib + 9 integration）全部通过。
- `cargo clippy --lib` **0 warnings**。
- 手动测试检查单 `docs/test_checklist.md` 已清空并修正为当前项目状态，等待 **P3.1 手动冒烟测试**。
- 本地 `main` 领先 `origin/main`。

---

## 待办队列

### P2 — 使用体验优化（全部完成）

> 目标：在 188 节点规模下降低 Mod 作者的选择困难、学习曲线与搜索噪音。已全部达成。

- [x] **P2.1 节点库搜索增强**
- [x] **P2.2 属性面板参数折叠**
- [x] **P2.3 CallMethod 方法下拉**
- [x] **P2.4 节点描述 i18n 补全**
- [x] **P2.5 常用节点收藏 / 置顶**
- [x] **P2.6 _state 探针选择器**
- [x] **P2.7 条件表达式实时校验**
- [x] **P2.8 场景分类再细分**
- [x] **P2.9 生成器专项测试补全**
- [x] **P2.10 For 自带 start/stop/step**
- [x] **P2.11 CreateArea cuboid 参数集**
- [x] **Checklist 暴露 UI 缺陷修复**（窗口最大化、节点库拖拽虚影、命名空间/坐标删除按钮）

### P3 — 发布准备

- [ ] **P3.1 手动冒烟测试**：按 `docs/test_checklist.md`（v0.3.0 版）跑一遍新建 → 编辑 → 导出完整流程。
- [x] **P3.2 构建与打包**：Release 构建，打包字体、命名空间、README、AGENTS.md、LICENSE。
- [ ] **P3.3 发布预览版**：GitHub Release `v0.3.0-alpha`，附已知限制说明。

---

## 已完成测试归纳

### 自动化测试

- **库单元测试**：`cargo test --lib` 191 项全部通过。新增核心覆盖包括：
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

### 手动测试

- `docs/test_checklist.md` 已按当前项目状态（Phase2 完成后）更新，所有结果列已清空为 **NT**。
- 新增/修正测试项：节点收藏、_state 探针选择器、条件表达式实时校验、CallMethod 方法下拉、For auto Range、CreateArea cuboid 等。
- 下一步由人工按该检查单执行 P3.1 冒烟测试并填写结果。

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
| 2026-07-20 | 归档 | 归档旧 TODO 为 `docs/archive/TODO_20260720_v11.md`，整理已完成测试并归纳到新 TODO | 已完成 |
| 2026-07-20 | P2 | Phase2 全部完成并合并到 `main`：P2.4–P2.11 + checklist UI 缺陷；`cargo test --lib` 191 项通过；`cargo test` 9 项 integration 通过；`cargo clippy --lib` 0 warnings | 已完成 |
| 2026-07-20 | 修复 | 修复 `If` 节点属性面板中条件模板 ComboBox 的 egui duplicate widget ID 警告；`cargo test --lib` 191 项通过 | 已完成 |
| 2026-07-20 | 文档 | 更新 `AGENTS.md`：增加未提交更改时禁止恢复记录的规则 | 已完成 |
| 2026-07-20 | 文档 | 清空 `docs/test_checklist.md` 测试结果并按当前项目状态修正测试项 | 已完成 |
| 2026-07-20 | 文档 | 更新 `CHANGELOG.md`：追加 P2.5/P2.6/P2.7/P2.10/P2.11/P2.9/egui duplicate ID 修复条目 | 已完成 |
