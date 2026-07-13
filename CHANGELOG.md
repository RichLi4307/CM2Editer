# Changelog

所有显著变更均记录于此。

格式遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)。

## [Unreleased]

### 新增（P1 低难度节点）

- **DestroyListener**：销毁当前监听器，生成 `listener = null`。
- **GetCurrentThread**：纯数据节点，输出当前线程引用 `_this`。
- **WaitForThread**：Flow 节点，等待子线程结束，生成 `{thread}.WaitForFinish()`。
- **全局变量数据节点**：GetSave、GetTime、GetTimeDiff、GetSettings、GetMod、GetMods，分别输出 `_save`、`_time`、`_timediff`、`_settings`、`_mod`、`_mods`。
- **For + Range 直连**：当 `For.iterable` 通过 Data 边连接 `Range.out_list` 时，生成 `for i in Range(0, 10)`。

### 新增（P1 节点分类与语义修正）

- **节点分类重构**：按 `.code` 语言概念将全部 168 个节点重新分为 11 个类别（Threading & Concurrency、Control Flow、Variables & Globals、Literals、Math & Logic、Conditions & Queries、Game API、Game API: Stats、Objects、String / File / List、Editor-only），同步更新 `src/api/definitions.rs` 与 `src/ui/theme.rs`。
- **通用变量节点**：
  - `Variable`（C 类纯数据节点）：读取当前作用域变量，生成变量名引用。
  - `SetVariable`（B 类 Flow 节点）：将值赋给当前作用域变量，生成 `name = value`。
- **代码生成语义修正**：移除标签末尾无条件追加的 `_result = null`，`_result` 仅在显式 `Return` 节点时生成。
- **全量节点生成测试**：在 `src/code_gen/generator.rs` 测试模块新增 13 个分类测试，覆盖全部 168 个节点，确保生成不 panic 且输出预期语义片段。

### 修复

- **CallFunction 函数名引号**：动态函数名现在不再被额外包裹引号，生成 `Foo(args)` 而非 `"Foo"(args)`。

### 文档

- 更新 `docs/agent_prompt.md`：将 commit message 规范改为中文前缀，并明确要求任何任务完成后必须提交一次 commit。
- 更新 `docs/TODO.md`：新增「Agent 交付规则」小节，规定完成任务必须更新 `CHANGELOG.md`、`docs/TODO.md`，并运行 `cargo test` 全过后再提交。
- 更新 `docs/node_types.md`：节点数量 159 → 168，新增线程/监听器与全局变量数据节点章节。
- 重写 `docs/tutorial_make_code.md`：适配实际 `.code` 生成结构、当前 Data 节点连线方式，新增 WaitForThread、Range+For、全局变量等章节，补充常见误区。
- 新增 `docs/architecture_evaluation.md`：基于 `.code` DSL 语义、当前实现和跨编辑器模式研究，系统评估 Start/Label/Flow 边/节点分类的错配，提出线程/标签容器化 redesign 方向与迁移路线图。
- 归档旧版文档：`docs/TODO.md` → `docs/archive/TODO_20260713_v8.md`，`docs/node_types.md` → `docs/archive/node_types_20260713_v1.md`。
- 重写新架构核心文档：`docs/TODO.md`（新架构待办）、`docs/node_types.md`（按 `.code` 语言概念分类）、`docs/json_schema.md`（v2.0 容器化格式）、`docs/agent_prompt.md`（v3.0-architecture）、`docs/tutorial_make_code.md`（基于容器化模型）、`docs/migration_guide.md`（v1.x → v2.0 迁移指南）。
- 完成 P0 核心图模型重构：
  - 新增 `src/graph/container.rs`：容器化数据结构 `ThreadContainer` / `LabelContainer` / `ListenerContainer`。
  - 重写 `src/serializer/json.rs`：JSON 格式升级为 `2.0`，顶层结构 `threads: [...]`，不再兼容 v1.0。
  - 重写 `src/code_gen/generator.rs`：基于容器化图生成 `.code`，移除 BFS 子标签推断。
  - 重写 `src/graph/validation.rs`：按容器检查，移除 Flow DAG/菱形警告，新增标签名唯一性、容器内边检查。
  - 从 `NodeType` 枚举中移除 `Start` / `Label`（变体数 168 → 166）。
  - 更新 `src/project.rs`：新建工程默认生成 `main` 线程容器。
  - 重新启用 `src/app.rs` 和 `src/ui` 模块，并迁移到容器化模型。
