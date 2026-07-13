# CM2Editer

![Rust CI](https://github.com/RichLi4307/CM2Editer/actions/workflows/rust.yml/badge.svg)
![Rust](https://img.shields.io/badge/rust-2024_dea584?logo=rust&logoColor=white)
![egui](https://img.shields.io/badge/egui-0.31-amber)
![Tests](https://img.shields.io/badge/tests-109%20passed-brightgreen)
![Version](https://img.shields.io/badge/version-0.2.2-blue)
![License](https://img.shields.io/badge/license-MIT-blue)

CM2Editer 是给游戏 **Secret Flasher Manaka** 做自定义任务的一个节点式可视化编辑器。如果你给这个游戏制作 Custom Missions 2 的任务Mod，可以直接在画布上拖节点、连端口、填参数，最后导出成 `.code` 文件，让 Custom Missions 2 加载器读取。

Custom Missions 2 是 Crisp2002 写的第三方任务加载器，目前版本 2.2.1。CM2Editer能把节点图翻译成Custom Missions 2加载器能识别的脚本，省得作者纯手写代码。

## 当前版本

**v0.2.2** — 补齐 Data 端口链路（Goto/CreateListener `out_label`/`out_name` 完整可用）、Label 节点支持 Data 边自动获取名称、节点预览显示 🔗 连接状态、CreateCondition 下拉补全 6 个 `Exposed_*` 暴露条件、验证器 BFS 重构消除子标签误报。110 项测试通过。

## 这个工具是什么

- **做什么的**：给 [Secret Flasher Manaka](https://f95zone.to/threads/secret-flasher-manaka-v1-1-1-sheablesoft.256682/)（SheableSoft）做自定义V2任务，用节点图代替手写 `.code` 脚本。
- **输出什么**：兼容 [Custom Missions 2](https://f95zone.to/threads/secret-flasher-manaka-custom-missions-1-2-1-version-2-2-1.263276/) 加载器（Crisp2002，v2.2.1）的 `.code` 任务文件。
- **怎么用**：在画布上拖节点、连端口、填参数，编辑器会把每个 `.code` 文件保存为工程文件夹中的节点图 JSON，最终生成工程内所有 `.code` 文件和 `meta.json`。
- **技术栈**：Rust 处理数据、序列化和代码生成，egui 做默认桌面 GUI。
- **给谁用**：给这款游戏做 自定义V2任务Mod 的作者。

## 当前阶段

- **Phase 1–4**（数据层、序列化/代码生成、UI、集成测试、工程管理）已完成。
- **Phase 5**（新功能）进行中：
  - ✅ 5.1.1 DataFlow 重构 — 节点参数自动生成 Data 端口，虚线渲染，属性面板数据源下拉框
  - ✅ 5.1.2 参数类型重构 — `ParamType::Enum`，场景/动作/技能/物品等 20+ 参数改为下拉选择
  - ✅ 5.1.3 命名空间管理 — cosplay/adult_toy/avatar_type 等命名空间 JSON 文件，悬浮选择器窗口
  - ✅ 5.2.2 静态检查 — 多 Start 警告、不可达节点警告
  - ✅ 5.2.3 Start 菱形依赖警告
  - ✅ 5.2.4 错误详情面板 — 状态栏错误计数可点击展开详情列表
  - ✅ 5.4.1 旧图档必填参数默认值修复
- 底部面板重构为三列可拖拽布局（代码 ┃ JSON ┃ DataFlow），DataFlow 方块巧克力板排列，Data 虚线可单独选中删除。

## 能做什么

CM2Editer 目前实现了 CM2 API 的核心功能。以下是**确实可用**的能力：

### ✅ 已实现且可用

- **控制流**：`Start` → `If`/`While`/`For` → `Label` → `Goto` → `Break`/`Return` 完整状态机
- **线程与监听器**：`CreateThread`、`CreateListener`、`CreateListenerLocal`，每帧轮询 + Goto 状态切换
- **Boolean 管道**：`GetStateBool`/`GetStateNumber` → `CompareNumbers` → `LogicAnd/Or/Not` → `If` 的 11 节点条件组合
- **条件对象**：`CreateCondition` + `CheckCondition` → `If`，下拉覆盖 30+ 常用条件关键词
- **坐标系统**：`GetPosition` + 16 个默认坐标预设 + `MakeVector`/`BreakVector`
- **命名空间管理**：cosplay 等 7 个命名空间，188 条 cosplay 条目全部带中文翻译
- **游戏状态读写**：`SetPlayerPosition`/`SetStage`/`SetAction`/`SetFutanari`/`SetSkill`/`LockHandcuffs`/`SetVibrator`/`SetPiston`/`PlaySoundEffect` 等 20+ 个 API 调用
- **数值操作**：RP/体力/快感/物品数量的加减、读取
- **对象创建**：`CreateList`、`CreateMissionPanel`、`CreateArea`、`CreateNPC`、`CreateGallery` 等
- **数据流（DataFlow）**：虚线连接 Data 端口传递参数，代码生成优先使用 Data 值

### ⚠️ 未实现或受限

以下是真实 `.code` 任务中**常用但 CM2Editer 尚未支持**的功能：

| 缺失功能 | 影响 |
|---------|------|
| **`list.Insert`/`.Remove`/`.Contains`/`.Count`** 等列表操作 | 无法操作动态列表 |
| **Gallery API**（`.Show()`/`.Confirmed()`） | 拍照功能不可用 |
| **MessengerChat API**（`.Add()`/`.SetButtons()`/`.Clicked()`） | 聊天 UI 不可用 |
| **`_save` / `_settings` / `_time` / `_timediff`** 全局变量 | 无法做存档/计时逻辑 |
| **`elseif` 多分支** | 仅支持 `if/else`，多条件需嵌套 |
| **`thread.WaitForFinish`** | 无法等待子线程 |
| **`listener = null`** 销毁监听器 | 无显式销毁方式 |
| **`_this` 当前线程引用** | 无法传递线程引用 |
| **对象方法调用**（`Area.Inside`/`NPC.Warp` 等） | 需通过泛型 `CallMethod` 手写参数 |

> 以上 9 项已列入 `docs/TODO.md` 的 P1 优先级。其余未列举的 CM2 API 全部实现。详见 `docs/code_api_reference.md`。

## 怎么运行

项目用 Rust 2024 Edition 和 Cargo 管理依赖。

### 环境要求

- Rust 工具链（stable），通过 [rustup](https://rustup.rs/) 安装
- 主要开发平台是 Windows；Linux 和 macOS 也能跑数据层

### 把任务放进游戏需要什么

生成 `.code` 文件后，要放到游戏里运行，需要：

- **游戏本体**：[Secret Flasher Manaka](https://f95zone.to/threads/secret-flasher-manaka-v1-1-1-sheablesoft.256682/) v1.1.1（推荐） 或 v1.1.3（SheableSoft）
- **加载器**：Custom Missions v2.2.1（Crisp2002，[F95Zone 发布页](https://f95zone.to/threads/secret-flasher-manaka-custom-missions-1-2-1-version-2-2-1.263276/)）
  - 把导出的工程文件夹（包含 `meta.json` 和若干 `.code` 文件）放到游戏目录的 `CustomMissions2` 文件夹里
- **编辑器字体**：CM2Editer 已内置 [思源黑体 / Source Han Sans SC](https://github.com/adobe-fonts/source-han-sans)（Adobe，SIL Open Font License 1.1），位于 `assets/fonts/思源黑体/`。程序启动时会优先加载内置字体，若缺失则回退到 Windows 系统字体。字体授权文件见同目录 `LICENSE.txt`。
- **命名空间数据**：`assets/namespaces/` 下的 JSON 文件提供 cosplay、成人玩具、外观类型等枚举数据。Release 发行时应一并打包此目录。

### 常用命令

```bash
# 检查并编译
cargo check

# 跑单元测试
cargo test

# 构建 Debug 版本
cargo build

# 构建 Release 版本（产物 target/release/CM2Editer.exe）
cargo build --release

# 静态检查
cargo clippy

# 格式化
cargo fmt

# 启动 GUI
cargo run
```

> `cargo run` 启动可视化编辑器。底部三列面板：左侧代码预览、中间 JSON 预览、右侧 DataFlow 数据方块，均可拖拽调整宽度。底部面板整体可拖拽调整高度。支持 Ctrl+Z/Y 撤销/重做、Space 搜索添加节点、Delete 删除选中项（连线优先仅删连线）、点击状态栏错误计数查看详情。

## 项目结构

```text
CM2Editer/
├── Cargo.toml
├── src/
│   ├── app.rs           # 应用状态、撤销/重做、主循环与底部面板布局
│   ├── main.rs          # 应用入口
│   ├── error.rs         # 全局错误类型 FlowError（含 Warning 非阻塞警告）
│   ├── project.rs       # 工程管理：meta.json、多 .code 文件、保存/导出
│   ├── graph/           # 节点、边、图和验证器（含静态检查与菱形依赖警告）
│   ├── api/             # 节点定义、枚举常量、命名空间注册表
│   │   ├── definitions.rs
│   │   ├── enums.rs     # 游戏常量枚举（场景/动作/技能/物品/音效等）
│   │   ├── namespace.rs # 命名空间注册表（cosplay/adult_toy/avatar_type 等）
│   │   └── registry.rs
│   ├── serializer/      # JSON 工程文件保存和版本迁移（含必填参数默认值补填）
│   ├── code_gen/        # 节点图 → .code 代码生成（Data 端口优先）
│   └── ui/              # egui 界面
│       ├── panels/      # 面板：节点库、属性、数据菜单、JSON 预览、代码编辑、
│       │                #       命名空间选择器、工程文件树、状态栏、meta 编辑
│       ├── canvas.rs
│       ├── edge_renderer.rs
│       ├── interaction.rs
│       ├── node_renderer.rs
│       └── theme.rs
├── tests/               # 集成测试（代码生成、示例验证、JSON 往返）
├── assets/
│   ├── fonts/           # 思源黑体字体
│   └── namespaces/      # cosplay/adult_toy/avatar_type 等命名空间 JSON
└── docs/                # 设计文档、TODO、示例
```

## 文档

- [AGENTS.md](AGENTS.md)：开发约束、JSON 契约、UI 设计规范和文档地图
- [docs/TODO.md](docs/TODO.md)：项目进度和任务追踪
- [docs/node_types.md](docs/node_types.md)：全部节点类型清单
- [docs/json_schema.md](docs/json_schema.md)：编辑器保存的 JSON 格式定义
- [docs/rust_project_skeleton.md](docs/rust_project_skeleton.md)：Rust 项目骨架说明

## 说明

> **本编辑器是第三方社区作品，和官方没关系。**

CM2Editer 是我自己独立维护的社区工具，给 **Secret Flasher Manaka**（SheableSoft）的 **Custom Missions** 模组做辅助编辑。它和 SheableSoft、F95Zone 以及 Custom Missions 的作者 Crisp2002 没有官方合作、背书或隶属关系。

**Secret Flasher Manaka** 的名称、商标和相关知识产权归 SheableSoft 所有。**Custom Missions** 的名称、商标和相关知识产权归 Crisp2002 所有。本 README 里提到这些名字只是为了说明兼容性，不代表任何所有权或关联关系。

开发时我只参考了 Custom Missions 作者公开的 API 文档和示例 `.code` 文件，按公开的接口规范生成兼容任务文件。本项目的源代码里不包含 Secret Flasher Manaka 游戏本体或 Custom Missions 加载器的代码/二进制文件，也不涉及逆向工程。

如果遇到游戏本体或加载器本身的问题（比如崩溃、兼容性、安装等），请去 F95Zone 对应发布页反馈。如果是编辑器的问题（比如节点连不上、导出文件格式不对、UI 显示异常），可以在这个仓库开 Issue。

---

## 致谢

- **夕拾酒** — 解释了 `.code` 的本质是类 Python 语言，阐明了 CM2 模组的工作流程（Thread/Listener/Goto 状态机模型），为本项目的代码生成器重构提供了关键指导。

## 许可证

本项目采用 [MIT License](LICENSE) 开源。

Copyright (c) 2026 RichLi4307.
