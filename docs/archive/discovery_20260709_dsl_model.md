# 2026-07-09 .code DSL 重大发现与修复记录

## 发现 1：.code 语言模型是类 Python，不是类 Lua

**时间**: 2026-07-09 凌晨
**证据**: 用户提供的 80+ 个前辈手搓 `.code` 文件 + `docs/documentation.html` 官方英文文档

### 关键差异

| 我们的旧认知 | 实际模型 |
|-------------|---------|
| `main:` 是硬编码程序入口 | `main:` 只是一个标签名，由顶层 `CreateThread("main")` 触发 |
| 代码全部在 `main:` 内 | 顶层代码（无标签）在模块加载时自动执行 |
| `If(true) [` 括号语法 | `if condition` 小写 + 缩进块 |
| Lua 风格 | Python 风格（缩进定义作用域，无显式 end） |

### 修复（P0 代码生成器重构）

1. **顶层 CreateThread**：为所有标签生成 top-level CreateThread（`dbcb2a2`）
2. **_result = null 收尾**：每个标签末尾自动追加（Return 已有时跳过）
3. **Goto 语法**：`Goto(label)` → `thread.Goto(label)`
4. **操作符**：`&&` / `||` 而非 `and` / `or`

## 发现 2：标签生命周期断裂

**时间**: 2026-07-09 中午
**问题**: Goto 节点引用的目标标签不存在于 `graph.labels`，生成的 `.code` 有断裂引用

### 修复（`fb1adf3`）

`collect_labels()` 新增自动发现：

- 扫描所有 Goto / CreateThread 节点的 target label 名
- 不在 `graph.labels` 中的，自动创建空标签（只有 `_result = null`）
- CreateListener 目标**排除**（它们是嵌套子标签，定义在父标签内部）

## 产出文件

| 文件 | 说明 |
|------|------|
| `docs/code_api_reference.md` | 基于官方英文文档 + 80+ 例的 DSL 权威参考 |
| `docs/if_condition_design.md` | Monitor→Condition→If 管道架构设计 |
| `docs/code_pseudocode_map.md` | 112 个 .code 文件的 Python 伪代码映射表 |
| `docs/问题清单.md` | 当前问题清单（P0/P1/P2） |
| `src/code_gen/generator.rs` | 重构后的代码生成器 |

## 统计数据

- NodeType 变体数：154
- 测试数：109（93 lib + 16 integration）
- 代码行数：~2100 app.rs, ~920 generator.rs
- v0.1.1 release：6.7 MB
