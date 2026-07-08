# CM2Editer 项目进度清单

> 用途：单人项目管理 + Agent 任务追踪
> 更新规则：每次 Agent 交付后，由用户（或 Agent）更新对应条目状态
> 文件位置：`docs/TODO.md`
> 更新时间：2026-07-08 20:34

---

## 当前状态

| 阶段 | 状态 | 备注 |
|------|------|------|
| Phase 0：项目初始化 | ✅ 完成 | 2026-07-05 |
| Phase 1：数据层 | ✅ 完成 | 2026-07-05 |
| Phase 2：序列化与代码生成 | ✅ 完成 | 2026-07-06 |
| Phase 3：UI 层 | ✅ 完成 | 2026-07-07 |
| Phase 3 三轮修复 | ✅ 完成 | 2026-07-08 |
| Phase 4：集成测试与打磨 | ✅ 完成 | 2026-07-08 |
| Phase 4.5：工程/项目管理 | ✅ 完成 | 2026-07-08 |
| Phase 5：新功能与发布 | 🔄 5.1~5.4 已大部分完成 | DataFlow / 枚举参数 / 命名空间 / 静态检查 / 错误面板 / 底栏重构 / ID 冲突修复 |
| Phase 6：Monitor→Condition 管道 | ✅ Phase 1 完成 | 7 个新 Boolean/Condition 节点 + If 条件模板 + Data 递归解析 + code_gen 对齐 CM2 DSL |

---

## Phase 5：新功能与发布（Backlog）

### 5.1 高复杂度

| 任务 | 优先级 | 状态 | 说明 |
|------|--------|------|------|
| 5.1.1 **DataFlow 重构** | P0 | ✅ | 参数 Data 端口、虚线、属性面板数据源、Data 边选中渲染，108 tests |
| 5.1.2 **参数类型重构** | P1 | ✅ | `ParamType::Enum`；20+ 参数下拉选择；参考文档已切换到 `docs/documentation.html`（英文原版） |
| 5.1.3 **命名空间管理** | P1 | ✅ | 7 个 namespace JSON；悬浮搜索选择器 |
| 5.1.4 **坐标"语言糖"** | P2 | 📋 | 预制坐标/视角变量；需评估与数学节点语义冲突 |

### 5.2 中复杂度

| 任务 | 优先级 | 状态 | 说明 |
|------|--------|------|------|
| 5.2.1 **左栏二级菜单** | P2 | ✅ | 三个一级标签：工程（合并节点库）、命名空间、坐标 |
| 5.2.2 **静态检查** | P3 | ✅ | 多 Start 警告、不可达节点 |
| 5.2.3 **Start 多路连接警告** | P3 | ✅ | 菱形依赖检测 |
| 5.2.4 **错误详情面板** | P3 | ✅ | 点击状态栏错误数弹出详情窗口 |
| 5.2.5 **画布状态机 Debug 显示** | P3 | 📋 | 开发者模式覆盖层 |

### 5.3 低复杂度

| 任务 | 优先级 | 状态 | 说明 |
|------|--------|------|------|
| 5.3.1 **底部面板可调高度** | P3 | ✅ | 三合一底栏（代码┃JSON┃DataFlow），双拖拽分隔线，ScrollArea 防溢出 |
| 5.3.2 **端口吸附环** | P3 | 📋 | |
| 5.3.3 **左栏拖出节点** | P3 | 📋 | |
| 5.3.4 **节点大小可调整** | P3 | 📋 | |
| 5.3.5 **电路连接线风格** | P3 | 📋 | |

### 5.4 快速修复

