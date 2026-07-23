# Changelog

所有显著变更均记录于此。

格式遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)。

## [Unreleased]

### 重构（P3.4 UI 设计规范落地 — 阶段 A 令牌化）

- 在 `src/ui/theme.rs` 中新增 `tokens` 模块：统一提供间距、圆角、字号、背景/边框/文本/语义色等全部设计令牌。
- 将 `src/ui` 与 `src/app.rs` 中所有硬编码 `Color32` 字面量替换为 `tokens::*`；移除旧的 `Theme::BACKGROUND` 等常量。
- 新增 `tokens::with_alpha` 助手与 `stage_palette_color` 函数，用于选择器卡片等需要按场景名稳定着色的场景。
- 新增守卫测试：
  - `tokens_values_match_spec`：断言所有令牌数值与 `docs/ui_design_spec.md` 第 2 节一致。
  - `no_bare_color32_outside_theme`：扫描 `src/ui` 业务代码，禁止 `Color32::from_rgb/from_rgba/from_gray/from_rgba_premultiplied` 裸调用。
- `cargo test --lib` 196 项通过；`cargo clippy --lib` 0 warnings。

### 文档

- 新增 `docs/ui_design_spec.md`：完整 UI 设计规范与优化规划。涵盖设计令牌（间距/圆角/字号/颜色体系）、五区布局尺寸、控件规格（按钮三语义、ComboBox 宽度三档 100/160/200、选择器卡片统一 140×48）、画布节点/连线规范、弹窗统一模板、Toast 通知系统设计、P0-P2 优化清单与 A-D 四阶段实施路线图。

### 修复（P0 v0.3.0 issues）

- 修复 `If` / `While` 条件代码生成被引号包裹：新增 `resolve_condition`，对字符串字面量条件去除 JSON 引号并直接作为 `.code` 表达式输出；Data 端口连接的条件仍走 `evaluate_data_output`。
- 为 `If` / `While` 及其 `elseif` 链统一添加外层括号，生成 `if (condition) / elseif (condition) / while (condition)`，避免解释器优先级解析不稳定。
- 新建 `.code` 文件时默认使用空图，不再强制创建 `main` 线程；通过 `add_code_file` 与无内部 JSON 的 `.code` 加载回退均改为 `ContainerGraph::default_empty()`。P3.1 后新建项目的主文件也使用空图，由用户自行创建容器。
- 修复 `elseif` 分支条件编辑不直观：`properties.rs` 中对 `elseif_*_condition` 动态参数也使用条件模板下拉框（ComboBox 快速填充 + 文本微调），与主 `condition` 字段体验一致。
- 为支持空图，将 `App.selected_container` 改为 `Option<SelectedContainer>`，`current_label()` / `label_ref()` / `label_mut()` 均返回 `Option`，并同步处理所有调用点（画布渲染、属性面板、复制/删除、撤销重做等）。

### 修复（egui duplicate widget ID）

- 修复 `If` 节点属性面板中出现 `First use of widget ID FF44` / `Second use of widget ID FF44` 警告：条件模板 ComboBox 的 `id_salt` 从固定值改为按参数 key（如 `condition` / `elseif_0_condition`）生成唯一 ID；同时修复 `CallMethod` 方法下拉框使用固定 ID 的问题。
- `cargo test --lib` 191 项通过；`cargo clippy --lib` 0 warnings。

### 测试

- 更新 `test_if_elseif_else_chain` 与 `test_multi_branch_if_node` 断言，验证无引号且带括号的新格式。
- 新增 `test_new_code_file_starts_empty`，验证新建项目主文件仍保留默认线程，而 `add_code_file` 创建的附加文件为空图。
- 更新 `test_label_ref_switches_between_label_and_listener` 与 `test_label_mut_switches_between_label_and_listener`，适配 `Option<SelectedContainer>`。
- `cargo test`：155 项 lib tests + 9 项 integration tests = 164 项全部通过。

### 杂项

- 专项清理全部 23 个 `cargo clippy --lib` warning：移除多余借用/克隆/强制类型转换、`ListenerKind` 改用 `#[derive(Default)]`、合并可折叠 if、用 `is_some_and`/`next_back`/`to_vec` 替代冗余写法、`evaluate_target` 删除未使用的 `source_port` 参数、`handle_dragging_node` 改为按引用传递位置快照、`render_with_data` 标记 `#[allow(clippy::too_many_arguments)]`。
- 同步移除 `app.rs` 中因此不再使用的 `NodeDefinition` 与 `PortGeometry` import。
- `cargo clippy` 与 `cargo test` 均通过。

### 发布说明

- 版本号与 `Cargo.toml` 对齐至 **0.3.0**。
- 完整测试套件：**192 项 lib tests + 9 项 integration tests = 201 项全部通过**。
- Release 构建产物：`target/release/CM2Editer.exe`（需连同 `assets/fonts/`、`assets/namespaces/`、`assets/i18n/`、README、AGENTS.md、LICENSE 一起分发）。

### 修复（P3.1 手动冒烟测试缺陷）

