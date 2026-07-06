# CustomMissions2 流编辑器 — Rust 项目骨架

> 版本：v1.0
> 用途：定义 Rust 项目目录结构、模块划分和初始代码
> 阅读对象：Agent（后端实现）
> 状态：**规划稿 / 设计草案**；`Cargo.toml`、目录结构及已实现的源文件以仓库实际文件为准，本文档中的依赖列表和阶段状态为规划草案，实现时以代码为准。
> 相关文档：
>
> - 节点清单：[node_types.md](node_types.md)
> - JSON 规范：[json_schema.md](json_schema.md)
> - 开发约束：[agent_prompt.md](agent_prompt.md)

---

## 一、目录结构

```text
CM2Editer/
├── Cargo.toml                 # 项目配置
├── Cargo.lock                 # 依赖锁定（自动生成）
├── LICENSE                    # MIT License
├── README.md                  # 项目说明
├── assets/                    # 静态资源（图标、字体、主题）
│   ├── icons/
│   └── themes/
├── src/
│   ├── main.rs                # 入口点
│   ├── lib.rs                 # 库入口（可选）
│   ├── app.rs                 # 应用主循环 / 状态管理（也可扩展为 app/ 目录）
│   ├── config.rs              # 配置加载与保存
│   ├── error.rs               # 全局错误类型定义
│   ├── graph/                 # 图数据结构（核心）
│   │   ├── mod.rs
│   │   ├── node.rs            # Node 结构体 + 端口
│   │   ├── edge.rs            # Edge 结构体
│   │   ├── graph.rs           # Graph 容器（节点+边集合）
│   │   ├── types.rs           # 节点类型枚举、端口类型枚举
│   │   └── validation.rs      # 图验证器（环检测、类型检查等）
│   ├── serializer/            # 序列化 / 反序列化
│   │   ├── mod.rs
│   │   ├── json.rs            # JSON ↔ Graph 转换
│   │   └── migration.rs       # 版本迁移逻辑
│   ├── code_gen/              # 代码生成（Graph → .code）
│   │   ├── mod.rs
│   │   ├── generator.rs       # 主生成器
│   │   ├── formatter.rs       # 缩进与格式化
│   │   └── templates/         # 代码模板
│   │       └── node_templates.json
│   ├── ui/                    # UI 层（前端交互）
│   │   ├── mod.rs
│   │   ├── canvas.rs          # 画布（无限网格、平移缩放）
│   │   ├── node_renderer.rs   # 节点渲染
│   │   ├── edge_renderer.rs   # 连线渲染
│   │   ├── panels/            # 面板
│   │   │   ├── mod.rs
│   │   │   ├── node_library.rs
│   │   │   ├── properties.rs
│   │   │   ├── json_preview.rs
│   │   │   └── status_bar.rs
│   │   ├── interaction.rs     # 交互逻辑
│   │   └── theme.rs           # 主题与颜色配置
│   └── api/                   # 游戏 API 定义（节点清单的数据层）
│       ├── mod.rs
│       ├── definitions.rs     # 所有函数/对象的静态定义
│       └── registry.rs        # 节点注册表（运行时查询）
├── tests/                     # 集成测试
│   ├── fixtures/              # 测试数据（示例 JSON、.code 文件）
│   ├── test_graph.rs
│   ├── test_serializer.rs
│   └── test_validation.rs
└── docs/                      # 文档
    ├── agent_prompt.md
    ├── node_types.md
    ├── json_schema.md
    └── rust_project_skeleton.md
```

---

## 二、Cargo.toml

```toml
[package]
name = "CM2Editer"
version = "0.1.0"
edition = "2024"
authors = ["richli"]
license = "MIT"
description = "A node-based visual editor for CustomMissions2 task scripts"
repository = "https://github.com/richli/CM2Editer"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
uuid = { version = "1.8", features = ["v4"] }
egui = "0.31"
eframe = "0.31"

[lints.rust]
unused_extern_crates = "warn"
unused_import_braces = "warn"
unused_lifetimes = "warn"

[lints.clippy]
all = { level = "warn", priority = -1 }
unwrap_used = "deny"
expect_used = "deny"
redundant_clone = "warn"
needless_pass_by_value = "warn"
```

### 依赖说明

| 依赖 | 用途 |
|------|------|
| `serde` / `serde_json` | JSON 序列化与反序列化 |
| `thiserror` | 错误处理 |
| `uuid` | 全局唯一 ID 生成 |
| `egui` / `eframe` | 即时模式 GUI |

> 注意：实际 `Cargo.toml` 中未引入 `glam`；`Vec2` 由 `src/graph/node.rs` 中的自定义结构体实现，详见 `graph::node::Vec2`。
> 实际 `Cargo.toml` 中已配置 `[lints.rust]` 与 `[lints.clippy]`，用于统一代码风格与禁止 `unwrap` / `expect`。

---

## 三、核心模块说明

各模块的完整源代码见 `src/` 目录。以下是模块职责与关键设计要点速览：