| 任务 | 优先级 | 状态 | 说明 |
|------|--------|------|------|
| 5.4.1 **必填参数 null 错误** | P2 | ✅ | JSON 加载时补填缺失 required 参数默认值 |
| 5.4.2 **egui ID 冲突** | P2 | ✅ | 7 个 ScrollArea + 1 个 CollapsingHeader 加 `id_salt` |
| 5.4.3 **底栏弹回/自缩** | P2 | ✅ | 三合一面板 + `BottomMain` 统一控制 |
| 5.4.4 **`.code` 生成语法对齐** | P2 | ✅ | `If(true) [`→`if true`；`While()->`while`；`For()->`for i in`；`Break->`break` |

---

## Phase 6：Monitor→Condition 管道（Boolean 节点系统）

> 架构文档：`docs/if_condition_design.md`；API 参考：`docs/code_api_reference.md`

### 6.1 新节点（Phase 1 完成）

| 节点 | 分类 | 输出 | 作用 |
|------|------|------|------|
| Boolean | Math | Boolean 常量 | true/false |
| GetStateBool | Game Functions | Boolean | 读 `_state.*` 布尔（18 项） |
| GetStateNumber | Game Functions | Number | 读 `_state.*` 数值（8 项含 Bodypaint） |
| CompareNumbers | Math | Boolean | a op b（6 种比较符） |
| LogicAnd | Math | Boolean | `(a) && (b)` |
| LogicOr | Math | Boolean | `(a) \|\| (b)` |
| LogicNot | Math | Boolean | `!(a)` |

### 6.2 代码生成

| 特性 | 状态 |
|------|------|
| `evaluate_data_output()` 递归解析 Data 边链 | ✅ |
| Boolean→If 生成 `if true` / `if false` | ✅ |
| GetStateBool→If 生成 `if _state.Futanari` | ✅ |
| CompareNumbers→If 生成 `if _state.Ecstasy >= 50` | ✅ |
| LogicAnd/Or/Not 组合解析 | ✅ |

### 6.3 If 条件模板下拉

| 特性 | 状态 |
|------|------|
| 30+ 预设表达式 ComboBox | ✅ |
| 分类：字面量 / 角色状态 / 环境 / 装备拘束 / 数值比较 | ✅ |
| Data 连线时模板自动隐藏 | ✅ |

---

## Phase 7：后续规划（v0.2+）

| 任务 | 说明 |
|------|------|
| 7.1 `_save` 持久数据节点 | 读/写跨会话存档变量 |
| 7.2 `GetPosition` 节点 | 输出 `_state.Position.*` 各字段 |
| 7.3 `CheckEquipment` 节点 | `_state.AdultToys[key] != null` |
| 7.4 `CheckCosplay` 节点 | 命名空间选择器 → `Cosplay_{key}` 条件 |
| 7.5 条件选择窗口 | 类似命名空间选择器的分类浏览窗口 |
| 7.6 `CreateCondition` Data 节点 | 输出 Condition Object → 连 Gallery/Area |
| 7.7 代码预览语法高亮 | TextEdit 升级为 `.code` 语法着色 |
| 7.8 Foreach 节点 | 生成 `Foreach(list, thread)` |
| 7.9 跨文件 Goto 标签 | 多 `.code` 间标签引用 |

---

## Agent 工作日志

| 日期 | 任务 | 说明 | 状态 |
|------|------|------|------|
| 2026-07-05 | Phase 0-2 | 项目骨架、数据层、序列化、代码生成 | ✅ |
| 2026-07-06 | Phase 2 补充 | CreateThread/Listener、并发语义测试；65 tests | ✅ |
| 2026-07-07 | Phase 3 | egui 界面、canvas、node/edge renderer、interaction、panels | ✅ |
| 2026-07-07 | 三轮修复 | 快捷键、框选、环检测、多 Start 标签等 24 项；76+7 tests | ✅ |
| 2026-07-08 | Phase 4-4.5 | 工程管理、多 .code、文件树、meta 编辑器、导出；81+16 tests | ✅ |
| 2026-07-08 | Phase 5.1.1 | DataFlow 重构；83+16 tests | ✅ |
| 2026-07-08 | Phase 5.1.2 | ParamType::Enum；90+16 tests | ✅ |
| 2026-07-08 | Phase 5.1.3 | 命名空间注册表 + 选择器窗口；90+16 tests | ✅ |
| 2026-07-08 | Phase 5.2.2-5.2.4 | 静态检查、菱形警告、错误详情面板；108 tests | ✅ |
| 2026-07-08 | Phase 5.4.1 | JSON 加载补填必填参数默认值；108 tests | ✅ |
| 2026-07-08 | 底栏+Data 面板 | 三合一底栏、巧克力板方块、分隔线拖拽；108 tests | ✅ |
| 2026-07-08 | egui ID 冲突 | 7 ScrollArea + 1 CollapsingHeader id_salt | ✅ |
| 2026-07-08 | .code DSL 对齐 | `If()`→`if`、`While()`→`while`、`For()`→`for in` | ✅ |
| 2026-07-08 | API 文档 | `docs/code_api_reference.md`（基于官方英文 doc + 80+ 例反推） | ✅ |
| 2026-07-08 | Phase 6 | 7 个 Boolean/Condition 节点 + If 模板 + `evaluate_data_output`；108 tests | ✅ |
| 2026-07-08 | Bodypaint 修复 | 从 GetStateBool（Boolean）→ GetStateNumber（Number） | ✅ |
| 2026-07-08 | Break 修复 | 已存在 Break 节点，code_gen `Break`→`break` | ✅ |
| 2026-07-08 | Release | `cargo build --release` 产物 6.5 MB；字体瘦身 110→32 MB | ✅ |
| 2026-07-08 | 文档更新 | README v0.1.0、CHANGELOG v0.1.0、TODO 重构 | ✅ |
