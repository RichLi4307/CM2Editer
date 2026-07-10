# CM2Editer 项目进度清单

> 版本：v0.2.2
> 更新时间：2026-07-10 12:10
> 归档：旧版本见 `docs/archive/TODO_20260710_v7.md`

---

## 当前状态

| 阶段 | 状态 | 备注 |
|------|------|------|
| Phase 0–4.5：基础建设 | ✅ 完成 | 数据层 / 序列化 / 代码生成 / UI / 集成 / 工程管理 |
| Phase 5：新功能 | ✅ 完成 | DataFlow / 枚举参数 / 命名空间 / 静态检查 / 错误面板 / 底栏 / 热键 / ID 冲突 |
| Phase 6：Boolean 管道 | ✅ 完成 | 8 个纯数据节点 + If 条件模板 + evaluate_data_output 递归解析 |
| Phase 7：坐标/条件系统 | ⚡ 已大部分实现 | GetPosition/MakeVector/BreakVector + CheckCondition/Equipment/Cosplay |
| **P0 代码生成器重构** | ✅ 完成 | 顶层 CreateThread、if/while/for/break 语法对齐、标签生命周期、Goto out_label、Labels 自动发现 |
| **验证器 BFS 重构** | ✅ 完成 | 同时从 Start + 子标签入口出发，消除误报 |
| **Data 端口链路** | ✅ 完成 | A 类节点 + evaluate_data_output 三级配合，out_label/out_name 正确解析 |

---

## 待办队列

### P0：必须修

| 任务 | 来源 | 说明 |
|------|------|------|
| Goto 节点无法自动生成标签 | 📋 已记录 | 手动填写标签可行，自动注册待修复。`graph.labels` 不因 Goto 标签创建而自动添加 id |

### P1：重要

| 任务 | 说明 |
|------|------|
| **Goto 参数传递语法** | `thread.Goto("x", key=value)` → key 成为目标标签的局部变量。当前 `Goto.args` 为 Object 类型，需验证 end-to-end |
| **线程作用域** | 同一线程内标签共享变量；Goto 清除旧上下文。需在代码生成时保留线程局部变量 |
| **Listener 作用域** | `CreateListenerLocal("x")` vs `CreateListener("x")`——Local 保留调用者变量 |
| **`_save` 持久数据节点** | 读/写跨会话存档变量 |

### P2：UI 打磨

| 任务 | 说明 |
|------|------|
| 端口吸附环 | 鼠标靠近端口时放大，方便连线 |
| 电路连接线风格 | 用折线替代贝塞尔曲线 |
| 画布状态机 Debug 覆盖层 | 开发者模式显示节点执行序 |
| 节点大小可调节 | Resize handle |
| 代码预览语法高亮 | TextEdit 升级为 `.code` 语法着色 |
| 跨文件 Goto 标签 | 多 `.code` 间标签引用 |
| 条件选择窗口 | 类似命名空间选择器的分类浏览窗口 |
| 画布 Minimap 预览图 | 右下角可折叠/展开的缩略图 |
| 缩放画布是否影响节点 | 待用户确认：目前缩放不改变节点大小 |

---

## 用户备注区

<!-- 在此处留下你的笔记、发现或待办。Agent 不会覆盖本区域 -->

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
| 2026-07-08 | Break 修复 + Release | code_gen 对齐；v0.1.1 release；字体瘦身 | ✅ |
| 2026-07-09 | P0 代码生成器重构 | 顶层 CreateThread、`_result=null`、`thread.Goto`、标签自动发现 | ✅ |
| 2026-07-09 | Phase 6 扩展 | CheckCondition/Equipment/Cosplay 节点；StringConstant | ✅ |
| 2026-07-09 | Label 管理 | 标签增删改、左栏面板、Label 节点 Data 边注册 | ✅ |
| 2026-07-09 | 验证器 BFS 重构 | 子标签入口节点可达性；Goto out_label 端口修复 | ✅ |
| 2026-07-09 | 文档审计 | code_api_reference 修复、NodeType 159、版本同步、旧文档归档 | ✅ |
| 2026-07-10 | v0.2.2 release + CI 修复 | 夹具同步、tag 重打、文档归档 | ✅ |
| 2026-07-10 | 标签管理修复 | Ren 内联编辑、删除清理、改名清理、验证器 BFS | ✅ |
| 2026-07-10 | 字体子集化 | 33MB→5.8MB、OFL 合规改名、发行包 29.5→9.0MB | ✅ |
