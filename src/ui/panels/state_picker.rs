//! `_state` 探针选择器浮动窗口。
//!
//! 为 `GetStateBool` / `GetStateNumber` 提供 `_state` 嵌套字段的树形选择，
//! 返回完整点分路径（如 `Position.x`、`AdultToys.Handcuff`）。

use crate::ui::i18n::I18n;

/// 状态字段的数据类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateType {
    Boolean,
    Number,
    String,
}

impl StateType {
    /// 返回类型在 UI 中的短标签。
    fn label(self) -> &'static str {
        match self {
            StateType::Boolean => "bool",
            StateType::Number => "num",
            StateType::String => "str",
        }
    }
}

/// 状态树叶子节点。
#[derive(Debug, Clone)]
pub struct StateLeaf {
    /// 完整点分路径，生成代码时直接拼接到 `_state.` 后。
    pub path: String,
    /// 界面显示名称（可 i18n 化）。
    pub display: String,
    /// 字段数据类型。
    pub state_type: StateType,
}

impl StateLeaf {
    /// 创建新叶子。
    fn new(path: impl Into<String>, display: impl Into<String>, state_type: StateType) -> Self {
        Self {
            path: path.into(),
            display: display.into(),
            state_type,
        }
    }
}

/// 状态树分类节点。
#[derive(Debug, Clone)]
pub struct StateCategory {
    /// i18n 键前缀，用于界面显示。
    pub key: String,
    pub children: Vec<StateLeaf>,
}

impl StateCategory {
    /// 创建新分类。
    fn new(key: impl Into<String>, children: Vec<StateLeaf>) -> Self {
        Self {
            key: key.into(),
            children,
        }
    }
}

/// 构建静态的 `_state` 字段树。
///
/// 数据来源：`docs/documentation.html` 附录中的 `_state` 全局变量列表。
/// 只包含 Mod 作者最常用、最稳定的字段，避免把整个巨大对象暴露成选项。
pub fn build_state_tree() -> Vec<StateCategory> {
    vec![
        StateCategory::new(
            "character",
            vec![
                StateLeaf::new("Futanari", "扶她", StateType::Boolean),
                StateLeaf::new("Sitting", "坐着", StateType::Boolean),
                StateLeaf::new("Orgasm", "高潮", StateType::Boolean),
                StateLeaf::new("Moving", "移动中", StateType::Boolean),
                StateLeaf::new("Crouching", "蹲下", StateType::Boolean),
                StateLeaf::new("Peeing", "排尿中", StateType::Boolean),
                StateLeaf::new("Dashing", "奔跑", StateType::Boolean),
                StateLeaf::new("InLight", "在光照中", StateType::Boolean),
                StateLeaf::new("NearNPC", "靠近 NPC", StateType::Boolean),
                StateLeaf::new("Watched", "被注视", StateType::Boolean),
                StateLeaf::new("ShowingOff", "展示中", StateType::Boolean),
                StateLeaf::new("Bukkake", "颜射", StateType::Boolean),
                StateLeaf::new("Blindfolded", "眼罩", StateType::Boolean),
                StateLeaf::new("Invisible", "隐形", StateType::Boolean),
                StateLeaf::new("InOpenToilet", "在开放厕所", StateType::Boolean),
                StateLeaf::new("FPCamera", "第一人称相机", StateType::Boolean),
                StateLeaf::new("IsDayTime", "白天", StateType::Boolean),
                StateLeaf::new("GameOver", "游戏结束", StateType::Boolean),
                StateLeaf::new("Action", "动作", StateType::String),
                StateLeaf::new("FoundNPC", "发现玩家的 NPC", StateType::Number),
            ],
        ),
        StateCategory::new(
            "stats",
            vec![
                StateLeaf::new("Ecstasy", "快感", StateType::Number),
                StateLeaf::new("Detection", "警戒", StateType::Number),
                StateLeaf::new("Rank", "等级", StateType::Number),
                StateLeaf::new("HeartRate", "心率", StateType::Number),
                StateLeaf::new("Stamina", "体力", StateType::Number),
                StateLeaf::new("StaminaMax", "最大体力", StateType::Number),
                StateLeaf::new("Moisture", "湿润度", StateType::Number),
                StateLeaf::new("Bodypaint", "身体彩绘", StateType::Number),
                StateLeaf::new("RpBonus", "RP 加成", StateType::Number),
            ],
        ),
        StateCategory::new(
            "equipment",
            vec![
                StateLeaf::new("Vibrator", "跳蛋", StateType::Boolean),
                StateLeaf::new("Piston", "活塞", StateType::Boolean),
            ],
        ),
        StateCategory::new(
            "adult_toys",
            vec![
                // 实际判断通常用 `AdultToys.Handcuff != null`。
                StateLeaf::new("AdultToys.Handcuff", "手铐(拥有)", StateType::Boolean),
                StateLeaf::new("AdultToys.Vibrator", "跳蛋(拥有)", StateType::Boolean),
            ],
        ),
        StateCategory::new(
            "position",
            vec![
                StateLeaf::new("Position.stage", "场景", StateType::String),
                StateLeaf::new("Position.x", "X", StateType::Number),
                StateLeaf::new("Position.y", "Y", StateType::Number),
                StateLeaf::new("Position.z", "Z", StateType::Number),
                StateLeaf::new("Position.rx", "RX", StateType::Number),
                StateLeaf::new("Position.ry", "RY", StateType::Number),
                StateLeaf::new("Position.rz", "RZ", StateType::Number),
                StateLeaf::new("Position.rw", "RW", StateType::Number),
                StateLeaf::new("Position.laststage", "上一场景", StateType::String),
            ],
        ),
        StateCategory::new(
            "camera",
            vec![
                StateLeaf::new("Camera.stage", "场景", StateType::String),
                StateLeaf::new("Camera.x", "X", StateType::Number),
                StateLeaf::new("Camera.y", "Y", StateType::Number),
                StateLeaf::new("Camera.z", "Z", StateType::Number),
                StateLeaf::new("Camera.rx", "RX", StateType::Number),
                StateLeaf::new("Camera.ry", "RY", StateType::Number),
                StateLeaf::new("Camera.rz", "RZ", StateType::Number),
                StateLeaf::new("Camera.rw", "RW", StateType::Number),
                StateLeaf::new("Camera.pitch", "俯仰", StateType::Number),
                StateLeaf::new("Camera.yaw", "偏航", StateType::Number),
            ],
        ),
        StateCategory::new(
            "handcuffs",
            vec![
                StateLeaf::new("Handcuffs.State", "状态", StateType::String),
                StateLeaf::new("Handcuffs.Type", "类型", StateType::String),
            ],
        ),
    ]
}