### 新增（P2 — UI 与编辑器重构）

- 将 `App` 内部图模型从旧 `Graph` 迁移到 `ContainerGraph` / `GraphDocument`，新增 `SelectedContainer` 与 `ContainerKind` 跟踪当前编辑容器。
- 左侧工程树按 `.code` 文件 → 线程 → 标签/监听器层级展示，点击切换画布容器。
- 画布仅渲染当前 `LabelContainer` / `ListenerContainer` 内部节点与边；移除旧 `Start` / `Label` 节点显示。
- 新增入口钉渲染（`src/ui/entry_pin.rs`）：为当前容器绘制 Flow 起点，并连接到入口节点 `in_flow` 端口。
- 新增线程概览图面板（`src/ui/panels/overview.rs`）：以网格布局展示标签/监听器及 `Goto` / `CreateThread` / `CreateListener` / `ForeachNode` 关系，双击节点跳转对应容器。

### 测试

- 新增 `ui::panels::overview` 单元测试：验证概览图能从 `Goto` 节点提取目标标签关系并生成唯一布局。
- 新增 `graph::container::tests::test_entry_node_id_prefers_top_left_no_incoming_flow`，验证入口节点按位置和无入边规则稳定选择。
- `cargo test --lib`：94 个 lib tests 全部通过。

### 修复

- 统一入口节点判定逻辑：新增 `LabelContainer::entry_node_id()`，按“最靠左上、无 Flow 入边”稳定选择入口节点；`src/ui/entry_pin.rs` 与 `src/code_gen/generator.rs` 复用同一逻辑，避免 HashMap 顺序导致入口钉/生成结果跳动。
- 修复 `GetSave` 节点定义：新增 `key` 参数，输出类型改为 `Any`，代码生成器输出 `_save.key`，与 `docs/node_types.md` 描述一致。
- 恢复 `src/main.rs` 启动 eframe 应用，移除 P0 时期的 UI 屏蔽提示。

### 文档

- 重写 `docs/tutorial_make_code.md`：对齐当前 UI 工作流程（默认 `main` 标签、入口钉说明、节点连接、保存与导出），并注明当前 UI 暂不支持新建标签/线程/监听器。
- 将教程第五步改为使用 `GetStateNumber(Rank)` 读取角色等级/经验，区分 RP、`_state` 状态与 `_save` 存档读取；补充 RP 与角色经验键名说明。
- 新增项目根目录 `AGENTS.md`（Kilo 风格），集中存放 AI 开发约束、文档置信上下级与查阅指南；原 `docs/agent_prompt.md` 内容已合并迁移。

---

## [0.2.2] — 2026-07-10

### 修复（Data 端口链路）
- **Goto `out_label` 端口**完整可用：节点定义 + 代码生成器 + `evaluate_data_output` 三级配合，输出实际 label 值而非变量名 `var_xxx`。
- **CreateListener/CreateThread `out_name` 端口**同样修复：`evaluate_data_output` 新增映射，`out_name` → `labelName` 参数值。
- **Label 节点 Data 边自动获取名称**：`collect_labels` 扫描 Label 节点并通过 `resolve_param_opt` 解析 `name` 参数（Data 边值优先于文本框输入）。
- **节点预览**：连接到 Data 边的参数在画布节点上显示 `🔗` 链接状态，不再顽固显示空默认值。

