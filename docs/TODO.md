# CM2Editer 项目 TODO

> **版本**: 0.3.0
> **日期**: 2026-07-17
> **目标**: P0 语法硬缺口与 P1 高频 API 已全部补齐，当前重心转向 P2 使用体验优化与 P3 发布准备。
> **旧版已归档**: `docs/archive/TODO_20260717_v10.md`（P0–P1 完成详情）
> **当前节点**: `NodeType` 188 变体，155 项 lib tests 通过

---

## 当前状态

- 新架构（ThreadContainer / LabelContainer / ListenerContainer）已落地，JSON v2.0。
- P0 语法缺口：事件监听器、停止音频、全局变量、性高潮、elseif 折叠、复合赋值、多分支 If、动态端口基础设施已全部完成。
- **P0 v0.3.0 issues 已修复**：IF/While 条件取消引号包裹并统一外层括号；新建 `.code` 文件默认空图；`elseif_*_condition` 使用条件模板编辑器。
- P1 高频 API：Log 级别、Translate、List 六方法、NPC 五方法、FunctionExists / GetModVersion 已全部完成。
- 节点库按场景分类（7 个顶层场景、20+ 子分类）；i18n zh/en/ja 已接入，188 个节点均有 zh/en 名称键。
- `cargo test --lib`：**155** 项通过；`cargo clippy --lib`：**23** warnings（无新增，均为历史遗留）。

---

## 待办队列

### P0 - v0.3.0 issues

- [x] IF节点代码生成器生成的代码中，条件被引号包裹，导致失效。
- [x] 开发者反馈，IF在使用时最好加上括号，解释器性能不稳定，无括号容易出错
- [x] 创建文件会默认创建一个线程，这是非必要的，不是每个文件都需要写个线程（该点来自用户反馈，如需更多信息可以参考别人写的成品.code）
- [x] elseif分支的condition选项编辑方式不直观。

### P2 — 使用体验优化（按优先级排序）

> 目标：在 188 节点规模下降低 Mod 作者的选择困难、学习曲线与搜索噪音。

#### P2-高：立即收益最大

- [x] **P2.1 节点库搜索增强**
  - 支持按场景分类过滤 + 模糊匹配 + 最近使用记录
  - 当前 188 节点下搜索 `Set` / `Get` 前缀匹配过多，用户难以定位
  - 位置：`src/ui/panels/node_library/`（搜索框与过滤逻辑）

- [x] **P2.2 属性面板参数折叠**
  - 参数数量超过阈值（如 >4）时默认折叠为“高级参数”区域，常用参数保持展开
  - 目标：降低 `CreateNPC`（8 参数）、`CreateCondition`（条件组合）等节点的认知负担
  - 位置：`src/ui/panels/properties.rs`

- [x] **P2.3 CallMethod 方法下拉**
  - 选中对象类型后弹出方法下拉 + 参数模板，替代手输大小写敏感方法名
  - 覆盖 50+ 对象方法的低成本方案，减少用户查阅官方文档的时间
  - 位置：`src/ui/panels/properties.rs` + `src/api/method_registry.rs`（新增）

#### P2-中：持续提升可用性

- [ ] **P2.4 节点描述 i18n 补全**
  - 当前只有部分节点有详细 `description`，P1 新增节点有，老节点描述较简单
  - 目标：所有 188 个节点在 zh/en 均有详细描述，hover 即能理解用途
  - 位置：`assets/i18n/zh.json` / `en.json`（批量补全）

- [ ] **P2.5 常用节点收藏 / 置顶**
  - 用户可将高频节点固定到节点库顶部，减少重复搜索
  - 位置：`src/ui/panels/node_library/`（收藏状态持久化）

- [ ] **P2.6 _state 探针选择器**
  - `_state.Position.x`、`_state.Camera.pitch`、`_state.Handcuffs.Type` 等嵌套路径树形选择，类型安全输出
  - 复用命名空间选择器模式
  - 位置：`src/ui/panels/`（新增 `state_picker.rs`）

- [ ] **P2.7 条件表达式实时校验**
  - 括号配平、token 合法性提示（条件组合编辑器增强）
  - 位置：`src/ui/panels/condition_editor.rs`

