use serde::{Deserialize, Serialize};

/// 编辑器支持的所有节点类型
///
/// 每个变体对应 CustomMissions2 API 中定义的一个函数调用、控制结构
/// 或对象构造函数/方法
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum NodeType {
    // ── 控制流 ──
    /// 入口点；每个 graph 必须恰好有一个
    Start,
    /// 标签定义，Goto 的目标
    Label,
    /// 跳转到一个标签
    Goto,
    /// 条件分支（True / False）
    If,
    /// 条件成立时循环
    While,
    /// 遍历一个 list
    For,
    /// 跳出循环
    Break,
    /// 返回一个值，设置 `_result`
    Return,
    /// 等待若干秒
    Wait,
    /// 等待某个 SetEvent 触发
    WaitForEvent,

    // ── 通用函数 ──
    /// 输出日志到控制台
    Log,
    /// 读/写全局变量
    Global,
    /// 读/写局部变量
    Local,
    /// 获取值的类型名称
    GetType,
    /// 获取当前游戏语言
    GetLanguage,
    /// 导出所有变量
    DumpVariables,
    /// 导出单个变量
    DumpVariable,
    /// 动态调用函数
    CallFunction,
    /// 动态调用对象上的方法
    CallMethod,
    /// 创建一个颜色 list [r, g, b, a]
    Color,
    /// 生成一个数值范围
    Range,
    /// 设置跨项目事件
    SetEvent,
    /// 获取事件数据
    GetEvent,

    // ── 游戏功能：物品与装备 ──
    /// 在世界中掉落一个物品
    DropItem,
    /// 拾取一个物品
    CollectItem,
    /// 设置振动器强度
    SetVibrator,
    /// 设置活塞强度
    SetPiston,
    /// 给玩家锁上手铐
    LockHandcuffs,
    /// 解锁手铐
    UnlockHandcuffs,
    /// 装备 cosplay 物品
    EquipCosplay,
    /// 卸下 cosplay 物品
    UnequipCosplay,
    /// 卸下所有 cosplay 物品
    UnequipAllCosplay,
    /// 设置 cosplay 拥有状态
    OwnCosplay,
    /// 装备成人玩具
    EquipAdultToy,
    /// 卸下成人玩具
    UnequipAdultToy,

    // ── 游戏功能：玩家状态 ──
    /// 设置玩家的位置和旋转
    SetPlayerPosition,
    /// 切换到不同的 stage
    SetStage,
    /// 设置相机的 pitch、yaw 和锁定状态
    SetCamera,
    /// 设置玩家当前动作
    SetAction,
    /// 设置扶她状态
    SetFutanari,
    /// 启用或禁用技能
    SetSkill,
    /// 设置任意玩家数据
    SetPlayerData,
    /// 设置技能快捷键位
    SetSkillShortcut,
    /// 获取技能快捷键位
    GetSkillShortcut,
    /// 获取范围内的随机位置
    GetRandomPosition,

    // ── 游戏功能：数值统计（RP、Ecstasy、Stamina 等）──
    /// 增加当前获得的 RP
    AddCurrentEarnRP,
    /// 设置当前获得的 RP
    SetCurrentEarnRP,
    /// 获取当前获得的 RP
    GetCurrentEarnRP,
    /// 增加持有的 RP
    AddCurrentRP,
    /// 设置持有的 RP
    SetCurrentRP,
    /// 获取持有的 RP
    GetCurrentRP,
    /// 设置 ecstasy（快感）值
    SetEcstasy,
    /// 增加 ecstasy 值
    AddEcstasy,
    /// 获取 ecstasy 值
    GetEcstasy,
    /// 设置 stamina 值
    SetStamina,
    /// 增加 stamina 值
    AddStamina,
    /// 获取 stamina 值
    GetStamina,
    /// 设置 moisture（膀胱）值
    SetMoisture,
    /// 增加 moisture 值
    AddMoisture,
    /// 获取 moisture 值
    GetMoisture,
    /// 设置物品数量
    SetItemCount,
    /// 增加物品数量
    AddItemCount,
    /// 获取物品数量
    GetItemCount,

    // ── 游戏功能：游戏控制 ──
    /// 设置/获取是否允许 game over
    CanGameOver,
    /// 强制触发 game over
    TriggerGameOver,
    /// 播放音效
    PlaySoundEffect,
    /// 设置 stage 评价上限
    SetStageRankLimit,
    /// 获取 stage 评价上限
    GetStageRankLimit,
    /// 启用或禁用传送门
    SetPortalEnabled,
    /// 获取 stage 中的所有 waypoint
    GetAllWaypoints,
    /// 设置性爱姿势
    SetSexPosition,
    /// 取消性爱状态
    DeactivateSex,
    /// 配置性爱菜单
    SetSexMenu,

    // ── 额外游戏功能 ──
    /// 显示全屏黑屏/颜色覆盖层
    ShowBlackscreen,
    /// 获取 snapshot 元数据
    GetSnapshotData,
    /// 获取所有 snapshot 引用
    GetAllSnapshots,
    /// 标记一个 snapshot 为已删除
    DeleteSnapshot,
    /// 从文件路径获取图片引用
    GetImageReference,

    // ── 图形 ──
    /// 设置图形选项
    SetGraphicsOption,
    /// 获取图形选项值
    GetGraphicsOption,

    // ── 数学：标准 ──
    /// 随机浮点数
    Random,
    /// 随机整数
    RandomInt,
    /// 正弦值
    Sin,
    /// 余弦值
    Cos,
    /// 正切值
    Tan,
    /// 反正弦
    Asin,
    /// 反余弦
    Acos,
    /// 反正切
    Atan,
    /// 向下取整
    Floor,
    /// 向上取整
    Ceil,
    /// 四舍五入到最接近的整数
    Round,
    /// 截断小数部分
    Trunc,
    /// 数值的符号
    Sign,
    /// 绝对值
    Abs,
    /// 自然对数
    LogN,
    /// 以 2 为底的对数
    Log2,
    /// 以 10 为底的对数
    Log10,
    /// 一组数字中的最小值
    Min,
    /// 一组数字中的最大值
    Max,

    // ── 数学：向量 ──
    /// 创建一个 3D 向量
    Vector,
    /// 创建一个四元数
    Quaternion,
    /// 向量的长度
    Vector3Length,
    /// 向量的平方长度
    Vector3SqrLength,
    /// 两个向量相加
    Vector3Add,
    /// 两个向量相减
    Vector3Sub,
    /// 缩放向量
    Vector3Scale,
    /// 两个向量的点积
    Vector3Dot,
    /// 两个向量的叉积
    Vector3Cross,
    /// 用四元数旋转向量
    Vector3Rotate,
    /// 两个向量之间的距离
    Vector3Distance,

    // ── 字符串函数 ──
    /// 字符串长度
    Length,
    /// 转换为小写
    Lower,
    /// 转换为大写
    Upper,
    /// 查找子串索引
    Find,
    /// 提取子串
    SubString,
    /// 用参数格式化字符串
    Format,
    /// 将字符串转换为数字
    ToNumber,

    // ── 文件函数 ──
    /// 检查文件是否存在
    FileExists,
    /// 列出目录中的文件
    GetFiles,
    /// 获取文件扩展名
    GetFileExtension,

    // ── 对象构造函数 / 方法 ──
    /// 创建一个空的 list
    CreateList,
    /// 复制一个 list（浅拷贝或深拷贝）
    Copy,
    /// 从 JSON 文件创建 list
    CreateListFromJson,

    /// 创建一个线程
    CreateThread,
    /// 创建一个 listener（父作用域）
    CreateListener,
    /// 创建一个 listener（局部作用域）
    CreateListenerLocal,
    /// 创建 mission panel
    CreateMissionPanel,
    /// 创建 mission menu item
    CreateMissionMenuItem,
    /// 创建一个区域
    CreateArea,
    /// 创建一个 zone（多个 area 的组合）
    CreateZone,
    /// 创建一个 condition 对象
    CreateCondition,
    /// 创建一个 item condition
    CreateItemCondition,
    /// 创建一个交互区域
    CreateInteractArea,
    /// 创建一个文本对象
    CreateText,
    /// 创建一个 messenger 聊天
    CreateMessengerChat,
    /// 创建音频源
    CreateAudio,
    /// 创建一个 gallery
    CreateGallery,
    /// 创建 snapshot 相机
    CreateSnapshot,
    /// 创建或连接一个 NPC
    CreateNPC,
    /// 创建一个输入检测器
    CreateInput,

    // ── 特殊 ──
    /// Mission 元数据（不序列化到 .code）
    Meta,
    /// Comment 节点（不序列化）
    Comment,
    /// 可视化分组框
    Group,

    // ── Phase 6: Data-only 布尔/条件评估节点（Monitor → Condition 管道）──
    /// 输出一个布尔常量（true/false）
    Boolean,
    /// 读取 _state 中任意布尔变量
    GetStateBool,
    /// 读取 _state 中任意数值变量
    GetStateNumber,
    /// 比较两个数值（>=, ==, !=, <, <=）
    CompareNumbers,
    /// 逻辑与（&&）
    LogicAnd,
    /// 逻辑或（||）
    LogicOr,
    /// 逻辑非（!）
    LogicNot,
    // Phase 7: 坐标系统
    GetPosition,
    MakeVector,
    BreakVector,
}