### 修复（代码生成与验证器）
- **验证器 BFS 重构**：不可达检查同时从 Start 节点和各子标签入口节点出发，子标签（如 `check_loop`）内的 Flow 链不再误报"未连接 Start"。
- **CreateCondition.id / CreateItemCondition.id** 从 required 改为 optional，不填时不输出 `id=""`。
- **CONDITION_TYPES** 下拉补全 6 个 `Exposed_*` 暴露条件（`Exposed_None`, `Front`, `Upper`, `HipCrouch`, `Hip`, `All`）。

### 改进
- **节点库拖拽**：从左侧面板拖出节点到画布，带蓝色虚影跟随。
- **全局热键门控**：文本框焦点时所有画布热键（Ctrl+Z/Y/C/V/Delete/Space）全部屏蔽，只对文本框生效。
- **Ctrl+V 事件双保险**：`consume_key` + `Event::Paste` 两套机制同时门控。
- **窗口初始化优化**：`eframe wgpu` 后端 + `desired_maximum_frame_latency=1` + `opt-level=1` debug profile，减少闪烁。
- **ParamTextEdit 持久缓冲区**：跨帧保形，回车/失焦才提交，不吞字、不闪烁。

### 文档
- `agent_prompt.md` 新增 A/B/C 三类节点修改规则 + `evaluate_data_output` 分支要求
- `node_types.md` 新增代码生成兼容性章节
- `docs/hotkey_management.md` 热键管理规范
- `docs/test_checklist.md` 70 项测试清单

### 测试
- `cargo test --lib` : 93 项通过
- `cargo clippy` : 仅 4 个 pre-existing 警告

---

## [0.2.1] — 2026-07-10

### 新增
- `Label` 节点名称变化时自动注册到标签管理器
- `CreateThread`/`CreateListener`/`CreateListenerLocal` 新增 `out_name` 端口（String 输出标签名）
- `StringConstant` 节点：输出字面量 String 值
- `NumberConstant` 节点：输出字面量 Number 值
- `CheckCondition` 节点：条件 Object → Boolean（`cond.Check()`）
- `CheckEquipment` 节点：检查装备（`_state.AdultToys.{type} != null`）
- `CheckCosplay` 节点：检查服装（`Cosplay_{key}`），接入命名空间选择器
- `CompareNumbers` a/b 参数支持属性面板手动填写常量（数据端口优先）
- 节点库拖拽到画布落点创建（带拖拽虚影跟随）
- `ParamTextEdit` 统一文本组件——持久缓冲区，回车/失焦提交，不吞字
- 属性面板参数纵向布局 + 节点介绍标题
- 标签管理面板：删除/重命名按钮（非 main 标签）

### 修复
- 全局热键门控：文本框焦点时 Ctrl+Z/Y/C/V/Delete/Space 全不对画布生效
- Ctrl+V 粘贴事件双保险（consume_key + Event::Paste 两套机制）
- `Object`/`List` 类型参数跳过 ComboBox 直通文本框
- `warn_unreachable_nodes` 跳过数据节点和独立标签入口节点
- `CompareNumbers` 4 端口→2 端口（删除从 `with_inputs` 产生的重复端口）
- `CreateListener` 缺失 `with_params` 导致属性面板无法编辑
- 欢迎页修复 + 窗口自适应屏幕大小
- 多热键文档 `docs/hotkey_management.md`

### 文档
- `docs/node_types.md` 重写：159 个实际实现的节点清单
- `docs/tutorial_make_code.md`：6 步实战教程（Flow/Data/Listener/Goto/CheckCondition 全流程）
- `docs/if_condition_design.md` 按设计文档验收
- `docs/test_checklist.md` 70 项测试清单
- `docs/test_flow_diagram.md` 流程设计图

### 测试
- NodeType 变体：158 → 159
- `cargo test --lib`：93 项通过
- `cargo clippy`：仅 4 个 pre-existing 警告

---

## [0.2.0] — 2026-07-09

