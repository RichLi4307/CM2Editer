# CM2Editer 项目 TODO

> **版本**: 0.3.0
> **日期**: 2026-07-16
> **目标**: 补齐 `.code` 语法硬缺口（P0），逐步覆盖高频 API（P1），持续打磨使用体验（P2）。
> **旧版已归档**: `docs/archive/TODO_20260716_v9.md`
> **缺口依据**: `docs/syntax_coverage.md`（2026-07-16 全量审计：168 节点覆盖约 90% 语法要素，无法绕行的硬缺口仅 EventListener 与 StopAudio）

---

## 当前状态

- 新架构（ThreadContainer / LabelContainer / ListenerContainer）已落地，`NodeType` 168 变体，JSON v2.0。
- i18n 三语（zh/en/ja）已接入；`zh.json` 节点描述已升级为 `docs/node_details.md` 提取的详细版。
- 节点库按场景分类（`catalog.rs`）；CreateCondition 组合编辑器、id 数据流输入、For+Range 直连已实现。
- `cargo test` 131 项 lib tests + 9 项 integration tests 通过；版本号 v0.3.0（欢迎页从 Cargo.toml 注入）。

---

## 待办队列

### P0 — 语法硬缺口修复（来自 syntax_coverage.md）

> 目标：解除"无法表达/无法绕行"的缺口，保证常见脚本都能用节点拼出合法 `.code`。

- [x] **CreateEventListener / CreateEventListenerLocal 节点** ✅ 2026-07-16
  - 官方：`CreateEventListener(LabelName, EventName[, Named_Or_Unnamed_Parameter]...)`（kb part_004:204）
  - 事件驱动监听器，`SetEvent` 触发时执行，注入 `__eventdata_` / `__eventname_` 局部变量；轮询 Listener 语义无法替代
  - 实现：NodeType 新增两变体（168→170）；`labelName`/`eventName` 按官方签名走位置参数，`params` 对象展开为命名参数；`out_name` 数据输出；概览图新增两种关系边；节点库归入 `scene.mission_flow.threading`；zh/en i18n 键齐全 |
- [x] **StopAudio 节点** ✅ 2026-07-16
  - 官方：`StopAudio(InstanceID[, FadeOutTime])`（kb part_003:1764）
  - 全局函数（非对象方法），CallMethod 无法表达；停止 `Audio.Play()` 返回的实例
  - 实现：A 类显式生成位置参数 `StopAudio(id)` / `StopAudio(id, fade)`；NodeType 170→171；API 分类 `Game API`，场景分类 `scene.visual_ui.audio_screen`；zh/en i18n 键齐全 |
- [x] **_stagechanged / _name 全局变量节点** ✅ 2026-07-16
  - 官方：kb part_002:118 / :140
  - `_stagechanged`（Boolean，本帧是否发生场景切换）是监听器中做一次性初始化逻辑的常用手段；`_name` 为当前工程文件夹名
  - 实现：C 类纯数据节点 `GetStageChanged`（Boolean，输出 `_stagechanged`）与 `GetProjectName`（String，输出 `_name`）；NodeType 171→173；归入 `Variables & Globals` 与 `scene.data_get.global_vars`；zh/en i18n 键齐全 |
- [x] **TriggerSexOrgasm 节点** ✅ 2026-07-16
  - 官方：`TriggerSexOrgasm()`（kb part_003:1687）
  - 原子语义（隐含 ecstasy=1），组合 SetEcstasy(1)+SetAction 只是近似
  - 实现：B 类 Flow 节点，无参数，生成 `TriggerSexOrgasm()`；NodeType 173→174；API 分类 `Game API: Stats`，场景分类 `scene.data_set.player_state`；zh/en i18n 键齐全 |
- [x] **生成器 elseif 折叠** ✅ 2026-07-16
  - 官方：`elseif` 关键字（kb part_003:66）
  - False 分支首节点为 If 且无其他入度时，生成 `elseif` 而非嵌套 `else { if ... }`；提升生成代码可读性
  - 实现：修改 `generate_if`，遍历 false 链，对单一流入前驱的 If 节点生成 `elseif`，否则回退 `else`；新增 `is_single_flow_predecessor` 辅助函数；新增 1 个 elseif 链生成器测试；`cargo test` 136 项通过 |
- [ ] **SetVariable 复合赋值**
  - 官方：`i += 1` 等（kb part_002:166）
  - 参数增加 `op` 枚举（`=` / `+=` / `-=` / `*=` / `/=`），默认 `=`；避免 `i = i + 1` 的多节点拼凑
  - 位置：`src/api/definitions.rs`（SetVariable 定义）+ `generator.rs`（赋值生成）

> 节点变更必须同步更新 `docs/node_types.md`（A/B/C 分类与计数），并补充生成器测试。

### P1 — 高频 API 补节点

- [ ] **Log 增加 level 枚举**（Info/Warning/Error），覆盖官方 `Warning` / `Error`（kb part_003:13-23）；一个参数覆盖两个缺失 API
- [ ] **Translate 节点**：`Translate(Key[, Param1][, Param2]...)`（kb part_003:156），本地化高频函数
- [ ] **List 六方法节点**：Insert / Remove / Count / Contains / IndexOf / Keys（kb part_004:50-100），List 是核心集合类型，目前全靠 CallMethod 手输
- [ ] **NPC 高频方法节点**：Warp / AddWaypoint / IsAlive / SeesPlayer / SeesFlashing（kb part_004:822-1017，共 22 个方法，先做 5 个）
- [ ] **FunctionExists / GetModVersion**：跨 mod 防御性调用与依赖检查

