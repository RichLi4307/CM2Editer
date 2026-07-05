use serde::{Deserialize, Serialize};

/// All node types supported by the editor.
///
/// Each variant corresponds to a function call, control structure, or
/// object constructor/method defined in the CustomMissions2 API.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum NodeType {
    // ── Control Flow ──
    /// Entry point; every graph must have exactly one.
    Start,
    /// Label definition, target of Goto.
    Label,
    /// Jump to a label.
    Goto,
    /// Conditional branch (True / False).
    If,
    /// Loop while condition holds.
    While,
    /// Iterate over a list.
    For,
    /// Break out of a loop.
    Break,
    /// Return a value, sets `_result`.
    Return,
    /// Wait for a number of seconds.
    Wait,
    /// Wait until a SetEvent fires.
    WaitForEvent,

    // ── General Functions ──
    /// Log output to console.
    Log,
    /// Read/write a global variable.
    Global,
    /// Read/write a local variable.
    Local,
    /// Get the type name of a value.
    GetType,
    /// Get the current game language.
    GetLanguage,
    /// Dump all variables.
    DumpVariables,
    /// Dump a single variable.
    DumpVariable,
    /// Dynamically call a function.
    CallFunction,
    /// Dynamically call a method on an object.
    CallMethod,
    /// Create a color list [r, g, b, a].
    Color,
    /// Generate a numeric range.
    Range,
    /// Set a cross-project event.
    SetEvent,
    /// Get event data.
    GetEvent,

    // ── Game Functions: Items & Equipment ──
    /// Drop an item in the world.
    DropItem,
    /// Pick up an item.
    CollectItem,
    /// Set vibrator strength.
    SetVibrator,
    /// Set piston strength.
    SetPiston,
    /// Lock handcuffs on the player.
    LockHandcuffs,
    /// Unlock handcuffs.
    UnlockHandcuffs,
    /// Equip cosplay items.
    EquipCosplay,
    /// Unequip cosplay items.
    UnequipCosplay,
    /// Unequip all cosplay items.
    UnequipAllCosplay,
    /// Set cosplay ownership status.
    OwnCosplay,
    /// Equip adult toys.
    EquipAdultToy,
    /// Unequip adult toys.
    UnequipAdultToy,

    // ── Game Functions: Player State ──
    /// Set the player's position and rotation.
    SetPlayerPosition,
    /// Switch to a different stage.
    SetStage,
    /// Set the camera pitch, yaw, and lock.
    SetCamera,
    /// Set the player's current action.
    SetAction,
    /// Set futanari state.
    SetFutanari,
    /// Enable or disable a skill.
    SetSkill,
    /// Set arbitrary player data.
    SetPlayerData,
    /// Set a skill shortcut slot.
    SetSkillShortcut,
    /// Get a skill shortcut slot.
    GetSkillShortcut,
    /// Get a random position within range.
    GetRandomPosition,

    // ── Game Functions: Numeric Stats (RP, Ecstasy, Stamina, etc.) ──
    /// Add to current earned RP.
    AddCurrentEarnRP,
    /// Set current earned RP.
    SetCurrentEarnRP,
    /// Get current earned RP.
    GetCurrentEarnRP,
    /// Add to held RP.
    AddCurrentRP,
    /// Set held RP.
    SetCurrentRP,
    /// Get held RP.
    GetCurrentRP,
    /// Set ecstasy value.
    SetEcstasy,
    /// Add to ecstasy value.
    AddEcstasy,
    /// Get ecstasy value.
    GetEcstasy,
    /// Set stamina value.
    SetStamina,
    /// Add to stamina value.
    AddStamina,
    /// Get stamina value.
    GetStamina,
    /// Set moisture (bladder) value.
    SetMoisture,
    /// Add to moisture value.
    AddMoisture,
    /// Get moisture value.
    GetMoisture,
    /// Set item count.
    SetItemCount,
    /// Add to item count.
    AddItemCount,
    /// Get item count.
    GetItemCount,

    // ── Game Functions: Game Control ──
    /// Set/get whether game over is possible.
    CanGameOver,
    /// Force trigger game over.
    TriggerGameOver,
    /// Play a sound effect.
    PlaySoundEffect,
    /// Set a stage rank limit.
    SetStageRankLimit,
    /// Get a stage rank limit.
    GetStageRankLimit,
    /// Enable or disable a portal.
    SetPortalEnabled,
    /// Get all waypoints in the stage.
    GetAllWaypoints,
    /// Set the sex position.
    SetSexPosition,
    /// Deactivate sex.
    DeactivateSex,
    /// Configure the sex menu.
    SetSexMenu,

    // ── Additional Game Functions ──
    /// Show a full-screen black/color overlay.
    ShowBlackscreen,
    /// Get snapshot metadata.
    GetSnapshotData,
    /// Get all snapshot references.
    GetAllSnapshots,
    /// Mark a snapshot as deleted.
    DeleteSnapshot,
    /// Get an image reference from a file path.
    GetImageReference,

    // ── Graphics ──
    /// Set a graphics option.
    SetGraphicsOption,
    /// Get a graphics option value.
    GetGraphicsOption,

    // ── Math: Standard ──
    /// Random floating-point number.
    Random,
    /// Random integer.
    RandomInt,
    /// Sine of an angle.
    Sin,
    /// Cosine of an angle.
    Cos,
    /// Tangent of an angle.
    Tan,
    /// Arcsine.
    Asin,
    /// Arccosine.
    Acos,
    /// Arctangent.
    Atan,
    /// Floor rounding.
    Floor,
    /// Ceiling rounding.
    Ceil,
    /// Round to nearest integer.
    Round,
    /// Truncate fractional part.
    Trunc,
    /// Sign of a number.
    Sign,
    /// Absolute value.
    Abs,
    /// Natural logarithm.
    LogN,
    /// Base-2 logarithm.
    Log2,
    /// Base-10 logarithm.
    Log10,
    /// Minimum of a list of numbers.
    Min,
    /// Maximum of a list of numbers.
    Max,

    // ── Math: Vectors ──
    /// Create a 3D vector.
    Vector,
    /// Create a quaternion.
    Quaternion,
    /// Length of a vector.
    Vector3Length,
    /// Squared length of a vector.
    Vector3SqrLength,
    /// Add two vectors.
    Vector3Add,
    /// Subtract two vectors.
    Vector3Sub,
    /// Scale a vector.
    Vector3Scale,
    /// Dot product of two vectors.
    Vector3Dot,
    /// Cross product of two vectors.
    Vector3Cross,
    /// Rotate a vector by a quaternion.
    Vector3Rotate,
    /// Distance between two vectors.
    Vector3Distance,

    // ── String Functions ──
    /// String length.
    Length,
    /// Convert to lowercase.
    Lower,
    /// Convert to uppercase.
    Upper,
    /// Find substring index.
    Find,
    /// Extract a substring.
    SubString,
    /// Format a string with parameters.
    Format,
    /// Convert string to number.
    ToNumber,

    // ── File Functions ──
    /// Check if a file exists.
    FileExists,
    /// List files in a directory.
    GetFiles,
    /// Get a file extension.
    GetFileExtension,

    // ── Object Constructors / Methods ──
    /// Create an empty list.
    CreateList,
    /// Copy a list (shallow or deep).
    Copy,
    /// Create a list from a JSON file.
    CreateListFromJson,

    /// Create a thread.
    CreateThread,
    /// Create a listener (parent scope).
    CreateListener,
    /// Create a listener (local scope).
    CreateListenerLocal,
    /// Create a mission panel.
    CreateMissionPanel,
    /// Create a mission menu item.
    CreateMissionMenuItem,
    /// Create an area.
    CreateArea,
    /// Create a zone (multi-area combo).
    CreateZone,
    /// Create a condition object.
    CreateCondition,
    /// Create an item condition.
    CreateItemCondition,
    /// Create an interaction area.
    CreateInteractArea,
    /// Create a text object.
    CreateText,
    /// Create a messenger chat.
    CreateMessengerChat,
    /// Create an audio source.
    CreateAudio,
    /// Create a gallery.
    CreateGallery,
    /// Create a snapshot camera.
    CreateSnapshot,
    /// Create or connect an NPC.
    CreateNPC,
    /// Create an input detector.
    CreateInput,

    // ── Special ──
    /// Mission metadata (not serialized to .code).
    Meta,
    /// Comment node (not serialized).
    Comment,
    /// Visual grouping box.
    Group,
}