### 新增
- 代码生成重构：`CreateThread` 顶层生成、`_result=null` 标签收尾、`thread.Goto` 语法
- GoTo/CreateThread 目标标签自动发现与注册
- If/While 条件模板下拉框（30+ 预设表达式）
- `evaluate_data_output()` Data 边递归解析链
- 7 个 Boolean 管道节点（Boolean/GetStateBool/GetStateNumber/CompareNumbers/LogicAnd/LogicOr/LogicNot）
- 3 个坐标节点（GetPosition/MakeVector/BreakVector）+ 选择器 + 16 预设
- cosplay 命名空间二级分类 + 中文翻译 188 项（EMBEDDED_COSPLAY_LIB）
- 三标签左栏（工程/命名空间/坐标）
- If 条件模板下拉

### 修复
- `.code` 语法对齐：`If(true) [`→`if true`、`While→while`、`For→for in`、`Break→break`
- Bodypaint 类型 Boolean→Number 修正
- JSON 加载必填参数默认值补填
- egui ID 冲突：7 个 ScrollArea + 1 CollapsingHeader `id_salt`
- 底栏三合一 + ScrollArea 防溢出
- 字体 110MB→32MB

### 文档
- `docs/code_api_reference.md`：基于英文官方文档 + 80+ .code 反推的 DSL 权威参考
- `docs/code_pseudocode_map.md`：112 个 .code 文件的 Python 伪代码映射表

---

## [0.1.1] — 2026-07-09

### 新增（Phase 6：Monitor→Condition 管道）
- 7 个纯数据 Boolean/Condition 节点：`Boolean`、`GetStateBool`、`GetStateNumber`、`CompareNumbers`、`LogicAnd`、`LogicOr`、`LogicNot`。无 Flow 端口，通过 DataFlow 连线组合后喂给 If/While。
- `evaluate_data_output()` 递归解析 Data 边链，生成完整 `.code` 表达式（`(_state.Ecstasy >= 50) && (_state.NearNPC)`）。
- If/While 属性面板新增 30+ 条条件模板 ComboBox，Data 连线时自动隐藏。

### 新增（坐标系统）
- 3 个坐标节点：`GetPosition`、`MakeVector`、`BreakVector`。
- 坐标预设注册表（`src/api/coordinate.rs`）+ 16 个默认坐标（`assets/coordinates/default.json`）。
- 坐标选择器浮动窗口：按场景分组卡片 + 搜索。
- Vector 参数属性面板显示 📍 按钮，GetPosition 节点一键填充。

### 新增（命名空间管理）
- 命名空间面板改为交互式卡片布局：CollapsingHeader 一级分类 + 二级子分类，点击卡片复制 key。
- cosplay 命名空间 188 条目全部带中文翻译（来源：EMBEDDED_COSPLAY_LIB）。
- 命名空间选择器窗口支持二级分类展示。
- Import/Export/Add 按钮（内联表单直接写 JSON）。

### 新增（UI）
- 启动欢迎页：标题 + 打开工程/新建工程按钮 + Space 快捷键提示。
- 左栏重构为三标签：工程（合并节点库）、命名空间、坐标。
- 代码生成语法对齐 CM2 DSL：`If(true) [`→`if true`、`While→while`、`For→for in`、`Break→break`。
- egui ID 冲突修复：7 个 ScrollArea + 1 个 CollapsingHeader 加 `id_salt`。
- 参数类型标签（`[str]`/`[num]`/`[xyz]`）+ Vector 多字段编辑器。
- 窗口自适应屏幕大小（3000×2000 → OS clamp）。

### 修复
- 底栏弹回/自缩：三合一面板统一控制。
- Bodypaint 类型从 Boolean 修正为 Number。
- Data-only 节点不再误报"不可达"警告。
- emoji 按钮白框：全部替换为纯文本标签。
- JSON 加载时补填缺失必填参数默认值。
- 文本输入框限宽，防止撑开左边栏。
- 命名空间反序列化修复：`category` 字段正确读取。

