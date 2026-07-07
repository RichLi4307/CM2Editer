# CM2Editer

![Rust CI](https://github.com/RichLi4307/CM2Editer/actions/workflows/rust.yml/badge.svg)
![Crates.io](https://img.shields.io/badge/crates.io-unpublished-lightgrey)
![Rust](https://img.shields.io/badge/rust-2024%20edition-dea584?logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue)
![egui](https://img.shields.io/badge/egui-0.31-amber)

CM2Editer 是给游戏 **Secret Flasher Manaka** 做自定义任务的一个节点式可视化编辑器。如果你给这个游戏制作 Custom Missions 2 的任务Mod，可以直接在画布上拖节点、连端口、填参数，最后导出成 `.code` 文件，让 Custom Missions 2 加载器读取。

Custom Missions 2 是 Crisp2002 写的第三方任务加载器，目前版本 2.2.1。CM2Editer能把节点图翻译成Custom Missions 2加载器能识别的脚本，省得作者纯手写代码。

## 这个工具是什么

- **做什么的**：给 [Secret Flasher Manaka](https://f95zone.to/threads/secret-flasher-manaka-v1-1-1-sheablesoft.256682/)（SheableSoft）做自定义V2任务，用节点图代替手写 `.code` 脚本。
- **输出什么**：兼容 [Custom Missions 2](https://f95zone.to/threads/secret-flasher-manaka-custom-missions-1-2-1-version-2-2-1.263276/) 加载器（Crisp2002，v2.2.1）的 `.code` 任务文件。
- **怎么用**：在画布上拖节点、连端口、填参数，编辑器会把每个 `.code` 文件保存为工程文件夹中的节点图 JSON，最终生成工程内所有 `.code` 文件和 `meta.json`。
- **技术栈**：Rust 处理数据、序列化和代码生成，egui 做默认桌面 GUI。新增 `src/project.rs` 管理工程文件夹、`meta.json` 和多 `.code` 文件。
- **给谁用**：给这款游戏做 自定义V2任务Mod 的作者。

## 当前阶段

Phase 1（数据层）和 Phase 2（序列化与代码生成）已完成：节点图可以保存成 JSON 工程文件，也能从 JSON 还原，带版本迁移，最终生成 `.code`。`cargo test` 全部通过，clippy 没有警告。

Phase 3（UI 层）已完成：GUI 框架为 egui + eframe，四栏布局已跑通。交互功能包括节点拖拽、中键平移、框选、从端口拖线创建连线、右键菜单、Space 快速搜索、Delete 删除、Ctrl+Z/Y 撤销/重做；工具栏支持打开 JSON、保存、导出 JSON、导出 `.code`、运行预览。当前 `cargo run` 会启动一个可用的可视化编辑器。

Phase 4（集成测试与打磨）已完成：端到端集成测试、示例验证、性能测试、边界测试、错误处理、快捷键、文档同步均已完成，`cargo test` 全部通过。

Phase 4.5（工程/项目管理，发布前必做）已完成：编辑器已从"单一文件编辑器"升级为"工程管理器"。现在一个工程对应一个文件夹，包含 `meta.json`、一个或多个 `.code` 文件以及编辑器内部的 `.cm2editor/` 节点图 JSON。支持新建/打开工程文件夹、左栏文件树管理多个 `.code` 文件、右栏 `meta.json` 文本编辑、底部 `.code` 文本编辑器、保存工程时同步更新所有文件、导出工程到 `CustomMissions2` 目录。

Phase 5（新功能）已规划：在工程管理完成后，推进 DataFlow 重构、参数类型重构、命名空间管理、坐标"语言糖"、左栏二级菜单等。

Phase 5（新功能）已规划：基于作者反馈，优先进行 **DataFlow 重构**（让 If/While 条件可连接数据流）、**参数类型重构**（可枚举参数改为下拉表）、**命名空间管理**（引入 `selected_cosplay.json` 等）、**坐标"语言糖"**、**左栏二级菜单**等。

## 能做什么

用 CM2Editer 做任务，就是在节点图里描述“玩家进入任务后会发生什么”。你可以：

- **搭建任务流程**：从 `Start` 节点开始，用 `If` 做分支、`While`/`For` 做循环、`Goto` 做跳转，再用 `Wait` 和 `WaitForEvent` 控制时间点和事件触发。
- **控制游戏场景**：切换场景、设置玩家位置/动作/摄像机、播放音效、触发游戏结束。
- **操作 NPC 和角色**：生成 NPC、设置路径点、让 NPC 瞬移或播放动作、读取 NPC 状态（是否看到玩家、警觉度等）。
- **放置任务触发器**：创建区域（Area/Zone）、交互区域（InteractArea）、条件（Condition/ItemCondition），检测玩家是否进入或满足条件。
- **做任务 UI**：用 `MissionPanel` 和 `MissionMenuItem` 设置任务面板文字、进度条和菜单项；用 `Text` 和 `Messenger` 显示字幕和手机聊天。
- **管理玩家数值**：RP、体力、快感、物品数量等，可以进行加减或读取。
- **多线程和事件**：通过 `CreateThread`/`CreateListener` 并行跑逻辑，用 `SetEvent`/`GetEvent` 跨线程通信。
- **导出和验证**：验证节点图合法性后，生成 Custom Missions 2 能读取的 `.code` 文件和 `meta.json`；以工程文件夹形式保存，方便管理多个 `.code` 文件和再次编辑。

## 怎么运行

项目用 Rust 2024 Edition 和 Cargo 管理依赖。

### 环境要求

- Rust 工具链，建议用 `stable` 并保证 `cargo` 是最新的，可以通过 [rustup](https://rustup.rs/) 安装
- 主要开发平台是 Windows；Linux 和 macOS 也能跑数据层

### 把任务放进游戏需要什么

生成 `.code` 文件后，要放到游戏里运行，需要：

- **游戏本体**：[Secret Flasher Manaka](https://f95zone.to/threads/secret-flasher-manaka-v1-1-1-sheablesoft.256682/) v1.1.1（推荐） 或 v1.1.3（SheableSoft）
- **加载器**：Custom Missions v2.2.1（Crisp2002，[F95Zone 发布页](https://f95zone.to/threads/secret-flasher-manaka-custom-missions-1-2-1-version-2-2-1.263276/)）
  - 把导出的工程文件夹（包含 `meta.json` 和若干 `.code` 文件）放到游戏目录的 `CustomMissions2` 文件夹里
- **编辑器字体**：CM2Editer 已内置 [思源黑体 / Source Han Sans SC](https://github.com/adobe-fonts/source-han-sans)（Adobe，SIL Open Font License 1.1），位于 `assets/fonts/思源黑体/`。程序启动时会优先加载内置字体，若缺失则回退到 Windows 系统字体（微软雅黑 / 黑体）。字体授权文件见同目录 `LICENSE.txt`。
- **游戏中文字体**（如果需要显示中文）：把 `NotoSerifSC-Regular.otf` 或 `.ttf` 放到游戏根目录，游戏使用 Noto Serif（UI/字幕）和 Noto Sans（手机界面）字体

### 常用命令

```bash
# 检查并编译
cargo check

# 跑单元测试
cargo test

# 构建 Debug 版本
cargo build

# 构建 Release 版本
cargo build --release

# 静态检查
cargo clippy -- -D warnings

# 格式化
cargo fmt

# 启动 GUI（会打开 egui 窗口）
cargo run
```

> `cargo run` 现在会启动一个带四栏布局的 egui 编辑器窗口，支持拖节点、连端口、改参数、新建/打开工程文件夹、管理多个 `.code` 文件、编辑 `meta.json`、导出整个工程到 `CustomMissions2` 目录、撤销/重做。Phase 5 将引入 DataFlow 数据流连线和参数下拉表。

## 项目结构

```text
CM2Editer/
├── Cargo.toml           # 项目依赖配置
├── src/
│   ├── app.rs           # 应用状态、撤销/重做、主循环与布局
│   ├── main.rs          # 应用入口
│   ├── error.rs         # 全局错误类型 FlowError
│   ├── project.rs       # 工程管理：meta.json、多 .code 文件、保存/导出
│   ├── graph/           # 节点、边、图和验证器（任务逻辑的结构基础）
│   ├── api/             # 节点定义和注册表（有哪些节点可以用）
│   ├── serializer/      # JSON 工程文件保存和版本迁移
│   ├── code_gen/        # 把节点图翻译成 .code 文件
│   └── ui/              # egui 界面（主题、画布、节点/连线渲染、交互、面板）
├── tests/               # 集成测试
└── docs/                # 设计文档、节点清单、JSON 格式、示例
```

## 文档

- [docs/TODO.md](docs/TODO.md)：项目进度和任务追踪
- [docs/agent_prompt.md](docs/agent_prompt.md)：开发约束、JSON 契约和 UI 设计规范
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

## 许可证

本项目采用 [MIT License](LICENSE) 开源。

Copyright (c) 2026 RichLi4307.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
