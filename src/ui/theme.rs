// 颜色来源：docs/node_types.md 第 12 节
// 若修改本文档颜色，必须同步更新 src/ui/theme.rs

use egui::Color32;

use crate::graph::types::PortType;

/// UI 全局主题常量。
///
/// 所有颜色均为 `egui::Color32` 形式，便于在即时模式 UI 中直接使用。
pub struct Theme;

impl Theme {
    /// 画布背景色
    pub const BACKGROUND: Color32 = Color32::from_rgb(30, 30, 30);
    /// 网格线颜色
    pub const GRID: Color32 = Color32::from_rgb(50, 50, 50);
    /// 节点卡片背景色
    pub const NODE_BACKGROUND: Color32 = Color32::from_rgb(45, 45, 45);
    /// 节点默认边框色
    pub const NODE_BORDER: Color32 = Color32::from_rgb(80, 80, 80);
    /// 选中节点发光边框色（蓝色）
    pub const SELECTED_GLOW: Color32 = Color32::from_rgb(33, 150, 243);
    /// 错误节点边框色（红色）
    pub const ERROR: Color32 = Color32::from_rgb(244, 67, 54);
    /// 主要文本颜色
    pub const TEXT: Color32 = Color32::from_rgb(255, 255, 255);
    /// 次要/暗淡文本颜色
    pub const TEXT_DIM: Color32 = Color32::from_rgb(180, 180, 180);
    /// 框选 Window 模式（左→右）矩形颜色
    pub const BOX_SELECT_WINDOW: Color32 = Color32::from_rgb(33, 150, 243);
    /// 框选 Crossing 模式（右→左）矩形颜色
    pub const BOX_SELECT_CROSSING: Color32 = Color32::from_rgb(76, 175, 80);
    /// 临时连线默认颜色
    pub const WIRE_DEFAULT: Color32 = Color32::from_rgb(200, 200, 200);
    /// 临时连线非法颜色（红色）
    pub const WIRE_INVALID: Color32 = Color32::from_rgb(244, 67, 54);
    /// 临时连线已占用颜色（橙色）
    pub const WIRE_OCCUPIED: Color32 = Color32::from_rgb(255, 152, 0);
}

/// 根据节点分类返回标题栏颜色。
///
/// 输入的 `category` 对应 `NodeDefinition::category` 中使用的英文标识符。
/// 颜色映射与 `docs/node_types.md` 第 12 节的节点分类颜色编码表保持一致。
///
/// 未知分类默认返回特殊节点的灰色，避免 UI 因未覆盖分类而 panic。
#[must_use]
pub fn category_color(category: &str) -> Color32 {
    match category {
        // 控制流 - 紫色
        "Control" => Color32::from_rgb(156, 39, 176),
        // 通用函数 - 蓝色
        "General Functions" => Color32::from_rgb(33, 150, 243),
        // 游戏动作 - 绿色
        "Game Functions: Items"
        | "Game Functions: Player"
        | "Game Functions: Additional"
        | "Game Functions: Control"
        | "Graphics" => Color32::from_rgb(76, 175, 80),
        // 数值操作 - 橙色
        "Game Functions: Stats" => Color32::from_rgb(255, 152, 0),
        // 对象构造 - 青色
        "Objects" => Color32::from_rgb(0, 188, 212),
        // 数学 - 灰色
        "Math" | "Math: Vector" => Color32::from_rgb(96, 125, 139),
        // 字符串 - 粉色
        "String" => Color32::from_rgb(233, 30, 99),
        // 文件 - 棕色
        "File" => Color32::from_rgb(121, 85, 72),
        // 等待/事件 - 黄色（当前注册表中 Wait 归入 Control，此处为预留）
        "Wait" | "Wait/Event" => Color32::from_rgb(255, 235, 59),
        // 特殊 - 灰色
        "Special" | "Meta" | "Comment" | "Group" => Color32::from_rgb(117, 117, 117),
        // 对象方法（当前注册表未使用独立分类，预留）
        "Object Methods" => Color32::from_rgb(3, 169, 244),
        // 未知分类兜底
        _ => Color32::from_rgb(117, 117, 117),
    }
}

/// 将 `PortType` 映射为 UI 端口圆点颜色。
///
/// 内部调用 `PortType::color()` 后再转换为 `egui::Color32`，
/// 确保数据层与 UI 层颜色定义单一来源。
#[must_use]
pub fn port_color(port_type: &PortType) -> Color32 {
    let [r, g, b, a] = port_type.color();
    Color32::from_rgba_premultiplied(r, g, b, a)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::types::PortType;

    #[test]
    fn theme_colors_are_valid() {
        // 仅验证主题常量可以被创建且不 panic
        let _ = Theme::BACKGROUND;
        let _ = Theme::SELECTED_GLOW;
        let _ = Theme::ERROR;
    }

    #[test]
    fn category_color_matches_doc_table() {
        assert_eq!(category_color("Control"), Color32::from_rgb(156, 39, 176));
        assert_eq!(
            category_color("General Functions"),
            Color32::from_rgb(33, 150, 243)
        );
        assert_eq!(
            category_color("Game Functions: Stats"),
            Color32::from_rgb(255, 152, 0)
        );
        assert_eq!(category_color("Objects"), Color32::from_rgb(0, 188, 212));
        assert_eq!(category_color("Math"), Color32::from_rgb(96, 125, 139));
        assert_eq!(category_color("String"), Color32::from_rgb(233, 30, 99));
        assert_eq!(category_color("File"), Color32::from_rgb(121, 85, 72));
        assert_eq!(category_color("Special"), Color32::from_rgb(117, 117, 117));
    }

    #[test]
    fn unknown_category_defaults_to_gray() {
        assert_eq!(category_color("Unknown"), Color32::from_rgb(117, 117, 117));
    }

    #[test]
    fn port_color_matches_data_layer() {
        assert_eq!(
            port_color(&PortType::Flow),
            Color32::from_rgb(255, 255, 255)
        );
        assert_eq!(
            port_color(&PortType::Number),
            Color32::from_rgb(66, 165, 245)
        );
    }
}