/// Port data type for flow and data ports on nodes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum PortType {
    /// Execution flow (white). Controls the order of node execution.
    Flow,
    /// Numeric value (blue). Integer or floating-point.
    Number,
    /// String value (pink). Text data.
    String,
    /// Boolean value (red). True or false.
    Boolean,
    /// List value (yellow). Key-value collection (keys are strings).
    List,
    /// Object reference (green). Game objects like Thread, Area, NPC.
    Object,
    /// Any type (grey). Dynamic type that matches everything.
    Any,
}

impl PortType {
    /// Check whether a connection from `self` to `other` is type-compatible.
    ///
    /// Same types are always compatible. `Any` is compatible with every type.
    /// Other cross-type connections are disallowed (they require an explicit
    /// conversion node).
    pub fn is_compatible_with(&self, other: &PortType) -> bool {
        match (self, other) {
            (a, b) if a == b => true,
            (PortType::Any, _) | (_, PortType::Any) => true,
            _ => false,
        }
    }

    /// RGBA colour associated with this port type, for use in the UI.
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
    fn test_same_type_is_compatible() {
        assert!(PortType::Flow.is_compatible_with(&PortType::Flow));
        assert!(PortType::Number.is_compatible_with(&PortType::Number));
        assert!(PortType::String.is_compatible_with(&PortType::String));
        assert!(PortType::Boolean.is_compatible_with(&PortType::Boolean));
        assert!(PortType::List.is_compatible_with(&PortType::List));
        assert!(PortType::Object.is_compatible_with(&PortType::Object));
        assert!(PortType::Any.is_compatible_with(&PortType::Any));
    }

