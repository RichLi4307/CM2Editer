use crate::api::definitions::ParamType;

/// 返回所有预定义的枚举常量集合。
///
/// 这些值来自 `docs/documentation_zh.html` 中的“游戏常量”和“Mod 常量”章节，
/// 以及 `docs/selected_cosplay.json` 中的角色扮演服装键。

/// 场景类型（StageType）
pub const STAGE_TYPES: &[&str] = &[
    "None", "Apart", "Convenience", "FashionShop", "Residence", "ShoppingMall",
    "StationFront", "Park", "Mansion", "TokyoStreet", "Suburbs", "Street", "City",
    "BarberShop", "Laundry", "Underpass",
];

/// 玩家动作（Action）
pub const ACTIONS: &[&str] = &[
    "None", "OldOnaniNormal", "OldGanimataWalk", "Pinpon", "ConbiniTakeGoods",
    "CrouchCry", "EatMedicine", "SadHandcuffAtMap", "SwitchTimeStop", "SwitchPistonMachine",
    "PickingCoat", "Pick", "Drop", "ChangeClothes", "DroppingClothes", "HandOver",
    "InsertAnalPlug", "ExtractAnalPlug", "CommonEquip", "IntoWasher", "TakeFromWasher",
    "UseBuyMachine", "DrinkWater", "PeeNormal", "TakeOffPants", "TakeOnPants", "TakeOffBra",
    "TakeOnBra", "Sad", "AttachHandcuffs", "PutHandcuffsOnMap", "HandcuffsAtMap",
    "UnlockHandcuffsAtMap", "AttachEyeMask", "SwitchVibrator", "PickUpItem", "SitDown",
    "StandUp", "PutDildoFloor", "PutDildoWall", "UseDildoFloorPussy1", "UseDildoFloorPussy2",
    "UseDildoFloorPussy3", "UseDildoFloorPussy4", "UseDildoFloorPussy5", "UseDildoFloorAnal1",
    "UseDildoFloorAnal2", "UseDildoFloorAnal3", "UseDildoFloorAnal4", "UseDildoFloorAnal5",
    "UseDildoFloorFella1", "UseDildoFloorFella2", "UseDildoFloorFella3", "UseDildoFloorFella4",
    "UseDildoFloorFella5", "UseDildoWallPussy1", "UseDildoWallPussy2", "UseDildoWallPussy3",
    "UseDildoWallPussy4", "UseDildoWallPussy5", "UseDildoWallAnal1", "UseDildoWallAnal2",
    "UseDildoWallAnal3", "UseDildoWallAnal4", "UseDildoWallAnal5", "UseDildoWallFella1",
    "UseDildoWallFella2", "UseDildoWallFella3", "UseDildoWallFella4", "UseDildoWallFella5",
    "UseDildoFloorWaitPussy", "UseDildoFloorWaitAnal", "UseDildoFloorWaitFella",
    "UseDildoWallWaitPussy", "UseDildoWallWaitAnal", "UseDildoWallWaitFella",
    "UseDildoFloorPussyEcstasyA", "UseDildoFloorAnalEcstasyA", "UseDildoFloorFellaEcstasyA",
    "UseDildoWallPussyEcstasyA", "UseDildoWallAnalEcstasyA", "UseDildoWallFellaEcstasyA",
    "PickDildo", "SitDildo", "SitDildoPut", "SitDildoPick", "SitDildoMoveAnal",
    "SitDildoMovePussy", "PickDildoWall", "GanimataWalk", "AhegaoDoublePiece", "HipShake",
    "GanimataHip", "KaikyakuFella", "Dogeza", "DogTintin", "IBalance", "WakimiseCrouch",
    "MituasiOnani", "Tebura", "PeeKaikyaku", "PeeDog", "ChikubiRotate", "OnaniYotuashi",
    "OnaniNeGanimata", "OnaniNormal", "NeKataashiage", "OnaniArmKuri", "OnaniSikoru",
    "GanimataKoshiHeko", "Haigure", "PeeStand", "DogezaUpHead", "PoseEnd", "SexStandBack",
    "SexKijoui",
];

/// 技能（Skill）
pub const SKILLS: &[&str] = &[
    "None", "Mental", "Stamina", "CoatLevel", "Flasher", "Raper", "ContinueMission", "Slow",
    "NpcDirect", "Sneak", "AutoSlow", "FixFps", "MaxAccessoryNum", "AutoBaretaSlow",
    "HideStrangeUi", "NoFastTravel", "Perspective", "MyPace", "TimeStop", "CantDash",
    "DisableHideCostume", "FixTps", "Exhibitionism", "Sex", "AutoAddMoisture",
    "NoReinforceEffect", "GanimataWalk", "AhegaoDoublePiece", "HipShake", "GanimataHip",
    "KaikyakuFella", "Dogeza", "DogTintin", "IBalance", "WakimiseCrouch", "MituasiOnani",
    "Tebura", "PeeKaikyaku", "PeeDog", "ChikubiRotate", "OnaniYotuashi", "OnaniNeGanimata",
    "OnaniNormal", "NeKataashiage", "OnaniArmKuri", "Sikoru", "GanimataKoshiHeko", "Haigure",
    "PeeStand",
];

/// 物品（Item）
pub const ITEMS: &[&str] = &[
    "None", "Water", "Dildo", "InvisiblePotion", "FutanariPotion", "FutanariInversePotion",
    "BodyPaint", "InvisibleInversePotion", "InvisiblePotionReusable", "BodyPaintReusable",
    "BodyPaintWasher", "HandcuffKey", "VibeRemocon", "DroneController", "DebugEarnRp",
    "DebugEarnMental", "DebugEarnRp2", "DebugEarnExp", "DebugEarnExp2",
];

