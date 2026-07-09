# CM2Editer v0.1.1 客户端测试检查单

> 用 JM 标注 [已测试/通过]，用 DN 标注 [已测试/不通过]，用 NT 标注 [未测试]

---

## 1. 欢迎页 + 工程管理

| # | 测试项 | 预期 | 结果 |
|---|--------|------|------|
| 1.1 | 启动程序，查看欢迎页 | 显示深色卡片 + 打开工程/新建工程按钮 + Space 提示 | |
| 1.2 | 点击"打开工程" | 弹出文件对话框，选择工程文件夹后加载 | |
| 1.3 | 点击"新建工程" | 弹出新建对话框，填写名称后创建含 Start 节点的空图 | |
| 1.4 | 窗口大小 | 占屏幕大部分区域（≈75%） | |

---

## 2. 左栏：工程 / 命名空间 / 坐标标签

| # | 测试项 | 预期 | 结果 |
|---|--------|------|------|
| 2.1 | 工程标签：节点库可搜索 | 输入文字过滤节点列表 | |
| 2.2 | 工程标签：展开节点分类 | CollapsingHeader 展开/折叠 | |
| 2.3 | 工程标签：点击节点创建 | 画布上出现新节点 | |
| 2.4 | 工程标签：下方项目文件树 | 显示 meta.json 和 .code 文件 | |
| 2.5 | 命名空间标签：命名空间按钮可点击 | 弹出命名空间浏览窗口 | |
| 2.6 | 命名空间标签：cosplay 有二级分类 | 展开 cosplay → 显示头部/上装/下装/生殖等子分类 | |
| 2.7 | 命名空间标签：点击卡片复制 key | 复制到剪贴板 + 状态栏提示 | |
| 2.8 | 坐标标签：场景分组 | 按 stage 分组显示 | |
| 2.9 | 坐标标签：展开场景 | CollapsingHeader 展开/折叠，内部显示坐标名和 x/y/z | |
| 2.10 | 坐标/命名空间 Add 按钮 | 内联表单可添加条目，添加后即时刷新 | |

---

## 3. 画布操作

| # | 测试项 | 预期 | 结果 |
|---|--------|------|------|
| 3.1 | 拖拽节点 | 节点跟随鼠标移动 | |
| 3.2 | 滚轮缩放 | 画布缩放 | |
| 3.3 | 中键平移 | 画布平移 | |
| 3.4 | 框选节点 | 矩形框选多个节点 | |
| 3.5 | 点击节点选中 | 属性面板显示参数 | |
| 3.6 | Space 打开搜索 | 搜索窗口弹出，输入节点名过滤 | |
| 3.7 | Delete 删除节点 | 选中节点被删除 | |
| 3.8 | Ctrl+Z 撤销 | 撤销上一步 | |
| 3.9 | Ctrl+Y 重做 | 重做上一步 | |
| 3.10 | Ctrl+C/V 复制粘贴 | 复制节点到新位置 | |

---

## 4. 连线（Flow + Data）

| # | 测试项 | 预期 | 结果 |
|---|--------|------|------|
| 4.1 | 拖拽输出端口到输入端口 | 创建 Flow 连线 | |
| 4.2 | 拖拽 Data 输出到 Data 输入 | 创建 Data 虚线连线 | |
| 4.3 | 选中节点 → Data 虚线可见 | 相关 Data 边渲染 | |
| 4.4 | 点击虚线选中 | 虚线高亮 | |
| 4.5 | Delete 删选中虚线 | 仅删虚线，不删节点 | |
| 4.6 | 连接 If.condition ← Boolean 输出 | Data 连线建立 | |
| 4.7 | 连接 CompareNumbers.a ← GetStateNumber | Data 连线建立 | |

---

## 5. 节点参数编辑（属性面板）

### 5.1 Boolean 常量

| # | 测试项 | 预期 | 结果 |
|---|--------|------|------|
| 5.1.1 | 拖入 Boolean 节点 | 出现在画布上 | |
| 5.1.2 | 参数 value | 下拉框显示 true / false | |
| 5.1.3 | 切换 value | 选中后更新 | |
| 5.1.4 | out_value 端口类型 | Boolean | |

### 5.2 GetStateBool

| # | 测试项 | 预期 | 结果 |
|---|--------|------|------|
| 5.2.1 | 拖入 GetStateBool 节点 | 出现在画布上 | |
| 5.2.2 | 参数 stateKey | 下拉框显示 18 个状态（Futanari, Sitting...） | |
| 5.2.3 | 选择 Futanari | 选中后刷新 | |
| 5.2.4 | out_value 端口类型 | Boolean | |

### 5.3 GetStateNumber

| # | 测试项 | 预期 | 结果 |
|---|--------|------|------|
| 5.3.1 | 拖入 GetStateNumber 节点 | 出现在画布上 | |
| 5.3.2 | 参数 stateKey | 下拉框显示 8 个状态（含 Bodypaint） | |
| 5.3.3 | 选择 Ecstasy | 选中后刷新 | |
| 5.3.4 | out_value 端口类型 | Number | |

### 5.4 CompareNumbers

