//! Scene-based node catalog.
//!
//! The node library no longer groups nodes by the raw API category used in
//! `NodeDefinition::category`. Instead, this module provides an independent
//! *scene* classification: a single node may appear in multiple scenes when it
//! is useful in different contexts (e.g. `GetItemCount` is both a query and an
//! item operation).
//!
//! The catalog is intentionally static and UI-only. It does not affect code
//! generation, serialization, or validation.

use crate::graph::types::NodeType;

/// A top-level scene category.
///
/// Each category contains one or more subcategories. The `id` is used as an i18n
/// key prefix (`scene.{id}`) and should match the keys added to the translation
/// files.
#[derive(Debug, Clone)]
pub struct SceneCategory {
    /// I18n key for the category label (e.g. `scene.mission_flow`).
    pub id: &'static str,
    /// Subcategories inside this scene.
    pub subcategories: Vec<SceneSubcategory>,
}

/// A subcategory inside a scene.
#[derive(Debug, Clone)]
pub struct SceneSubcategory {
    /// I18n key for the subcategory label (e.g. `scene.mission_flow.control`).
    pub id: &'static str,
    /// Node types that belong to this subcategory.
    pub nodes: Vec<NodeType>,
}

impl SceneCatalog {
    /// Returns the full scene-based catalog used by the node library.
    pub fn categories() -> Vec<SceneCategory> {
        vec![
            SceneCategory {
                id: "scene.mission_flow",
                subcategories: vec![
                    SceneSubcategory {
                        id: "scene.mission_flow.threading",
                        nodes: vec![
                            NodeType::CreateThread,
                            NodeType::CreateListener,
                            NodeType::CreateListenerLocal,
                            NodeType::CreateEventListener,
                            NodeType::CreateEventListenerLocal,
                            NodeType::DestroyListener,
                            NodeType::GetCurrentThread,
                            NodeType::WaitForThread,
                        ],
                    },
                    SceneSubcategory {
                        id: "scene.mission_flow.control",
                        nodes: vec![
                            NodeType::Goto,
                            NodeType::If,
                            NodeType::While,
                            NodeType::For,
                            NodeType::Break,
                            NodeType::Return,
                            NodeType::Wait,
                            NodeType::WaitForEvent,
                            NodeType::ForeachNode,
                            NodeType::CallFunction,
                            NodeType::CallMethod,
                            NodeType::TriggerGameOver,
                        ],
                    },
                    SceneSubcategory {
                        id: "scene.mission_flow.mission_panel",
                        nodes: vec![
                            NodeType::CreateMissionPanel,
                            NodeType::CreateMissionMenuItem,
                        ],
                    },
                    SceneSubcategory {
                        id: "scene.mission_flow.events",
                        nodes: vec![NodeType::SetEvent, NodeType::GetEvent],
                    },
                ],
            },
            SceneCategory {
                id: "scene.conditions",
                subcategories: vec![
                    SceneSubcategory {
                        id: "scene.conditions.compare",
                        nodes: vec![NodeType::CompareNumbers],
                    },
                    SceneSubcategory {
                        id: "scene.conditions.logic",
                        nodes: vec![
                            NodeType::Boolean,
                            NodeType::LogicAnd,
                            NodeType::LogicOr,
                            NodeType::LogicNot,
                        ],
                    },
                    SceneSubcategory {
                        id: "scene.conditions.state_check",
                        nodes: vec![
                            NodeType::CreateCondition,
                            NodeType::CreateItemCondition,
                            NodeType::CheckCondition,
                            NodeType::CheckEquipment,
                            NodeType::CheckCosplay,
                            NodeType::CanGameOver,
                            NodeType::GetStateBool,
                            NodeType::GetStateNumber,
                            NodeType::FunctionExists,
                            NodeType::FileExists,
                            NodeType::GetStageChanged,
                            NodeType::CollectItem,
                            NodeType::ListContains,
                            NodeType::NPCIsAlive,
                            NodeType::NPCSeesPlayer,
                            NodeType::NPCSeesFlashing,
                        ],
                    },
                ],
            },
            SceneCategory {
                id: "scene.data_get",
                subcategories: vec![
                    SceneSubcategory {
                        id: "scene.data_get.player_info",
                        nodes: vec![
                            NodeType::GetSkillShortcut,
                            NodeType::GetRandomPosition,
                            NodeType::GetPosition,
                            NodeType::GetAllWaypoints,
                            NodeType::GetStageRankLimit,
                            NodeType::GetSnapshotData,
                            NodeType::GetCurrentEarnRP,
                            NodeType::GetCurrentRP,
                            NodeType::GetEcstasy,
                            NodeType::GetStamina,
                            NodeType::GetMoisture,
                            NodeType::GetStateNumber,
                        ],
                    },
                    SceneSubcategory {
                        id: "scene.data_get.items_equipment",
                        nodes: vec![
                            NodeType::CollectItem,
                            NodeType::CheckEquipment,
                            NodeType::CheckCosplay,
                            NodeType::GetItemCount,
                        ],
                    },
                    SceneSubcategory {
                        id: "scene.data_get.globals",
                        nodes: vec![
                            NodeType::Global,
                            NodeType::Local,
                            NodeType::GetSave,
                            NodeType::GetTime,
                            NodeType::GetTimeDiff,
                            NodeType::GetSettings,
                            NodeType::GetMod,
                            NodeType::GetMods,
                            NodeType::GetStageChanged,
                            NodeType::GetProjectName,
                            NodeType::Variable,
                            NodeType::GetEvent,
                            NodeType::GetType,
                            NodeType::GetLanguage,
                            NodeType::FunctionExists,
                            NodeType::GetModVersion,
                        ],
                    },
                    SceneSubcategory {
                        id: "scene.data_get.literals",
                        nodes: vec![
                            NodeType::NumberConstant,
                            NodeType::StringConstant,
                            NodeType::Boolean,
                            NodeType::Color,
                            NodeType::Range,
                        ],
                    },
                ],
            },
            SceneCategory {
                id: "scene.data_set",
                subcategories: vec![
                    SceneSubcategory {
                        id: "scene.data_set.player_position",
                        nodes: vec![
                            NodeType::SetPlayerPosition,
                            NodeType::SetStage,
                            NodeType::SetPortalEnabled,
                        ],
                    },
                    SceneSubcategory {
                        id: "scene.data_set.player_stats",
                        nodes: vec![
                            NodeType::SetStageRankLimit,
                            NodeType::SetGraphicsOption,
                            NodeType::AddCurrentEarnRP,
                            NodeType::SetCurrentEarnRP,
                            NodeType::AddCurrentRP,
                            NodeType::SetCurrentRP,
                            NodeType::SetEcstasy,
                            NodeType::AddEcstasy,
                            NodeType::SetStamina,
                            NodeType::AddStamina,
                            NodeType::SetMoisture,
                            NodeType::AddMoisture,
                            NodeType::SetAction,
                            NodeType::SetSkill,
                            NodeType::SetSkillShortcut,
                            NodeType::SetPlayerData,
                        ],
                    },
                    SceneSubcategory {
                        id: "scene.data_set.player_sex",
                        nodes: vec![
                            NodeType::SetFutanari,
                            NodeType::SetSexPosition,
                            NodeType::DeactivateSex,
                            NodeType::SetSexMenu,
                            NodeType::TriggerSexOrgasm,
                            NodeType::CanGameOver,
                        ],
                    },
                    SceneSubcategory {
                        id: "scene.data_set.cosplay",
                        nodes: vec![
                            NodeType::EquipCosplay,
                            NodeType::UnequipCosplay,
                            NodeType::UnequipAllCosplay,
                            NodeType::OwnCosplay,
                        ],
                    },
                    SceneSubcategory {
                        id: "scene.data_set.adult_toys",
                        nodes: vec![
                            NodeType::SetVibrator,
                            NodeType::SetPiston,
                            NodeType::EquipAdultToy,
                            NodeType::UnequipAdultToy,
                            NodeType::LockHandcuffs,
                            NodeType::UnlockHandcuffs,
                        ],
                    },
                    SceneSubcategory {
                        id: "scene.data_set.items",
                        nodes: vec![
                            NodeType::DropItem,
                            NodeType::CollectItem,
                            NodeType::SetItemCount,
                            NodeType::AddItemCount,
                            NodeType::GetItemCount,
                        ],
                    },
                    SceneSubcategory {
                        id: "scene.data_set.variables",
                        nodes: vec![
                            NodeType::Global,
                            NodeType::Local,
                            NodeType::SetVariable,
                            NodeType::SetEvent,
                        ],
                    },
                ],
            },
            SceneCategory {
                id: "scene.data_process",
                subcategories: vec![
                    SceneSubcategory {
                        id: "scene.data_process.trig",
                        nodes: vec![
                            NodeType::Sin,
                            NodeType::Cos,
                            NodeType::Tan,
                            NodeType::Asin,
                            NodeType::Acos,
                            NodeType::Atan,
                        ],
                    },
                    SceneSubcategory {
                        id: "scene.data_process.rounding",
                        nodes: vec![
                            NodeType::Floor,
                            NodeType::Ceil,
                            NodeType::Round,
                            NodeType::Trunc,
                            NodeType::Sign,
                            NodeType::Abs,
                        ],
                    },
                    SceneSubcategory {
                        id: "scene.data_process.random",
                        nodes: vec![
                            NodeType::Random,
                            NodeType::RandomInt,
                        ],
                    },
                    SceneSubcategory {
                        id: "scene.data_process.math_general",
                        nodes: vec![
                            NodeType::LogN,
                            NodeType::Log2,
                            NodeType::Log10,
                            NodeType::Min,
                            NodeType::Max,
                            NodeType::CompareNumbers,
                            NodeType::NumberConstant,
                        ],
                    },
                    SceneSubcategory {
                        id: "scene.data_process.string",
                        nodes: vec![
                            NodeType::Length,
                            NodeType::Lower,
                            NodeType::Upper,
                            NodeType::Find,
                            NodeType::SubString,
                            NodeType::Format,
                            NodeType::Translate,
                            NodeType::ToNumber,
                            NodeType::GetFileExtension,
                            NodeType::StringConstant,
                        ],
                    },
                    SceneSubcategory {
                        id: "scene.data_process.list",
                        nodes: vec![
                            NodeType::CreateList,
                            NodeType::Copy,
                            NodeType::CreateListFromJson,
                            NodeType::Range,
                            NodeType::ListInsert,
                            NodeType::ListRemove,
                            NodeType::ListCount,
                            NodeType::ListContains,
                            NodeType::ListIndexOf,
                            NodeType::ListKeys,
                        ],
                    },
                    SceneSubcategory {
                        id: "scene.data_process.file",
                        nodes: vec![NodeType::FileExists, NodeType::GetFiles],
                    },
                    SceneSubcategory {
                        id: "scene.data_process.vector_color",
                        nodes: vec![
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
                            NodeType::GetPosition,
                            NodeType::MakeVector,
                            NodeType::BreakVector,
                            NodeType::Color,
                        ],
                    },
                ],
            },
            SceneCategory {
                id: "scene.visual_ui",
                subcategories: vec![
                    SceneSubcategory {
                        id: "scene.visual_ui.visual",
                        nodes: vec![
                            NodeType::SetCamera,
                            NodeType::CreateArea,
                            NodeType::CreateZone,
                            NodeType::CreateText,
                            NodeType::CreateMessengerChat,
                            NodeType::CreateGallery,
                            NodeType::CreateSnapshot,
                            NodeType::CreateNPC,
                            NodeType::NPCWarp,
                            NodeType::NPCAddWaypoint,
                            NodeType::NPCIsAlive,
                            NodeType::NPCSeesPlayer,
                            NodeType::NPCSeesFlashing,
                            NodeType::GetAllSnapshots,
                            NodeType::GetImageReference,
                            NodeType::GetGraphicsOption,
                            NodeType::DeleteSnapshot,
                            NodeType::CreateMissionPanel,
                            NodeType::CreateMissionMenuItem,
                        ],
                    },
                    SceneSubcategory {
                        id: "scene.visual_ui.audio_screen",
                        nodes: vec![
                            NodeType::CreateAudio,
                            NodeType::StopAudio,
                            NodeType::PlaySoundEffect,
                            NodeType::ShowBlackscreen,
                        ],
                    },
                    SceneSubcategory {
                        id: "scene.visual_ui.input_interact",
                        nodes: vec![NodeType::CreateInput, NodeType::CreateInteractArea],
                    },
                ],
            },
            SceneCategory {
                id: "scene.editor",
                subcategories: vec![SceneSubcategory {
                    id: "scene.editor.editor",
                    nodes: vec![
                        NodeType::Meta,
                        NodeType::Comment,
                        NodeType::Group,
                        NodeType::Log,
                        NodeType::DumpVariables,
                        NodeType::DumpVariable,
                    ],
                }],
            },
        ]
    }

