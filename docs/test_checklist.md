# CM2Editer v0.3.0 客户端测试检查单

> 用 JM 标注 [已测试/通过]，用 DN 标注 [已测试/不通过]，用 NT 标注 [未测试]
>
> 本版已适配新架构（ThreadContainer / LabelContainer / ListenerContainer），Start / Label 节点弃用；
> 并新增/优化：场景分类节点库、节点收藏、_state 探针选择器、条件组合编辑器实时校验、条件模板、id 数据流输入、全局变量数据节点、i18n 详细描述、For+Range 直连、CreateArea 长方体、CallMethod 方法下拉等。

---

## 1. 欢迎页 + 工程管理

| # | 测试项 | 预期 | 结果 | 备注 |
|---|--------|------|------|------|
| 1.1 | 启动程序，查看欢迎页 | 显示深色卡片 + 打开工程/新建工程按钮 + Space 提示 | JM | 没有欢迎页 |
| 1.2 | 点击"打开工程" | 弹出文件对话框，选择工程文件夹后加载 | JM | |
| 1.3 | 点击"新建工程" | 弹出新建对话框，填写名称后创建工程；画布为空，用户可在项目树新建 ThreadContainer/Label | DN | 已修复：不再自动创建线程，项目树新增 +T/+Lb/+Ls 与删除按钮 |
| 1.4 | 窗口大小 | 启动时默认最大化，窗口无闪烁、不超出屏幕 | JM | 已修复最大化 |
| 1.5 | Ctrl+S 保存工程 | 同步写回 meta.json、所有 .code 与 .cm2editor/*.code.json | JM | |
| 1.6 | 保存后再打开 | 节点图、视口位置完整还原 | JM | |

---

## 2. 左栏：节点库 / 工程树 / 命名空间 / 坐标

### 2.1 节点库（场景分类）

| # | 测试项 | 预期 | 结果 | 备注 |
|---|--------|------|------|------|
| 2.1.1 | 节点库一级分类 | 显示任务/流程、条件判定、数据获取、数据修改、数据处理、视觉/UI、编辑器等场景分类 | JM | 见手动测试备注 |
| 2.1.2 | 二级分类折叠 | 一级 → 二级 → 节点列表逐级折叠展开 | JM | P2.8 已细分 player_state / math 等 |
| 2.1.3 | 搜索框按节点名过滤 | 输入文字过滤节点列表 | DN | 已修复：搜索支持大小写不敏感 |
| 2.1.4 | 搜索框按场景关键字过滤 | 输入分类名（如"条件"）可命中对应分类下节点 | JM | |
| 2.1.5 | 拖出节点放置画布 | 画布上出现新节点；拖拽虚影宽度足够、单行无换行 | JM | 已修复最小宽度与截断 |
| 2.1.6 | 跨场景节点 | 同一节点（如 CanGameOver）可出现在多个场景分类中 | JM | |
| 2.1.7 | 节点悬停/属性面板描述 | 属性面板显示 1–2 句中文详细描述 | DN | 已修复：节点库与画布悬停均显示描述 Tooltip |
| 2.1.8 | 节点收藏/置顶 | 节点库顶部显示收藏区域；每个节点可星标切换；状态持久化 | JM | P2.5 新增 |

### 2.2 工程文件树

| # | 测试项 | 预期 | 结果 | 备注 |
|---|--------|------|------|------|
| 2.2.1 | 文件树显示 | 显示 meta.json 和 .code 文件 | JM | |
| 2.2.2 | 节点库/文件树分隔条 | 可拖拽调整上下高度；悬停/拖拽时高亮蓝色 + ResizeVertical 光标 | JM | |
| 2.2.3 | 文件树独立滚动 | 上下文过长时文件树自己出现滚动条，底部按钮不被挤出 | DN | 已修复：底部按钮移入 ScrollArea，不再溢出或速度不一致 |

### 2.3 命名空间标签

| # | 测试项 | 预期 | 结果 | 备注 |
|---|--------|------|------|------|
| 2.3.1 | 命名空间按钮可点击 | 弹出命名空间浏览窗口 | JM | |
| 2.3.2 | cosplay 二级分类 | 展开 cosplay → 显示头部/上装/下装/生殖等子分类 | JM | |
| 2.3.3 | 点击卡片复制 key | 复制到剪贴板 + 状态栏提示 | JM | |
| 2.3.4 | Add / 删除按钮 | 内联表单可添加条目；每条条目右侧可删除，需二次确认 | JM | 已新增删除按钮 |

### 2.4 坐标标签

| # | 测试项 | 预期 | 结果 | 备注 |
|---|--------|------|------|------|
| 2.4.1 | 场景分组 | 按 stage 分组显示 | JM | 已新增删除按钮 |
| 2.4.2 | 展开场景 | CollapsingHeader 展开/折叠，内部显示坐标名和 x/y/z | JM | |
| 2.4.3 | Add / 删除按钮 | 内联表单可添加条目；每条条目可删除，需二次确认 | JM | 已新增删除按钮 |

---

## 3. 画布操作

| # | 测试项 | 预期 | 结果 | 备注 |
|---|--------|------|------|------|
| 3.1 | 拖拽节点 | 节点跟随鼠标移动 | JM | |
| 3.2 | 滚轮缩放 | 画布缩放 | JM | |
| 3.3 | 中键平移 | 画布平移 | JM | |
| 3.4 | 框选节点 | 矩形框选多个节点 | JM | |
| 3.5 | 点击节点选中 | 属性面板显示参数 | JM | |
| 3.6 | Space 打开搜索 | 搜索窗口弹出，输入节点名过滤 | JM | 已修复：搜索支持大小写不敏感 |
| 3.7 | Delete 删除节点 | 选中节点被删除 | JM | |
| 3.8 | Ctrl+Z 撤销 | 撤销上一步 | JM | |
| 3.9 | Ctrl+Y 重做 | 重做上一步 | JM | |
| 3.10 | Ctrl+C/V 复制粘贴 | 复制节点到新位置 | JM | |
| 3.11 | 容器内拖入节点 | 拖节点进入 LabelContainer / ListenerContainer，归属随位置切换 | JM | |

---

## 4. 连线（Flow + Data）

| # | 测试项 | 预期 | 结果 | 备注 |
|---|--------|------|------|------|
| 4.1 | 拖拽输出端口到输入端口 | 创建 Flow 连线 | JM | |
| 4.2 | 拖拽 Data 输出到 Data 输入 | 创建 Data 虚线连线 | JM | |
| 4.3 | 选中节点 → Data 虚线可见 | 相关 Data 边渲染 | JM | |
| 4.4 | 点击虚线选中 | 虚线高亮 | JM | |
| 4.5 | Delete 删选中虚线 | 仅删虚线，不删节点 | JM | |
| 4.6 | 连接 If.condition ← Boolean 输出 | Data 连线建立 | JM | |
| 4.7 | 连接 CompareNumbers.a ← GetStateNumber | Data 连线建立 | JM | |
| 4.8 | 连接 CreateCondition.id ← StringConstant | id 数据输入端口接受 String 数据连线 | JM | |
| 4.9 | 连接 For.iterable ← Range.out_list | Data 连线建立，代码生成 `for i in Range(0, 10)` | NT | |
| 4.10 | 连接 For.iterable ← 其他 List 输出 | Data 连线建立，代码生成 `for i in {list}` | NT | |

---

## 5. 容器结构（新架构）

| # | 测试项 | 预期 | 结果 | 备注 |
|---|--------|------|------|------|
| 5.1 | ThreadContainer 显示 | 线程容器含标签列表，入口钉位于标签左上角 | NT | |
| 5.2 | 新建 LabelContainer | 可添加新标签，入口钉自动生成 | NT | |
| 5.3 | 新建 ListenerContainer | 可添加监听器容器，入口钉自动生成 | NT | |
| 5.4 | 容器内 Flow 连线 | 同一容器内 Flow 表示顺序执行 | NT | |
| 5.5 | 跨容器禁止 Flow | 验证器报错或 UI 阻止跨容器 Flow 边 | NT | |
| 5.6 | 同线程跨 Label 的 Data 边 | 允许，代码生成按拓扑解析 | NT | |
| 5.7 | 旧工程兼容 | 打开 v1.x 工程提示迁移，Start/Label 反序列化不崩溃 | NT | |

---

## 6. 节点参数编辑（属性面板）

### 6.1 Boolean 常量

| # | 测试项 | 预期 | 结果 | 备注 |
|---|--------|------|------|------|
| 6.1.1 | 拖入 Boolean 节点 | 出现在画布上 | NT | |
| 6.1.2 | 参数 value | 下拉框显示 true / false | NT | |
| 6.1.3 | 切换 value | 选中后更新 | NT | |
| 6.1.4 | out_value 端口类型 | Boolean | NT | |

### 6.2 GetStateBool / GetStateNumber

| # | 测试项 | 预期 | 结果 | 备注 |
|---|--------|------|------|------|
| 6.2.1 | GetStateBool stateKey | 自由文本输入 + "选择状态" 按钮；打开树形选择器 | NT | P2.6 改为字符串 + 探针选择器 |
| 6.2.2 | GetStateNumber stateKey | 同上，选择 Number 类型路径 | NT | |
| 6.2.3 | out_value 端口类型 | Boolean / Number | NT | |

### 6.3 CompareNumbers / Logic 节点

| # | 测试项 | 预期 | 结果 | 备注 |
|---|--------|------|------|------|
| 6.3.1 | CompareNumbers operator | 下拉框显示 >= == != > < <= | NT | |
| 6.3.2 | Data 端口 a, b | Number 类型，可连线；无连接时 DragValue 手动输入 | NT | |
| 6.3.3 | LogicAnd / LogicOr | a, b Boolean 输入，out_result Boolean 输出 | NT | |
| 6.3.4 | LogicNot | a Boolean 输入，out_result Boolean 输出 | NT | |

### 6.4 If / While

| # | 测试项 | 预期 | 结果 | 备注 |
|---|--------|------|------|------|
| 6.4.1 | If.condition 参数 | 条件下拉模板可选 + 文本框可编辑；多个条件模板同时存在时无 egui 重复 ID 警告 | NT | 已修复 widget ID |
| 6.4.2 | If.condition Data 连线后 | 只读显示 🔗，模板消失 | NT | |
| 6.4.3 | While.condition 参数 | 同 If | NT | |

### 6.5 CreateCondition 条件组合编辑器（新增）

| # | 测试项 | 预期 | 结果 | 备注 |
|---|--------|------|------|------|
| 6.5.1 | condition 参数"编辑条件..."按钮 | 弹出条件组合编辑器窗口 | NT | |
| 6.5.2 | 点击条件 token 插入 | 在表达式框当前光标处插入 token 中文译名+原始 token 双行按钮 | NT | |
| 6.5.3 | AND/OR/NOT 包裹选区 | 选中一段表达式后点 AND/OR/NOT，选区被 `[...]`/`(...)`/`!` 包裹 | NT | |
| 6.5.4 | 括号内逗号追加 | 光标已在 `[...]` 或 `(...)` 内时，按条件 token 在逗号后追加；按 AND/OR 直接追加逗号 | NT | |
| 6.5.5 | 无焦点连续点击 | 文本框失焦后连续点条件 A、B，A 不被替换，B 追加在上次光标后 | NT | |
| 6.5.6 | SubCondition 复用列表 | 显示当前标签内已有条件 ID，点击插入 `SubCondition_<id>` | NT | |
| 6.5.7 | 确认回写 | 确认后参数更新到节点，生成 `CreateCondition("...")` | NT | |
| 6.5.8 | id 参数说明 | 属性面板显示 id 用于 SubCondition 复用的中/英文说明 | NT | |
| 6.5.9 | id 数据流输入 | StringConstant → id 数据连线后生成 `CreateCondition("...", id=var_xxx)`；无连接时回退常量 | NT | |
| 6.5.10 | CreateItemCondition 同上 | id 数据端口行为一致 | NT | |
| 6.5.11 | 表达式实时校验 | 括号不匹配、未知 token、空组等在预览下方红字提示，不阻塞确认 | NT | P2.7 新增 |

### 6.6 坐标节点

| # | 测试项 | 预期 | 结果 | 备注 |
|---|--------|------|------|------|
| 6.6.1 | GetPosition coord_id | 文本框 + "选坐标" 按钮 | NT | |
| 6.6.2 | GetPosition stage | 下拉框显示 16 个场景 | NT | |
| 6.6.3 | MakeVector / BreakVector | xyz Number ↔ List 互转 | NT | |

### 6.7 Vector / Color 参数（通用）

| # | 测试项 | 预期 | 结果 | 备注 |
|---|--------|------|------|------|
| 6.7.1 | CreateArea.position / shape | type 下拉显示 sphere / cylinder / cuboid；cuboid 显示 position2 / w 参数 | NT | P2.11 新增长方体 |
| 6.7.2 | SetPlayerPosition.position | 📍 按钮 + x/y/z DragValue 字段 | NT | |
| 6.7.3 | ShowBlackscreen.color | DragValue 字段 | NT | |
| 6.7.4 | 参数名旁类型标签 | [str]/[num]/[xyz]/[list] 等 | NT | |

### 6.8 Goto / CallFunction / 线程控制 / CallMethod

| # | 测试项 | 预期 | 结果 | 备注 |
|---|--------|------|------|------|
| 6.8.1 | Goto.label 参数 | 文本输入标签名 | NT | |
| 6.8.2 | Goto.args 参数（可选） | Object 类型 | NT | |
| 6.8.3 | CallFunction.function / params | 文本输入函数名 + List 参数 | NT | |
| 6.8.4 | DestroyListener | 生成 `listener = null` | NT | |
| 6.8.5 | GetCurrentThread | 纯数据节点，输出 `_this` | NT | |
| 6.8.6 | WaitForThread | 生成 `{thread}.WaitForFinish()` | NT | |
| 6.8.7 | CallMethod.method | 根据 thread 连接的对象类型，显示方法下拉；选中后自动填充 params 模板 | NT | P2.3 方法下拉 |

### 6.9 全局变量数据节点（新增）

| # | 测试项 | 预期 | 结果 | 备注 |
|---|--------|------|------|------|
| 6.9.1 | GetSave | 输出 `_save` | NT | |
| 6.9.2 | GetTime / GetTimeDiff | 输出 `_time` / `_timediff` | NT | |
| 6.9.3 | GetSettings | 输出 `_settings` | NT | |
| 6.9.4 | GetMod / GetMods | 输出 `_mod` / `_mods` | NT | |

---

## 7. 底部面板

| # | 测试项 | 预期 | 结果 | 备注 |
|---|--------|------|------|------|
| 7.1 | 底栏整体高度拖拽 | 拖拽调整高度，不弹回 | NT | |
| 7.2 | 代码┃JSON┃DataFlow 三列 | 三列并排 | NT | |
| 7.3 | 分隔线拖拽 | 鼠标变双箭头，拖拽可调整列宽 | NT | |
| 7.4 | 分隔线可见 | 灰色竖线，悬停变蓝 | NT | |
| 7.5 | .code 预览滚动 | 长内容可滚动，不撑高底栏 | NT | |
| 7.6 | JSON 预览滚动 | 长 JSON 可滚动，不撑高底栏 | NT | |
| 7.7 | DataFlow 方块 | 巧克力板排列，点击选中节点 | NT | |
| 7.8 | 状态栏错误计数 | 显示错误数 | NT | |
| 7.9 | 点击错误计数 | 弹出错误详情窗口 | NT | |

---

## 8. 代码生成验证

| # | 测试项 | 预期 | 结果 | 备注 |
|---|--------|------|------|------|
| 8.1 | 入口 → Log → Return 链路 | `main:` 标签内 `Log(...)`，显式 Return 才生成 `_result = null` | NT | 旧版 Start 链路作废 |
| 8.2 | 入口 → Goto("step1") | 顶层 `CreateThread` + `thread.Goto("step1")` | NT | |
| 8.3 | GetStateBool→If | `if (_state.Futanari)` | NT | P0 fix 带外层括号 |
| 8.4 | CompareNumbers(a=GetStateNumber(Ecstasy), b=90)→If | `if (_state.Ecstasy >= 90)` | NT | |
| 8.5 | LogicAnd(Boolean(true), GetStateBool)→If | `if ((true) && (_state.Futanari))` | NT | |
| 8.6 | Ctrl+S 保存 → .code 文件生成 | `main.code` 内容与预览一致 | NT | |
| 8.7 | 从节点图生成按钮 | 同步实时 graph → 刷新 .code | NT | |
| 8.8 | 无 Return 节点时标签末尾**不**追加 `_result = null` | 标签末尾干净收尾 | NT | |
| 8.9 | 字符串参数不带 JSON 引号 | `_state.Ecstasy` 而非 `"_state.Ecstasy"` | NT | |
| 8.10 | 操作符不带 JSON 引号 | `>=` 而非 `">="` | NT | |
| 8.11 | CreateCondition 位置参数语法 | `CreateCondition("Exposed_All")`，id 为空时省略 | NT | |
| 8.12 | CreateCondition id 常量 | `CreateCondition("...", id="MyID")` | NT | |
| 8.13 | CreateCondition id 数据流 | `CreateCondition("...", id=var_xxx_id)` | NT | |
| 8.14 | CreateItemCondition 空 id 省略 | 不输出 `id=""` | NT | |
| 8.15 | For + Range 直连 | `for i in Range(0, 10)` | NT | |
| 8.16 | For 无 iterable 时使用 start/stop/step | `for i in Range(0, 10)` 或 `Range(0, 10, 2)` | NT | P2.10 |
| 8.17 | DestroyListener | `listener = null` | NT | |
| 8.18 | WaitForThread | `t.WaitForFinish()` | NT | |
| 8.19 | GetCurrentThread | 引用处生成 `_this` | NT | |
| 8.20 | CreateArea cuboid | `CreateArea(type="cuboid", ..., x1=..., y1=..., z1=..., x2=..., y2=..., z2=..., w=..., h=...)` | NT | P2.11 |
| 8.21 | CreateArea sphere / cylinder | 球体生成 x/y/z/r；圆柱体生成 x/y/z/r/h | NT | |

---

## 9. i18n / 多语言

| # | 测试项 | 预期 | 结果 | 备注 |
|---|--------|------|------|------|
| 9.1 | 中文界面 | 面板、按钮、节点名、参数名全部中文 | NT | |
| 9.2 | 节点描述（zh） | 属性面板显示详细中文描述（1-2 句） | NT | 188 个节点 description 已覆盖 |
| 9.3 | 英文界面回退 | 缺失键回退英文，不显示原始 key | NT | |
| 9.4 | 日文界面 | 场景分类、condition 译名有日文键 | NT | |

---

## 10. 验证与错误处理

| # | 测试项 | 预期 | 结果 | 备注 |
|---|--------|------|------|------|
| 10.1 | 保存前验证 | 导出前调用 GraphValidator，错误批量显示在底部面板 | NT | |
| 10.2 | 跨容器 Flow 边报错 | 验证器给出明确错误信息 | NT | |
| 10.3 | Meta / Comment / Group 特殊节点 | 允许存在，跳过拓扑排序与代码生成 | NT | |
| 10.4 | 数据端口多入边报错 | 一个 Data 输入端口多条入边时验证报错 | NT | |

## 11. 手动测试备注

- 节点分类有误，如NPCIsAlive是条件判断节点，输出布尔值，而没有出现在条件相关节点中。预约全部节点的 节点-语义-代码生成-用户使用场景-分类 校验
- 坐标标签希望在中文下使用场景名+中文名，其他语言不处理
- 不需要有默认主线程功能的情况下，重新研究入口钉子用途或平替
- 属性面板选择坐标/面向时，无法手动输入，只能打开弹窗
- NPCIsAlive请新增下拉选择已有NPC输出端口的功能
- 新增视觉效果，节点开始连线时（已选择出端口，在拖动线时），把不适配的端口变灰