- **默认线程策略**：移除新建工程自动创建 `main` 线程，新建/打开工程回退与 `App::new_graph` 均使用 `ContainerGraph::default_empty()`；项目树新增 `+T`/`-T`/`-Lb`/`-Ls` 按钮，支持创建/删除线程、标签与监听器，使用户可自行搭建容器结构而不强制默认线程。
- **搜索大小写不敏感**：`node_library` 模糊匹配改为对查询与目标均转小写，节点库与 Space 搜索均支持大写字母；补充 `test_fuzzy_match` 大小写用例。
- **描述悬停**：节点库列表、搜索窗口、画布节点悬停时显示 `I18n.node_description` Tooltip，统一工程树/画布/属性面板的信息入口。
- **文件树底部控件**：将新建/保存/导出按钮移入 `ScrollArea`，解决按钮被挤出屏幕及下降速度滞后于文件树的问题。
- **节点分类审计**：新增 `test_boolean_output_nodes_are_in_conditions`，强制所有 Boolean 输出节点必须出现在 `scene.conditions`；将 `FunctionExists`、`GetStageChanged`、`CollectItem`、`FileExists`、`ListContains`、`NPCIsAlive`、`NPCSeesPlayer`、`NPCSeesFlashing` 归入 `scene.conditions.state_check`。
- **坐标标签中文显示**：坐标面板条目改为「场景中文名 - 名称」；新增 `I18n.stage_name` 与 `stage.*` 中文翻译，其他语言保持原样。
- **Vector/Quaternion 手动输入**：空数组（默认未设置）时提供 JSON 数组文本输入，避免只能打开弹窗；Quaternion 不再打开坐标选择器（仅 Vector 使用坐标预设）。
- **NPC 输出端口下拉选择**：`npc` Object 参数新增专用下拉框，列出图中 `out_npc` 或标签为 "NPC" 的输出端口，选中后自动转为 Data 引用。
- **连线端口灰化**：拖拽连线时，`NodeRenderer` 将源端口类型不兼容的输入端口渲染为 25% 透明度，提升连接反馈。
- `cargo test --lib` 192 项通过；`cargo test` 192 lib + 9 integration 全部通过；`cargo clippy --lib` 0 warnings。

### 修复（P2 交互回归）

- 修复节点库渲染黑框：P2.1 的搜索框与场景过滤下拉框在同一行，左栏宽度不足时导致水平布局溢出。改为搜索框独占一行，过滤下拉框占满剩余宽度。
- 修复底栏两个竖直分割线可互相越过：新增每帧归一化与拖拽边界约束，保证 `split1 < split2` 且最小间距 15%，避免三列宽度异常或渲染错误。

### 优化（P2.3 CallMethod 方法下拉）

- 新增 `src/api/method_registry.rs`：统一方法注册表，按对象类型（Thread/NPC/Area/List/Audio/Text/...）收录 40+ 个对象方法，含方法名、参数类型与默认值模板。
- 属性面板对 `CallMethod.method` 参数渲染方法下拉框：
  - 若 `thread` 端口已连接对象，自动按对象类型过滤方法；
  - 未连接时列出所有方法，按对象类型分组；
  - 选中方法后自动填充方法名与 `params` 参数模板（默认值）。
- 修复 `CallMethod` 代码生成：从 `CallMethod(thread=..., method=..., params=...)` 改为正确的 `var = object.Method(args)` 语法。
- 新增 `test_generate_call_method_with_object_reference` 生成器测试；方法注册表自带 4 个单元测试。
- 新增 i18n 键 `label.select_method`（zh/en/ja）。
- `cargo test` 154 项 lib tests + 9 项 integration tests 通过。

### 优化（P2.2 属性面板参数折叠）

- 属性面板参数数量超过 4 个时，前 4 个保持展开，剩余参数默认折叠到“高级参数”区域。
- 参数按 `NodeDefinition` 声明顺序排列，避免 `HashMap` 无序带来的跳动。
- 新增 i18n 键 `label.advanced_params`（zh/en/ja）。
- 位置：`src/ui/panels/properties.rs`。
- `cargo test` 149 项 lib tests + 9 项 integration tests 通过。

### 优化（P2.1 节点库搜索增强）

- 节点库面板新增场景分类过滤下拉框，支持按 `scene.*` 一级分类快速筛选。
- 搜索匹配升级为字符级模糊匹配，同时匹配节点显示名、NodeType 名、API 分类及场景分类标签。
- 新增“最近使用”区域，记录用户最近创建的 10 个节点，去重置顶，持久化到 `%APPDATA%/CM2Editer/settings.json`。
- 位置：`src/ui/panels/node_library/mod.rs`、`src/app.rs`、`src/settings.rs`；新增 zh/en/ja i18n 键 `node_library.recent` / `filter_all` / `filter_label`。
- 测试：新增 `test_fuzzy_match`、`test_record_recent_dedup_and_limit`；`cargo test` 149 项 lib tests + 9 项 integration tests 通过。

### 文档（P2.4 节点描述 i18n 补全）

- 补全 `assets/i18n/en.json` 中全部 188 个 `node.{NodeType}.description`，每个描述均精炼为 1–2 句用户向说明，覆盖控制流、线程/监听器、变量、游戏 API、数学/向量、字符串/列表/文件、对象构造等全部类别。
- `assets/i18n/zh.json` 中对应中文描述已保持完整覆盖，未改动节点名称、端口与参数键。
- 验证：`cargo test --lib` 154 项通过；`cargo clippy --lib` 23 个警告（均为 pre-existing，无新增）。

### 新增（P2.5 节点收藏 / 置顶）

