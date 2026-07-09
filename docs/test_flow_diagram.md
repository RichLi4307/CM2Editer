# CM2Editer v0.1.1 任务流程设计示意图

## 端到端模组制作流程

```text
┌────────────────────────────────────────────────────────────────────────┐
│                         1. 启动 CM2Editer                              │
│  欢迎页 → [New 新建工程] → 输入名称 → 自动创建 Start 节点 + meta.json  │
│         → [Open 打开工程] → 选择文件夹 → 加载已有工程                  │
└────────────────────────────────┬───────────────────────────────────────┘
                                 │
                                 ▼
┌────────────────────────────────────────────────────────────────────────┐
│                     2. 搭建节点图（左侧节点库拖入）                    │
│                                                                        │
│  常用模式：                                                            │
│                                                                        │
│  ┌── 模式 A：简单判断 ───────────────────────────────────┐             │
│  │                                                       │             │
│  │  [Start]                                              │             │
│  │    │ Flow                                             │             │
│  │    ▼                                                  │             │
│  │  [CreateListener("check_loop")]                       │             │
│  │    │ Flow                                             │             │
│  │    ▼                                                  │             │
│  │  [Log("started")]                                     │             │
│  │    ─── check_loop: 标签 ───                           │             │
│  │  [GetStateNumber(Ecstasy)]──Data──→[Compare.a]        │             │
│  │                                    [Compare.b=90]     │             │
│  │                                    [Compare.op= >=]   │             │
│  │                                       │ Boolean       │             │
│  │                                       ▼               │             │
│  │                                    [If]               │             │
│  │                                   ┌──┴──┐             │             │
│  │                              true▼     ▼false         │             │
│  │                          [Log(">=90")]  [Log("<90")]  │             │
│  │                                                       │             │
│  └───────────────────────────────────────────────────────┘             │
│                                                                        │
│  ┌── 模式 B：条件组合 ────────────────────────────────┐                │
│  │                                                    │                │
│  │  [GetStateBool(Futanari)]──Data──→[LogicAnd.a]     │                │
│  │  [GetStateBool(NearNPC)] ──Data──→[LogicAnd.b]     │                │
│  │                                      │ Boolean     │                │
│  │                                      ▼             │                │
│  │                                   [If]             │                │
│  └────────────────────────────────────────────────────┘                │
│                                                                        │
│  ┌── 模式 C：状态机 ─────────────────────────────────────────┐         │
│  │                                                           │         │
│  │  [Start] ──Flow──→ [Goto("step1")]                        │         │
│  │  ─── step1: ──                                            │         │
│  │  [CreateListener("wait")]                                 │         │
│  │    ─── wait: ──                                           │         │
│  │    [GetStateNumber(Ecstasy)]──→[Compare.a]                │         │
│  │                                [Compare.b=50]             │         │
│  │                                   │                       │         │
│  │                                   ▼                       │         │
│  │                              [If]──true──→[Goto("step2")] │         │
│  └───────────────────────────────────────────────────────────┘         │
└────────────────────────────────┬───────────────────────────────────────┘
                                 │
                                 ▼
┌────────────────────────────────────────────────────────────────────────┐
│                    3. 连接端口（画布交互）                             │
│                                                                        │
│  Flow 连线（实线）：输出 Flow → 输入 Flow                              │
│  Data 连线（虚线）：输出 Data → 输入 Data                              │
│                                                                        │
│  规则：                                                                │
│  • Flow 边遵循执行序，代码按 Flow 方向生成                             │
│  • Data 边仅传递值，不参与控制流                                       │
│  • Data 节点（Boolean/GetStateBool 等）无 Flow 端口，纯数据源          │
│  • Data 虚线仅在选中相关节点时渲染                                     │
│  • 选中虚线后 Delete 仅删虚线（不误删节点）                            │
└────────────────────────────────┬───────────────────────────────────────┘
                                 │
                                 ▼
┌────────────────────────────────────────────────────────────────────────┐
│                    4. 编辑参数（右侧属性面板）                         │
│                                                                        │
│  [str] = 文本框                                                        │
│  [enum] = 下拉框                                                       │
│  [num] = 数字输入                                                      │
│  [xyz] = x/y/z 三字段                                                  │
│  [bool] = 复选框                                                       │
│  [list] = JSON 文本框                                                  │
│                                                                        │
│  If.condition: 模板下拉 + 文本框（Data 连线时隐藏）                    │
│  GetPosition.coord_id: "选坐标" 按钮 → 坐标选择器                      │
│  Vector 参数:  按钮 → 坐标选择器                                       │
│  Cosplay 参数: "选择..." 按钮 → 命名空间选择器                         │
└────────────────────────────────┬───────────────────────────────────────┘
                                 │
                                 ▼
┌────────────────────────────────────────────────────────────────────────┐
│                    5. 保存 / 生成 .code                                │
│                                                                        │
│  Ctrl+S 保存：                                                         │
│    sync_active_to_project()                                            │
│      → regenerate_code()                                               │
│        → collect_labels()                    ← 发现标签                │
│        → 顶层 CreateThread 生成               ← 所有标签入口           │
│        → generate_sequence()                  ← 跟随 Flow 边           │
│        → evaluate_data_output()               ← 递归 Data 链           │
│        → _result = null 收尾                  ← 每个标签               │
│    → save()                                                            │
│      → 写入 main.code                                                  │
│      → 写入 meta.json                                                  │
│      → 写入 .cm2editor/main.code.json                                  │
│                                                                        │
│  [Regen] 按钮：同步实时 graph 后 regenerate_code()                     │
│                                                                        │
│  导出 .code：save_file_dialog → generate_code_to_file()                │
└────────────────────────────────┬───────────────────────────────────────┘
                                 │
                                 ▼
┌────────────────────────────────────────────────────────────────────────┐
│                    6. 查看结果（底部面板）                             │
│                                                                        │
│  ┌──────────────┬───────────────┬──────────────────┐                   │
│  │ 代码预览     │ JSON 预览     │ DataFlow 菜单    │                   │
│  │ .code 文本   │ 节点图 JSON   │ 数据输出方块     │                   │
│  │ 可手动编辑   │ 只读          │ 点击选中节点     │                   │
│  └──────────────┴───────────────┴──────────────────┘                   │
│                                                                        │
│  状态栏：错误计数 → 点击展开详情                                       │
│  底栏高度：顶部边缘拖拽                                                │
│  列宽：竖直分隔线拖拽                                                  │
└────────────────────────────────────────────────────────────────────────┘
```