### 测试
- NodeType 变体：143 → 153。
- `cargo test`：109 项全部通过。
- `cargo clippy`：仅 4 个 pre-existing 警告。
- Release 构建产物：`target/release/CM2Editer.exe`（~6.7 MB）。

> **维护说明**：每次新增变更时，在所有已有条目的**末尾追加**新条目，严禁删除或覆盖旧条目。所有旧版本均保存在 git 历史中（`git show <commit>:CHANGELOG.md`）。

---

## [Unreleased]

### 新增

- 框选：按拖拽方向区分 Window（左→右，严格包含）与 Crossing（右→左，相交）模式，绘制半透明选择框。
- 多选拖拽：框选多个节点后拖动任一节点，所有选中节点同步移动。
- 连线交互：支持单击/框选连线，选中后 Delete 删除；`EdgeRenderer` 新增命中矩形。
- 拖线反馈：兼容端口高亮，不兼容/已占用/会成环端口显示红/橙色提示，非法连接被拒绝。
- 复制/粘贴：右键菜单与全局快捷键 Ctrl+C/Ctrl+V，支持多节点及节点间连线复制。
- 右键删除节点：级联删除关联边，撤销可恢复。
- 加载 JSON：工具栏“打开 JSON”按钮使用 `rfd` 文件对话框。
- 空画布体验：启动为空画布，显示欢迎提示；工具栏新增“新建”按钮。
- 验证器可视化：`GraphValidator` 返回环上节点 ID，错误节点渲染红色边框。
- 损坏 JSON 处理：`load_json` 错误通过状态栏显示，程序不 panic。
- `Vec2` 新增 `Add`、`Sub`、`AddAssign`、`Div<f32>`、`DivAssign<f32>` 运算实现。

### 变更

- 中文显示字体由"仅 Windows 系统字体回退"改为"优先加载内置思源黑体（Source Han Sans SC），缺失时回退到系统字体"。
- 将 `src/思源黑体/` 移至 `assets/fonts/思源黑体/`，并保留 `LICENSE.txt` 以符合 SIL Open Font License 1.1 要求。
- `src/app.rs` 的 `setup_fonts` 优先读取 `assets/fonts/思源黑体/OTF/SimplifiedChinese/SourceHanSansSC-Regular.otf`，同时加载 Bold 字体以支持粗体中文。
- `FlowError` 改为可 `Clone`，`CycleDetected` 携带环上节点 ID；`Io`/`Json` 变体存储字符串以支持 Clone。
- `StatusBarPanel` 接收 `&[FlowError]` 以显示错误数量。

### 已解决问题

- P0.1 滚轮缩放跳跃：改为方向判定，一格放大/缩小 10%。
- P0.2 中键平移去重：仅 `Canvas` 处理，移除 `interaction.rs` 的 `Panning` 状态。
- P1.1 中文显示方块：优先使用内置思源黑体，缺失时回退 Windows 系统字体（微软雅黑 / 黑体）。
- P1.2 左栏目录排序：分类聚合改用 `BTreeMap` 稳定排序。

### 文档

- 归档旧 `docs/紧急修复清单.md` 至 `docs/archive/紧急修复清单-2026-07-07-14-34.md`。
- 重构并新增 `docs/问题清单.md`，仅保留当前未解决问题，末尾生成临时检查单。
- 更新 `README.md`，说明编辑器内置思源黑体字体及其许可证。
- 更新 `docs/问题清单.md`，标记 P0–P3 问题为已解决并附手工验证清单。

### 修复（2026-07-08 第三轮）

- Log 节点参数编辑：将 `output` 参数类型从 `List` 改为 `String`，新建 Log 节点可正常输入非数字字符串。
- 空图导出警告：导出 `.code` 时若图为空，弹出提示"图为空，导出的 .code 文件内容为空"。
- 多 Start 节点标签：无显式标签时，每个 Start 节点生成独立标签 `main`、`main_1`、`main_2`…
- Space 快捷键屏蔽：在文本输入控件中按 Space 不再误触搜索窗口，仅在画布无键盘焦点时触发。
- 折叠节点高度：折叠时节点高度自适应为标题栏 + 端口最小高度。
- 导出 JSON 路径同步：导出 JSON 成功后更新 `current_file`，后续 Ctrl+S 直接保存到同一文件。

