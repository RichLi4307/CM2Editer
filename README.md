# CM2Editer

![Rust CI](https://github.com/RichLi4307/CM2Editer/actions/workflows/rust.yml/badge.svg)

CM2Editer 是面向游戏 **Secret Flasher Manaka**（由 SheableSoft 开发）的 **Custom Missions 2** 加载器（作者：Crisp2002，版本 2.2.1）的节点式可视化任务编辑器，目标用户是 Mod 作者。通过拖拽节点、连接端口并填写参数，即可为 Custom Missions 2 加载器生成兼容的自定义任务脚本（`.code` 文件），无需手写代码。

## 项目定位

- **目标游戏**：[Secret Flasher Manaka](https://f95zone.to/threads/secret-flasher-manaka-v1-1-1-sheablesoft.256682/)（SheableSoft）— 本编辑器为 [Custom Missions 2](https://f95zone.to/threads/secret-flasher-manaka-custom-missions-1-2-1-version-2-2-1.263276/) 加载器生成兼容的 `.code` 任务文件。
- **核心工作流**：可视化画布 → 中间 JSON → 加载器可读取的 `.code` 脚本。
- **技术栈**：Rust（后端数据 / 序列化 / 代码生成）+ egui（默认桌面 GUI 框架）。

## 当前阶段

本项目仍处于早期开发中，已完成数据层（图结构、节点/边/端口、验证器）及单元测试。后续将实现 JSON 序列化、`.code` 代码生成、GUI 编辑器及端到端示例验证。

## 功能预览

- 节点式编辑：拖拽节点、连接 Flow/Data 端口、编辑参数。
- 数据验证：节点 ID 唯一性、边端点存在性、端口类型兼容性、Flow 环检测、必填参数检查等。
- 序列化：保存/加载 JSON 中间格式，支持版本迁移。
- 代码生成：遍历图生成 CustomMissions 2 可读取的 `.code` 文件。
- 导出接口：导出 JSON / 导出 `.code`。

## 构建与运行

本项目使用 Rust 2024 Edition 与 Cargo 管理依赖。

### 环境要求

- Rust 工具链（建议使用 `stable`，并安装最新版 `cargo`）：
  - 可通过 [rustup](https://rustup.rs/) 安装。
- 支持平台：Windows（主要开发平台）、Linux / macOS 可兼容运行数据层。

### 运行时依赖

CM2Editer 生成的自定义任务需要以下环境才能运行：

- **游戏本体**：[Secret Flasher Manaka](https://f95zone.to/threads/secret-flasher-manaka-v1-1-1-sheablesoft.256682/) v1.1.1 或 v1.1.3（SheableSoft）
- **加载器**：Custom Missions v2.2.1（作者 Crisp2002 — [F95Zone 发布页](https://f95zone.to/threads/secret-flasher-manaka-custom-missions-1-2-1-version-2-2-1.263276/)）
  - 将生成的 `.code` 文件放入游戏目录的 `CustomMissions2` 文件夹中即可生效。
- **字体补充**（中文等非拉丁字符）：将 `NotoSerifSC-Regular.otf`/`.ttf` 等字体文件放入游戏根目录，游戏使用 Noto Serif（UI/字幕）与 Noto Sans（手机界面）字体。

### 常用命令

```bash
# 检查代码并编译
cargo check

# 运行单元测试
cargo test

# 构建 Debug 版本
cargo build

# 构建 Release 版本
cargo build --release

# 运行 Clippy 静态检查
cargo clippy -- -D warnings

# 格式化代码
cargo fmt

# 启动 GUI 编辑器（完成 UI 层后可用）
cargo run
```

> 当前仓库中 GUI 尚未实现，`cargo run` 暂时只进入占位入口。
> 本编辑器生成的 `.code` 文件要求游戏已安装 Custom Missions v2.2.1 加载器才能运行。

## 项目结构

```text
CM2Editer/
├── Cargo.toml           # 项目依赖配置
├── src/
│   ├── main.rs          # 应用入口
│   ├── error.rs         # 全局错误类型 FlowError
│   ├── graph/           # 图数据结构（节点/边/图/验证器）
│   ├── api/             # 节点静态定义与注册表（待实现）
│   ├── serializer/      # JSON 序列化与版本迁移（待实现）
│   ├── code_gen/        # .code 脚本生成器（待实现）
│   └── ui/              # egui 界面层（待实现）
├── tests/               # 集成测试
└── docs/                # 设计文档、节点清单、JSON 契约、示例
```

## 文档索引

- [docs/TODO.md](docs/TODO.md)：项目进度与任务追踪。
- [docs/agent_prompt.md](docs/agent_prompt.md)：开发约束、JSON 契约与 UI 设计规范。
- [docs/node_types.md](docs/node_types.md)：全部节点类型清单。
- [docs/json_schema.md](docs/json_schema.md)：编辑器保存的 JSON 格式定义。
- [docs/examples/new npc type/](docs/examples/new%20npc%20type/)：示例任务。
- [docs/rust_project_skeleton.md](docs/rust_project_skeleton.md)：Rust 项目骨架说明。

## 免责声明 / 独立声明

> **本编辑器是第三方社区作品，非官方出品。**

**CM2Editer** 是一款面向游戏 **Secret Flasher Manaka**（SheableSoft）的 **Custom Missions** 加载器模组生态的独立可视化流编辑器。本编辑器由社区开发者独立维护，与：

- **Secret Flasher Manaka**（SheableSoft、F95Zone）及其开发者
- **Custom Missions** 加载器作者 Crisp2002

**均不存在官方合作、背书或隶属关系**。

### 知识产权归属

- **Secret Flasher Manaka** 的名称、标识及相关知识产权归 SheableSoft 所有。
- **Custom Missions** 的名称、标识及相关知识产权归 Crisp2002 所有。
- 本编辑器中提及上述名称仅用于兼容性说明，不暗示任何所有权或关联关系。

### API 使用说明

本编辑器在开发过程中仅参考了 Custom Missions 作者公开的 API 文档与示例任务文件格式（`.code` 文件），通过调用**公开接口规范**生成兼容的任务文件。本编辑器的源代码中**不包含** Secret Flasher Manaka 游戏本体或 Custom Missions 加载器本体的任何代码或二进制文件，不涉及对游戏或加载器的逆向工程。

### MIT 免责声明

本编辑器基于 MIT 许可证开源，按"**原样**"（AS IS）提供，**不提供任何明示或暗示的担保**，包括但不限于适销性、特定用途适用性及不侵权的默示担保。作者不对因使用本编辑器而导致的任何直接或间接损失承担责任。

### 社区边界

- 如果你遇到 **Secret Flasher Manaka 游戏本体** 或 **Custom Missions 加载器**本身的问题（如游戏崩溃、兼容性问题、安装问题），请向对应的官方发布渠道（F95Zone）反馈。
- 如果你遇到**本编辑器**的问题（如节点连接异常、导出文件格式错误、UI 显示错误），欢迎在本仓库提交 Issue 或 Pull Request。

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
