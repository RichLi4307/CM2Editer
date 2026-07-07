# Changelog

所有显著变更均记录于此。

格式遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)。

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