## 代码生成管道

```text
graph.nodes + graph.edges + graph.labels
            │
            ▼
    ┌────────────────┐
    │ collect_labels │  ← 从 graph.labels + 自动发现 Goto/CreateThread 目标
    └──────┬─────────┘
           │
           ▼
    ┌───────────────────┐
    │ 顶层 CreateThread │  ← var_X_thread = CreateThread("X")
    └──────┬────────────┘
           │
           ▼
    ┌───────────────┐
    │ label:        │  ← 每个标签生成
    │  ...          │
    │  _result=null │
    └──────┬────────┘
           │
           ▼
    ┌───────────────────┐
    │ generate_sequence │  ← 跟随 Flow 边
    └──────┬────────────┘
           │
           ├── If → generate_if(condition via evaluate_data_output)
           ├── While → generate_while
           ├── Goto → thread.Goto(label)
           ├── Log → Log(output=...)
           ├── CreateThread → var = CreateThread(labelName=...)
           ├── CreateListener → var = CreateListener(labelName=...)
           ├── CallFunction → funcName(args)
           ├── ForeachNode → var = Foreach(list, thread)
           ├── Return → _result = value
           ├── Start/Label → 跳过（贯通）
           ├── Boolean/GetState*/CompareNumbers/Logic*/GetPosition/...
           │   → 不在 Flow 链中，通过 evaluate_data_output 被动解析
           └── 其他 → generate_node_call (var = Func(params))
```

## DataFlow 递归解析链

```text
If.condition ← Data 边 ← CompareNumbers.out_result
                            │
                            ├── CompareNumbers.a ← Data 边 ← GetStateNumber.out_value
                            │                                 │ stateKey=Ecstasy
                            │                                 └─→ _state.Ecstasy
                            │
                            ├── CompareNumbers.b ← param literal 90
                            │
                            └── CompareNumbers.operator ← param literal >=
                            
  → 最终生成: _state.Ecstasy >= 90
```

## 线程 / 监听器生命周期

```code
模块加载时:
  var_main = CreateThread("main")    → 立即执行 main: 一次
  var_listener = CreateThread("loop") → 立即执行 loop: 一次

main:                                 loop:
  var_L = CreateListener("check")      ...           ← 每帧执行
  _result = null                       _result = null

  check:                              后续:
    if condition                        thread.Goto → 下一状态
      thread.Goto("next")               context 销毁，新状态开始
    _result = null
```