/// 掉落物品类型（DropItemType）
pub const DROP_ITEM_TYPES: &[&str] = &[
    "None", "Coat", "Hoodie", "Basket", "Pants", "Bra", "HandcuffKey", "VibeRemocon",
    "DildoFloor", "DildoWall",
];

/// 成人玩具（AdultToy）
pub const ADULT_TOYS: &[&str] = &[
    "AnalPlug", "Vibrator", "EyeMask", "Handcuff", "KeyHandcuff", "TimerHandcuff", "TitRotor",
    "KuriRotor", "PistonFuta", "PistonAnal", "PistonPussy",
];

/// 手铐类型（HandcuffsType）
pub const HANDCUFFS_TYPES: &[&str] = &["Handcuff", "KeyHandcuff", "TimerHandcuff"];

/// 跳蛋强度（SetVibrator）
pub const VIBRATOR_STRENGTHS: &[&str] = &["Off", "Low", "High", "Random"];

/// 活塞强度（SetPiston）
pub const PISTON_STRENGTHS: &[&str] = &["Off", "Low", "Medium", "High", "Random"];

/// 性爱体位（SetSexPosition）
pub const SEX_POSITIONS: &[&str] = &["StandBack", "Kijoui"];

/// 图形选项（SetGraphicsOption）
pub const GRAPHICS_OPTIONS: &[&str] = &[
    "MotionBlur", "Brightness", "Contrast", "ColorTemperature", "BloomStrength",
    "BloomBiasDark", "BloomBiasLight", "CameraAngle1st", "CameraAngle3rd",
];

/// 音效名（PlaySoundEffect）
pub const SOUND_EFFECTS: &[&str] = &[
    "NoType", "FootStepHeel", "FootStepHeelJog", "ClothesSound", "ClothesDrop", "HeartBeat",
    "FootStepHeelCrouch", "Kuchu", "Shiofuki", "DokinHigh", "DokinLow", "Ok", "Close",
    "BuyMachine1", "BuyMachine2", "Drink", "PeeNormal", "Equip", "SleepBed", "Failed",
    "Picking", "Unlock", "AttachHandcuffs", "RankUp", "SuburbsDaytime", "SyburbsNight",
    "MyRoom", "KillTime", "BaibuLow", "BaibuHigh", "Barefoot", "FootStepSneaker", "BraAttach",
    "BraDetach", "PantsDetach", "VibratorSwitch", "Breath1", "Breath2", "SlowIn", "SlowOut",
    "Gogogo", "Reinforce", "CommonEquip", "ShoppingMallBgm", "Piston", "PutDildo",
    "CollectAll", "Buy", "UiSelect", "UiDecide", "UiCancel", "UiSlideOpen", "UiSlideClose",
    "OpenDoor", "CloseDoor", "Pinpon", "CarIdling", "CarStart", "CarStop", "CarPass",
    "BgmParkDayTime", "BgmParkNight", "FootStepGrass", "FootStepGravel", "ToiletDoorOpen",
    "ToiletDoorClose", "ToiletFutaOpen", "ToiletFutaClose", "FootStepMetal", "MentalAlert",
    "Drone", "ShopDoor", "ShoppingMallNight", "ElevatorMove", "ElevatorOpen", "ElevatorClose",
    "ElevatorBell", "Regi", "MansionPostReceiveOpen", "MansionPostReceiveClose",
    "ToiletPeeWater", "ToiletHandWash", "ToiletLock", "ToiletUnlock", "ToiletWater",
    "BgmFashionShop", "CurtainOpen", "CurtainClose", "WasherOpen", "WasherClose",
    "WasherStart", "WasherLoop", "WasherBeep", "Syasei", "TimeStop", "FootStepSneakerJog",
    "Rotor", "UICompleteMission",
];

/// 条件类型（CreateCondition / Condition 对象）
pub const CONDITION_TYPES: &[&str] = &[
    "CoatDropped", "CoatFrontClosed", "CoatFrontOpen1", "CoatFrontOpen2", "CoatBackClosed",
    "CoatBackOpen", "Blindfolded", "NoHandcuffs", "HandcuffsBack", "HandcuffsObject",
    "NormalHandcuffs", "KeyedHandcuffs", "TimedHandcuffs", "Moving", "Crouching", "Peeing",
    "InLight", "Sitting", "Dashing", "Orgasm", "Futanari", "Invisible", "InOpenToilet",
    "Bukkake", "NearNPC", "Watched", "ShowingOff", "VibrationOff", "VibrationLow",
    "VibrationHigh", "VibrationRandom", "PistonOff", "PistonLow", "PistonMedium", "PistonHigh",
    "PistonRandom", "IsDayTime", "NPCArea", "Bodypaint", "FPCamera", "GameOver",
];

/// 将字符串切片转换为 `Vec<String>`。
#[must_use]
pub fn to_strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|&s| s.to_string()).collect()
}

/// 判断某个参数类型是否应使用枚举下拉框。
#[must_use]
pub fn is_enum(param_type: ParamType) -> bool {
    matches!(param_type, ParamType::Enum)
}

/// 返回给定枚举集合的首个值作为默认值，空集合则返回空字符串。
#[must_use]
pub fn first_or_empty(values: &[&str]) -> String {
    values.first().map_or_else(String::new, |&s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage_types_count() {
        assert_eq!(STAGE_TYPES.len(), 16);
        assert!(STAGE_TYPES.contains(&"Residence"));
    }

    #[test]
    fn test_to_strings() {
        let strings = to_strings(&["a", "b"]);
        assert_eq!(strings, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn test_first_or_empty() {
        assert_eq!(first_or_empty(&["A", "B"]), "A");
        assert_eq!(first_or_empty(&[]), "");
    }
}