| # | 测试项 | 预期 | 结果 |
|---|--------|------|------|
| 5.4.1 | 拖入 CompareNumbers 节点 | 出现在画布上 | |
| 5.4.2 | 参数 operator | 下拉框显示 >= == != > < <= | |
| 5.4.3 | Data 端口 a, b | Number 类型，可连线 | |
| 5.4.4 | out_result 端口类型 | Boolean | |
| 5.4.5 | 无 Data 连接时手动填 b 值 | DragValue 输入数字 | |

### 5.5 LogicAnd / LogicOr / LogicNot

| # | 测试项 | 预期 | 结果 |
|---|--------|------|------|
| 5.5.1 | 拖入 LogicAnd 节点 | a, b Boolean 输入，out_result Boolean 输出 | |
| 5.5.2 | 拖入 LogicOr 节点 | 同上 | |
| 5.5.3 | 拖入 LogicNot 节点 | a Boolean 输入，out_result Boolean 输出 | |

### 5.6 If / While

| # | 测试项 | 预期 | 结果 |
|---|--------|------|------|
| 5.6.1 | If.condition 参数 | 条件下拉模板可选 + 文本框可编辑 | |
| 5.6.2 | If.condition Data 连线后 | 只读显示 🔗，模板消失 | |
| 5.6.3 | While.condition 参数 | 同 If | |

### 5.7 坐标节点

| # | 测试项 | 预期 | 结果 |
|---|--------|------|------|
| 5.7.1 | 拖入 GetPosition 节点 | 出现在画布上 | |
| 5.7.2 | coord_id 参数 | 文本框 + "选坐标" 按钮 | |
| 5.7.3 | stage 参数 | 下拉框显示 16 个场景 | |
| 5.7.4 | x/y/z 参数 | DragValue 数字编辑 | |
| 5.7.5 | out_position, out_stage 端口 | List 和 String 类型 | |
| 5.7.6 | MakeVector 节点 | x/y/z Number 输入 + DragValue + out_vec List 输出 | |
| 5.7.7 | BreakVector 节点 | in_vec List 输入 + x/y/z Number 输出 | |

### 5.8 Vector / Color 参数（通用）

| # | 测试项 | 预期 | 结果 |
|---|--------|------|------|
| 5.8.1 | CreateArea.position | 📍 按钮 + x/y/z DragValue 字段 | |
| 5.8.2 | SetPlayerPosition.position | 同上 | |
| 5.8.3 | ShowBlackscreen.color | DragValue 字段 | |
| 5.8.4 | 参数名旁类型标签 | [str]/[num]/[xyz]/[list] 等 | |

### 5.9 Goto / CallFunction

| # | 测试项 | 预期 | 结果 |
|---|--------|------|------|
| 5.9.1 | Goto.label 参数 | 文本输入标签名 | |
| 5.9.2 | Goto.args 参数（可选） | Object 类型 | |
| 5.9.3 | CallFunction.function 参数 | 文本输入函数名 | |
| 5.9.4 | CallFunction.params 参数 | List 类型 | |

---

## 6. 底部面板

| # | 测试项 | 预期 | 结果 |
|---|--------|------|------|
| 6.1 | 底栏整体高度拖拽 | 拖拽调整高度，不弹回 | |
| 6.2 | 代码┃JSON┃DataFlow 三列 | 三列并排 | |
| 6.3 | 分隔线拖拽 | 鼠标变双箭头，拖拽可调整列宽 | |
| 6.4 | 分隔线可见 | 灰色竖线，悬停变蓝 | |
| 6.5 | .code 预览滚动 | 长内容可滚动，不撑高底栏 | |
| 6.6 | JSON 预览滚动 | 长 JSON 可滚动，不撑高底栏 | |
| 6.7 | DataFlow 方块 | 巧克力板排列，点击选中节点 | |
| 6.8 | 状态栏错误计数 | 显示错误数 | |
| 6.9 | 点击错误计数 | 弹出错误详情窗口 | |

---

## 7. 代码生成验证

| # | 测试项 | 预期 | 结果 |
|---|--------|------|------|
| 7.1 | Start → Log → Return 链路 | `if true` →...Log→ _result = null | |
| 7.2 | Start → Goto("step1") | 顶层 `var_main/var_step1_thread = CreateThread` + `thread.Goto("step1")` | |
| 7.3 | GetStateBool→If | `if _state.Futanari` | |
| 7.4 | CompareNumbers(a=GetStateNumber(Ecstasy), b=90)→If | `if _state.Ecstasy >= 90` | |
| 7.5 | LogicAnd(Boolean(true), GetStateBool)→If | `if (true) && (_state.Futanari)` | |
| 7.6 | Ctrl+S 保存 → .code 文件生成 | `main.code` 内容与预览一致 | |
| 7.7 | 从节点图生成按钮 | 同步实时 graph → 刷新 .code | |
| 7.8 | _result = null 收尾 | 每个标签末尾自动追加 | |
| 7.9 | 字符串参数不带 JSON 引号 | `_state.Ecstasy` 而非 `"_state.Ecstasy"` | |
| 7.10 | 操作符不带 JSON 引号 | `>=` 而非 `">="` | |
