# CM2Editer 项目进度清单

> 用途：单人项目管理 + Agent 任务追踪
> 更新规则：每次 Agent 交付后，由用户（或 Agent）更新对应条目状态
> 文件位置：`docs/TODO.md`

---

## 当前冲刺（Current Sprint）

| 任务 | 状态 | 负责人 | 最后更新 |
| ----- | ----- | -------- | ---------- |
| 初始化 Rust 项目骨架（Cargo.toml + 目录结构） | ✅ 完成 | Agent | 2026-07-05 |
| 实现 `error.rs` 全局错误类型 | ✅ 完成 | Agent | 2026-07-05 |
| 实现 `graph/types.rs`（NodeType / PortType 枚举） | ✅ 完成 | Agent | 2026-07-05 |
| 实现 `graph/node.rs`（Node / Port / ParamValue 结构） | ✅ 完成 | Agent | 2026-07-05 |
| 实现 `graph/edge.rs`（Edge / EdgeEndpoint 结构） | ✅ 完成 | Agent | 2026-07-05 |
| 实现 `graph/graph.rs`（Graph 容器：增删节点/边） | ✅ 完成 | Agent | 2026-07-05 |
| 实现 `graph/validation.rs`（图验证器基础） | ✅ 完成 | Agent | 2026-07-05 |
| 实现 `api/definitions.rs`（节点/端口/参数定义） | ✅ 完成 | Agent | 2026-07-06 |
| 实现 `api/registry.rs`（节点注册表） | ✅ 完成 | Agent | 2026-07-06 |

---

## Phase 0：项目初始化（地基）

- [x] **0.1** 创建 Git 仓库，提交初始 `.gitignore`（Rust 模板）
- [x] **0.2** 编写 `Cargo.toml`（依赖：serde、serde_json、thiserror、uuid；`Vec2` 由自定义结构体实现，未采用 `glam`）
- [x] **0.3** 创建完整目录结构（`src/graph/`、`src/serializer/`、`src/code_gen/`、`src/ui/`、`src/api/`、`tests/`）
- [x] **0.4** 编写 `README.md`（项目简介、构建命令、MIT License）
- [x] **0.5** 配置 `rustfmt.toml` / `clippy` 规则（统一代码风格）
- [x] **0.6** 第一次 `cargo build` 通过，确认环境无问题

**验收标准：**`cargo check` 和 `cargo test` 能正常运行（即使测试为空）。

---

## Phase 1：数据层（核心骨架）

> 目标：不依赖 GUI，先把"图"的数据结构跑通，能单元测试。

### 1.1 错误与类型系统

- [x] **1.1.1** 实现 `src/error.rs` — `FlowError` 枚举 + `Result<T>` 别名
- [x] **1.1.2** 实现 `src/graph/types.rs` — `NodeType` 枚举（143 种节点变体，含控制流、通用函数、游戏函数、数学/字符串/文件函数、对象构造函数及特殊节点）、`PortType` 枚举（含兼容性与颜色）
- [x] **1.1.3** 实现 `src/api/mod.rs` + `src/api/definitions.rs` — `NodeDefinition` / `PortDefinition` / `ParamDefinition` 结构
- [x] **1.1.4** 实现 `src/api/registry.rs` — 节点注册表（静态定义 → 运行时查询）

### 1.2 图数据结构

- [x] **1.2.1** 实现 `src/graph/node.rs` — `Node` / `Port` / `ParamValue` / `Vec2`
- [x] **1.2.2** 实现 `src/graph/edge.rs` — `Edge` / `EdgeEndpoint`
- [x] **1.2.3** 实现 `src/graph/graph.rs` — `Graph` 容器（`add_node`、`remove_node`、`add_edge`、级联删除）
- [x] **1.2.4** 实现 `src/graph/mod.rs` — 模块导出（含 types）

### 1.3 图验证器

- [x] **1.3.1** 实现 `check_unique_ids` — 节点 ID 唯一性
- [x] **1.3.2** 实现 `check_edge_endpoints` — 端点存在性
- [x] **1.3.3** 实现 `check_type_compatibility` — 端口类型匹配
- [x] **1.3.4** 实现 `check_single_input_per_port` — 数据端口单入边
- [x] **1.3.5** 实现 `check_no_cycles` — Flow 边 DAG 检测（Kahn 算法）
- [x] **1.3.6** 实现 `check_required_params` — 必填参数检查（基础版：检查 `ParamValue::Null`；待接入 `api::definitions`）
- [x] **1.3.7** 实现 `GraphValidator::validate` — 统一入口

### 1.4 单元测试（数据层）

- [x] **1.4.1** 测试：添加/删除节点，级联删除边
- [x] **1.4.2** 测试：创建环，验证器正确报错
- [x] **1.4.3** 测试：类型不匹配连接，验证器正确报错
- [x] **1.4.4** 测试：数据端口多入边，验证器正确报错
- [x] **1.4.5** 测试：必填参数缺失，验证器正确报错（基础版：Null 参数）

