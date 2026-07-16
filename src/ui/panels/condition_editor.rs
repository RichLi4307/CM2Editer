//! Floating condition editor for the `CreateCondition` node's `condition` parameter.
//!
//! The game accepts complex condition expressions using `AND` `[A, B]`, `OR` `(A, B)`,
//! and `NOT` `!A`. Existing condition IDs can be reused via `SubCondition_<id>`.
//!
//! The editor provides a visual helper: an editable expression box plus buttons
//! for inserting logical operators, a searchable list of base conditions, and a
//! list of condition IDs already defined in the current label so users can pick
//! reusable sub-conditions without memorizing syntax.

use crate::graph::container::LabelContainer;
use crate::graph::node::ParamValue;
use crate::graph::types::NodeType;
use crate::ui::i18n::I18n;

/// Persistent state for the condition editor window.
#[derive(Debug, Clone)]
pub struct ConditionEditorState {
    pub open: bool,
    /// Parameter key that will receive the final expression.
    pub param_key: String,
    /// Editable expression value.
    pub value: String,
    /// Search filter for the base condition list.
    pub search: String,
    /// A short flash message shown after inserting a token.
    pub flash: Option<String>,
    /// Frame counter for the flash message.
    pub flash_frames: u32,
}

impl ConditionEditorState {
    /// Create a new editor state for the given parameter and initial value.
    pub fn new(param_key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            open: true,
            param_key: param_key.into(),
            value: value.into(),
            search: String::new(),
            flash: None,
            flash_frames: 0,
        }
    }

    /// Set a one-time flash message that will be displayed for a few frames.
    fn set_flash(&mut self, message: impl Into<String>) {
        self.flash = Some(message.into());
        self.flash_frames = 6;
    }
}

/// Base condition categories used in the picker. Each entry is an i18n key and
/// the list of condition names that belong to that category.
const CONDITION_CATEGORIES: &[(&str, &[&str])] = &[
    (
        "condition.category.clothing",
        &[
            "CoatDropped",
            "CoatFrontClosed",
            "CoatFrontOpen1",
            "CoatFrontOpen2",
            "CoatBackClosed",
            "CoatBackOpen",
        ],
    ),
    ("condition.category.face", &["Blindfolded"]),
    (
        "condition.category.handcuffs",
        &[
            "NoHandcuffs",
            "HandcuffsBack",
            "HandcuffsObject",
            "NormalHandcuffs",
            "KeyedHandcuffs",
            "TimedHandcuffs",
        ],
    ),
    (
        "condition.category.body",
        &[
            "Moving",
            "Crouching",
            "Peeing",
            "InLight",
            "Sitting",
            "Dashing",
            "Orgasm",
            "Futanari",
            "Invisible",
            "InOpenToilet",
            "Bukkake",
            "NearNPC",
            "Watched",
            "ShowingOff",
            "Bodypaint",
            "FPCamera",
            "GameOver",
        ],
    ),
    (
        "condition.category.exposure",
        &[
            "Exposed_None",
            "Exposed_Front",
            "Exposed_Upper",
            "Exposed_HipCrouch",
            "Exposed_Hip",
            "Exposed_All",
        ],
    ),
    (
        "condition.category.vibrator",
        &[
            "VibrationOff",
            "VibrationLow",
            "VibrationHigh",
            "VibrationRandom",
        ],
    ),
    (
        "condition.category.piston",
        &[
            "PistonOff",
            "PistonLow",
            "PistonMedium",
            "PistonHigh",
            "PistonRandom",
        ],
    ),
    ("condition.category.environment", &["IsDayTime", "NPCArea"]),
];

/// Floating condition composition editor.
pub struct ConditionEditor;

