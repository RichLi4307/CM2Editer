// 颜色与设计令牌来源：docs/ui_design_spec.md 第 2 节
// 若修改本文档，必须同步更新 docs/ui_design_spec.md

use egui::Color32;

use crate::graph::types::PortType;

/// 设计令牌。
///
/// 所有视觉常量（间距、圆角、字体、颜色）均由此模块提供，
/// 禁止在 UI 业务代码中硬编码 egui 颜色字面量。
pub mod tokens {
    use egui::Color32;

    // Spacing
    pub const SPACE_1: f32 = 4.0;
    pub const SPACE_2: f32 = 8.0;
    pub const SPACE_3: f32 = 12.0;
    pub const SPACE_4: f32 = 16.0;
    pub const SPACE_5: f32 = 24.0;
    pub const SPACE_6: f32 = 32.0;

    // Radii
    pub const RADIUS_SMALL: f32 = 4.0;
    pub const RADIUS_MEDIUM: f32 = 6.0;
    pub const RADIUS_LARGE: f32 = 8.0;

    // Typography
    pub const TEXT_DISPLAY: f32 = 22.0;
    pub const TEXT_HEADING: f32 = 16.0;
    pub const TEXT_TITLE: f32 = 14.0;
    pub const TEXT_BODY: f32 = 13.0;
    pub const TEXT_CAPTION: f32 = 12.0;
    pub const TEXT_MICRO: f32 = 11.0;

    // Colors
    pub const BG_APP: Color32 = Color32::from_rgb(30, 30, 30);
    pub const BG_PANEL: Color32 = Color32::from_rgb(37, 37, 38);
    pub const BG_CARD: Color32 = Color32::from_rgb(45, 45, 45);
    pub const BG_ELEVATED: Color32 = Color32::from_rgb(56, 56, 56);
    pub const BG_OVERLAY: Color32 = Color32::from_rgba_premultiplied(18, 18, 18, 153);
    pub const BORDER_SUBTLE: Color32 = Color32::from_rgb(60, 60, 60);
    pub const BORDER_DEFAULT: Color32 = Color32::from_rgb(80, 80, 80);
    pub const BORDER_STRONG: Color32 = Color32::from_rgb(107, 107, 107);
    pub const BORDER_FOCUS: Color32 = Color32::from_rgb(33, 150, 243);
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(255, 255, 255);
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(180, 180, 180);
    pub const TEXT_DISABLED: Color32 = Color32::from_rgb(107, 107, 107);
    pub const TEXT_LINK: Color32 = Color32::from_rgb(100, 181, 246);
    pub const ACCENT: Color32 = Color32::from_rgb(33, 150, 243);
    pub const SUCCESS: Color32 = Color32::from_rgb(76, 175, 80);
    pub const WARNING: Color32 = Color32::from_rgb(255, 152, 0);
    pub const ERROR: Color32 = Color32::from_rgb(244, 67, 54);
    pub const ENTRY_AMBER: Color32 = Color32::from_rgb(255, 193, 7);
    pub const GRID: Color32 = Color32::from_rgb(50, 50, 50);
    pub const WIRE_DEFAULT: Color32 = Color32::from_rgb(200, 200, 200);
    pub const WIRE_INVALID: Color32 = ERROR;
    pub const WIRE_OCCUPIED: Color32 = WARNING;

    /// 将颜色按给定 alpha 通道做 premultiplied 混合。
    pub fn with_alpha(color: Color32, alpha: u8) -> Color32 {
        let a = alpha as u16;
        Color32::from_rgba_premultiplied(
            (color.r() as u16 * a / 255) as u8,
            (color.g() as u16 * a / 255) as u8,
            (color.b() as u16 * a / 255) as u8,
            alpha,
        )
    }
}