#### P2-低： backlog

- [ ] **P2.8 场景分类再细分**
  - `scene.data_set.player_state`（20+ 节点）拆分为 `position` / `stats` / `sex`
  - `scene.data_process.math`（19 节点）拆分为 `trig` / `rounding` / `random`
  - 位置：`src/ui/panels/node_library/catalog.rs`

- [ ] **P2.9 生成器专项测试补全**
  - 当前很多 A 类节点只有通用 `assert_flow_node_generates`，未验证完整语义
  - 目标：为所有 A 类节点增加完整语义测试（如 CreateNPC、Translate、多分支 If）
  - 位置：`src/code_gen/generator.rs` 测试模块

- [ ] **P2.10 For 自带 start/stop/step**
  - 无 iterable 连线时自动包装 `Range()`，减少用户手动创建 Range 节点
  - 位置：`src/code_gen/generator.rs`（`generate_for`）

- [ ] **P2.11 CreateArea cuboid 参数集**
  - 官方支持 sphere / cylinder / cuboid 三种，当前缺 cuboid（x1..z2, w, h）
  - 位置：`src/api/definitions.rs` + `src/code_gen/generator.rs`

### P3 — 发布准备

- [ ] **P3.1 手动冒烟测试**：按 `docs/test_checklist.md`（v0.3.0 版）跑一遍新建 → 编辑 → 导出完整流程
- [x] **P3.2 构建与打包**：Release 构建，打包字体、命名空间、README、AGENTS.md、LICENSE
- [ ] **P3.3 发布预览版**：GitHub Release `v0.3.0-alpha`，附已知限制说明

---

## Agent 交付规则

1. **更新 `CHANGELOG.md`** — 每次功能交付后追加条目。
2. **更新 `docs/TODO.md`** — 标记已完成任务 ✅，追加工作日志条目。
3. **`cargo test` 全过再 commit** — 163 项（154 lib + 9 integration）全部通过为提交门槛。
4. **任何任务完成后必须提交一次 commit** — 不要留下未提交改动。
5. **commit message 用中文前缀** — 格式 `<类型>: <简要描述>`，例如：`新增: EventListener 节点`。
6. **重大文档变更需归档** — 将旧版按 `{文件名}_{YYYYMMDD}_v{序号}.md` 放入 `docs/archive/`。

---

## 用户备注区

- 节点数量已达 188，P2 阶段核心目标是**降低选择困难与搜索噪音**，而非继续堆节点。
- 专用节点与通用节点的平衡：高频 API 做专用节点（P1 已覆盖），冷门 API 走 `CallMethod` / `CallFunction`，通用方法下拉（P2.3）是兜底方案。
- 新增节点前先读 `docs/node_types.md` 与 `AGENTS.md` 的节点修改强制规则（A/B/C 类）。
- `docs/syntax_coverage.md` 是 P0–P2 的来源文档，缺口细节以它为准。

---

## Agent 工作日志

| 日期 | 任务编号 | 说明 | 状态 |
|------|----------|------|------|
| 2026-07-20 | P0 | 修复 P0 v0.3.0 issues：IF/While 条件去引号 + 加括号；新建 .code 文件默认空图；elseif 条件改用模板编辑器；`cargo test` 164 项全过 | 已完成 |
| 2026-07-17 | P3.2 | Release 打包：版本号核对 0.3.0、README 与 CHANGELOG 同步、`cargo test` 163 项全过 | 已完成 |
| 2026-07-17 | 文档-TODO | 归档 v10，新 TODO 按优先级整理 P2 使用体验优化与 P3 发布准备 | 已完成 |
| 2026-07-17 | 修复 | 节点库渲染黑框 + 底栏分割线可互相越过 | 已完成 |
| 2026-07-17 | P2.3 | CallMethod 方法下拉：方法注册表 + 属性面板下拉 + 参数模板 + 代码生成修复 | 已完成 |
| 2026-07-17 | P2.2 | 属性面板参数折叠：>4 参数自动收进“高级参数”区域 | 已完成 |
| 2026-07-17 | P2.1 | 节点库搜索增强：场景分类过滤 + 模糊匹配 + 最近使用记录（持久化） | 已完成 |