**Phase 1 验收标准：**

- `cargo test` 全部通过
- 可以纯代码构建一个 Graph，添加节点和边，运行验证器

---

## Phase 2：序列化与代码生成

> 目标：图 ↔ JSON 能双向转换；图 → `.code` 能生成游戏脚本。

### 2.1 JSON 序列化

- [ ] **2.1.1** 实现 `src/serializer/json.rs` — `Graph → JSON`（含 `meta` 透传）
- [ ] **2.1.2** 实现 `src/serializer/json.rs` — `JSON → Graph`（含版本检查）
- [ ] **2.1.3** 实现 `src/serializer/migration.rs` — 版本迁移（1.0 → 1.1 → 1.2 → 1.3）
- [ ] **2.1.4** 集成测试：JSON 往返（Graph → JSON → Graph）数据一致性

### 2.2 代码生成器

- [ ] **2.2.1** 实现 `src/code_gen/formatter.rs` — 缩进管理器（Tab 缩进）
- [ ] **2.2.2** 实现 `src/code_gen/generator.rs` — 遍历 Flow 边生成行代码
- [ ] **2.2.3** 处理 `Label` 节点 — 生成 `labelname:`
- [ ] **2.2.4** 处理 `If` / `While` / `For` — 生成缩进块
- [ ] **2.2.5** 处理 `Goto` / `CreateThread` / `CreateListener` — 跳转与并发语义
- [ ] **2.2.6** 处理参数引用 — 端口连接 → 变量名替换
- [ ] **2.2.7** 处理 `Return` — 生成 `_result` 赋值
- [ ] **2.2.8** 生成 `.code` 文件到磁盘
- [ ] **2.2.9** 集成测试：用 `docs/examples/` 的示例验证输出

**Phase 2 验收标准：**

- 所有 4 个示例任务（`NPC_type`、`Test`、`MessengerExample`、`drop bra and panties`）能导入为 Graph，再导出为 `.code`，语义一致
- JSON 保存/加载不丢数据

---

## Phase 3：UI 层（前端）

> ⚠️ 前置检查：开始 Phase 3 前务必先读 `docs/interaction_spec.md`
> UX 债务：拖线实时环检测反馈、空画布启动体验
> 不解决这两点，Phase 3 验收标准直接不合格。
> 目标：能用鼠标拖方块、连线、改参数、看 JSON。

### 3.1 GUI 框架选型与搭建

- [ ] **3.1.1** 确定 GUI 框架（egui / iced / Tauri+Web）
- [ ] **3.1.2** 初始化框架项目结构，跑通窗口
- [ ] **3.1.3** 实现 `src/ui/theme.rs` — 颜色主题、节点分类色表（顶部添加颜色来源注释：`// 颜色来源：docs/node_types.md 第 12 节`）

### 3.2 画布（Canvas）

- [ ] **3.2.1** 实现 `src/ui/canvas.rs` — 无限网格背景
- [ ] **3.2.2** 实现画布平移（中键拖拽）
- [ ] **3.2.3** 实现画布缩放（滚轮，以鼠标为中心，0.1x ~ 4x）
- [ ] **3.2.4** 实现 `viewport` 保存/恢复（JSON 中 `viewport` 字段）

### 3.3 节点渲染

- [ ] **3.3.1** 实现 `src/ui/node_renderer.rs` — 节点卡片（标题栏 + 端口）
- [ ] **3.3.2** 按分类着色标题栏
- [ ] **3.3.3** 渲染端口圆点（左入右出，颜色按类型）
- [ ] **3.3.4** 渲染参数预览（折叠/展开）
- [ ] **3.3.5** 选中状态（蓝色发光边框）
- [ ] **3.3.6** 错误节点高亮（红色边框）

### 3.4 连线渲染

- [ ] **3.4.1** 实现 `src/ui/edge_renderer.rs` — 贝塞尔曲线连接
- [ ] **3.4.2** 支持 `waypoints` 中间点
- [ ] **3.4.3** 连线高亮（靠近兼容端口时）

### 3.5 交互

- [ ] **3.5.1** 实现 `src/ui/interaction.rs` — 节点拖拽（左键）
- [ ] **3.5.2** 框选（Shift + 拖拽）
- [ ] **3.5.3** 从端口拖出线创建连线
- [ ] **3.5.4** 右键节点菜单（复制、删除、折叠）
- [ ] **3.5.5** 双击空白 / 按 Space — 快速搜索创建节点
- [ ] **3.5.6** Delete 删除选中节点/边
- [ ] **3.5.7** Ctrl+Z / Ctrl+Y — 撤销/重做（至少 50 步）
- [ ] **3.5.8** Ctrl+S — 保存 JSON

### 3.6 面板