/// 端口数据类型，用于节点上的 flow 端口和 data 端口
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum PortType {
    /// 执行流（白色）控制节点的执行顺序
    Flow,
    /// 数值类型（蓝色）整数或浮点数
    Number,
    /// 字符串类型（粉色）文本数据
    String,
    /// 布尔类型（红色）true 或 false
    Boolean,
    /// List 类型（黄色）键值集合（key 为字符串）
    List,
    /// 对象引用（绿色）游戏对象如 Thread、Area、NPC
    Object,
    /// 任意类型（灰色）动态类型，可与任何类型匹配
    Any,
}

impl PortType {
    /// 检查从 `self` 到 `other` 的连接是否类型兼容
    ///
    /// 相同类型总是兼容的`Any` 与所有类型兼容
    /// 其他跨类型连接不允许（需要显式的转换节点）
    pub fn is_compatible_with(&self, other: &PortType) -> bool {
        match (self, other) {
            (a, b) if a == b => true,
            (PortType::Any, _) | (_, PortType::Any) => true,
            _ => false,
        }
    }

    /// 与此端口类型关联的 RGBA 颜色，用于 UI 显示
    pub fn color(&self) -> [u8; 4] {
        match self {
            PortType::Flow => [255, 255, 255, 255],
            PortType::Number => [66, 165, 245, 255],
            PortType::String => [244, 143, 177, 255],
            PortType::Boolean => [239, 83, 80, 255],
            PortType::List => [255, 202, 40, 255],
            PortType::Object => [102, 187, 106, 255],
            PortType::Any => [189, 189, 189, 255],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_type_roundtrip_and_compatibility() {
        let variants = [
            PortType::Flow,
            PortType::Number,
            PortType::String,
            PortType::Boolean,
            PortType::List,
            PortType::Object,
            PortType::Any,
        ];
        for v in &variants {
            let json = serde_json::to_string(v).expect("serialize failed");
            let back: PortType = serde_json::from_str(&json).expect("deserialize failed");
            assert_eq!(*v, back);
            assert!(v.is_compatible_with(v));
            assert!(PortType::Any.is_compatible_with(v));
            assert!(v.is_compatible_with(&PortType::Any));
        }
    }

    #[test]
    fn test_port_type_incompatible() {
        assert!(!PortType::Number.is_compatible_with(&PortType::String));
        assert!(!PortType::String.is_compatible_with(&PortType::Boolean));
    }

    #[test]
    fn test_node_type_count() {
        // 当前 NodeType 应包含 143 种节点变体（含控制流、通用函数、游戏函数、
        // 数学/字符串/文件函数、对象构造函数及 Meta/Comment/Group 特殊节点）。
        // 对象方法（如 Area.Inside、NPC.Warp）不单独映射为枚举变体，
        // 运行时通过 (Object, MethodName) 组合或 CallMethod 表示。
        let variants: Vec<_> = [
            NodeType::Start,
            NodeType::Label,
            NodeType::Goto,
            NodeType::If,
            NodeType::While,
            NodeType::For,
            NodeType::Break,
            NodeType::Return,
            NodeType::Wait,
            NodeType::WaitForEvent,
            NodeType::Log,
            NodeType::Global,
            NodeType::Local,
            NodeType::GetType,
            NodeType::GetLanguage,
            NodeType::DumpVariables,
            NodeType::DumpVariable,
            NodeType::CallFunction,
            NodeType::CallMethod,
            NodeType::Color,
            NodeType::Range,
            NodeType::SetEvent,
            NodeType::GetEvent,
            NodeType::DropItem,
            NodeType::CollectItem,
            NodeType::SetVibrator,
            NodeType::SetPiston,
            NodeType::LockHandcuffs,
            NodeType::UnlockHandcuffs,
            NodeType::EquipCosplay,
            NodeType::UnequipCosplay,
            NodeType::UnequipAllCosplay,
            NodeType::OwnCosplay,
            NodeType::EquipAdultToy,
            NodeType::UnequipAdultToy,
            NodeType::SetPlayerPosition,
            NodeType::SetStage,
            NodeType::SetCamera,
            NodeType::SetAction,
            NodeType::SetFutanari,
            NodeType::SetSkill,
            NodeType::SetPlayerData,
            NodeType::SetSkillShortcut,
            NodeType::GetSkillShortcut,
            NodeType::GetRandomPosition,
            NodeType::AddCurrentEarnRP,
            NodeType::SetCurrentEarnRP,
            NodeType::GetCurrentEarnRP,
            NodeType::AddCurrentRP,
            NodeType::SetCurrentRP,
            NodeType::GetCurrentRP,
            NodeType::SetEcstasy,
            NodeType::AddEcstasy,
            NodeType::GetEcstasy,
            NodeType::SetStamina,
            NodeType::AddStamina,
            NodeType::GetStamina,
            NodeType::SetMoisture,
            NodeType::AddMoisture,
            NodeType::GetMoisture,
            NodeType::SetItemCount,
            NodeType::AddItemCount,
            NodeType::GetItemCount,
            NodeType::CanGameOver,
            NodeType::TriggerGameOver,
            NodeType::PlaySoundEffect,
            NodeType::SetStageRankLimit,
            NodeType::GetStageRankLimit,
            NodeType::SetPortalEnabled,
            NodeType::GetAllWaypoints,
            NodeType::SetSexPosition,
            NodeType::DeactivateSex,
            NodeType::SetSexMenu,
            NodeType::ShowBlackscreen,
            NodeType::GetSnapshotData,
            NodeType::GetAllSnapshots,
            NodeType::DeleteSnapshot,
            NodeType::GetImageReference,
            NodeType::SetGraphicsOption,
            NodeType::GetGraphicsOption,
            NodeType::Random,
            NodeType::RandomInt,
            NodeType::Sin,
            NodeType::Cos,
            NodeType::Tan,
            NodeType::Asin,
            NodeType::Acos,
            NodeType::Atan,
            NodeType::Floor,
            NodeType::Ceil,
            NodeType::Round,
            NodeType::Trunc,
            NodeType::Sign,
            NodeType::Abs,
            NodeType::LogN,
            NodeType::Log2,
            NodeType::Log10,
            NodeType::Min,
            NodeType::Max,
            NodeType::Vector,
            NodeType::Quaternion,
            NodeType::Vector3Length,
            NodeType::Vector3SqrLength,
            NodeType::Vector3Add,
            NodeType::Vector3Sub,
            NodeType::Vector3Scale,
            NodeType::Vector3Dot,
            NodeType::Vector3Cross,
            NodeType::Vector3Rotate,
            NodeType::Vector3Distance,
            NodeType::Length,
            NodeType::Lower,
            NodeType::Upper,
            NodeType::Find,
            NodeType::SubString,
            NodeType::Format,
            NodeType::ToNumber,
            NodeType::FileExists,
            NodeType::GetFiles,
            NodeType::GetFileExtension,
            NodeType::CreateList,
            NodeType::Copy,
            NodeType::CreateListFromJson,
            NodeType::CreateThread,
            NodeType::CreateListener,
            NodeType::CreateListenerLocal,
            NodeType::CreateMissionPanel,
            NodeType::CreateMissionMenuItem,
            NodeType::CreateArea,
            NodeType::CreateZone,
            NodeType::CreateCondition,
            NodeType::CreateItemCondition,
            NodeType::CreateInteractArea,
            NodeType::CreateText,
            NodeType::CreateMessengerChat,
            NodeType::CreateAudio,
            NodeType::CreateGallery,
            NodeType::CreateSnapshot,
            NodeType::CreateNPC,
            NodeType::CreateInput,
            NodeType::Meta,
            NodeType::Comment,
            NodeType::Group,
            // Phase 5: Data-only Boolean evaluation nodes (Monitor→Condition pipeline)
            NodeType::Boolean,
            NodeType::GetStateBool,
            NodeType::GetStateNumber,
            NodeType::CompareNumbers,
            NodeType::LogicAnd,
            NodeType::LogicOr,
            NodeType::LogicNot,
            // Phase 7: 坐标系统
            NodeType::GetPosition,
            NodeType::MakeVector,
            NodeType::BreakVector,
        ]
        .to_vec();
        assert_eq!(variants.len(), 153);
    }
}