| 模块 | 文件 | 职责 | 关键设计 |
|------|------|------|----------|
| error | `src/error.rs` | 全局错误类型 | `FlowError` 枚举 + `Result<T>` 别名 |
| graph/types | `src/graph/types.rs` | 类型系统 | `NodeType`（143 种） + `PortType` + 兼容性检查 |
| graph/node | `src/graph/node.rs` | 节点结构 | `Node` / `Port` / `ParamValue`(Literal/Ref/Null) / `Vec2` |
| graph/edge | `src/graph/edge.rs` | 连线结构 | `Edge` / `EdgeEndpoint` / waypoints |
| graph/graph | `src/graph/graph.rs` | 图容器 | `Graph { nodes, edges, labels }` + 增删查操作 |
| graph/validation | `src/graph/validation.rs` | 图验证器 | `GraphValidator::validate` + Kahn 环检测 |
| api/definitions | `src/api/definitions.rs` | 节点元数据 | `NodeDefinition` / `PortDefinition` / `ParamDefinition` |
| api/registry | `src/api/registry.rs` | 节点注册表 | 类型名 ↔ NodeDefinition 查询 |

### NodeType 计数规则

- 当前 `NodeType` 枚举包含 **143 个变体**：涵盖控制流、通用函数、游戏函数、数学/字符串/文件函数、对象构造函数，以及 `Meta` / `Comment` / `Group` 特殊节点。
- `node_types.md` 中列出的**对象方法**（如 `Area.Inside`、`NPC.Warp`、`Text.Add` 等）不单独映射为 `NodeType` 枚举变体；运行时通过 `(Object, MethodName)` 组合或 `CallMethod` 动态调用表示。
- 因此，`NodeType` 的计数口径为「可直接实例化的节点类型」，而非「API 上可触发的所有函数调用」。若需扩展对象方法为独立节点类型，请先更新 `NodeType` 枚举、注册表和 `node_types.md` 第 13 节。

---

## 四、模块划分原则

模块划分遵循**单向依赖**原则，避免循环引用。

| 层级 | 职责 | 依赖关系 |
|------|------|----------|
| **api** | 静态定义所有节点类型、参数、端口 | 不依赖其他模块 |
| **graph** | 核心数据结构（节点、边、图） | 依赖 `api::types` |
| **serializer** | JSON 读写、版本迁移 | 依赖 `graph`；不依赖 `api` |
| **code_gen** | 生成 `.code` 文件 | 依赖 `graph`、`api` |
| **ui** | 用户界面、画布交互 | 依赖 `graph`、`api` |
| **app** | 应用状态、主循环、事件分发 | 依赖所有上层模块 |

### 依赖图

```text
           ┌─────┐
           │ api │
           └──┬──┘
              │
     ┌────────┼────────┐
     ▼        ▼        ▼
  ┌──────┐ ┌────┐ ┌──────────┐
  │graph │ │ ui │ │ code_gen │
  └──┬───┘ └────┘ └─────┬────┘
     │                  │
     ▼                  ▼
  ┌──────────┐     ┌─────────┐
  │serializer│     │   app   │
  └──────────┘     └─────────┘
```

---

## 五、构建与运行

```bash
cargo build
cargo run
cargo test
cargo test -- --nocapture
cargo build --release
cargo fmt
cargo check
cargo clippy -- -D warnings
cargo doc --open
```

### 推荐 CI 流程

```yaml
steps:
  - uses: actions/checkout@v4
  - uses: dtolnay/rust-toolchain@stable
  - run: cargo fmt --check
  - run: cargo clippy -- -D warnings
  - run: cargo test
```

---

## 六、下一步任务（给 Agent）

按以下顺序实现可降低模块间耦合：

### 阶段 1：数据层 ✅ 已完成

1. ✅ 实现 `api::definitions` — 为每个 `NodeType` 定义参数模板和端口模板
2. ✅ 实现 `api::registry` — 运行时节点查询（根据类型名获取定义）
3. ✅ 实现 `graph::validation::check_required_params` — 接入 `api::definitions` 完成必填参数检查

### 阶段 2：序列化与代码生成 ✅ 已完成

1. ✅ 实现 `serializer::migration` — 版本迁移逻辑
2. ✅ 实现 `serializer::json` — JSON 工程文件读写
3. ✅ 实现 `code_gen::generator` — 将 Graph 导出为 `.code` 文件
4. ✅ 实现 `code_gen::formatter` — 缩进与格式化

### 阶段 3：UI 层

1. ✅ 实现 `ui::theme` — 颜色主题与节点分类色表
2. [ ] 实现 `ui::canvas` — 无限画布（网格、平移、缩放、viewport）
3. [ ] 实现 `ui::node_renderer` — 节点卡片渲染
4. [ ] 实现 `ui::edge_renderer` — 连线渲染（支持 waypoints）
5. [ ] 实现 `ui::interaction` — 拖拽、框选、连线创建
6. [ ] 实现 `ui::panels::node_library` — 左栏分类树 + 搜索
7. [ ] 实现 `ui::panels::properties` — 右栏参数编辑表单
8. [ ] 实现 `ui::panels::json_preview` — 底部实时 JSON 预览
9. [ ] 实现 `ui::panels::status_bar` — 底部状态栏

### 阶段 4：集成

1. [ ] 集成测试 — 端到端：创建图 → 保存 JSON → 加载 → 验证 → 生成代码
2. [ ] 所有示例任务导入测试
