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
    /// Parameter key that will receive the final value.
    pub param_key: String,
    /// Editable expression value.
    pub value: String,
    /// Search filter for the base condition list.
    pub search: String,
    /// A short flash message shown after inserting a token.
    pub flash: Option<String>,
    /// Frame counter for the flash message.
    pub flash_frames: u32,
    /// Last known text cursor range as character indices (start, end). Used to
    /// insert operators at the caret or wrap the current selection when the user
    /// clicks toolbar buttons.
    pub cursor_range: Option<(usize, usize)>,
    /// The actual egui id of the expression [`TextEdit`], so we can re-load its
    /// stored state (including the caret position) on the same frame even after
    /// toolbar buttons steal focus.
    pub text_edit_id: Option<egui::Id>,
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
            cursor_range: None,
            text_edit_id: None,
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
                let output = egui::TextEdit::multiline(&mut state.value)
                    .id_salt("condition_editor_expression")
                    .desired_rows(3)
                    .desired_width(f32::INFINITY)
                    .font(egui::TextStyle::Monospace)
                    .show(ui);
                state.text_edit_id = Some(output.response.id);
                state.cursor_range = output.cursor_range.map(|r| {
                    let range = r.as_sorted_char_range();
                    (range.start, range.end)
                });

                // Toolbar
                let cursor = load_cursor_range(ctx, state.text_edit_id).or(state.cursor_range);
                ui.horizontal(|ui| {
                    if ui
                        .button(i18n.text("condition_editor.and"))
                        .on_hover_text(i18n.text("condition_editor.and_hint"))
                        .clicked()
                    {
                        insert_and(&mut state.value, cursor);
                    }
                    if ui
                        .button(i18n.text("condition_editor.or"))
                        .on_hover_text(i18n.text("condition_editor.or_hint"))
                        .clicked()
                    {
                        insert_or(&mut state.value, cursor);
                    }
                    if ui
                        .button(i18n.text("condition_editor.not"))
                        .on_hover_text(i18n.text("condition_editor.not_hint"))
                        .clicked()
                    {
                        insert_not(&mut state.value, cursor);
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
                                                let cursor = load_cursor_range(ctx, state.text_edit_id)
                                                    .or(state.cursor_range);
                                                insert_token(&mut state.value, item, cursor);
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
                                        let cursor = load_cursor_range(ctx, state.text_edit_id)
                                            .or(state.cursor_range);
                                        insert_token(&mut state.value, &token, cursor);
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

/// Extract a character range from a string as a new `String`.
fn char_slice(expression: &str, start: usize, end: usize) -> String {
    expression.chars().skip(start).take(end.saturating_sub(start)).collect()
}

/// Replace the character range `[start, end)` with `replacement`.
fn replace_char_range(expression: &mut String, start: usize, end: usize, replacement: &str) {
    let before: String = expression.chars().take(start).collect();
    let after: String = expression.chars().skip(end).collect();
    *expression = format!("{}{}{}", before, replacement, after);
}

/// Insert `text` at the given character index.
fn insert_at_char_index(expression: &mut String, char_idx: usize, text: &str) {
    let before: String = expression.chars().take(char_idx).collect();
    let after: String = expression.chars().skip(char_idx).collect();
    *expression = format!("{}{}{}", before, text, after);
}

/// Return the character immediately before the given character index.
fn char_before(expression: &str, char_idx: usize) -> Option<char> {
    if char_idx == 0 {
        None
    } else {
        expression.chars().nth(char_idx - 1)
    }
}

/// Determine which bracket pair (if any) the caret at `char_idx` is inside.
/// Returns `'['` for AND brackets, `'('` for OR brackets, or `None`.
fn enclosing_bracket(expression: &str, char_idx: usize) -> Option<char> {
    let mut stack: Vec<char> = Vec::new();
    for (i, c) in expression.chars().enumerate() {
        if i == char_idx {
            return stack.last().copied();
        }
        match c {
            '[' => stack.push('['),
            '(' => stack.push('('),
            ']' if stack.last() == Some(&'[') => {
                stack.pop();
            }
            ')' if stack.last() == Some(&'(') => {
                stack.pop();
            }
            _ => {}
        }
    }
    // Caret is past the last character.
    stack.last().copied()
}

/// Load the current caret / selection range from egui's stored TextEdit state.
///
/// This is needed because toolbar buttons steal focus from the TextEdit, making
/// `TextEditOutput::cursor_range` from the previous frame unreliable. The stored
/// state keeps the last caret position even after focus is lost.
fn load_cursor_range(ctx: &egui::Context, id: Option<egui::Id>) -> Option<(usize, usize)> {
    let id = id?;
    let te_state = egui::widgets::text_edit::TextEditState::load(ctx, id)?;
    let range = te_state.cursor.char_range()?;
    let start = range.primary.index.min(range.secondary.index);
    let end = range.primary.index.max(range.secondary.index);
    Some((start, end))
}

/// Insert an AND group at the caret, or wrap the current selection.
fn insert_and(expression: &mut String, cursor_range: Option<(usize, usize)>) {
    if let Some((start, end)) = cursor_range {
        if start != end {
            // Wrap selected text in `[...]`.
            let selected = char_slice(expression, start, end);
            replace_char_range(expression, start, end, &format!("[{}]", selected));
            return;
        }
        // Inside an existing bracket group: add a comma-separated entry.
        if enclosing_bracket(expression, start).is_some() {
            insert_at_char_index(expression, start, ", ");
            return;
        }
        // Otherwise insert a fresh AND template at the caret.
        insert_at_char_index(expression, start, "[, ]");
    } else {
        append_template(expression, "[, ]");
    }
}

/// Insert an OR group at the caret, or wrap the current selection.
fn insert_or(expression: &mut String, cursor_range: Option<(usize, usize)>) {
    if let Some((start, end)) = cursor_range {
        if start != end {
            let selected = char_slice(expression, start, end);
            replace_char_range(expression, start, end, &format!("({})", selected));
            return;
        }
        if enclosing_bracket(expression, start).is_some() {
            insert_at_char_index(expression, start, ", ");
            return;
        }
        insert_at_char_index(expression, start, "(, )");
    } else {
        append_template(expression, "(, )");
    }
}

/// Insert a NOT operator at the caret, or wrap the current selection.
fn insert_not(expression: &mut String, cursor_range: Option<(usize, usize)>) {
    if let Some((start, end)) = cursor_range {
        if start != end {
            let selected = char_slice(expression, start, end);
            replace_char_range(expression, start, end, &format!("!{}", selected));
            return;
        }
        insert_at_char_index(expression, start, "!");
    } else {
        append_template(expression, "!");
    }
}

/// Insert a base condition token at the caret. If text is selected, it is
/// replaced. If the caret is inside a bracket group, a leading comma is added
/// when appropriate.
fn insert_token(
    expression: &mut String,
    token: &str,
    cursor_range: Option<(usize, usize)>,
) {
    if let Some((start, end)) = cursor_range {
        if start != end {
            replace_char_range(expression, start, end, token);
            return;
        }
        let needs_comma = if enclosing_bracket(expression, start).is_some() {
            match char_before(expression, start) {
                Some('[') | Some('(') | Some(',') | Some('!') | Some(' ') => false,
                _ => !expression.is_empty(),
            }
        } else {
            !expression.is_empty() && !expression.ends_with(['[', '(', ',', '!', ' '])
        };
        if needs_comma {
            insert_at_char_index(expression, start, ", ");
            insert_at_char_index(expression, start + 2, token);
        } else {
            insert_at_char_index(expression, start, token);
        }
    } else {
        append_token(expression, token);
    }
}

/// Append a template to the end of the expression, adding a leading separator
/// if needed. Used as a fallback when no cursor information is available.
fn append_template(expression: &mut String, template: &str) {
    if expression.is_empty() {
        expression.push_str(template);
        return;
    }
    if expression.ends_with(|c: char| c.is_alphanumeric() || c == '_') {
        expression.push_str(", ");
    }
    expression.push_str(template);
}

/// Append a token to the end of the expression, adding a leading separator if
/// needed. Kept for non-cursor code paths.
fn append_token(expression: &mut String, token: &str) {
    if !expression.is_empty() && !expression.ends_with(['[', '(', ',', '!', ' ']) {
        expression.push_str(", ");
    }
    expression.push_str(token);
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
    fn test_insert_and_wraps_selection() {
        let mut s = "A, B".to_string();
        let range = cursor_range(0, 4);
        insert_and(&mut s, Some(range));
        assert_eq!(s, "[A, B]");
    }

    #[test]
    fn test_insert_and_adds_comma_inside_bracket() {
        let mut s = "[A, B]".to_string();
        let range = cursor_range(5, 5); // caret between 'B' and ']'
        insert_and(&mut s, Some(range));
        assert_eq!(s, "[A, B, ]");
    }

    #[test]
    fn test_insert_and_inserts_template_at_caret() {
        let mut s = "A".to_string();
        let range = cursor_range(1, 1); // caret after 'A'
        insert_and(&mut s, Some(range));
        assert_eq!(s, "A[, ]");
    }

    #[test]
    fn test_insert_or_wraps_selection() {
        let mut s = "A, B".to_string();
        let range = cursor_range(0, 4);
        insert_or(&mut s, Some(range));
        assert_eq!(s, "(A, B)");
    }

    #[test]
    fn test_insert_or_adds_comma_inside_bracket() {
        let mut s = "(A, B)".to_string();
        let range = cursor_range(5, 5);
        insert_or(&mut s, Some(range));
        assert_eq!(s, "(A, B, )");
    }

    #[test]
    fn test_insert_not_wraps_selection() {
        let mut s = "A".to_string();
        let range = cursor_range(0, 1);
        insert_not(&mut s, Some(range));
        assert_eq!(s, "!A");
    }

    #[test]
    fn test_insert_not_inserts_at_caret() {
        let mut s = "A".to_string();
        let range = cursor_range(0, 0); // caret before 'A'
        insert_not(&mut s, Some(range));
        assert_eq!(s, "!A");
    }

    #[test]
    fn test_insert_token_adds_comma_inside_bracket() {
        let mut s = "[A, B]".to_string();
        let range = cursor_range(5, 5); // caret after 'B'
        insert_token(&mut s, "C", Some(range));
        assert_eq!(s, "[A, B, C]");
    }

    #[test]
    fn test_insert_token_no_comma_after_open_bracket() {
        let mut s = "[A]".to_string();
        let range = cursor_range(1, 1); // caret after '['
        insert_token(&mut s, "B", Some(range));
        assert_eq!(s, "[BA]");
    }

    #[test]
    fn test_insert_token_replaces_selection() {
        let mut s = "A, B, C".to_string();
        let range = cursor_range(0, 4); // select "A, B"
        insert_token(&mut s, "X", Some(range));
        assert_eq!(s, "X, C");
    }

    fn cursor_range(start: usize, end: usize) -> (usize, usize) {
        (start, end)
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
