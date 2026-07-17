//! Object method registry for the generic `CallMethod` node.
//!
//! While many high-frequency object methods have dedicated nodes (e.g. `NPCWarp`),
//! `CallMethod` is the fallback for the rest. This module provides a static list
//! of known object methods so the property panel can render a dropdown instead of
//! forcing the user to type a case-sensitive method name.

use serde_json::Value;

use crate::api::definitions::ParamType;

/// A single method available on an object type.
#[derive(Debug, Clone)]
pub struct MethodSignature {
    /// Object type name, e.g. `"Thread"`, `"NPC"`, `"List"`.
    pub object_type: &'static str,
    /// Method name as it appears in `.code`, e.g. `"Goto"`, `"Warp"`.
    pub method_name: &'static str,
    /// Parameters accepted by the method, used to build a template in the editor.
    pub params: Vec<MethodParam>,
}

/// A parameter of an object method.
#[derive(Debug, Clone)]
pub struct MethodParam {
    pub name: &'static str,
    pub param_type: ParamType,
    pub default: Value,
}

impl MethodSignature {
    /// Build a JSON object template from the method parameters.
    pub fn params_template(&self) -> Value {
        let mut map = serde_json::Map::new();
        for param in &self.params {
            map.insert(param.name.to_string(), param.default.clone());
        }
        Value::Object(map)
    }

    /// Human-readable label for the dropdown: "ObjectType.MethodName".
    pub fn full_label(&self) -> String {
        format!("{}.{}", self.object_type, self.method_name)
    }
}

/// All object methods currently registered.
pub fn all_methods() -> &'static [MethodSignature] {
    static METHODS: std::sync::OnceLock<Vec<MethodSignature>> = std::sync::OnceLock::new();
    METHODS.get_or_init(build_methods)
}

/// All known object type names.
pub fn object_types() -> Vec<&'static str> {
    let mut types: Vec<_> = all_methods()
        .iter()
        .map(|m| m.object_type)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    types.sort();
    types
}

/// Methods available for a specific object type.
pub fn methods_for_object_type(object_type: &str) -> Vec<&'static MethodSignature> {
    all_methods()
        .iter()
        .filter(|m| m.object_type == object_type)
        .collect()
}

/// Infer the object type name from a source node type.
///
/// Returns the object-type string used by the method registry, or `None` if
/// the source node is not a known object constructor.
pub fn object_type_from_node_type(node_type: crate::graph::types::NodeType) -> Option<&'static str> {
    use crate::graph::types::NodeType;
    match node_type {
        NodeType::CreateThread
        | NodeType::CreateListener
        | NodeType::CreateListenerLocal
        | NodeType::CreateEventListener
        | NodeType::CreateEventListenerLocal
        | NodeType::GetCurrentThread => Some("Thread"),
        NodeType::CreateNPC => Some("NPC"),
        NodeType::CreateArea => Some("Area"),
        NodeType::CreateZone => Some("Zone"),
        NodeType::CreateAudio => Some("Audio"),
        NodeType::CreateText => Some("Text"),
        NodeType::CreateSnapshot => Some("Snapshot"),
        NodeType::CreateCondition => Some("Condition"),
        NodeType::CreateItemCondition => Some("ItemCondition"),
        NodeType::CreateMissionPanel => Some("MissionPanel"),
        NodeType::CreateMissionMenuItem => Some("MissionMenuItem"),
        NodeType::CreateMessengerChat => Some("MessengerChat"),
        NodeType::CreateGallery => Some("Gallery"),
        NodeType::CreateInput => Some("Input"),
        NodeType::CreateInteractArea => Some("InteractArea"),
        NodeType::CreateList | NodeType::CreateListFromJson | NodeType::Copy => Some("List"),
        _ => None,
    }
}