/// 8 色场景舞台调色板，用于坐标预设等需要按场景名稳定着色的场景。
#[must_use]
pub fn stage_palette_color(index: usize) -> Color32 {
    const PALETTE: [Color32; 8] = [
        Color32::from_rgb(33, 150, 243),
        Color32::from_rgb(76, 175, 80),
        Color32::from_rgb(255, 152, 0),
        Color32::from_rgb(156, 39, 176),
        Color32::from_rgb(0, 188, 212),
        Color32::from_rgb(233, 30, 99),
        Color32::from_rgb(255, 235, 59),
        Color32::from_rgb(121, 85, 72),
    ];
    PALETTE[index % PALETTE.len()]
}

/// 根据节点分类返回标题栏颜色。
///
/// 输入的 `category` 对应 `NodeDefinition::category` 中使用的英文标识符。
/// 颜色映射与 `docs/node_types.md` 2.1-2.10 中定义的节点分类颜色编码表保持一致。
///
/// 未知分类默认返回特殊节点的灰色，避免 UI 因未覆盖分类而 panic。
#[must_use]
pub fn category_color(category: &str) -> Color32 {
    match category {
        // 控制流 - 紫色
        "Control Flow" => Color32::from_rgb(156, 39, 176),
        // 变量与全局 - 蓝色
        "Variables & Globals" => Color32::from_rgb(33, 150, 243),
        // 游戏 API - 绿色
        "Game API" => Color32::from_rgb(76, 175, 80),
        // 数值统计 - 橙色
        "Game API: Stats" => Color32::from_rgb(255, 152, 0),
        // 线程与并发 - 青色
        "Threading & Concurrency" => Color32::from_rgb(0, 188, 212),
        // 对象构造 - 青色
        "Objects" => Color32::from_rgb(0, 188, 212),
        // 数学与逻辑 - 灰色
        "Math & Logic" => Color32::from_rgb(96, 125, 139),
        // 字面量 - 蓝绿色
        "Literals" => Color32::from_rgb(0, 150, 136),
        // 条件与查询 - 紫粉色
        "Conditions & Queries" => Color32::from_rgb(171, 71, 188),
        // 字符串 / 文件 / 列表 - 粉色
        "String / File / List" => Color32::from_rgb(233, 30, 99),
        // 编辑器专用 - 深灰色
        "Editor-only" => Color32::from_rgb(117, 117, 117),
        // 兼容旧分类（UI 恢复过渡期）
        "Control" => Color32::from_rgb(156, 39, 176),
        "General Functions" => Color32::from_rgb(33, 150, 243),
        "Game Functions: Items"
        | "Game Functions: Player"
        | "Game Functions: Additional"
        | "Game Functions: Control"
        | "Graphics" => Color32::from_rgb(76, 175, 80),
        "Game Functions: Stats" => Color32::from_rgb(255, 152, 0),
        "Math" | "Math: Vector" => Color32::from_rgb(96, 125, 139),
        "String" => Color32::from_rgb(233, 30, 99),
        "File" => Color32::from_rgb(121, 85, 72),
        "Wait" | "Wait/Event" => Color32::from_rgb(255, 235, 59),
        "Special" | "Meta" | "Comment" | "Group" => Color32::from_rgb(117, 117, 117),
        // 对象方法（当前注册表未使用独立分类，预留）
        "Object Methods" => Color32::from_rgb(3, 169, 244),
        // 未知分类兜底
        _ => Color32::from_rgb(117, 117, 117),
    }
}