- 在 `src/settings.rs` 中新增 `favorite_node_types` 持久化字段，与 `recent_node_types` 一起保存到 `%APPDATA%/CM2Editer/settings.json`。
- `src/app.rs` 加载/保存收藏列表，并在节点库顶部和搜索窗口顶部显示“收藏”区域。
- `src/ui/panels/node_library/mod.rs` 为每个节点项增加星标切换按钮，支持在库列表和搜索窗口中快速收藏/取消收藏。
- 新增 i18n 键 `node_library.favorites` / `button.favorite` / `button.unfavorite`（zh/en/ja）。
- `cargo test --lib` 173 项通过；`cargo clippy --lib` 0 warnings。

### 新增（P2.6 _state 探针选择器）

- 新增 `src/ui/panels/state_picker.rs`：提供 `_state` 嵌套路径树形选择器，按 Boolean / Number 类型分组（角色状态、属性、Position、Camera、AdultToys 等）。
- `GetStateBool` / `GetStateNumber` 的 `stateKey` 参数改为自由字符串，并添加“选择状态”按钮打开悬浮窗口。
- `src/app.rs` 新增 `state_picker` 状态；`src/ui/panels/properties.rs` 在属性面板调用选择器，选中路径后回写为 `stateKey`。
- 新增 i18n 键 `state_picker.*`（zh/en/ja）。
- 测试：`state_picker.rs` 包含 3 个单元测试；`cargo test --lib` 173 项通过；`cargo clippy --lib` 0 warnings。

### 增强（P2.7 条件表达式实时校验）

- 在 `src/ui/panels/condition_editor.rs` 新增 `validate_condition_expression`：检查方括号/圆括号配平、括号不交叉、token 合法性、无空组、逗号只在括号内、`!` 只能作为前缀。
- 条件编辑器预览下方以红色实时显示校验错误，不阻塞确认按钮。
- 新增 2 个单元测试覆盖 11 个有效和 18 个无效样例。
- `cargo test --lib` 165 项通过；`cargo clippy --lib` 23 个 warnings（均为既有，无新增）。

### 优化（P2.10 For 节点自带 start/stop/step）

- `For` 节点定义增加 `start` / `stop` / `step` 可选参数，并新增 `iterable` 数据输入端口。
- `src/code_gen/generator.rs` 中当 `iterable` 未连接且未提供时，自动生成 `Range(start, stop)` 或 `Range(start, stop, step)`。
- 保留 Data 端口连接 `iterable` 的原有行为；`step` 为 0 或 1 时省略。
- 新增 `test_for_with_connected_iterable_uses_source`、`test_for_without_iterable_uses_default_range`、`test_for_with_step_uses_three_arg_range`。
- 更新 `docs/node_types.md` 中 `For` 的说明与 `.code` 示例。
- `cargo test --lib` 173 项通过；`cargo clippy --lib` 0 warnings。

### 新增（P2.11 CreateArea cuboid 参数集）

- `CreateArea` 的 `type` 参数改为枚举（sphere / cylinder / cuboid），并新增 `position2` / `w` 等长方体参数。
- `src/code_gen/generator.rs` 新增 `generate_create_area`，按形状输出官方签名：球体 `x,y,z,r`；圆柱体 `x,y,z,r,h`；长方体 `x1,y1,z1,x2,y2,z2,w,h`。
- 新增 `extract_vector_components` 以支持 Vector 字面量与动态表达式。
- 更新 `docs/node_types.md` 与 `docs/node_details.md`。
- `cargo test --lib` 173 项通过；`cargo clippy --lib` 0 warnings。

### 修复（checklist 暴露 UI 缺陷）

- 窗口启动：在 `src/main.rs` 中设置启动时最大化，解决窗口部分在屏幕外及闪烁问题。
- 节点库拖拽虚影：在 `src/app.rs` 中使用节点显示名、最小宽度 140px、截断模式，避免过窄换行。
- 命名空间与坐标面板：在 `src/ui/panels/namespace_picker.rs` / `src/ui/panels/coordinate_picker.rs` 及 `src/api/namespace.rs` / `src/api/coordinate.rs` 中增加删除按钮与确认对话框，删除后持久化到对应 JSON 文件。
- 更新 `docs/test_checklist.md` 对应条目。
- `cargo test --lib` 173 项通过；`cargo clippy --lib` 0 warnings。

### 测试（P2.9 生成器专项测试补全）

- 在 `src/code_gen/generator.rs` 新增 A 类节点语义测试，覆盖：
  - `Goto`（带 args）、`Return`（有值 / null）、`Break`（在 While 内）。
  - `If`（true/false 分支、Data 边条件、动态 elseif 分支）。
  - `While`（body/break 目标、Data 边条件）。
  - `For`（已连接的 Range 迭代器）。
  - `CallFunction`（字面量函数名与 List 参数）、`CallMethod`（对象引用 + 无参方法）、`ForeachNode`。
  - `DestroyListener`、`WaitForThread`、`Wait`、`WaitForEvent`。
  - `CreateNPC`（多参数）、`Translate`（空参数列表不输出额外参数）。
- 修复生成器实现细节以支持测试：
  - 补全 `Wait` / `WaitForEvent` 的 A 类生成臂。
  - `ForeachNode` 与 `WaitForThread` 对 `threadVar` / `thread` 字符串参数去除 JSON 引号。
  - 调整 `Goto` 参数拼接格式。
- 所有生成器测试断言与 P0 fix 保持一致（`if (condition)` / `while (condition)` / `elseif (condition)`）。
- `cargo test --lib` 191 项通过；`cargo clippy --lib` 0 warnings。

### 新增（FunctionExists / GetModVersion，P1.5）