### 修复（2026-07-07 第二轮）：修复单击节点无法选中（之前仅拖拽可选中），单击边/空白正常清除选区

- 快捷键失效：全局快捷键 Ctrl+C/Ctrl+V/Ctrl+Z/Ctrl+Y/Delete 改用 `consume_key`，避免被文本输入消费。
- 保存/导出对话框：保存 JSON、导出 JSON、导出 .code 按钮均改为 `rfd` 文件保存对话框。
- 框选虚线：Crossing 模式（右→左拖拽）选择框改为虚线绘制。
- 图层顺序：连线渲染在节点下层，选中节点置顶渲染，多选拖动时被联动节点不会被未选中节点遮挡。
- 视口裁剪：仅渲染画布可见区域附近（边距 50px）的节点和连线，大幅优化大量节点时的帧率。
- 拖线目标端口填充：拖拽连线时悬停的输入端口圆点颜色随目标状态变化（兼容/不兼容/占用/成环）。
- 空白处右键菜单：右键画布空白处可弹出"粘贴"菜单（需剪贴板非空）。
- 搜索窗口关闭：搜索窗口支持 Escape 键和 ✕ 关闭按钮退出。

### 新增（Phase 4.5：工程/项目管理）

- 工程管理核心模型：新增 `src/project.rs`，实现 `MissionMeta`、`Setting`、`Project`、`CodeFile` 数据结构，支持 `meta.json` 多语言标题/描述和设置项。
- 工程文件夹结构：一个工程 = 文件夹 + `meta.json` + 多个 `.code` 文件 + 编辑器内部 `.cm2editor/*.code.json` 节点图。
- 新建/打开工程：工具栏新增"新建工程""打开工程"，支持选择父文件夹并输入工程名称，自动创建 `meta.json` 和带 `Start` 节点的 `main.code`。
- 保存工程：工具栏"保存工程 (Ctrl+S)" 同时写回 `meta.json`、所有 `.code` 文件和内部节点图 JSON。
- 导出工程：工具栏"导出工程"将项目文件夹（含 `meta.json`、`.code`、资源目录）复制到目标文件夹（如 `CustomMissions2`）。
- 左栏工程文件树：新增"工程"/"节点库"标签页，支持查看 `meta.json`、切换 `.code` 文件、新建/重命名/删除 `.code` 文件。
- 右栏 `meta.json` 编辑器：以文本形式编辑 `meta.json`，实时解析并提示 JSON 错误。
- 底部 `.code` 文本编辑器：查看和手动编辑当前 `.code` 文件代码，支持从节点图重新生成。
- 多 `.code` 文件切换：每个 `.code` 文件对应独立节点图与画布状态，切换时自动同步当前图到工程。

### 变更（Phase 4.5）

- 工具栏由单一文件操作升级为工程操作：移除"打开 JSON"/"导出 JSON"，改为"打开工程"/"保存工程"/"导出工程"；保留单文件"导出 .code"作为兼容入口。
- `App` 移除旧 `current_file`，改为 `project: Option<Project>` 管理当前工程状态。

### 文档（Phase 4.5）

- 更新 `README.md`：说明工程文件夹结构、多 `.code` 管理、`meta.json` 编辑和导出到 `CustomMissions2`。
- 更新 `docs/TODO.md`：标记 Phase 4.5 全部 P0 任务完成。
- 更新 `docs/agent_prompt.md`：反映工程管理已实现，并补充 `project/` 模块边界。

### 测试（Phase 4.5）

- 新增 `src/project.rs` 单元测试：`MissionMeta` 序列化、Setting 反序列化、工程创建/打开/增删改 `.code` 文件。
- `cargo test`：81 个 lib tests + 16 个 integration tests 全部通过，0 失败。

