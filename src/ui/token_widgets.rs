//! 统一 UI 控件封装。
//!
//! 将 `Button` 与 `ComboBox` 的常用样式收敛到设计令牌，避免业务代码散落
//! 硬编码宽度/高度/圆角。封装不隐藏 egui 原生能力，只提供默认规范值。

use egui::{Button, ComboBox, CornerRadius, Response, Ui, WidgetText};

use crate::ui::theme::tokens;

/// 规范定义的下拉框宽度档位。
///
/// 见 `docs/ui_design_spec.md` 第 4.2 节：除语言选择器 96 为特殊例外，
/// 所有下拉框必须使用三档之一，禁止自由写 120/180 等中间值。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropdownWidth {
    /// 紧凑档位：数据源类型切换等短选项（100px）。
    Small,
    /// 标准档位：枚举参数、条件模板（160px）。
    Medium,
    /// 宽松档位：方法名、NPC 列表等长文本（200px）。
    Large,
    /// 语言选择器固定宽度（96px），特殊例外。
    Language,
}

impl DropdownWidth {
    /// 返回该档位对应的逻辑像素宽度。
    #[must_use]
    pub fn value(self) -> f32 {
        match self {
            Self::Small => tokens::DROPDOWN_SM,
            Self::Medium => tokens::DROPDOWN_MD,
            Self::Large => tokens::DROPDOWN_LG,
            Self::Language => tokens::LANGUAGE_COMBO_WIDTH,
        }
    }
}

/// 标准按钮：最小高度 28、圆角 4，与工具栏按钮规格一致。
pub fn button(ui: &mut Ui, text: impl Into<WidgetText>) -> Response {
    ui.add(
        Button::new(text)
            .min_size(egui::vec2(0.0, tokens::TOOLBAR_BUTTON_HEIGHT))
            .corner_radius(CornerRadius::same(tokens::RADIUS_SMALL as u8)),
    )
}

/// 工具栏按钮：与 `button` 当前等价，保留语义以便未来区分工具栏专用样式。
pub fn toolbar_button(ui: &mut Ui, text: impl Into<WidgetText>) -> Response {
    button(ui, text)
}

/// 规范化的 `ComboBox` 封装。
///
/// 统一宽度档位、圆角与展开方向；业务代码只传入 id_salt、当前显示文本与内容。
/// 使用方式与原生 `ComboBox` 的 `show_ui` 相同：
///
/// ```ignore
/// token_combo_box(ui, "my_id", DropdownWidth::Medium, "请选择").show_ui(ui, |ui| { ... });
/// ```
pub fn token_combo_box(
    ui: &mut Ui,
    id_salt: impl std::hash::Hash,
    width: DropdownWidth,
    selected_text: impl Into<WidgetText>,
) -> TokenComboBox {
    let _ = ui;
    TokenComboBox {
        id_salt: egui::Id::new(id_salt),
        width,
        selected_text: selected_text.into(),
    }
}

/// `token_combo_box` 返回的 builder，用于继续调用 `show_ui`。
pub struct TokenComboBox {
    id_salt: egui::Id,
    width: DropdownWidth,
    selected_text: egui::WidgetText,
}

impl TokenComboBox {
    /// 渲染下拉框内容，语义与 `egui::ComboBox::show_ui` 一致。
    pub fn show_ui<R>(
        self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> egui::InnerResponse<Option<R>> {
        ComboBox::from_id_salt(self.id_salt)
            .width(self.width.value())
            .selected_text(self.selected_text)
            .show_ui(ui, add_contents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dropdown_widths_match_spec() {
        assert_eq!(DropdownWidth::Small.value(), 100.0);
        assert_eq!(DropdownWidth::Medium.value(), 160.0);
        assert_eq!(DropdownWidth::Large.value(), 200.0);
        assert_eq!(DropdownWidth::Language.value(), 96.0);
    }
}