- 新增 `NodeType::FunctionExists`（C 类，Boolean 输出）与 `GetModVersion`（C 类，List 输出）。
- 代码生成器在 `evaluate_data_output` 中生成 `FunctionExists("name")` 与 `GetModVersion()` / `GetModVersion("guid")`。
- `General Functions` 分类；节点库归入 `scene.data_get.globals`；补充 zh/en i18n；更新 `docs/node_types.md` 与 AGENTS.md 计数（186→188）。
- 测试：新增 `test_generate_function_exists_and_mod_version` 覆盖无参数与带 GUID；`cargo test` 147 项通过。

### 新增（NPC 高频方法节点，P1.4）

- 新增 5 个 NPC 对象方法节点（`NodeType` 181 → 186）：
  - Flow 节点：`NPCWarp`、`NPCAddWaypoint`
  - Data 节点：`NPCIsAlive`、`NPCSeesPlayer`、`NPCSeesFlashing`
- 代码生成器 A 类生成：`npc.Warp(position?, rotation?)` / `npc.AddWaypoint(position, rotation?, last?)` / `npc.IsAlive()` / `npc.SeesPlayer()` / `npc.SeesFlashing()`。
- `Objects` 分类；节点库归入 `scene.visual_ui.visual`；补充 zh/en i18n；更新 `docs/node_types.md` 与 AGENTS.md 计数（181→186）。
- 测试：新增 `test_generate_npc_methods` 覆盖全部 5 个方法；`cargo test` 146 项通过。

### 新增（List 六方法节点，P1.3）

- 新增 6 个 List 对象方法节点（`NodeType` 175 → 181）：
  - Flow 节点：`ListInsert`、`ListRemove`
  - Data 节点：`ListCount`、`ListContains`、`ListIndexOf`、`ListKeys`
- 代码生成器 A 类生成：`list.Insert(index?, values...)` / `list.Remove(index?, count?)`，Data 节点在 `evaluate_data_output` 中生成 `list.Count()` / `list.Contains(value)` / `list.IndexOf(value)` / `list.Keys()`。
- `Objects` 分类；节点库归入 `scene.data_process.list`；补充 zh/en i18n；更新 `docs/node_types.md` 与 AGENTS.md 计数（175→181）。
- 测试：新增 `test_generate_list_methods` 覆盖全部 6 个方法；`cargo test` 145 项通过。

### 新增（Translate 节点，P1.2）

- 新增 `NodeType::Translate`（175 变体）：`General Functions` 分类，`key` 必填参数 + `params` 可选 List 参数，输出 `out_value`（String）。
- 代码生成器 A 类生成：`Translate(key)` 或 `Translate(key, arg1, arg2, ...)`（列表字面量展开为位置参数）。
- 节点库归入 `scene.data_process.string`；补充 zh/en i18n 键；更新 `docs/node_types.md` 与 AGENTS.md 计数（174→175）。
- 测试：新增 `test_generate_translate` 覆盖无参数与参数展开；`cargo test` 144 项通过。

### 新增（Log 级别枚举，P1.1）

- 为 `Log` 节点新增 `level` 枚举参数：`Info` / `Warning` / `Error`，默认 `Info`。
- 代码生成器对 `Log` 改为 A 类特判：`Info` → `Log(output)`，`Warning` → `Warning(output)`，`Error` → `Error(output)`，覆盖官方 `Warning` / `Error` 全局函数。
- 旧图无 `level` 参数时回退到 `Log(...)`，保持兼容。
- 更新 `docs/node_types.md` 日志子系统说明；补充 zh/en i18n 键 `node.Log.param.level`；更新并扩展 `test_generate_log` 测试覆盖三种级别；`cargo test` 143 项通过。

### 新增（多分支 If 节点，P0.8）

- 升级 `If` 节点支持多分支 `elseif`：在节点定义中增加 `elseif_branches` 动态端口组，每个逻辑分支包含一个 Flow 输出端口 `elseif_N_branch` 与一个 Boolean 条件参数 `elseif_N_condition`。
- 扩展 `DynamicPortGroup` 支持多成员（一个逻辑成员同时生成端口和参数），是 P0.7 基础设施的自然完善。
- 重写代码生成器 `generate_if`：先读取本节点动态分支，按顺序生成 `if ... elseif ... elseif ... else`；继续复用 P0.5 的链式 `If` 折叠逻辑处理旧图。
- 多分支汇合点通过 `find_join_node_many` 自动计算所有分支的公共后续节点。
- 属性面板自动渲染动态分支管理 UI（`+`/`-` 按钮）和每个分支的 `condition` 参数编辑；`i18n::param_display_name` 新增动态参数显示名回退。
- 旧图 `If` 节点无 `elseif_branches` 时保持兼容，按传统 `if / else` 生成。
- 更新 `docs/node_types.md` 中 `If` 的 `.code` 示例；新增 `test_multi_branch_if_node` 专项生成器测试；`cargo test` 143 项 lib tests 通过，`cargo clippy --lib` 22 warnings（无新增）。

### 新增（动态端口基础设施，P0.7）