- [ ] **3.6.1** 实现 `src/ui/panels/node_library.rs` — 左栏分类树 + 搜索
- [ ] **3.6.2** 实现 `src/ui/panels/properties.rs` — 右栏参数编辑（输入框/下拉/开关）
- [ ] **3.6.3** 实现 `src/ui/panels/json_preview.rs` — 底部实时 JSON 预览
- [ ] **3.6.4** 实现 `src/ui/panels/status_bar.rs` — 底部状态栏（错误数、坐标、缩放）

### 3.7 应用主循环

- [ ] **3.7.1** 实现 `src/app.rs` — 应用状态管理（当前文件、选中项、剪贴板）
- [ ] **3.7.2** 实现 `src/main.rs` — 入口，启动 GUI
- [ ] **3.7.3** 工具栏：保存 | 撤销 | 重做 | 导出 JSON | 导出 `.code` | 运行预览

**Phase 3 验收标准：**

- 能打开编辑器，从左栏拖节点到画布
- 能连 Flow 线和 Data 线
- 能改参数，保存为 JSON，再加载回来
- 能导出 `.code` 文件

---

## Phase 4：集成测试与打磨

- [ ] **4.1** 端到端测试：创建图 → 保存 JSON → 加载 → 验证 → 生成 `.code`
- [ ] **4.2** 用 `docs/examples/` 全部 4 个示例完整验证序列化与代码生成
- [ ] **4.3** 性能测试：100+ 节点画布不卡顿
- [ ] **4.4** 边界测试：空图、单节点、全折叠、全展开
- [ ] **4.5** 错误处理：加载损坏 JSON 时友好提示（不 panic）
- [ ] **4.6** 键盘快捷键完整实现（对照 `agent_prompt.md` 第 5.3 节）
- [ ] **4.7** 节点分类颜色与 `node_types.md` 色表一致
- [ ] **4.8** 文档同步：更新 `README.md`、`CHANGELOG.md`

**Phase 4 验收标准：**

- 所有示例任务能正确导入导出
- 无已知崩溃路径

---

## Phase 5：发布与后续

- [ ] **5.1** 整理 `docs/` 目录，确保所有 `.md` 与代码一致
- [ ] **5.2** 编写用户手册（如何安装、如何写第一个任务）
- [ ] **5.3** GitHub Release v0.1.0（Windows 可执行文件）
- [ ] **5.4** 收集反馈，建立 Issue 模板
- [ ] **5.5** 规划 v0.2.0（如：子图/Group 节点、主题切换、多语言 UI）

---

## Agent 工作日志

> 每次 Agent 完成一个（或一批）任务后，在这里追加记录。
> 格式：`[日期] 任务编号/名 — 简要说明 — 状态`

| 2026-07-05 | 0.5 | 配置 `rustfmt.toml`、`.clippy.toml` 与 `Cargo.toml [lints]`，统一代码风格 | ✅ |
| 2026-07-05 | 0.4 | 编写 `README.md`（项目简介、构建命令、MIT License） | ✅ |
| 2026-07-05 | Phase 0.2,0.3,0.6 | 创建 Cargo.toml、目录结构，cargo build 通过 | ✅ |
| 2026-07-05 | 1.1.1 | 实现 `error.rs` — FlowError + Result | ✅ |
| 2026-07-05 | 1.1.2 | 实现 `graph/types.rs` — NodeType(143种) + PortType | ✅ |
| 2026-07-05 | 1.2.4 | 实现 `graph/mod.rs` 模块导出 | ✅ |
| 2026-07-05 | 当前冲刺 | 实现 `graph/node.rs`、`edge.rs`、`graph.rs`、`validation.rs` 及单元测试；`cargo check` / `cargo test` / `cargo clippy` 通过 | ✅ |
| 2026-07-06 | 1.1.3,1.1.4 | 实现 `api/definitions.rs`、`api/registry.rs`、更新 `NodeType`/`ParamValue` 派生；`cargo test` 通过（34 项） | ✅ |

---

## 快速参考：给 Agent 发任务时的模板

```markdown
请实现以下功能：

**阶段**：Phase X
**任务**：任务编号 + 名称
**模块**：`src/xxx/xxx.rs`
**依赖**：`graph::types`, `graph::node`
**输入/输出**：...
**约束**：
- 使用 Result 错误处理，禁止 unwrap
- 写文档注释（///）
- 包含单元测试
- 不引入 unsafe

**验收标准**：
1. [ ] `cargo check` 通过
2. [ ] `cargo test` 通过
3. [ ] `cargo clippy` 无警告
4. [ ] `cargo fmt` 已格式化
```text

---

## 备注区（用户随手记）

- GUI 框架倾向：**egui**（开发快，即时模式，适合工具型编辑器）
- 项目路径：`D:\Workshop\CM2Editer`（Windows 开发机）
- 树莓派不跑 GUI，只做后端测试
- 记得给 GitHub 仓库加 topic：`rust`, `egui`, `node-editor`, `custom-missions-2`, `visual-scripting`