impl ConditionEditor {
    /// Show the editor window. Returns `Some(String)` when the user confirms.
    pub fn show(
        ctx: &egui::Context,
        state: &mut ConditionEditorState,
        label: &LabelContainer,
        i18n: &I18n,
    ) -> Option<String> {
        if !state.open {
            return None;
        }

        let mut confirmed = false;
        let mut cancelled = false;

        egui::Window::new(i18n.text("condition_editor.title"))
            .id(egui::Id::new("condition_editor"))
            .collapsible(false)
            .resizable(true)
            .default_size([520.0, 560.0])
            .show(ctx, |ui| {
                ui.label(i18n.text("condition_editor.expression_label"));
                let text_edit = egui::TextEdit::multiline(&mut state.value)
                    .id_salt("condition_editor_expression")
                    .desired_rows(3)
                    .desired_width(f32::INFINITY)
                    .font(egui::TextStyle::Monospace);
                ui.add(text_edit);

                // Toolbar
                ui.horizontal(|ui| {
                    if ui
                        .button(i18n.text("condition_editor.and"))
                        .on_hover_text(i18n.text("condition_editor.and_hint"))
                        .clicked()
                    {
                        insert_at_cursor_or_end(&mut state.value, "[, ]", "condition_editor.and");
                    }
                    if ui
                        .button(i18n.text("condition_editor.or"))
                        .on_hover_text(i18n.text("condition_editor.or_hint"))
                        .clicked()
                    {
                        insert_at_cursor_or_end(&mut state.value, "(, )", "condition_editor.or");
                    }
                    if ui
                        .button(i18n.text("condition_editor.not"))
                        .on_hover_text(i18n.text("condition_editor.not_hint"))
                        .clicked()
                    {
                        insert_at_cursor_or_end(&mut state.value, "!", "condition_editor.not");
                    }
                    if ui.button(i18n.text("condition_editor.clear")).clicked() {
                        state.value.clear();
                    }
                });

                // Preview / flash message
                let preview = normalize_condition_expression(&state.value);
                ui.horizontal(|ui| {
                    ui.label(i18n.text("condition_editor.preview"));
                    if preview.is_empty() {
                        ui.label(egui::RichText::new(i18n.text("condition_editor.empty_preview")).color(egui::Color32::from_gray(120)));
                    } else {
                        ui.add(egui::Label::new(
                            egui::RichText::new(preview.clone()).monospace().color(egui::Color32::from_rgb(160, 220, 160)),
                        ));
                    }
                });
                if let Some(flash) = state.flash.as_ref() {
                    ui.label(
                        egui::RichText::new(flash)
                            .size(11.0)
                            .color(egui::Color32::from_rgb(255, 220, 120)),
                    );
                    if state.flash_frames == 0 {
                        state.flash = None;
                    } else {
                        state.flash_frames -= 1;
                    }
                }
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label(i18n.text("condition_editor.search"));
                    ui.text_edit_singleline(&mut state.search);
                });

                let query = state.search.to_lowercase();
                let existing_ids = collect_existing_condition_ids(label);

                egui::ScrollArea::vertical()
                    .id_salt("condition_editor_scroll")
                    .auto_shrink([false, true])
                    .max_height(260.0)
                    .show(ui, |ui| {
                        // Base conditions grouped by category.
                        for (category_key, items) in CONDITION_CATEGORIES {
                            let filtered: Vec<&str> = items
                                .iter()
                                .copied()
                                .filter(|item| item.to_lowercase().contains(&query))
                                .collect();
                            if filtered.is_empty() {
                                continue;
                            }
                            let header = i18n.format(
                                "condition_editor.group_count",
                                &[&i18n.text(category_key), &filtered.len().to_string()],
                            );
                            egui::CollapsingHeader::new(header)
                                .id_salt(format!("cond_cat_{}", category_key))
                                .default_open(query.is_empty())
                                .show(ui, |ui| {
                                    ui.horizontal_wrapped(|ui| {
                                        for item in filtered {
                                            if condition_token_button(ui, item).clicked() {
                                                append_token(&mut state.value, item);
                                                state.set_flash(i18n.format(
                                                    "condition_editor.inserted",
                                                    &[item],
                                                ));
                                            }
                                        }
                                    });
                                });
                        }

                        // Existing IDs that can be reused as SubCondition_<id>.
                        if !existing_ids.is_empty() {
                            ui.separator();
                            ui.label(i18n.text("condition_editor.existing_ids"));
                            ui.horizontal_wrapped(|ui| {
                                for id in &existing_ids {
                                    let token = format!("SubCondition_{}", id);
                                    if (query.is_empty() || token.to_lowercase().contains(&query))
                                        && condition_token_button(ui, &token).clicked()
                                    {
                                        append_token(&mut state.value, &token);
                                        state.set_flash(i18n.format(
                                            "condition_editor.inserted",
                                            &[&token],
                                        ));
                                    }
                                }
                            });
                        }

                        if existing_ids.is_empty() && query.is_empty() {
                            ui.label(
                                egui::RichText::new(i18n.text("condition_editor.no_existing_ids"))
                                    .color(egui::Color32::from_gray(120))
                                    .size(11.0),
                            );
                        }
                    });

                ui.separator();
                ui.label(
                    egui::RichText::new(i18n.text("condition_editor.syntax_help"))
                        .size(11.0)
                        .color(egui::Color32::from_gray(140)),
                );
                ui.horizontal(|ui| {
                    if ui.button(i18n.text("button.confirm")).clicked() {
                        confirmed = true;
                    }
                    if ui.button(i18n.text("button.cancel")).clicked() {
                        cancelled = true;
                    }
                });
            });

        if cancelled {
            state.open = false;
            None
        } else if confirmed {
            state.open = false;
            Some(normalize_condition_expression(&state.value))
        } else {
            None
        }
    }
}