/// 选择器持久状态。
#[derive(Debug, Clone)]
pub struct StatePickerState {
    pub open: bool,
    /// 接收选择结果的参数名。
    pub param_key: String,
    pub search: String,
}

impl StatePickerState {
    pub fn new(param_key: impl Into<String>) -> Self {
        Self {
            open: true,
            param_key: param_key.into(),
            search: String::new(),
        }
    }
}

/// `_state` 选择器浮动窗口。
pub struct StatePicker;

impl StatePicker {
    /// 显示选择器。返回 `Some(path)` 当用户点选一个叶子。
    pub fn show(
        ctx: &egui::Context,
        state: &mut StatePickerState,
        i18n: &I18n,
        expected_type: StateType,
    ) -> Option<String> {
        if !state.open {
            return None;
        }

        let mut picked = None;
        let mut closed = false;

        egui::Window::new(i18n.text("state_picker.title"))
            .id(egui::Id::new("state_picker"))
            .collapsible(false)
            .resizable(true)
            .default_size([360.0, 420.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(i18n.text("state_picker.search"));
                    ui.text_edit_singleline(&mut state.search);
                });
                ui.separator();

                let tree = build_state_tree();
                let query = state.search.to_lowercase();
                let mut total_matches = 0usize;

                egui::ScrollArea::vertical()
                    .id_salt("state_picker_scroll")
                    .auto_shrink([false, true])
                    .max_height(300.0)
                    .show(ui, |ui| {
                        for cat in &tree {
                            let matches: Vec<&StateLeaf> = cat
                                .children
                                .iter()
                                .filter(|leaf| {
                                    query.is_empty()
                                        || leaf.path.to_lowercase().contains(&query)
                                        || leaf.display.to_lowercase().contains(&query)
                                })
                                .collect();
                            total_matches += matches.len();

                            if matches.is_empty() {
                                continue;
                            }

                            let cat_label = i18n.format(
                                "label.items_count",
                                &[&i18n.text(&format!("state_picker.category.{}", cat.key)), &matches.len().to_string()],
                            );
                            egui::CollapsingHeader::new(cat_label)
                                .id_salt(format!("state_picker_{}", cat.key))
                                .default_open(!query.is_empty())
                                .show(ui, |ui| {
                                    ui.horizontal_wrapped(|ui| {
                                        for leaf in matches {
                                            if ui.add(state_leaf_card(leaf, expected_type)).clicked() {
                                                picked = Some(leaf.path.clone());
                                            }
                                        }
                                    });
                                });
                        }
                    });

                if total_matches == 0 {
                    ui.label(i18n.text("state_picker.no_match"));
                }

                ui.separator();
                if ui.button(i18n.text("button.close")).clicked() {
                    closed = true;
                }
            });

        if closed {
            state.open = false;
        }
        if picked.is_some() {
            state.open = false;
        }
        picked
    }
}