    /// All node types that appear anywhere in the scene catalog.
    #[allow(dead_code)]
    pub fn all_scene_nodes() -> Vec<NodeType> {
        let mut nodes = Vec::new();
        for category in Self::categories() {
            for sub in category.subcategories {
                nodes.extend(sub.nodes);
            }
        }
        nodes
    }
}

/// Static scene catalog. Does not hold state.
pub struct SceneCatalog;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_nodes_covered() {
        let all_scene = SceneCatalog::all_scene_nodes();
        let all_defs = crate::api::registry::all_node_definitions();
        let scene_set: std::collections::HashSet<_> = all_scene.iter().copied().collect();
        for def in all_defs {
            assert!(
                scene_set.contains(&def.node_type),
                "node {:?} is missing from the scene catalog",
                def.node_type
            );
        }
    }

    #[test]
    fn test_categories_are_non_empty() {
        for category in SceneCatalog::categories() {
            assert!(
                !category.subcategories.is_empty(),
                "category {} has no subcategories",
                category.id
            );
            for sub in &category.subcategories {
                assert!(
                    !sub.nodes.is_empty(),
                    "subcategory {} has no nodes",
                    sub.id
                );
            }
        }
    }

    #[test]
    fn test_boolean_output_nodes_are_in_conditions() {
        use crate::graph::types::PortType;

        let all_defs = crate::api::registry::all_node_definitions();
        let categories = SceneCatalog::categories();
        let condition_nodes: std::collections::HashSet<_> = categories
            .iter()
            .filter(|c| c.id == "scene.conditions")
            .flat_map(|c| c.subcategories.iter())
            .flat_map(|s| s.nodes.iter().copied())
            .collect();

        let mut missing = Vec::new();
        for def in all_defs {
            let has_bool_output = def
                .outputs
                .iter()
                .any(|p| p.port_type == PortType::Boolean);
            if has_bool_output && !condition_nodes.contains(&def.node_type) {
                missing.push(def.node_type);
            }
        }

        assert!(
            missing.is_empty(),
            "Boolean output nodes missing from scene.conditions: {:?}",
            missing
        );
    }
}