/// A compact button for a condition token.
fn condition_token_button(ui: &mut egui::Ui, token: &str) -> egui::Response {
    ui.add(
        egui::Button::new(
            egui::RichText::new(token)
                .size(12.0)
                .monospace(),
        )
        .fill(egui::Color32::from_gray(40))
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(90))),
    )
}

/// Append a token to the expression, adding a leading separator if needed.
fn append_token(expression: &mut String, token: &str) {
    if !expression.is_empty() && !expression.ends_with(['[', '(', ',', '!', ' ']) {
        expression.push_str(", ");
    }
    expression.push_str(token);
}

/// Insert a template into the expression. Since egui does not expose cursor
/// position, we append at the end and rely on the user editing the text box
/// directly for precise placement. The `kind` parameter is used to show a
/// helpful flash message.
fn insert_at_cursor_or_end(expression: &mut String, template: &str, _kind: &str) {
    if expression.is_empty() {
        expression.push_str(template);
        return;
    }
    if expression.ends_with(|c: char| c.is_alphanumeric() || c == '_') {
        expression.push_str(", ");
    }
    expression.push_str(template);
}

/// Collect existing condition IDs from `CreateCondition` and `CreateItemCondition`
/// nodes in the current label. Empty IDs are ignored because they cannot be used
/// for reuse.
fn collect_existing_condition_ids(label: &LabelContainer) -> Vec<String> {
    let mut ids = Vec::new();
    for node in label.nodes.values() {
        if node.node_type != NodeType::CreateCondition && node.node_type != NodeType::CreateItemCondition {
            continue;
        }
        if let Some(ParamValue::Literal(v)) = node.params.get("id") {
            if let Some(s) = v.as_str() {
                if !s.trim().is_empty() {
                    ids.push(s.trim().to_string());
                }
            }
        }
    }
    ids.sort();
    ids.dedup();
    ids
}

/// Normalize a user-written condition expression for code generation.
///
/// The game treats `[A, B]` as AND and `(A, B)` as OR. We remove all whitespace
/// and then ensure commas are followed by exactly one space so the generated
/// `.code` is readable.
fn normalize_condition_expression(expr: &str) -> String {
    let expr = expr.trim();
    if expr.is_empty() {
        return String::new();
    }
    let expr = expr.replace(|c: char| c.is_whitespace(), "");
    expr.replace(",", ", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_condition_expression() {
        assert_eq!(normalize_condition_expression("  "), "");
        assert_eq!(normalize_condition_expression("A"), "A");
        assert_eq!(normalize_condition_expression("[A, B]"), "[A, B]");
        assert_eq!(normalize_condition_expression("[ A , B ]"), "[A, B]");
        assert_eq!(normalize_condition_expression("(A,B,C)"), "(A, B, C)");
        assert_eq!(normalize_condition_expression("!A"), "!A");
        assert_eq!(normalize_condition_expression("[A, (B, !C)]"), "[A, (B, !C)]");
    }

    #[test]
    fn test_append_token() {
        let mut s = String::new();
        append_token(&mut s, "A");
        assert_eq!(s, "A");
        append_token(&mut s, "B");
        assert_eq!(s, "A, B");
        append_token(&mut s, "C");
        assert_eq!(s, "A, B, C");
    }

    #[test]
    fn test_insert_at_cursor_or_end() {
        let mut s = String::new();
        insert_at_cursor_or_end(&mut s, "[, ]", "and");
        assert_eq!(s, "[, ]");

        let mut s = "A".to_string();
        insert_at_cursor_or_end(&mut s, "[, ]", "and");
        assert_eq!(s, "A, [, ]");
    }

    #[test]
    fn test_collect_existing_condition_ids() {
        use crate::graph::node::{Node, Vec2};

        let mut label = LabelContainer::default();
        let mut n1 = Node::new(NodeType::CreateCondition, Vec2::ZERO);
        n1.set_param("id", ParamValue::Literal(serde_json::json!("main")));
        let mut n2 = Node::new(NodeType::CreateCondition, Vec2::ZERO);
        n2.set_param("id", ParamValue::Literal(serde_json::json!("")));
        let mut n3 = Node::new(NodeType::CreateItemCondition, Vec2::ZERO);
        n3.set_param("id", ParamValue::Literal(serde_json::json!("item")));
        label.nodes.insert("n1".to_string(), n1);
        label.nodes.insert("n2".to_string(), n2);
        label.nodes.insert("n3".to_string(), n3);

        let ids = collect_existing_condition_ids(&label);
        assert_eq!(ids, vec!["item".to_string(), "main".to_string()]);
    }
}