/// 叶子卡片：显示显示名 + 路径 + 类型。
fn state_leaf_card(leaf: &StateLeaf, expected_type: StateType) -> impl egui::Widget + '_ {
    let type_mismatch = leaf.state_type != expected_type;
    move |ui: &mut egui::Ui| {
        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(130.0, 46.0), egui::Sense::click());

        let accent = if type_mismatch {
            egui::Color32::from_rgb(255, 152, 0)
        } else {
            egui::Color32::from_rgb(100, 180, 255)
        };
        let fill = if response.hovered() {
            accent.gamma_multiply(0.15)
        } else {
            egui::Color32::from_gray(32)
        };

        ui.painter().rect_filled(rect, 4.0, fill);
        ui.painter().rect_stroke(
            rect,
            4.0,
            egui::Stroke::new(1.2, accent.gamma_multiply(0.6)),
            egui::StrokeKind::Middle,
        );

        ui.painter().text(
            rect.left_top() + egui::vec2(6.0, 5.0),
            egui::Align2::LEFT_TOP,
            &leaf.display,
            egui::FontId::proportional(12.0),
            egui::Color32::WHITE,
        );

        ui.painter().text(
            rect.left_top() + egui::vec2(6.0, 22.0),
            egui::Align2::LEFT_TOP,
            format!("{} [{}]", leaf.path, leaf.state_type.label()),
            egui::FontId::proportional(9.0),
            egui::Color32::from_gray(170),
        );

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_tree_contains_expected_paths() {
        let tree = build_state_tree();
        let all_paths: Vec<String> = tree
            .iter()
            .flat_map(|cat| cat.children.iter().map(|leaf| leaf.path.clone()))
            .collect();

        assert!(all_paths.contains(&"Futanari".to_string()));
        assert!(all_paths.contains(&"Position.x".to_string()));
        assert!(all_paths.contains(&"Camera.pitch".to_string()));
        assert!(all_paths.contains(&"AdultToys.Handcuff".to_string()));
        assert!(all_paths.contains(&"Handcuffs.Type".to_string()));
    }

    #[test]
    fn test_state_tree_types_are_consistent() {
        let tree = build_state_tree();
        let leaves: Vec<&StateLeaf> = tree
            .iter()
            .flat_map(|cat| cat.children.iter())
            .collect();

        let futanari = leaves.iter().find(|l| l.path == "Futanari").expect("Futanari exists");
        assert_eq!(futanari.state_type, StateType::Boolean);

        let ecstasy = leaves.iter().find(|l| l.path == "Ecstasy").expect("Ecstasy exists");
        assert_eq!(ecstasy.state_type, StateType::Number);

        let pos_x = leaves.iter().find(|l| l.path == "Position.x").expect("Position.x exists");
        assert_eq!(pos_x.state_type, StateType::Number);
    }

    #[test]
    fn test_state_picker_state_builder() {
        let state = StatePickerState::new("stateKey");
        assert!(state.open);
        assert_eq!(state.param_key, "stateKey");
        assert!(state.search.is_empty());
    }

    #[test]
    fn test_search_filters_by_path_and_display() {
        let tree = build_state_tree();
        let query = "pitch".to_string().to_lowercase();
        let camera = tree.iter().find(|c| c.key == "camera").expect("camera category");
        let matches: Vec<&StateLeaf> = camera
            .children
            .iter()
            .filter(|leaf| {
                leaf.path.to_lowercase().contains(&query)
                    || leaf.display.to_lowercase().contains(&query)
            })
            .collect();
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].path, "Camera.pitch");
    }
}
