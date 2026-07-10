# CM2Editer 项目进度清单

> 版本：v0.2.2
> 更新时间：2026-07-11 00:54
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

> ✅ 全部完成。

| 任务 | 来源 | 说明 |
|------|------|------|
| Goto 节点无法自动生成标签 | 📋 已记录 | ~~手动填写标签可行，自动注册待修复~~ ✅：已在属性面板参数变更时自动注册（app.rs:1354-1389） |

### P1：重要（缺失的 .code 特性）—— 按实现难度排序

| 难度 | 任务 | 说明 | 工作量 |
|------|------|------|--------|
| 🟢 低 | **`listener = null`** | 销毁监听器的显式节点 | 1 个 B 类节点 + 1 行 code gen |
| 🟢 低 | **`_this` 当前线程引用** | `GetCurrentThread` 数据节点 | 1 个 C 类节点 |
| 🟢 低 | **`thread.WaitForFinish`** | 等待子线程结束 | 1 个 B 类节点 |
| 🟢 低 | **For + Range 直连** | `Range.out_range → For.iterable` 自动生成 `for i in Range(0,10)` | evaluate_data_output 分支 |
| 🟢 低 | **_save / _time / _timediff / _settings** | 6 个 C 类纯数据节点，复用现有模式 | 6 个 NodeType + 定义 |
| 🟡 中 | **Gallery API** | `.Show()` `.Confirmed()` `.GetSelection()` 3 个对象方法 | 3 个 A 类节点 |
| 🟡 中 | **list.Insert/Remove/Contains/Count/Clear/GetKeys** | 7 个列表操作方法 | 7 个 B 类节点 |
| 🟡 中 | **MessengerChat API** | `.Add()` `.SetButtons()` `.Clicked()` | 3 个 A 类节点（Add 参数复杂） |
| 🔴 高 | **`elseif` 多分支** | If 节点体系重构：多条件链 + UI | 架构级改动 |

> **预估总量**：5 个低难度 + 3 个中难度 + 1 个高难度。低/中难度部分约 2-3 个工作日。

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
| 2026-07-10 | P0 核实 + 标签重命名修复 | 确认 Goto 标签自动注册已工作；修复左栏标签重命名文本框缓冲缺失导致打字被覆盖的 bug（app.rs:962-984） | ✅ |