- 在 `graph::types` 中引入 `DynamicPortGroup` / `DynamicPortKind` / `DynamicPortTemplate`，支持 Input / Output / Param 三种动态端口/参数组。
- `NodeDefinition` 新增 `dynamic_ports` 字段，声明节点可运行时扩展的端口组（ID、前缀、最小/最大数量、模板）。
- `Node` 新增 `dynamic_ports: HashMap<String, Vec<String>>`，记录各组当前成员 ID；成员端口/参数仍保存在 `inputs` / `outputs` / `params` 中，序列化原样保存，旧图无此字段时默认空。
- `Command` 扩展 `AddDynamicPort` 与 `RemoveDynamicPort`（含级联边删除），支持撤销/重做。
- 属性面板返回 `PropertiesPanelAction` 枚举，统一处理参数修改与动态端口增删；对每个动态组渲染 `+`/`-` 按钮与成员列表。
- 验证器识别动态端口：检查成员 ID 在节点内唯一，且每个 ID 恰好存在于 `inputs` / `outputs` / `params` 之一。
- 新增 i18n 键 `label.dynamic_ports` / `tooltip.add_dynamic_port` / `tooltip.remove_dynamic_port`（zh/en/ja）。
- 测试：新增 `test_dynamic_port_add_and_remove_output`、`test_dynamic_param_add_and_remove`、`test_dynamic_port_serialization_roundtrip`；验证器新增 `test_dynamic_port_id_must_be_unique_and_exist`、`test_dynamic_port_must_exist_in_exactly_one_place`；`cargo test` 142 项 lib tests 通过，`cargo clippy --lib` 保持 22 个 pre-existing warnings。

### 新增（SetVariable 复合赋值，P0.6）

- 为 `SetVariable` 节点新增 `op` 可选枚举参数：`=` / `+=` / `-=` / `*=` / `/=`，默认 `=`。
- 代码生成器读取 `op` 并校验合法性，输出 `{name} {op} {value}`，例如 `i += 1`、`hp -= 5`。
- 旧图反序列化时若缺少 `op` 参数，自动回退到 `=`，保持向后兼容。
- 更新 zh/en i18n 的 `node.SetVariable.param.op` 键；在 `docs/node_types.md` 2.3 节补充 `SetVariable` / `Variable` 行，并将原"规划"说明改为已实现。
- 测试：新增 `test_set_variable_compound_assignment`，覆盖全部 5 种操作符；`cargo test` 137 项通过。

### 优化（代码生成：elseif 折叠，P0.5）

- 改进 `generate_if`：当 `If` 节点的 `out_false` 直接连接到另一个 `If` 节点，且该后继 `If` 只有一条来自当前 `If` 的 Flow 入边时，生成 `elseif` 而非嵌套的 `else { if ... }`。
- 支持连续折叠：多个 `If` 串成链时可以生成 `if ... elseif ... elseif ... else ...`。
- 新增辅助函数 `is_single_flow_predecessor` 判断节点是否只有一条来自指定前驱的 Flow 入边，确保不会错误折叠具有多个流入的 If 节点。
- 测试：新增 `test_if_elseif_else_chain`，验证 `if true ... elseif false ... else ...` 结构且不出现嵌套 `else { if`；`cargo test` 136 项通过。

### 新增（性高潮触发节点，P0.4）

- 新增 `TriggerSexOrgasm` 节点（`NodeType` 173 → 174）：官方 `TriggerSexOrgasm()` 无参数全局函数，返回 null；原子语义，隐式设置 ecstasy=1，此前只能用 `SetEcstasy(1)` + `SetAction` 近似。
- B 类无参数 Flow 节点，代码生成器直接输出 `TriggerSexOrgasm()`；API 分类 `Game API: Stats`，节点库归入 `scene.data_set.player_state`；补充 zh/en i18n 键；同步更新 `docs/node_types.md` 与 AGENTS.md 计数。
- 测试：将 `TriggerSexOrgasm` 加入 `test_generate_game_api_stats` 循环；计数断言更新至 174；`cargo test` 135 项通过。

### 新增（全局变量节点，P0.3）

- 新增 `GetStageChanged` / `GetProjectName` 节点（`NodeType` 171 → 173）：读取官方 Built-In Global Variables `_stagechanged` 和 `_name`。
  - `GetStageChanged`：C 类纯数据节点，Boolean 输出，生成 `_stagechanged`；用于监听器中检测本帧是否发生场景切换，从而做一次性初始化逻辑。
  - `GetProjectName`：C 类纯数据节点，String 输出，生成 `_name`；即当前工程文件夹名（在 `_mods` 中使用的标识）。
- 注册在 `Variables & Globals` API 分类；节点库归入 `scene.data_get.global_vars`；补充 zh/en i18n 键；同步更新 `docs/node_types.md` 与 AGENTS.md 计数。
- 测试：新增 `test_stagechanged_and_project_name_globals` 专项生成器测试，验证输出分别为 `_stagechanged` 和 `_name`；计数断言更新至 173；`cargo test` 135 项通过。

### 新增（停止音频节点，P0.2）

- 新增 `StopAudio` 节点（`NodeType` 170 → 171）：官方 `StopAudio(AudioInstanceID[, FadeOutTime])` 全局函数，用于停止 `Audio.Play()` 启动的音频实例；此前该函数不是对象方法，无法用 `CallMethod` 表达。
- 代码生成：A 类显式处理，`audioInstanceID` 走位置参数，`fadeOutTime` 非空时追加为第二个位置参数，生成 `StopAudio(42)` 或 `StopAudio(42, 1.5)`。
- API 分类 `Game API`，场景分类归入 `scene.visual_ui.audio_screen`；补充 zh/en i18n 键；同步更新 `docs/node_types.md` 与 AGENTS.md 计数。
- 测试：新增 `test_stop_audio_positional_args_and_optional_fade` 专项生成器测试；计数断言更新至 171；`cargo test` 134 项通过。