fn build_methods() -> Vec<MethodSignature> {
    use ParamType::*;

    let v = |s: &str| Value::String(s.to_string());
    let n = |f: f64| Value::Number(serde_json::Number::from_f64(f).unwrap_or_else(|| 0.into()));
    let b = |v: bool| Value::Bool(v);
    let arr = |items: Vec<Value>| Value::Array(items);

    let mut methods = Vec::new();

    // Thread / Listener
    methods.extend([
        MethodSignature {
            object_type: "Thread",
            method_name: "Goto",
            params: vec![MethodParam {
                name: "labelName",
                param_type: String,
                default: v(""),
            }],
        },
        MethodSignature {
            object_type: "Thread",
            method_name: "WaitForFinish",
            params: vec![],
        },
    ]);

    // NPC
    methods.extend([
        MethodSignature {
            object_type: "NPC",
            method_name: "Warp",
            params: vec![
                MethodParam {
                    name: "position",
                    param_type: Vector,
                    default: arr(vec![n(0.0), n(0.0), n(0.0)]),
                },
                MethodParam {
                    name: "rotation",
                    param_type: Vector,
                    default: arr(vec![n(0.0), n(0.0), n(0.0)]),
                },
            ],
        },
        MethodSignature {
            object_type: "NPC",
            method_name: "AddWaypoint",
            params: vec![
                MethodParam {
                    name: "position",
                    param_type: Vector,
                    default: arr(vec![n(0.0), n(0.0), n(0.0)]),
                },
                MethodParam {
                    name: "rotation",
                    param_type: Vector,
                    default: arr(vec![n(0.0), n(0.0), n(0.0)]),
                },
                MethodParam {
                    name: "last",
                    param_type: Boolean,
                    default: b(false),
                },
            ],
        },
        MethodSignature {
            object_type: "NPC",
            method_name: "IsAlive",
            params: vec![],
        },
        MethodSignature {
            object_type: "NPC",
            method_name: "SeesPlayer",
            params: vec![],
        },
        MethodSignature {
            object_type: "NPC",
            method_name: "SeesFlashing",
            params: vec![],
        },
    ]);

    // Area
    methods.extend([
        MethodSignature {
            object_type: "Area",
            method_name: "SetColor",
            params: vec![MethodParam {
                name: "color",
                param_type: Color,
                default: arr(vec![n(1.0), n(1.0), n(1.0), n(1.0)]),
            }],
        },
        MethodSignature {
            object_type: "Area",
            method_name: "SetOutline",
            params: vec![MethodParam {
                name: "enabled",
                param_type: Boolean,
                default: b(true),
            }],
        },
        MethodSignature {
            object_type: "Area",
            method_name: "SetCompass",
            params: vec![MethodParam {
                name: "enabled",
                param_type: Boolean,
                default: b(true),
            }],
        },
        MethodSignature {
            object_type: "Area",
            method_name: "Destroy",
            params: vec![],
        },
    ]);

    // Zone
    methods.extend([
        MethodSignature {
            object_type: "Zone",
            method_name: "GetDistance",
            params: vec![],
        },
        MethodSignature {
            object_type: "Zone",
            method_name: "Destroy",
            params: vec![],
        },
    ]);

    // Audio
    methods.extend([
        MethodSignature {
            object_type: "Audio",
            method_name: "Play",
            params: vec![],
        },
        MethodSignature {
            object_type: "Audio",
            method_name: "Stop",
            params: vec![MethodParam {
                name: "fadeOutTime",
                param_type: Number,
                default: n(0.0),
            }],
        },
        MethodSignature {
            object_type: "Audio",
            method_name: "SetVolume",
            params: vec![MethodParam {
                name: "volume",
                param_type: Number,
                default: n(1.0),
            }],
        },
    ]);

    // Text
    methods.extend([
        MethodSignature {
            object_type: "Text",
            method_name: "Add",
            params: vec![MethodParam {
                name: "text",
                param_type: String,
                default: v(""),
            }],
        },
        MethodSignature {
            object_type: "Text",
            method_name: "SetText",
            params: vec![MethodParam {
                name: "text",
                param_type: String,
                default: v(""),
            }],
        },
        MethodSignature {
            object_type: "Text",
            method_name: "SetVisible",
            params: vec![MethodParam {
                name: "visible",
                param_type: Boolean,
                default: b(true),
            }],
        },
        MethodSignature {
            object_type: "Text",
            method_name: "Destroy",
            params: vec![],
        },
    ]);

    // Snapshot
    methods.extend([
        MethodSignature {
            object_type: "Snapshot",
            method_name: "Save",
            params: vec![],
        },
        MethodSignature {
            object_type: "Snapshot",
            method_name: "Destroy",
            params: vec![],
        },
    ]);

    // Condition / ItemCondition
    methods.extend([
        MethodSignature {
            object_type: "Condition",
            method_name: "Check",
            params: vec![],
        },
        MethodSignature {
            object_type: "ItemCondition",
            method_name: "Check",
            params: vec![],
        },
    ]);

    // MissionPanel
    methods.extend([
        MethodSignature {
            object_type: "MissionPanel",
            method_name: "SetTitle",
            params: vec![MethodParam {
                name: "title",
                param_type: String,
                default: v(""),
            }],
        },
        MethodSignature {
            object_type: "MissionPanel",
            method_name: "SetProgress",
            params: vec![MethodParam {
                name: "progress",
                param_type: Number,
                default: n(0.0),
            }],
        },
        MethodSignature {
            object_type: "MissionPanel",
            method_name: "SetVisible",
            params: vec![MethodParam {
                name: "visible",
                param_type: Boolean,
                default: b(true),
            }],
        },
        MethodSignature {
            object_type: "MissionPanel",
            method_name: "Destroy",
            params: vec![],
        },
    ]);

    // MissionMenuItem
    methods.extend([
        MethodSignature {
            object_type: "MissionMenuItem",
            method_name: "SetText",
            params: vec![MethodParam {
                name: "text",
                param_type: String,
                default: v(""),
            }],
        },
        MethodSignature {
            object_type: "MissionMenuItem",
            method_name: "SetCompleted",
            params: vec![MethodParam {
                name: "completed",
                param_type: Boolean,
                default: b(false),
            }],
        },
    ]);

    // MessengerChat
    methods.extend([
        MethodSignature {
            object_type: "MessengerChat",
            method_name: "AddMessage",
            params: vec![
                MethodParam {
                    name: "text",
                    param_type: String,
                    default: v(""),
                },
                MethodParam {
                    name: "isPlayer",
                    param_type: Boolean,
                    default: b(false),
                },
            ],
        },
        MethodSignature {
            object_type: "MessengerChat",
            method_name: "AddButton",
            params: vec![MethodParam {
                name: "text",
                param_type: String,
                default: v(""),
            }],
        },
        MethodSignature {
            object_type: "MessengerChat",
            method_name: "SetVisible",
            params: vec![MethodParam {
                name: "visible",
                param_type: Boolean,
                default: b(true),
            }],
        },
        MethodSignature {
            object_type: "MessengerChat",
            method_name: "Destroy",
            params: vec![],
        },
    ]);

    // Gallery
    methods.extend([
        MethodSignature {
            object_type: "Gallery",
            method_name: "SetVisible",
            params: vec![MethodParam {
                name: "visible",
                param_type: Boolean,
                default: b(true),
            }],
        },
        MethodSignature {
            object_type: "Gallery",
            method_name: "Destroy",
            params: vec![],
        },
    ]);

    // Input
    methods.extend([
        MethodSignature {
            object_type: "Input",
            method_name: "GetPressed",
            params: vec![],
        },
        MethodSignature {
            object_type: "Input",
            method_name: "Destroy",
            params: vec![],
        },
    ]);

    // InteractArea
    methods.extend([
        MethodSignature {
            object_type: "InteractArea",
            method_name: "SetText",
            params: vec![MethodParam {
                name: "text",
                param_type: String,
                default: v(""),
            }],
        },
        MethodSignature {
            object_type: "InteractArea",
            method_name: "SetVisible",
            params: vec![MethodParam {
                name: "visible",
                param_type: Boolean,
                default: b(true),
            }],
        },
        MethodSignature {
            object_type: "InteractArea",
            method_name: "Destroy",
            params: vec![],
        },
    ]);

    // List
    methods.extend([
        MethodSignature {
            object_type: "List",
            method_name: "Insert",
            params: vec![
                MethodParam {
                    name: "index",
                    param_type: Number,
                    default: Value::Null,
                },
                MethodParam {
                    name: "values",
                    param_type: List,
                    default: arr(vec![]),
                },
            ],
        },
        MethodSignature {
            object_type: "List",
            method_name: "Remove",
            params: vec![
                MethodParam {
                    name: "index",
                    param_type: Number,
                    default: Value::Null,
                },
                MethodParam {
                    name: "count",
                    param_type: Number,
                    default: n(1.0),
                },
            ],
        },
        MethodSignature {
            object_type: "List",
            method_name: "Count",
            params: vec![],
        },
        MethodSignature {
            object_type: "List",
            method_name: "Contains",
            params: vec![MethodParam {
                name: "value",
                param_type: Object,
                default: Value::Null,
            }],
        },
        MethodSignature {
            object_type: "List",
            method_name: "IndexOf",
            params: vec![MethodParam {
                name: "value",
                param_type: Object,
                default: Value::Null,
            }],
        },
        MethodSignature {
            object_type: "List",
            method_name: "Keys",
            params: vec![MethodParam {
                name: "includeAll",
                param_type: Boolean,
                default: b(false),
            }],
        },
    ]);

    methods
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_has_thread_methods() {
        let thread_methods = methods_for_object_type("Thread");
        assert!(!thread_methods.is_empty());
        assert!(thread_methods.iter().any(|m| m.method_name == "Goto"));
    }

    #[test]
    fn test_object_type_from_node_type_maps_npc() {
        assert_eq!(
            object_type_from_node_type(crate::graph::types::NodeType::CreateNPC),
            Some("NPC")
        );
    }

    #[test]
    fn test_params_template_builds_object() {
        let m = MethodSignature {
            object_type: "NPC",
            method_name: "Warp",
            params: vec![
                MethodParam {
                    name: "position",
                    param_type: ParamType::Vector,
                    default: serde_json::json!([0, 0, 0]),
                },
                MethodParam {
                    name: "rotation",
                    param_type: ParamType::Vector,
                    default: serde_json::json!([0, 0, 0]),
                },
            ],
        };
        let template = m.params_template();
        assert!(template.is_object());
        assert!(template.get("position").is_some());
    }

    #[test]
    fn test_all_methods_count() {
        // Ensure the registry covers a meaningful number of methods.
        assert!(all_methods().len() >= 40, "expected at least 40 methods");
    }
}