    #[test]
    fn test_any_is_compatible_with_all() {
        for other in &[
            PortType::Flow,
            PortType::Number,
            PortType::String,
            PortType::Boolean,
            PortType::List,
            PortType::Object,
        ] {
            assert!(PortType::Any.is_compatible_with(other));
            assert!(other.is_compatible_with(&PortType::Any));
        }
    }

    #[test]
    fn test_different_types_not_compatible() {
        assert!(!PortType::Number.is_compatible_with(&PortType::String));
        assert!(!PortType::String.is_compatible_with(&PortType::Boolean));
        assert!(!PortType::Boolean.is_compatible_with(&PortType::Number));
        assert!(!PortType::List.is_compatible_with(&PortType::Object));
        assert!(!PortType::Object.is_compatible_with(&PortType::List));
        assert!(!PortType::Flow.is_compatible_with(&PortType::Number));
        assert!(!PortType::Number.is_compatible_with(&PortType::Flow));
    }

    #[test]
    fn test_color_values() {
        assert_eq!(PortType::Flow.color(), [255, 255, 255, 255]);
        assert_eq!(PortType::Number.color(), [66, 165, 245, 255]);
        assert_eq!(PortType::String.color(), [244, 143, 177, 255]);
        assert_eq!(PortType::Boolean.color(), [239, 83, 80, 255]);
        assert_eq!(PortType::List.color(), [255, 202, 40, 255]);
        assert_eq!(PortType::Object.color(), [102, 187, 106, 255]);
        assert_eq!(PortType::Any.color(), [189, 189, 189, 255]);
    }

    #[test]
    fn test_port_type_serde_roundtrip() {
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
        }
    }

    #[test]
    fn test_node_type_count() {
        let json = serde_json::to_string(&NodeType::Start).expect("serialize failed");
        assert_eq!(json, "\"Start\"");
    }
}