### P2 — 体验轮子（ backlog ）

- [ ] **_state 探针选择器**：`_state.Position.x`、`_state.Camera.pitch`、`_state.Handcuffs.Type` 等嵌套路径树形选择，类型安全输出（复用命名空间选择器模式）
- [ ] **CallMethod 方法下拉**：选中对象类型后弹出方法下拉 + 参数模板，替代手输大小写敏感方法名（覆盖 50+ 对象方法的低成本方案）
- [ ] **For 自带 start/stop/step**：无 iterable 连线时自动包装 `Range()`
- [ ] **CreateArea cuboid 参数集**：官方支持 sphere/cylinder/cuboid 三种，当前缺 cuboid（x1..z2, w, h）
- [ ] **条件表达式实时校验**：括号配平、token 合法性提示（条件组合编辑器增强）

### P3 — 发布准备（延续旧版未完成项）

- [ ] **P3.3 手动冒烟测试**：按 `docs/test_checklist.md`（v0.3.0 版）跑一遍新建 → 编辑 → 导出完整流程
- [ ] **P3.4 构建与打包**：Release 构建，打包字体、命名空间、README、AGENTS.md、LICENSE
- [ ] **P3.5 发布预览版**：GitHub Release `v0.3.0-alpha`，附已知限制说明

---

## Agent 交付规则

1. **更新 `CHANGELOG.md`** — 每次功能交付后追加条目。
2. **更新 `docs/TODO.md`** — 标记已完成任务 ✅，追加工作日志条目。
3. **`cargo test` 全过再 commit** — 131 项全部通过为提交门槛。
4. **任何任务完成后必须提交一次 commit** — 不要留下未提交改动。
5. **commit message 用中文前缀** — 格式 `<类型>: <简要描述>`，例如：`新增: EventListener 节点`。
6. **重大文档变更需归档** — 将旧版按 `{文件名}_{YYYYMMDD}_v{序号}.md` 放入 `docs/archive/`。

---

## 用户备注区

- P0 的判定标准是"无法绕行"：EventListener 与 StopAudio 是仅有的两个语义上无法替代的缺口；其余 P0 项（elseif、复合赋值）是生成质量问题。
- 对象方法生态不追求 100% 节点化：冷门 API 走 CallMethod，高频的才做专用节点（P1），更通用的解法是 CallMethod 方法下拉（P2）。
- 新增节点前先读 `docs/node_types.md` 与 `AGENTS.md` 的节点修改强制规则（A/B/C 类）。
- `docs/syntax_coverage.md` 是本轮 P0–P2 的来源文档，缺口细节（官方签名、kb 行号）以它为准。

---

## Agent 工作日志

| 日期 | 任务编号 | 说明 | 状态 |
|------|----------|------|------|
| 2026-07-16 | 实现-P0.5 | 生成器 `elseif` 折叠：修改 `generate_if` 支持 `elseif` 链，False 分支为单一流入前驱的 If 节点时折叠，否则回退 `else`；新增辅助函数 `is_single_flow_predecessor` 与 1 个 elseif 链生成器测试；`cargo test` 136 项通过 | 已完成 |
| 2026-07-16 | 实现-P0.4 | 新增 `TriggerSexOrgasm` 节点：B 类无参数 Flow 节点，生成 `TriggerSexOrgasm()`；NodeType 173→174；API 分类 `Game API: Stats`，场景分类 `scene.data_set.player_state`；zh/en i18n；`cargo test` 135 项通过 | 已完成 |
| 2026-07-16 | 实现-P0.3 | 新增 `GetStageChanged` / `GetProjectName` 全局变量节点：C 类输出 `_stagechanged` / `_name`；NodeType 171→173；归入 `Variables & Globals` 与 `scene.data_get.global_vars`；新增 2 个生成器测试（含专项验证）；`cargo test` 135 项通过 | 已完成 |
| 2026-07-16 | 实现-P0.2 | 新增 `StopAudio` 节点：A 类显式生成位置参数 `StopAudio(id)` / `StopAudio(id, fade)`；NodeType 170→171；API 分类 `Game API`，场景分类 `scene.visual_ui.audio_screen`；补充 zh/en i18n；新增 1 个专项生成器测试；`cargo test` 134 项通过 | 已完成 |
| 2026-07-16 | 实现-P0.1 | 新增 `CreateEventListener` / `CreateEventListenerLocal` 节点：NodeType 168→170；definitions 注册（Threading & Concurrency，B 类）；生成器复用 thread/listener 特判，`labelName`/`eventName` 走位置参数、`params` 对象展开；`evaluate_data_output` 支持 `out_name`；概览图新增两种关系边；catalog 归入 threading 子分类；zh/en i18n 键；新增 2 个专项生成器测试；同步更新 `docs/node_types.md` 与 AGENTS.md 计数；`cargo test` 133 项通过 | 已完成 |