/// 根据场景分类返回节点库标题栏颜色。
///
/// 颜色与 `docs/TODO.md` 中 Backlog 的场景分类设计对应，
/// 用于在场景分类节点库中区分一级分类。
#[must_use]
pub fn scene_category_color(category: &str) -> Color32 {
    match category {
        "scene.mission_flow" => Color32::from_rgb(156, 39, 176),   // purple
        "scene.conditions" => Color32::from_rgb(171, 71, 188),      // purple-pink
        "scene.data_get" => Color32::from_rgb(33, 150, 243),       // blue
        "scene.data_set" => Color32::from_rgb(255, 152, 0),        // orange
        "scene.data_process" => Color32::from_rgb(96, 125, 139),   // grey
        "scene.visual_ui" => Color32::from_rgb(0, 188, 212),       // cyan
        "scene.editor" => Color32::from_rgb(117, 117, 117),        // grey
        // Unknown scene defaults to grey.
        _ => Color32::from_rgb(117, 117, 117),
    }
}

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

    #[test]
    fn tokens_values_match_spec() {
        assert_eq!(tokens::SPACE_1, 4.0);
        assert_eq!(tokens::SPACE_2, 8.0);
        assert_eq!(tokens::SPACE_3, 12.0);
        assert_eq!(tokens::SPACE_4, 16.0);
        assert_eq!(tokens::SPACE_5, 24.0);
        assert_eq!(tokens::SPACE_6, 32.0);

        assert_eq!(tokens::RADIUS_SMALL, 4.0);
        assert_eq!(tokens::RADIUS_MEDIUM, 6.0);
        assert_eq!(tokens::RADIUS_LARGE, 8.0);

        assert_eq!(tokens::TEXT_DISPLAY, 22.0);
        assert_eq!(tokens::TEXT_HEADING, 16.0);
        assert_eq!(tokens::TEXT_TITLE, 14.0);
        assert_eq!(tokens::TEXT_BODY, 13.0);
        assert_eq!(tokens::TEXT_CAPTION, 12.0);
        assert_eq!(tokens::TEXT_MICRO, 11.0);

        assert_eq!(tokens::BG_APP, Color32::from_rgb(30, 30, 30));
        assert_eq!(tokens::BG_PANEL, Color32::from_rgb(37, 37, 38));
        assert_eq!(tokens::BG_CARD, Color32::from_rgb(45, 45, 45));
        assert_eq!(tokens::BG_ELEVATED, Color32::from_rgb(56, 56, 56));
        assert_eq!(tokens::BG_OVERLAY, Color32::from_rgba_premultiplied(18, 18, 18, 153));
        assert_eq!(tokens::BORDER_SUBTLE, Color32::from_rgb(60, 60, 60));
        assert_eq!(tokens::BORDER_DEFAULT, Color32::from_rgb(80, 80, 80));
        assert_eq!(tokens::BORDER_STRONG, Color32::from_rgb(107, 107, 107));
        assert_eq!(tokens::BORDER_FOCUS, Color32::from_rgb(33, 150, 243));
        assert_eq!(tokens::TEXT_PRIMARY, Color32::from_rgb(255, 255, 255));
        assert_eq!(tokens::TEXT_SECONDARY, Color32::from_rgb(180, 180, 180));
        assert_eq!(tokens::TEXT_DISABLED, Color32::from_rgb(107, 107, 107));
        assert_eq!(tokens::TEXT_LINK, Color32::from_rgb(100, 181, 246));
        assert_eq!(tokens::ACCENT, Color32::from_rgb(33, 150, 243));
        assert_eq!(tokens::SUCCESS, Color32::from_rgb(76, 175, 80));
        assert_eq!(tokens::WARNING, Color32::from_rgb(255, 152, 0));
        assert_eq!(tokens::ERROR, Color32::from_rgb(244, 67, 54));
        assert_eq!(tokens::ENTRY_AMBER, Color32::from_rgb(255, 193, 7));
        assert_eq!(tokens::GRID, Color32::from_rgb(50, 50, 50));
        assert_eq!(tokens::WIRE_DEFAULT, Color32::from_rgb(200, 200, 200));
        assert_eq!(tokens::WIRE_INVALID, tokens::ERROR);
        assert_eq!(tokens::WIRE_OCCUPIED, tokens::WARNING);
    }

    #[test]
    fn theme_colors_are_valid() {
        // 仅验证主题常量可以被创建且不 panic
        let _ = tokens::BG_APP;
        let _ = tokens::ACCENT;
        let _ = tokens::ERROR;
    }

    #[test]
    fn category_color_matches_doc_table() {
        assert_eq!(
            category_color("Control Flow"),
            Color32::from_rgb(156, 39, 176)
        );
        assert_eq!(
            category_color("Variables & Globals"),
            Color32::from_rgb(33, 150, 243)
        );
        assert_eq!(
            category_color("Game API: Stats"),
            Color32::from_rgb(255, 152, 0)
        );
        assert_eq!(
            category_color("Threading & Concurrency"),
            Color32::from_rgb(0, 188, 212)
        );
        assert_eq!(category_color("Objects"), Color32::from_rgb(0, 188, 212));
        assert_eq!(
            category_color("Math & Logic"),
            Color32::from_rgb(96, 125, 139)
        );
        assert_eq!(
            category_color("Literals"),
            Color32::from_rgb(0, 150, 136)
        );
        assert_eq!(
            category_color("Conditions & Queries"),
            Color32::from_rgb(171, 71, 188)
        );
        assert_eq!(
            category_color("String / File / List"),
            Color32::from_rgb(233, 30, 99)
        );
        assert_eq!(
            category_color("Editor-only"),
            Color32::from_rgb(117, 117, 117)
        );
    }

    #[test]
    fn unknown_category_defaults_to_gray() {
        assert_eq!(category_color("Unknown"), Color32::from_rgb(117, 117, 117));
    }

    #[test]
    fn scene_category_color_matches_expected() {
        assert_eq!(
            scene_category_color("scene.mission_flow"),
            Color32::from_rgb(156, 39, 176)
        );
        assert_eq!(
            scene_category_color("scene.conditions"),
            Color32::from_rgb(171, 71, 188)
        );
        assert_eq!(
            scene_category_color("scene.data_get"),
            Color32::from_rgb(33, 150, 243)
        );
        assert_eq!(
            scene_category_color("scene.data_set"),
            Color32::from_rgb(255, 152, 0)
        );
        assert_eq!(
            scene_category_color("scene.data_process"),
            Color32::from_rgb(96, 125, 139)
        );
        assert_eq!(
            scene_category_color("scene.visual_ui"),
            Color32::from_rgb(0, 188, 212)
        );
        assert_eq!(
            scene_category_color("scene.editor"),
            Color32::from_rgb(117, 117, 117)
        );
    }

    #[test]
    fn unknown_scene_category_defaults_to_gray() {
        assert_eq!(
            scene_category_color("scene.unknown"),
            Color32::from_rgb(117, 117, 117)
        );
    }

    #[test]
    fn stage_palette_color_cycles() {
        assert_eq!(stage_palette_color(0), Color32::from_rgb(33, 150, 243));
        assert_eq!(stage_palette_color(8), Color32::from_rgb(33, 150, 243));
    }

    #[test]
    fn with_alpha_matches_hand_computed() {
        assert_eq!(
            tokens::with_alpha(tokens::BG_APP, 128),
            Color32::from_rgba_premultiplied(15, 15, 15, 128)
        );
    }

    #[test]
    fn no_bare_color32_outside_theme() {
        let manifest = env!("CARGO_MANIFEST_DIR");
        let ui_dir = std::path::Path::new(manifest).join("src").join("ui");
        let mut offenders = Vec::new();
        for entry in walk_dir(&ui_dir) {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if path.extension().and_then(|s| s.to_str()) != Some("rs") {
                continue;
            }
            if path.file_name().and_then(|s| s.to_str()) == Some("theme.rs") {
                continue;
            }
            let content = std::fs::read_to_string(&path).unwrap();
            for (line_no, line) in content.lines().enumerate() {
                for pat in &[
                    "Color32::from_rgb",
                    "Color32::from_rgba",
                    "Color32::from_gray",
                    "Color32::from_rgba_premultiplied",
                ] {
                    if line.contains(pat) {
                        offenders.push(format!(
                            "{}:{}: {}",
                            path.display(),
                            line_no + 1,
                            line.trim()
                        ));
                    }
                }
            }
        }
        if !offenders.is_empty() {
            panic!("发现 UI 业务代码中硬编码 Color32：\n{}", offenders.join("\n"));
        }
    }

    fn walk_dir(dir: &std::path::Path) -> Vec<std::fs::DirEntry> {
        let mut result = Vec::new();
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    result.extend(walk_dir(&path));
                } else {
                    result.push(entry);
                }
            }
        }
        result
    }
}