### 新增（事件监听器节点，P0.1）

- 新增 `CreateEventListener` / `CreateEventListenerLocal` 节点（`NodeType` 168 → 170）：官方 `CreateEventListener(LabelName, EventName[, params...])` 的事件驱动监听器，仅在 `SetEvent` 触发时立即执行，标签内可用局部变量 `__eventdata_` / `__eventname_`，此前完全无法用节点表达（轮询 Listener 语义不等价）。
- 代码生成：`labelName` / `eventName` 按官方签名走前两个位置参数，`params` 对象展开为命名参数，生成 `var_{id}_out_listener = CreateEventListener("label", "event", key=value)`；`out_name` 数据输出与其他 Create* 节点一致。
- 概览图新增 `CreateEventListener` / `CreateEventListenerLocal` 两种关系边（样式同 Listener 虚线）；节点库归入 `scene.mission_flow.threading` 子分类。
- 新增 zh/en i18n 键（名称、详细描述、参数、端口）；同步更新 `docs/node_types.md`（2.1 节）与 AGENTS.md 计数规则。
- 测试：新增 `test_create_event_listener_positional_args_and_params`、`test_create_event_listener_local_generates_local_variant` 两个专项生成器测试；计数断言更新至 170；`cargo test` 133 项通过。

### 修复（代码生成）

- 修正 `CreateCondition` 的 `.code` 输出语法：官方 API 把 `Condition` 作为位置参数，且 `id` 为空字符串时省略，现在生成 `CreateCondition("Exposed_All")` 或 `CreateCondition("...", id="MyID")`。
- 修正 `CreateItemCondition`：当 `id` 参数为空字符串时不再输出无意义的 `id=""`。
- 同步更新 `tests/fixtures/example_drop_bra.code` 中的示例代码，使用新的正确语法。

### 新增（节点库场景分类）

- 新增 `src/ui/panels/node_library/catalog.rs`：按开发者实际使用场景（任务/流程、条件判定、数据获取、数据修改、数据处理、视觉/UI、编辑器专用）组织全部 168 个节点，支持二级折叠、按场景分类搜索。
- 节点注册层（`src/api/definitions.rs`）保持 API 分类不变；节点面板层独立场景分类，一个节点可出现在多个场景分类中。
- 更新 `NodeLibraryPanel`：左侧节点库按场景一级分类 → 二级分类 → 节点列表渲染；搜索窗口同时支持按节点名和场景分类关键字搜索。
- 新增 `src/ui/theme.rs::scene_category_color`：为场景分类提供稳定的节点库颜色。
- 在 `assets/i18n/zh.json`、`en.json`、`ja.json` 中补充全部场景分类翻译键。

### 修复（左栏滚动条与工程树布局）

- 在 `app.rs` 的 Project 标签页中，节点库与工程文件树之间新增可拖拽分隔条，用户可手动调整上下两块高度。
- `NodeLibraryPanel` 与 `ProjectTreePanel` 的 `ScrollArea` 均添加 `max_height` 与 `auto_shrink([false, true])`，保证在侧边栏高度不足时两者都独立出现滚动条，而不是整体被挤出屏幕。
- 工程文件树现在根据剩余可用空间自动计算滚动区域高度，避免上下文过长时底部按钮不可见。
- 将 `.code 文件` 分类标签也移入工程文件树的 `ScrollArea` 内，滚动时分类提示始终跟随列表。
- 左栏分隔条视觉风格对齐底栏分隔条：默认灰色、悬停/拖拽时高亮蓝色，并显示 `ResizeVertical` 光标。

### 新增（CreateCondition 条件组合编辑器）

- 将 `CreateCondition` 的 `condition` 参数从枚举下拉框改为字符串类型，支持 `.code` 官方组合语法：`[A, B]`（AND）、`(A, B)`（OR）、`!A`（NOT）与 `SubCondition_<id>`（复用已有条件）。
- 新增 `src/ui/panels/condition_editor.rs`：弹出式条件组合编辑器，提供可编辑表达式框、AND/OR/NOT 插入按钮、按分类折叠的基础条件列表、当前标签内已有条件 ID 复用列表，以及实时预览。
- 插入逻辑支持光标位置：工具按钮在文本框当前光标处插入；若有文本选区，AND/OR/NOT 会包裹选区；若光标已在 `[...]` 或 `(...)` 内，按条件 token 会在逗号后追加，按 AND/OR 则直接追加逗号（实现“与内再与只加一个逗号”）。
- 修复光标丢失问题：工具按钮点击会抢焦点导致上一帧 `cursor_range` 不可靠，改为把最后有效的光标/选区保存在 `state.cursor_range` 这个我们自己的缓冲区中，且只在 `TextEdit` 实际报告光标时才更新。按钮点击时直接使用 `state.cursor_range`，这样即使按钮点击使文本框失去焦点，选区仍然被保留并用于包裹。
- 修复“无焦点连续点击条件 A 后再点条件 B，A 被替换为 `,B`”的 bug：引入 `last_insert_pos` 字段，并在每次插入后同步更新 `state.cursor_range` 为新的光标位置，避免后续点击误用旧选区替换已有内容。
- 优化条件按钮美术：按钮改为 `min_size(90, 40)`，主文本显示条件译名，副文本显示原始 token；在 `zh.json` 中为全部 49 个基础条件补充 `condition.{token}` 中文翻译键，英文环境下仍回退显示原始 token。
- 在 `src/ui/panels/properties.rs` 中为 `CreateCondition` 的 `condition` 参数添加 **编辑条件...** 按钮，为 `id` 参数添加中文/英文说明，解释 ID 用于 `SubCondition_<id>` 复用。
- 在 `app.rs` 中管理 `condition_editor` 窗口状态，确认后通过 `Command::SetParam` 更新节点参数。
- 在 `assets/i18n/zh.json`、`en.json`、`ja.json` 中添加 `condition_editor.*` 与 `button.edit_condition` 翻译键。
- 为 `CreateCondition` 与 `CreateItemCondition` 增加 `id` 数据输入端口：ID 现在可通过数据流从其他节点（如 `StringConstant`）传入；代码生成器优先使用数据端口连接，无连接时仍回退到属性面板中的常量值。检查了所有创建条件/物品的节点，仅有这两个节点使用 `id` 字符串参数用于子条件复用，因此只在此处普及该功能。新增 2 个生成器测试验证数据流 ID 行为。
- 更新 `docs/node_types.md`：说明 `CreateCondition.id` / `CreateItemCondition.id` 同时支持常量输入与数据流输入。