---

## [0.1.0] — 2026-07-08

### 新增（Phase 5.1.1：DataFlow 重构）

- 节点参数自动生成 Data 输入端口（ID = 参数名），端口数自适应节点高度。
- DataFlow 边以**虚线**渲染，仅在单选相关节点时显示；Data 边自身被选中时也保持可见。
- 属性面板新增数据源下拉框：可切换"字面量"或引用其他节点的 Data 输出端口。
- 底部面板拆分为 JSON 预览（左）与 Data 菜单（右），通过 `Separator` 分隔。
- 代码生成优先使用 Data 端口连接的值，无连接时回退到 `ParamValue`。
- 新增 4 项单元测试；`cargo test` 96 项通过。

### 新增（Phase 5.1.2：参数类型重构）

- `src/api/enums.rs`：从 `documentation_zh.html` 提取游戏常量——场景类型、动作、技能、物品、掉落物、音效、成人玩具、手铐类型、振动器/活塞强度、体位、图形选项、条件类型。
- `ParamType::Enum` 变体，映射到 `PortType::String`，默认值为第一个选项。
- `ParamDefinition::options` 字段填充枚举选项；**20+ 个节点参数**从手动输入改为下拉选择。
- `PropertiesPanel` 对含 options 的参数渲染 `ComboBox` 下拉框。

### 新增（Phase 5.1.3：命名空间管理）

- `src/api/namespace.rs`：`Namespace`、`NamespaceEntry`、`NamespaceRegistry` 数据结构。
- 7 个命名空间 JSON 文件（`assets/namespaces/`）：cosplay、adult_toy、avatar_type、fixed_type、hair_type、body_paint_type、player_data。
- 浮动命名空间选择器窗口：搜索、多选/单选、中英文显示名切换。
- `cosplayKeys`、`toyNames`、`avatarType`、`dataName` 参数绑定到对应命名空间选择按钮。
- App 启动时自动加载命名空间注册表。

### 新增（Phase 5.2.2 + 5.2.3 + 5.2.4：静态检查与错误详情）

- `FlowError::Warning` 变体：非阻塞警告，不影响代码生成但展示在状态栏。
- 多 Start 节点警告：存在 >1 个 Start 时提示。
- 不可达节点警告：BFS 从 Start 沿 Flow 边遍历，标记不可达节点。
- 菱形依赖警告：检测 `Start→B` 与 `Start→A→B` 多路到达同节点的菱形结构。
- 状态栏错误计数改为可点击链接；点击弹出 `ErrorDetailWindow` 列出全部错误/警告描述。

### 新增（Data 面板 UI 重构）

- DataFlow 面板改为**水平换行方块**（巧克力板布局），每块 64×40px，按端口类型着色。
- 点击方块选中画布中对应节点。
- DataFlow 虚线可单独点选/框选；`delete_selected` 优先删连线（有连线选中时绝不删节点）。
- Data 边在自身被选中时继续保持渲染，确保点击选中后虚线不消失。

### 新增（底部面板重构）

- 三合一布局：`代码 ┃ JSON ┃ DataFlow` 并排于同一可拖拽 `TopBottomPanel`。
- 两条 6px 宽**可见竖直分隔线**（灰色/蓝色，鼠标悬停变 `ResizeHorizontal`），拖拽调整三列宽度，`drag_delta()` 安全计算比例。
- `.code` 预览和 JSON 预览均放入 `ScrollArea`，防止长内容撑高父面板。
- 底部面板整体高度可拖拽，`resizable(true)` + 固定 `default_height`。

### 修复

