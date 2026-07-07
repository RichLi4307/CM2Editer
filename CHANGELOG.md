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

### 文档

- 归档旧 `docs/TODO.md` 至 `docs/archive/TODO-2026-07-08.md`。
- 重构 `docs/TODO.md`：按工程复杂度（高/中/低/快速修复）重新组织 Phase 5 backlog。
- 更新 `docs/问题清单.md`：筛去已解决问题，保留未解决问题、TODO 与作者全部原文建议，附录标注已解决/未解决标签。
- 同步 `README.md`、`CHANGELOG.md`、`agent_prompt.md`、`agent_prompt_phase3.md` 与当前进度一致。
- 新增测试：`test_generate_two_starts_produce_separate_labels`、`test_generate_empty_graph_produces_empty_code`、`test_collapsed_node_height_is_smaller`、`test_collapsed_height_matches_ports`。

### 测试

- `cargo test`：76 个 lib tests + 7 个 integration tests 全部通过，0 失败。

---

> 后续计划：进入 Phase 5，优先完成 DataFlow 重构、参数类型重构、命名空间管理（`selected_cosplay.json`）等高复杂度任务。

### 修复（2026-07-07 第二轮）：修复单击节点无法选中（之前仅拖拽可选中），单击边/空白正常清除选区。
- 快捷键失效：全局快捷键 Ctrl+C/Ctrl+V/Ctrl+Z/Ctrl+Y/Delete 改用 `consume_key`，避免被文本输入消费。
- 保存/导出对话框：保存 JSON、导出 JSON、导出 .code 按钮均改为 `rfd` 文件保存对话框。
- 框选虚线：Crossing 模式（右→左拖拽）选择框改为虚线绘制。
- 图层顺序：连线渲染在节点下层，选中节点置顶渲染，多选拖动时被联动节点不会被未选中节点遮挡。
- 视口裁剪：仅渲染画布可见区域附近（边距 50px）的节点和连线，大幅优化大量节点时的帧率。
- 拖线目标端口填充：拖拽连线时悬停的输入端口圆点颜色随目标状态变化（兼容/不兼容/占用/成环）。
- 空白处右键菜单：右键画布空白处可弹出"粘贴"菜单（需剪贴板非空）。
- 搜索窗口关闭：搜索窗口支持 Escape 键和 ✕ 关闭按钮退出。