### 重构（节点库场景分类）

- 启动 6 个子代理按实际 API 使用场景审查全部 168 个节点的分类，避免望文生义。
- 根据子代理建议重构 `src/ui/panels/node_library/catalog.rs`：
  - `CreateCondition` / `CreateItemCondition`：从 `scene.visual_ui.visual` 移入 `scene.conditions.state_check`（它们是条件对象构造器，不是视觉/UI）。
  - `Log`：从 `scene.visual_ui.audio_screen` 移入 `scene.editor.editor`（控制台日志是调试工具，不是音频/屏幕效果）。
  - `SetCamera` / `GetAllSnapshots` / `DeleteSnapshot` / `GetImageReference` / `GetGraphicsOption`：移入 `scene.visual_ui.visual`（相机、快照、图像引用都是视觉资产/显示控制）。
  - `TriggerGameOver`：从 `scene.data_set.player_state` 移入 `scene.mission_flow.control`（强制结束游戏是流程控制）。
  - `GetItemCount`：从 `scene.data_get.player_info` 移入 `scene.data_get.items_equipment`（库存查询，不是生理属性）。
  - `CreateInteractArea`：从 `scene.visual_ui.visual` 移入 `scene.visual_ui.input_interact`（交互区域属于输入/交互机制）。
  - `CanGameOver`：同时归入 `scene.conditions.state_check`（有布尔输出，可作条件使用）。
  - 新增 `scene.data_process.file` 子分类，将 `FileExists` / `GetFiles` 从 `scene.data_process.string` 移入（文件系统操作不是字符串处理）。
- 同步更新 `assets/i18n/zh.json`、`en.json`、`ja.json`，新增 `scene.data_process.file` 翻译键。
- 更新 `docs/node_types.md` 中关于场景分类的说明。

### 文档（节点详细介绍）

- 启动 9 个子代理（按功能分组 + 补充缺失节点），基于 `src/api/definitions.rs`、`src/code_gen/generator.rs` 和官方 API 文档，为全部 168 个节点生成中文详细介绍。
- 合并为 `docs/node_details.md`，每个节点包含：中文名、官方 API 签名、返回值、作用说明、参数表、`.code` 使用案例、常见场景、相关节点。
- 为文档添加按字母索引的目录（`## 目录`），列出全部 168 个节点的锚点链接，方便快速跳转到目标节点。
- 该文档比官方 HTML 文档更结构化、更本地化，并作为属性面板未来改进的描述来源。
- 在 `docs/node_types.md` 中新增 5.3 节引用该文档。

### 文档（i18n 节点描述更新）

- 启动子代理从 `docs/node_details.md` 提取全部 168 个节点的"作用"段落，更新 `assets/i18n/zh.json` 中 `node.{Type}.description` 键（共 169 个 description 键，其中 `Boolean` 原描述也一并精炼）。
- 描述统一精炼为 1-2 句话（约 120 字以内），去除代码示例与冗余说明，适合属性面板直接显示；属性面板无需改动即自动生效。
- 15 个格式特殊的节点（Meta、Comment、Group、GetTime、GetSettings、Variable、SetVariable 等）在自动提取后人工精炼，去除要点符号。
- 验证 168 个 NodeType 变体全部拥有 description，无缺失；`cargo test --lib` 131 项通过（含 `test_bundled_zh_translations_load`）。

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

- **服装选择窗口多选可发现性**：`EquipCosplay` / `UnequipCosplay` / `OwnCosplay` / `CheckCosplay` 的 `cosplayKeys` 参数均支持多选。`EquipCosplay` / `UnequipCosplay` / `OwnCosplay` 生成数组参数；`CheckCosplay` 生成 `(Cosplay_A && Cosplay_B)` 这种带括号的逻辑与表达式，便于与 `LogicAnd` / `LogicOr` 节点组合。属性页弹窗分类视图在多选模式下使用 checkbox 列表，窗口标题显示“命名空间选择器 (可多选)”。左栏资产管理面板新增“多选”开关，可批量勾选复制 key。
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

### 新增（i18n）