- egui ID 冲突：7 个 `ScrollArea` + 1 个 `CollapsingHeader` 添加 `id_salt`，消除运行时红色 ID clash 警告。
- 代码生成按钮无效：`CodeEditorPanel` 接收 `&Graph` 参数，点击时先同步实时 graph 再生成。
- 旧图档必填参数 Null 错误：`NodeData → Node` 反序列化时自动补填缺失 required 参数的默认值。
- 底栏弹回/自缩：移除 `default_height ⇒ ui.available_size() ⇒ data_mut` 循环写逻辑，改用常量默认值。
- `node_library` 的 `ui.collapsing` 废弃 API 替换为 `CollapsingHeader::new().id_salt()`。

### 变更

- `GraphValidator` 新增 `warn_multiple_starts`、`warn_unreachable_nodes`、`warn_diamond_reachable` 三个非阻塞检查。
- `FlowError` 新增 `is_blocking()` 和 `is_warning()` 方法。
- `StatusBarPanel::show` 返回 `StatusBarEvent` 以支持点击事件。
- 底部面板 ID 从 `"bottom_panel"` 迁移为 `"bottom_main"`（合并后）。

### 文档

- `README.md` 更新至 v0.1.0：Phase 5 进度、DataFlow 功能、项目结构、Release 构建说明。
- `docs/TODO.md` 标记 5.1.2–5.4.1 为已完成。

### 测试

- `cargo test --lib`：92 项通过。
- `cargo test`（完整）：108 项通过（92 lib + 16 integration）。
- `cargo clippy`：仅 4 个 pre-existing 警告，零新增。
- Release 构建产物：`target/release/CM2Editer.exe`（~6.5 MB）。

---

## [0.1.0] — 2026-07-08（续）

### 新增（Phase 6：Monitor→Condition 管道）

- 7 个纯数据 Boolean 节点：`Boolean`、`GetStateBool`、`GetStateNumber`、`CompareNumbers`、`LogicAnd`、`LogicOr`、`LogicNot`。均无 Flow 端口，通过 DataFlow 连线组合后喂给 If/While 的 condition 端口。
- `evaluate_data_output()` 递归解析 Data 边链：从源码节点沿 Data 边回溯，生成完整 `.code` 表达式（如 `(_state.Ecstasy >= 50) && (_state.NearNPC)`）。
- If/While 属性面板新增 **30+ 条条件模板 ComboBox**（字面量/角色状态/环境/装备/数值比较），选中即填入表达式，Data 连线时自动隐藏。
- 代码生成语法对齐官方 DSL：`If(true) [`→`if true`、`While(false) [`→`while false`、`For(x) as i [`→`for i in x`、`Break`→`break`。

### 修复

- **Bodypaint 类型修正**：官方文档确认为 Number（非 Boolean），从 `GetStateBool` 移至 `GetStateNumber`。
- **Break 节点**：确认 Break 自 Phase 1 即存在；仅修正 code_gen 大小写 `Break`→`break`。
- **`documentation_zh.html` 引用**：全部文档和代码注释重定向到 `docs/code_api_reference.md`（官方英文原版 + 80+ 例反推）。
- **旧图档必填参数 Null**：`NodeData→Node` 反序列化时补填缺失 required 参数默认值。
- **egui ID 冲突**：7 个 `ScrollArea` + 1 个 `CollapsingHeader` 加 `id_salt`，消除运行时红色警告。
- **底栏弹回/自缩**：三合一面板统一控制高度 + `ScrollArea` 防溢出。

### 文档

- `docs/code_api_reference.md`：基于官方英文 `documentation.html` + 80+ 个前辈手搓 `.code` 反推的 CM2 DSL 权威参考（419 行）。
- `docs/if_condition_design.md`：Monitor→Condition→If 管道架构，含 7 节点规格表、DataFlow 示例、三阶段实施方案。
- `docs/TODO.md` 重构：分 Phase 5/6/7，16 条工作日志。
- `README.md`、`CHANGELOG.md` 同步至 v0.1.0。
- 旧版 TODO 归档至 `docs/archive/TODO_20260708_v5.md`。

### 测试

- NodeType 变体：143 → 150。
- `cargo test`：108 项全部通过。
- `cargo clippy`：仅 4 个 pre-existing 警告。



