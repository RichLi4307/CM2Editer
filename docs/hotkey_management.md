# CM2Editer 全局热键管理规范

## 原则

**文本框有焦点时，所有全局热键禁止触发**——Ctrl+Z 不是"撤销节点"，而是"撤销文本框中的输入"。

唯一例外：Ctrl+S（保存）始终生效。

## 实现

Ctrl+C/V 需要**双重保护**，因为 eframe 在不同平台上的行为不同：
- 部分平台（Windows 7/10 旧版 winit）Ctrl+C/V 作为 `Key::C`/`Key::V` 事件
- 部分平台（Wayland/Windows 11）Ctrl+C/V 被 eframe 转为 `Event::Copy`/`Event::Paste`

因此两层都要门控：

```rust
let keyboard_active = ctx.wants_keyboard_input();

// 层 1：consume_key（Key 事件平台）
if !keyboard_active && consume_key(CTRL, C) { copy_selected(); }
if !keyboard_active && consume_key(CTRL, V) { paste_at(); }

// 层 2：Event::Copy/Paste（Event 平台）
ctx.input_mut(|i| {
    i.events.retain(|event| {
        if search_open || keyboard_active { return true; }
        match event {
            Event::Copy => { copy_selected(); false }
            Event::Paste => { paste_at(); false }
            _ => true,
        }
    });
});
```

## 热键清单

| 热键 | 作用 | 文本框焦点时 | 位置 |
|------|------|-------------|------|
| `Space` | 切换搜索窗口 | ❌ 禁用 | `app.rs:update` |
| `Ctrl+Z` | 撤销节点操作 | ❌ 禁用 | `app.rs:update` |
| `Ctrl+Y` | 重做节点操作 | ❌ 禁用 | `app.rs:update` |
| `Ctrl+C` | 复制选中节点 | ❌ 禁用 | `app.rs:update` |
| `Ctrl+V` | 粘贴节点到画布 | ❌ 禁用 | `app.rs:update` |
| `Delete` | 删除选中 | ❌ 禁用 | `app.rs:update` |
| `Ctrl+S` | 保存工程 | ✅ 始终可用 | `app.rs:update` |
| `Escape` | 关闭对话框 | ✅ 仅对话框上下文 | `app.rs:draw_dialogs` |
| `Event::Copy` | 系统复制事件 | ❌ 文本框直通 | `app.rs:update` input_mut |
| `Event::Paste` | 系统粘贴事件 | ❌ 文本框直通 | `app.rs:update` input_mut |

## 新增热键流程

任何新热键必须回答三个问题：

1. **文本框焦点时是否应该触发？** → 绝大多数情况答案是"否"，不触发
2. **是 `consume_key` 还是 Event 级别？** → 节点操作用 `consume_key`，Ctrl+C/V 用 Event（eframe 转换）
3. **是否需要 `wants_keyboard_input()` 门控？** → 除非是 Ctrl+S 这类"永远该触发"的，都需要

## 相关文件

- `src/app.rs:update()` — 全局热键注册点
- `src/ui/panels/param_text_edit.rs` — 文本框实现（不注册全局热键）
- `src/ui/interaction.rs` — 画布交互（Ctrl/Shift 修饰符用于框选，非热键）