- 创建运行时国际化基础设施：`src/ui/i18n.rs` 提供 `I18n` 结构体，支持从 `assets/i18n/` 加载 JSON 翻译文件、运行时切换语言、缺失时 fallback 到英语。
- 创建 `assets/i18n/zh.json`、`assets/i18n/en.json` 首批翻译文件，`assets/i18n/ja.json` 作为占位文件由英语 fallback。
- 在 `App` 工具栏添加语言切换菜单（中文 / English / 日本語），切换即时生效。
- 完成阶段二 UI 面板文本迁移：工具栏、状态栏、对话框、欢迎界面、画布提示、工程树、属性面板、命名空间/坐标选择器、代码编辑器、数据菜单、meta 编辑器、状态栏、右键菜单、入口钉、参数文本编辑等全部改用 i18n 键。
- 新增翻译键命名空间：`app.*`、`panel.*`、`button.*`、`status.*`、`dialog.*`、`label.*`、`search.*`、`context_menu.*`、`code_editor.*`、`data_menu.*`、`status_bar.*`、`welcome.*` 等。

### 新增（i18n — 阶段三完成）

- 将 `If` / `While` 节点的条件模板列表从硬编码中文改为 i18n 键，新增 `template.*` 命名空间（如 `template.true`、`template.futanari`、`template.category.character_state`），支持中/英双语。
- 精翻 `assets/i18n/en.json` 全部 169 条节点描述（`node.*.description`），英文界面不再使用占位文本。
- 新增 `src/settings.rs`：语言偏好持久化到用户配置目录（Windows 为 `%APPDATA%/CM2Editer/settings.json`），切换语言时自动保存，启动时自动恢复。

### 修复（i18n）

- 修复语言切换无反应的问题：根因为 `assets/i18n/ja.json` 双逗号语法错误导致 `I18n::load_from_dir` 在加载 `ja.json` 时失败并中止，排在后面的 `zh.json` 没有被加载。已修复 `ja.json` 语法错误，并让 `load_from_dir` 跳过单个损坏文件、打印错误并继续加载其余文件。
- 修复 `zh.json` 中 `status_bar.world` / `status_bar.zoom` / `status_bar.world_empty` 仍为英文的问题。
- `I18n::load_bundled` 现在尝试多个翻译目录（当前工作目录、可执行文件目录、可执行文件父目录），避免从 `target/debug/` 直接运行或打包后找不到 `assets/i18n/`。
- 新增 `src/settings.rs`：语言偏好现在持久化到用户配置目录（Windows 为 `%APPDATA%/CM2Editer/settings.json`），切换语言时自动保存，下次启动自动恢复。
- 修复语言选择器显示语言代码问题，改为显示翻译后的语言名。
- 修复左栏命名空间卡片 `ns_card` 硬编码 `"zh"` 问题，改为使用当前语言。
- 修复 `label.position` 翻译占位符拼写错误 `{:..1}` → `{}`。`

### 新增（i18n — 节点元数据 P2）

- 将节点元数据全面接入 i18n：`I18n` 新增 `node_display_name`、`node_description`、`port_display_name`、`param_display_name` 辅助函数，缺失时 fallback 到 `NodeDefinition` 原中文元数据。
- `NodeLibraryPanel` 搜索与显示使用 i18n 节点显示名；搜索弹窗标题与分类显示已同步。
- `PropertiesPanel` 节点标题、描述、参数标签、数据源下拉框使用 i18n 读取端口/参数显示名。
- `NodeRenderer` 新增 `with_i18n` 构造方法，节点标题、端口标签渲染时实时翻译。
- `DataMenuPanel` 数据流小方块标签使用 i18n 端口显示名。
- `assets/i18n/zh.json` 补充全部 168 个节点的中文显示名、描述、端口名、参数名键。
- `assets/i18n/en.json` 补充全部 168 个节点的英文显示名（基于 `NodeType` PascalCase 分词 + 手动修正），端口名与参数名保持英文标识符；节点描述为简化版占位，后续可逐步精翻。

### 新增（P2 — UI 与编辑器重构）

- 将 `App` 内部图模型从旧 `Graph` 迁移到 `ContainerGraph` / `GraphDocument`，新增 `SelectedContainer` 与 `ContainerKind` 跟踪当前编辑容器。
- 左侧工程树按 `.code` 文件 → 线程 → 标签/监听器层级展示，点击切换画布容器。
- 画布仅渲染当前 `LabelContainer` / `ListenerContainer` 内部节点与边；移除旧 `Start` / `Label` 节点显示。
- 新增入口钉渲染（`src/ui/entry_pin.rs`）：为当前容器绘制 Flow 起点，并连接到入口节点 `in_flow` 端口。
- 新增线程概览图面板（`src/ui/panels/overview.rs`）：以网格布局展示标签/监听器及 `Goto` / `CreateThread` / `CreateListener` / `ForeachNode` 关系，双击节点跳转对应容器。

### 测试

- 新增 `ui::panels::overview` 单元测试：验证概览图能从 `Goto` 节点提取目标标签关系并生成唯一布局。
- 新增 `graph::container::tests::test_entry_node_id_prefers_top_left_no_incoming_flow`，验证入口节点按位置和无入边规则稳定选择。
- 新增 P3.2 UI 回归测试：`app::tests` 容器切换定位、`ui::entry_pin::tests` 入口钉端口选择、`ui::panels::overview::tests` CreateListener 关系、`project::tests` 工程保存/导出。
- `cargo test --lib`：108 个 lib tests 全部通过；总测试数 117 项。

### 修复

- 修复 `cargo clippy` 两个 `unwrap_used` 报错：`graph::validation.rs` 使用 `ok_or_else` 替代 `unwrap()`；`ui::panels::properties.rs` 使用 `if let Some` 替代 `unwrap()`。
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
