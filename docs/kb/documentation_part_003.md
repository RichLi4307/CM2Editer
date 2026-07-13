<!-- source: docs\documentation.html -->
<!-- part: 3 -->
<!-- line_range: 580-2639 -->

### General

Log([Output])

returns: null

  * Writes to the console

Warning([Output])

returns: null

  * Writes a warning to the console

Error([Output])

returns: null

  * Writes an error to the console

Global(VariableName[, Value])

returns: value of global variable

  * Can be used to create a global variable from within a function or access one if it is hidden by a local one.

Local(VariableName[, Value])

returns: value of local variable

  * Probably only useful to create a local variable with the same name as one from a higher context.  
Otherwise you would always access the already existing one.

GetType(Value)

returns: String

GetMods()

returns: List

  * Returns a list with the GUIDs of installed BepInEx mods.

GetModVersion([ModGUID])

returns: List or null

  * Returns a list with "Major", "Minor" and "Build" entries representing the version number of a mod.
  * If no parameter is specified, return the version of Custom Mission.
  * Returns null if a name is specified, but the mod is not installed.

DumpVariables([RecursionCount])

returns: null

  * Prints variables from the current scope and its parents to the console
  * Use ResursionCount to further output the contents of lists recursively

DumpVariable(Variable[, RecursionCount])

returns: null

  * Prints a single variable to the console
  * Mostly useful for lists if you also use RecursionCount
  * Use ResursionCount to further output the contents of lists recursively

CallFunction(function = FunctionName[, parameters = Parameters])
CallFunction(FunctionName[, Parameters]) CallFunction(FunctionReference)

returns: Value

  * Call a functions by passing its name as a string or a list containing the relevant data.

Examples:

    
    
    fn_parameters = CreateList(a = 2, b = 3)
    fn_names = CreateList("fn_add","fn_mult","fn_power",length=3)
    i = 0
    while i<fn_names.length
    	Log(CallFunction(fn_names[i], fn_parameters))
    	i = i + 1
    
    fn_add:
    	_result = a+b
    fn_mult:
    	_result = a*b
    fn_power:
    	_result = a**b
    
    
    fn_ref = CreateList(function="DropItem", parameters=CreateList(itemtype="Coat", stage="Residence", x=-26.60, y=-0.10, z=-120))
    CallFunction(fn_ref)
    does the same as
    DropItem(itemtype="Coat", stage="Residence", x=-26.60, y=-0.10, z=-120)

CallMethod(thread = Thread, method = MethodName[, parameters = Parameters])
CallMethod(Thread, MethodName[, Parameters]) CallMethod(MethodReference)

returns: Value

  * Call a method by passing a thread object and the method name as a string or a list containing the relevant data.

FunctionExists(FunctionName)

returns: Boolean

  * Check if a function with specific name exists. Because new functions can be created by other mods this is not trivial to answer.
  * By checking you can prevent crashing your mission when you call a non-existing function.
  * Only works for functions defined by the engine or extensions, not those defined in scripts.

Color(Red, Green, Blue[, Alpha])

returns: List

  * Creates a list with r, g, b, a items

Range(Stop) Range(Start, Stop[, Step])

returns: List

  * Creates a list of numbered items from Start to Stop with increments of Step
  * **Stop value is not included**
  * If not specified, default Start value is 0
  * If not specified, default Step value is 1

SetEvent(EventName[, Value])

returns: null

  * Sets an event that can be accessed by any project with GetEvent.
  * The event is only valid for the next frame.
  * You can pass data by setting the second parameter, including lists, but **excluding** objects.

GetEvent(EventName)

returns: List or null

  * Checks if an event has been set (in the previous frame).
  * Returns a numbered list of the data that has been passed by SetEvent. May also include null values if no data was passed.
  * If the event was not set, function returns null

### Game Functions

GetLanguage()

returns: String

  * Return value will be one of these depending on your settings: En, Ja, Ko, Sc, Tc

Translate(Key[, Param1][, Param2]...)

returns: String

  * Translates a string using the games Localizer class.
  * Only predefined strings can be localized. (mainly used for UI elements, items and missions)
  * List of strings and their English localization: 
        
        key => en
        item_peeing_not_water => You can't drink while peeing.
        item_cant_use_drone => You can't use it here.
        item_not_enough_rank_drone => I don't have the guts to use it now...
        item_cant_use_dildo_escalator => You can't place it on the escalator.
        item_get_water => Get a bottle of water.
        item_already_active_futanari => It's already grown.
        item_already_inactive_futanari => It's already gone.
        item_already_active_invisible => It's already transparent.
        item_already_inactive_invisible => It's already unlocked.
        item_handcuff_timer_remain => The timer is still running ({0} minutes left).
        item_handcuff_timer_end => The handcuff timer has ended.
        difficulty_casual => Casual
        difficulty_easy => Easy
        difficulty_original => Original
        vibe_off => Vibrator: Off
        vibe_low => Vibrator: Low
        vibe_high => Vibrator: High
        vibe_random => Vibrator: Random
        try_fashion_cant => You must take off your clothes to try it on.
        dropitem_override_text_coat => Coat
        dropitem_override_text_hoodie => Hoodie
        dropitem_override_text_pants => Put on panties
        dropitem_override_text_bra => Put on bra
        dropitem_override_text_handcuff_key => Pick up handcuff key
        dropitem_override_text_vibe_remocon => Pick up vibrator switch
        rp_buff_skill_prefix => Skill:
        rp_buff_expose => Flashing
        rp_buff_near_npc => Someone nearby
        rp_buff_light => In the light
        rp_buff_away_clothes => Away from clothes
        rp_buff_daytime => Daytime
        rp_buff_peeing => Peeing
        rp_buff_fear3 => Condition: Excellent
        rp_buff_fear4 => Condition: Good
        rp_buff_fear5 => Condition: Normal
        rp_buff_washer => Washing machine {0} min
        rp_buff_invisible => Invisibility
        rp_buff_timer_handcuff => Timer handcuffs {0} min
        rp_buff_random_mission => Random mission
        rp_buff_bareyasusa => Detection risk
        rp_buff_bad_visible => Restricted Vision
        rp_buff_handcuff => Cuffed (Behind Back)
        rp_buff_handcuff_at_map => Cuffed (To Object in Front)
        rp_buff_body_paint => Body Paint
        rp_buff_in_npc_sight => Being Watched
        rp_buff_continuous_bonus => Combo Clear Bonus: {0}
        rp_buff_difficulty => Difficulty: {0}
        rp_buff_ecstasyMotion => Climaxing
        ui_common_coat => Coat
        log_limit_mental => Mental Stamina is about to run out
        log_not_equip_keyhandcuff => Locked handcuffs are not on.
        log_not_have_handcuff_key => You don’t have a handcuff key
        log_not_have_vibe_remocon => You don’t have a vibrator remote
        log_not_rank_underwear => Not brave enough to take off your underwear…
        log_already_wear_clothes => You’re already dressed
        log_adjust_breast_size => Boobs size adjusted to match the costume
        log_cant_expose_by_daytime => Not brave enough to Flash in daylight
        log_cant_expose_by_mental => Not enough Mental Stamina
        log_cant_expose_by_crouch => Can’t undress while crouching
        log_cant_use_hand_by_handcuff => Unable to act: arms are restrained.
        log_cant_use_hand_by_crouch => Can’t do that while crouching
        log_cant_use_hand_by_other => Can’t do that now
        log_handcuff_success_unlock => Unlocked successfully
        log_handcuff_failed_unlock => Failed to unlock
        log_portal_cant_move_washer => Can’t move while clothes are in the washing machine
        log_portal_cant_move_rank => Not brave enough to go to {0} yet
        log_portal_cant_move_naked_to_home => You can't go home unless you wear the coat.
        log_portal_cant_move_naked => You can’t go out unless you wear the coat.
        log_portal_cant_move_drone => Can’t move during a Drone Mission
        log_portal_cant_move_timestop => Can’t move during Time Stop
        log_portal_cant_move_random_mission => Can’t move during a Random Mission
        log_use_npc => Someone is using it
        log_door_open => Open
        log_door_lock => Lock
        log_door_lock_complete => Locked
        log_door_unlock_complete => Unlocked
        log_door_locked => It’s locked
        log_not_have_product => You don’t have the item
        log_washer_no_rank_naked => Not brave enough to undress
        log_washer_cant_use_naked => Cannot use unless dressed
        log_washer_remain_time => Washing machine running ({0} min left)
        log_cant_by_expose_front => Must flash your front to do this
        log_cant_by_not_futanari => Can’t do unless futanari
        log_body_paint_first => Body paint is starting to fade
        log_body_paint_second => Body paint has faded considerably
        log_body_paint_lost => Body paint has completely faded
        log_not_active_time_stop => Time Stop skill is disabled
        log_cant_equip_by_not_futanari => Can’t equip unless futanari
        log_disappeared_invisible => Invisibility has worn off
        log_not_have_piston_machine => You don’t have a Piston Machine
        piston_off => Piston Machine: Off
        piston_low => Piston Machine: Low
        piston_middle => Piston Machine: Medium
        piston_high => Piston Machine: High
        piston_random => Piston Machine: Random
        log_not_have_dildo => You don't have a dildo.
        ui_common_change => Change
        ui_common_empty => Empty
        ui_common_cancel => Cancel
        ui_common_yes => Yes
        ui_common_no => No
        ui_common_times => {0} times
        ui_common_confirm => Confirm
        ui_common_ok => OK
        ui_common_decide => Confirm
        ui_achievement_release_costume => Costume shop unlocked
        ui_achievement_release_item => Item shop unlocked
        ui_achievement_highscore => High Score
        ui_achievement_achieve_rate => Completion Rate
        ui_achievement_received => Received
        ui_achievement_receive => Receive
        ui_achievement_released_shop => is now available in the shop
        ui_achievement_released_rp => Obtained {0} RP
        ui_buff_kind_rp_bonus => RP Multiplier
        ui_buff_kind_rp_bonusLastMult => RP Multiplier (Multiplicative)
        ui_buff_kind_rp_bonus_reinforce => (RP Multiplier from equipment upgrades)
        ui_buff_kind_bareyasusa => Detection Risk
        ui_buff_kind_heart_rate => Heart Rate
        ui_buff_kind_max_stamina => Max Stamina
        ui_buff_kind_rise_ecstasy => Ecstasy gauge gain
        ui_buff_kind_rise_mission_gauge => Mission Gauge Gain
        ui_buff_kind_consume_slow_gauge => Slow Gauge Consumption
        ui_buff_kind_reduce_cooltime => Skill Cooldown Reduction
        ui_buff_kind_gameover_penalty_rp => RP lost on Flash failure
        ui_buff_kind_gameover_penalty_rank => Rank EXP lost on Flash failure
        ui_buff_kind_mental_decrease_scale => Mental Consumption
        ui_buff_kind_unlockingSpeed => Handcuff Unlock Speed
        ui_save_complete => Saved
        ui_choose_handcuff_timer_bonus => RP Multiplier +{0}%
        ui_parts_option_setup_title => Customization: {0}
        ui_release_at_rank => Unlocks at Rank {0}
        ui_random => Random
        ui_random_mission => Random Mission
        ui_buff_kind_dash_speed => Dash Speed
        ui_earn_rp => RP Earned
        ui_bring_rp => RP Held
        ui_cant_equip_in_slow => Cannot change equipment while slowed
        ui_buff_kind_stamina_regenerate => Stamina Recovery Speed
        ui_rp_bonus => RP Mult.
        ui_ingame_manual_open_coat => Open Coat: {0}
        ui_ingame_manual_close_coat => Wear Coat: {0}
        ui_ingame_manual_open_hip => Flash Butt: {0}
        ui_ingame_manual_dash => Run: {0}
        ui_ingame_manual_crouch => Crouch: {0}
        ui_ingame_manual_menu => Menu: {0}
        ui_ingame_manual_system_menu => System Menu: {0}
        ui_dildo_limit => You can't place any more.
        ui_dildo_cant_put => There is no place to place it.
        ui_loadout_change => Your current customization will be lost.<n>Proceed?
        ui_loadout_update => Overwrite Loadout {0} with the current customization.<n>Are you sure?
        ui_customize_hair => Hairstyle
        ui_customize_reset_body => Initialize body settings.<n>Are you sure?
        ui_customize_reset_face => Initialize face settings.<n>Are you sure?
        ui_drone_cant_hide_costume => Cannot start while wearing casual clothes.
        ui_drone_cant_active_other => Another drone mission is in progress.
        ui_drone_cant_random_mission => Action unavailable during random mission.
        ui_drone_cant_condition => Preconditions not fulfilled.
        ui_drone_interrupt_confirm => Abort drone mission.<n>Are you sure?
        ui_not_enough_rank => Insufficient Flasher Rank
        ui_cant_fast_travel_strangeness => Cannot proceed because someone is suspicious of you.
        ui_cant_fast_travel_expose => Cannot fast travel while flashing
        ui_interact_curtain => Curtain
        ui_interact_door => Door
        ui_interact_coat => Coat
        ui_interact_washer => Washing Machine
        ui_interact_buymachine => Vending Machine
        ui_interact_closet => Closet
        ui_interact_pc => PC
        ui_interact_sit => Sit
        ui_interact_push => Push
        ui_interact_dildo => Dildo
        ui_interact_toilet => Toilet
        ui_interact_pinpon => Press Intercom
        ui_interact_take_goods => Take Item
        ui_interact_put_goods => Take Out Item
        ui_mission_rp_per_second => RP/sec
        ui_mission_achieve => Done
        ui_mission_complete => Complete
        ui_ring_menu_common_auto_run => Auto Run
        ui_ring_menu_common_chase => Chase
        m_item_dildo => Dildo
        m_item_vibe_switch => Vibrator Switch
        m_adult_goods_handcuff => Handcuffs
        ui_ring_menu_common_piston_switch => Piston Machine Switch
        m_skill_time_stop => Time Stop
        ui_ring_menu_change_1 => Ring Menu 1 has changed.
        ui_ring_sit_dildo_onani => Dildo Masturbation
        ui_ring_sit_put_dildo => Place Dildo
        ui_ring_sit_take_dildo => Take Dildo
        ui_ring_sit_move_dildo_pussy => Shift the dildo toward the vagina
        ui_ring_sit_move_dildo_anal => Shift the dildo toward the anus
        ui_ring_sit_dildo_dont_bring => You don't have a dildo.
        ui_ring_sit_not_put_dildo => No dildo is placed.
        ui_ring_sit_already_put_dildo => A dildo is already placed.
        ui_ring_sit_already_move_dildo => It has already been shifted.
        ui_ring_sit_cant_doing_onani => You can't do that while masturbating.
        ui_kill_time_cant_daytime => No guts to go exposed in the daytime.
        ui_kill_time_cant_area => You can't wait in this area.
        ui_kill_time_cant_elevator => You can't wait inside the elevator.
        ui_kill_time_cant_escalator => You can't wait on the escalator.
        ui_kill_time_cant_time_stop => You can't do that while time is stopped.
        ui_kill_time_cant_random_mission => You can't do that during a random mission.
        ui_kill_time_to_night => Kill time until night?
        ui_kill_time_to_daytime => Kill time until daytime?
        ui_kill_time_expose => <n>Since you're exposed, you need to wait for 10 seconds<n>without being seen by anyone  
        ui_kill_time_dildo_delete => <n>All dildos outside your home will be discarded.
        ui_fast_travel_cant_near_portal => You can only fast-travel near portals.
        ui_fast_travel_cant_drone_mission => You can't do that during a drone mission.
        ui_fast_travel_cant_skill => Fast travel is disabled by a skill.
        ui_interact_toilet_sit => Sit
        ui_interact_toilet_futa => Lid
        ui_interact_toilet_flush => Flush
        ui_interact_toilet_ride => Stand on Top
        ui_interact_dildo_pussy => Insert into vagina
        ui_interact_dildo_anal => Insert into anus
        ui_interact_dildo_mouth => Insert into mouth
        ui_interact_dildo_take => Retrieve
        ui_interact_close => Close
        ui_interact_open => Open
        ui_minutes_1 => 1 min
        ui_minutes_3 => 3 min
        ui_minutes_5 => 5 min
        ui_equip_effect => Equipment Effect
        ui_discomfort => Unease
        ui_reinforce_effect => Enhancement Effect
        ui_loadout_name => My Set
        ui_drone_cant_take_hide_costume => You can't wear casual clothes during a drone mission.
        ui_cant_take_hide_costume_near_coat => You need to be near the coat to wear casual clothes.
        ui_common_accessory => Accessory
        ui_common_costume => Costume
        ui_loadout_update_confirm => The current costume equipment status will overwrite My Set {0}. <n> Are you sure?
        ui_inventory_equip_num => Equipment slots:
        ui_inventory_over_equip_num => Equipment limit reached.
        ui_kill_time_cancel => Waiting was interrupted because someone saw you.
        ui_save_update_confirm => Overwrite?
        ui_mission_failed_rank_down => Rank dropped to {0}
        ui_mission_failed_lost_exp => Lost {0} Exp
        ui_mission_failed_earn_rp => Lost {0} earned RP
        ui_mission_failed_correct_drop_item => Retrieved items outside.
        ui_mission_panel_achieve_rate => Mission Completion: {0}%
        ui_option_reset_fov => Reset field of view to default.<n>Are you sure?
        ui_option_all_reset => Restore everything to initial state.<n>Are you sure?
        ui_option_reset_key_config => Reset all keys to default.<n>Are you sure?
        ui_option_save_change => Save changes?
        ui_option_mall_npc => Daytime Shopping Mall NPC multiplier
        ui_option_mall_npc_hide => Daytime ??? NPC multiplier
        ui_option_reset_appear_rate => Reset spawn ratios to default.<n>Are you sure?
        ui_option_reset_confirm => Reset to default.<n>Are you sure?
        ui_common_not_enough_rp => Not enough RP
        ui_shop_over_item => Carry limit reached
        ui_shop_purchased => Purchased {0}
        ui_reinforce_no_effect => No effect
        ui_reinforce_can_by_rank => Upgradable at rank {0}
        ui_reinforce_result_title => Upgrade Result:
        ui_common_rp_bonus_plus_percent => RP multiplier +{0}%
        ui_common_rp_bonus_percent => RP multiplier {0}%
        ui_pc_menu_no_passive => No passive skills learned
        ui_pc_menu_collect_all => Collect all placed objects.<n>Are you sure?
        ui_rank_confirm_need_exp => Required EXP: {0}
        ui_common_drone_mission => Drone Mission
        ui_common_roshutsu_rank => Flasher Rank: {0}
        ui_load_empty_data => Empty Slot
        ui_load_broken_data => Corrupted Data
        ui_load_fullVer_data => Full Version Data
        ui_load_play_time => Play Time: {0}
        ui_difficulty_description_casual => For those who want to play easier, <n> the speed at which exposure is noticed decreases <n> and you won’t lose anything even if you get a GAME OVER.
        ui_difficulty_description_easy => For those who are not good at games, <n> the speed at which exposure is noticed decreases. <n>Stamina increases and recovers faster.
        ui_difficulty_description_original => For those seeking real tension<n>Experience the game’s intended difficulty
        ui_skill_release_by_learn => Unlocks after learning {0}
        ui_common_learn => Learn
        ui_common_reinforce => Upgrade
        ui_skill_passive_extra_description => <n><n>Passive Skill: can be toggled on/off at the home PC.
        ui_sleep_black_sleep => Sleeping...
        ui_sleep_black_wait => Waiting...
        ui_system_menu_go_title_description => Return to title without saving<n>Are you sure?
        ui_tutorial_how_to_play => How to Play
        m_stage_apart => Home
        ui_tutorial_manual => Controls
        ui_tutorial_manual_pad => Controls (Gamepad)
        ui_tutorial_manual_oneHanded => Controls (One-Handed)
        ui_tutorial_rule => Rules
        ui_tutorial_discomfort => Unease
        ui_tutorial_reinforce => Equipment Upgrade
        ui_tutorial_pee => Urination
        ui_tutorial_fast_travel => Fast Travel
        ui_tutorial_vibe => Vibrator Items
        ui_tutorial_handcuff => Handcuff Items
        ui_tutorial_bodypaint => Body Paint
        ui_tutorial_piston => Piston Machine
        ui_tutorial_hide_costume => Casual Wear
        ui_tutorial_invisible => Invisibility
        ui_mission_area_crossing => Crosswalk
        ui_mission_area_entrance => Entrance
        ui_mission_area_regi => Checkout Counter
        ui_mission_area_park_central => Fountain Plaza
        ui_mission_area_park_maze => Maze
        ui_mission_area_bridge => Footbridge
        ui_drone_complete => Drone Mission Completed
        ui_random_complete => Random Mission Completed
        ui_fast_travel_add => New Fast Travel location added
        m_achievement_all_mission => Cleared all missions and drone missions
        m_achievement_high_score => Achieved a high score of {0} RP or more
        m_achievement_stage => Cleared all missions in {0}
        m_achievement_stage_percent => Clear {1}% of {0} missions
        m_adult_goods_eye_mask => Eye Mask
        m_adult_goods_handcuff_timer => Timed Handcuffs
        m_adult_goods_handcuff_key => Keyed Handcuffs
        m_adult_goods_piston_anal => Anal Piston Machine
        m_adult_goods_piston_futa => Cock Piston Machine
        m_adult_goods_piston_pussy => Vaginal Piston Machine
        m_adult_goods_vibe => Vibrator
        m_adult_goods_rotor => Nipple Rotor
        m_adult_goods_rotor_kuri => Clit Rotor
        m_adult_goods_description_eye_mask => Wearing it narrows your field of view
        m_adult_goods_description_handcuff => Wearing it restricts most actions. Unlocking takes some time.
        m_adult_goods_description_handcuff_timer => Handcuffs that cannot be unlocked until the set time elapses; returning home ends the timer. Mission conditions are met only while time remains.
        m_adult_goods_description_handcuff_key => Handcuffs that can be unlocked if you have the handcuff key; does not satisfy mission conditions.
        m_adult_goods_description_piston => Operable via piston machine switch
        m_adult_goods_description_vibe => Operable via vibrator switch
        m_bodypaint_oil => Body Oil
        m_bodypaint_inmon => Lewd Mark
        m_bodypaint_rakugaki => Graffiti
        m_bodypaint_tatoo_dark => Tattoo (Dark)
        m_bodypaint_tatoo_flower1 => Tattoo (Flower 1)
        m_bodypaint_tatoo_flower2 => Tattoo (Flower 2)
        m_bodypaint_tatoo_lotus => Tattoo (Lotus)
        m_bodypaint_tatoo_satanism => Tattoo (Demon)
        m_bodypaint_tatoo_slave => Tattoo (Slave)
        m_cosplay_category_bunny => Bunny
        m_cosplay_category_sister => Sister
        m_cosplay_category_cow => Cowgirl
        m_cosplay_category_general_genital => General (Genitals)
        m_cosplay_category_general_head => General (Head)
        m_cosplay_category_general_leg => General (Feet)
        m_cosplay_category_general_upper => General (Upper Body)
        m_cosplay_category_glossy => Down Jacket Set
        m_cosplay_category_jump_suit => Jumpsuit
        m_cosplay_category_latex_pony => Latex Pony
        m_cosplay_category_jailer => Guard
        m_cosplay_category_kemono => Furry Cosplay
        m_cosplay_category_ninja => Ninja
        m_cosplay_category_school_gal => School Uniform B
        m_cosplay_category_school_hoodie => School Uniform A
        m_cosplay_category_succubus_cosplay => Succubus Cosplay
        m_cosplay_category_suit_chic => Suit A
        m_cosplay_category_suit_luxe => Suit B
        m_cosplay_category_sweater => Adult Sweater
        m_cosplay_category_maid => Tactical Maid
        m_cosplay_category_wild_beast => Wild Beast
        m_cosplay_bunny_ear => Bunny Head
        m_cosplay_bunny_harness_top => Top Harness
        m_cosplay_bunny_harness_under => Under Harness
        m_cosplay_bunny_jacket => Jacket
        m_cosplay_bunny_nippless => Nipple Pasties
        m_cosplay_bunny_shoes => Shoes
        m_cosplay_bunny_sockes => Socks
        m_cosplay_bunny_tops => Top
        m_cosplay_sister_arm => Arm Frills
        m_cosplay_sister_curtain => Chest Curtain
        m_cosplay_sister_cloth => Top
        m_cosplay_sister_face => Face Veil
        m_cosplay_sister_head => Veil
        m_cosplay_sister_leotard => Leotard
        m_cosplay_sister_pants => Panties
        m_cosplay_sister_shoes => Shoes
        m_cosplay_sister_shoulder => Shoulder Frills
        m_cosplay_sister_collar => Neck Scarf
        m_cosplay_sister_socks => Socks
        m_cosplay_cow_boots => Cow Boots
        m_cosplay_cow_bra => Cow Bra
        m_cosplay_cow_choker => Bell Collar
        m_cosplay_cow_glove => Cow Gloves
        m_cosplay_cow_hair_band => Cow Hairband
        m_cosplay_cow_pants => Cow Panties
        m_cosplay_cow_tail => Cow Tail Plug
        m_cosplay_general_genital_bansoukou => Bandage (Genitals)
        m_cosplay_general_genital_gentle_hand => Gentleman’s Hand (Genitals)
        m_cosplay_general_genital_ninjin => Carrot
        m_cosplay_general_genital_ofuda => Ofuda (Genitals)
        m_cosplay_general_genital_beads => Anal Beads
        m_cosplay_general_genital_anal_plug => Anal Plug
        m_cosplay_general_genital_plug_light => Light-Up Anal Plug
        m_cosplay_general_genital_maebari => Crotch Patch
        m_cosplay_general_genital_tail_plug => Tail Plug
        m_cosplay_general_genital_rubbit_tail => Bunny Tail Plug
        m_cosplay_general_head_eye_censor => Black Censor Bar
        m_cosplay_general_head_houtai => Bandage Blindfold
        m_cosplay_general_head_glasses => Glasses
        m_cosplay_general_head_sun_glasses => Sunglasses
        m_cosplay_general_head_kuchikase => Gag
        m_cosplay_general_head_mask => Mask
        m_cosplay_general_head_mekakushi => Blindfold
        m_cosplay_general_head_osyaburi => Pacifier
        m_cosplay_general_head_police_hat => Police Hat
        m_cosplay_general_leg_high_heel_boots => High-Heeled Boots
        m_cosplay_general_leg_long_boots => Long Boots
        m_cosplay_general_leg_boots => Boots
        m_cosplay_general_leg_garter => Garter Belt
        m_cosplay_general_leg_heel => Heels
        m_cosplay_general_leg_stocking => Stockings
        m_cosplay_general_leg_stocking_net => Fishnet Stockings
        m_cosplay_general_leg_stocking_net_dark => Fishnet Stockings (Dark)
        m_cosplay_general_leg_stocking_net_thin => Fine Fishnet Stockings
        m_cosplay_general_leg_stocking_net_dark_thin => Fine Fishnet Stockings (Dark)
        m_cosplay_general_leg_stocking_nieso => Knee-high Socks
        m_cosplay_general_leg_stocking_race => Lace Stockings
        m_cosplay_general_leg_stocking_race_dark => Lace Stockings (Dark)
        m_cosplay_general_leg_stocking_stripe => Striped Stockings
        m_cosplay_general_leg_stocking_pin_heel => Stiletto Heels
        m_cosplay_general_leg_stocking_rain_boots => Rain Boots
        m_cosplay_general_leg_stocking_sneaker => Sneakers
        m_cosplay_general_upper_kubiwa_seisyori => Collar (Sex-use)
        m_cosplay_general_upper_bansoukou => Bandage (Nipples)
        m_cosplay_general_upper_body_net => Fishnet Shirt
        m_cosplay_general_upper_bra => Bra
        m_cosplay_general_upper_chack_bikini_bra => Open-Cup Bikini Bra
        m_cosplay_general_upper_chack_bikini_pants => Open-Crotch Bikini Bottoms
        m_cosplay_general_upper_condom => Condom
        m_cosplay_general_upper_cross_piasu => Cross Nipple Piercings
        m_cosplay_general_upper_full_glove => Full Gloves
        m_cosplay_general_upper_gentle_hand => Gentleman’s Hand (Chest)
        m_cosplay_general_upper_glove => Gloves
        m_cosplay_general_upper_harness => Harness
        m_cosplay_general_upper_kanban => Pervert Sign
        m_cosplay_general_upper_kikkou => Kikkou Shibari
        m_cosplay_general_upper_kubiwa => Collar
        m_cosplay_general_upper_ultimate_pants => Panties
        m_cosplay_general_upper_mufflar => Scarf
        m_cosplay_general_upper_necktie => Necktie
        m_cosplay_general_upper_glove_net => Fishnet Gloves
        m_cosplay_general_upper_nip_piasu => Nipple Piercings
        m_cosplay_general_upper_nippless => Nipple Pasties
        m_cosplay_general_upper_ofuda => Ofuda (Nipples)
        m_cosplay_general_upper_far => Fur
        m_cosplay_general_upper_long_leather_glove => Long Leather Gloves
        m_cosplay_general_upper_race_pants => Lace Panties
        m_cosplay_general_upper_rain_coat => Raincoat
        m_cosplay_general_upper_randoseru => Randoseru
        m_cosplay_general_upper_sukumizu => School Swimsuit
        m_cosplay_general_upper_swimsuit => Competitive Swimsuit
        m_cosplay_general_upper_micro_bikini_bra => Micro Bikini Bra
        m_cosplay_general_upper_micro_bikini_pants => Micro Bikini Bottoms
        m_cosplay_general_upper_rubber_suit => Rubber Suit
        m_cosplay_general_upper_sling_shot => Slingshot Bikini
        m_cosplay_glossy_boots => Long Boots
        m_cosplay_glossy_jacket => Down Jacket
        m_cosplay_glossy_knit => Knit Sweater
        m_cosplay_jump_suit_boots => Boots
        m_cosplay_jump_suit_suit => Suit
        m_cosplay_latex_pony_hip_belt => Hip Belt
        m_cosplay_latex_pony_boots => Boots
        m_cosplay_latex_pony_colllar => Collar
        m_cosplay_latex_pony_ear => Ears
        m_cosplay_latex_pony_full_face => Full-Face Mask
        m_cosplay_latex_pony_glove => Gloves
        m_cosplay_latex_pony_head => Headgear
        m_cosplay_latex_pony_nippless => Nipple Pasties
        m_cosplay_latex_pony_tail => Tail
        m_cosplay_latex_pony_tops => Top
        m_cosplay_jailer_boots => Boots
        m_cosplay_jailer_cap => Cap
        m_cosplay_jailer_reotard => Leotard
        m_cosplay_jailer_pants => Shorts
        m_cosplay_jailer_jacket => Jacket
        m_cosplay_jailer_bag => Leg Bag
        m_cosplay_kemono_ear_cat => Cat Ears
        m_cosplay_kemono_ear_fox => Fox Ears
        m_cosplay_kemono_foot => Shoes
        m_cosplay_kemono_hand => Gloves
        m_cosplay_kemono_tail => Tail
        m_cosplay_ninja_boots => Boots
        m_cosplay_ninja_mask => Mask
        m_cosplay_ninja_omen => Face Mask
        m_cosplay_ninja_pants => Panties
        m_cosplay_ninja_sleeve => Sleeves
        m_cosplay_ninja_tops => Top
        m_cosplay_school_gal_bag => Bag
        m_cosplay_school_gal_bracelet => Bracelet
        m_cosplay_school_gal_earing => Earrings
        m_cosplay_school_gal_necklace => Necklace
        m_cosplay_school_gal_ribbon => Ribbon
        m_cosplay_school_gal_shoes => Shoes
        m_cosplay_school_gal_skirt => Skirt
        m_cosplay_school_gal_sockes => Socks
        m_cosplay_school_gal_cardigan_summer => Summer Cardigan
        m_cosplay_school_gal_blazer => Blazer
        m_cosplay_school_gal_cardigan_winter => Winter Cardigan
        m_cosplay_school_gal_shirt_winter => Winter Shirt
        m_cosplay_school_gal_necktie => Necktie
        m_cosplay_school_gal_shirt_summer => Summer Shirt
        m_cosplay_school_hoodie_bag => Backpack
        m_cosplay_school_hoodie_hoodie => Hoodie
        m_cosplay_school_hoodie_ribbon => Ribbon
        m_cosplay_school_hoodie_shirt => Shirt
        m_cosplay_school_hoodie_shoes => Shoes
        m_cosplay_school_hoodie_skirt => Skirt
        m_cosplay_school_hoodie_socks => Socks
        m_cosplay_school_hoodie_necktie => Necktie
        m_cosplay_succubus_cosplay_horn => Horns
        m_cosplay_succubus_cosplay_pants => Panties
        m_cosplay_succubus_cosplay_skirt => Skirt
        m_cosplay_succubus_cosplay_tail => Tail
        m_cosplay_succubus_cosplay_tops => Top
        m_cosplay_succubus_cosplay_wing => Wings
        m_cosplay_suit_chic_card => Card
        m_cosplay_suit_chic_jacket => Jacket
        m_cosplay_suit_chic_pants => Suit Pants
        m_cosplay_suit_chic_panty => Panties
        m_cosplay_suit_chic_shirt => Shirt
        m_cosplay_suit_chic_shoes => Shoes
        m_cosplay_suit_chic_skirt => Skirt
        m_cosplay_suit_chic_stocking => Stockings
        m_cosplay_suit_luxe_chorker => Choker
        m_cosplay_suit_luxe_jacket => Jacket
        m_cosplay_suit_luxe_pants => Suit Pants
        m_cosplay_suit_luxe_shoes => Shoes
        m_cosplay_suit_luxe_tops => Top
        m_cosplay_sweater_arm => Arm Warmers
        m_cosplay_sweater_leg => Leg Warmers
        m_cosplay_sweater_pants => Panties
        m_cosplay_sweater_tops => Top
        m_cosplay_maid_ankle_belt => Ankle Belt
        m_cosplay_maid_apron => Apron
        m_cosplay_maid_harness_belt => Harness Belt
        m_cosplay_maid_choker => Choker
        m_cosplay_maid_bag => Leg Bag
        m_cosplay_maid_shoes => Loafers
        m_cosplay_maid_onepiece => One-piece Dress
        m_cosplay_maid_ribbon => Ribbon
        m_cosplay_maid_sleeve => Sleeves
        m_cosplay_maid_stocking => Stockings
        m_cosplay_wild_belt => Belt
        m_cosplay_wild_foot_mohu => Ankle Fluff
        m_cosplay_wild_hand_mohu => Wrist Fluff
        m_cosplay_wild_neck_mohu => Neck Fluff
        m_cosplay_wild_nippless_mohu => Nipple Fluff Pasties
        m_cosplay_wild_pants => Hot Pants
        m_cosplay_wild_upper_leg_mohu => Thigh Fluff
        m_drone_mission_blind_title => Pick up the handcuff key while flashing with a blindfold on!
        m_drone_mission_blind_description => The handcuff key is placed somewhere on the same map. Go pick it up while blindfolded. Its direction is shown on your compass.
        m_drone_mission_blind_condition1 => You have Keyed Handcuffs and the handcuff key, and you are not cuffed.
        m_drone_mission_blind_condition2 => You have an eye mask, and the X-Ray skill is OFF.
        m_drone_mission_blind_condition3 => You are flashing.
        m_drone_mission_blind_goal => Pick up the handcuff key.
        m_drone_mission_coat_title => Pick up the coat!
        m_drone_mission_coat_description => The coat is placed somewhere on the same map. Go get it while naked. Its direction is shown on your compass.
        m_drone_mission_coat_condition1 => You are wearing the coat.
        m_drone_mission_coat_goal => Put on the coat.
        m_drone_mission_coat_randomstart_title => Pick up the coat from a random starting position!
        m_drone_mission_coat_randomstart_description => The coat is placed somewhere on the same map. You will first be moved to a random spot. Its direction is shown on your compass.
        m_drone_mission_coat_find_title => Find the coat!
        m_drone_mission_coat_find_description => The coat is placed somewhere on the same map. Search for it while naked. No direction is shown.
        m_drone_mission_coat_find_randomstart_title => Find the coat starting from a random position!
        m_drone_mission_coat_find_randomstart_description => The coat is placed somewhere on the same map. You will first be moved to a random spot. No direction is shown.
        m_drone_mission_vibe_title => Pick up the coat while wearing a vibrator!
        m_drone_mission_vibe_description => Both the coat and the vibrator remote are placed on the same map. Go look for the coat. You may pick up the remote and turn the vibrator OFF. Its direction is shown on your compass.
        m_drone_mission_vibe_condition1 => You have a vibrator and its remote.
        m_drone_mission_vibe_find_title => Find the coat while wearing a vibrator!
        m_drone_mission_vibe_find_description => Both the coat and the vibrator remote are placed on the same map. Search for the coat. You may pick up the remote and turn the vibrator OFF. No direction is shown.
        m_drone_mission_handcuff_title => Pick up the handcuff key while flashing!
        m_drone_mission_handcuff_description => The handcuff key is placed somewhere on the same map. Go pick it up while cuffed. Its direction is shown on your compass.
        m_drone_mission_handcuff_find_title => Find the handcuff key while flashing!
        m_drone_mission_handcuff_find_description => The handcuff key is placed somewhere on the same map. Go pick it up while cuffed. No direction is shown on the compass.
        m_drone_mission_final_title => Final Challenge
        m_drone_mission_title_handcuff_coat_random => Pick up the handcuff key and coat at random locations!
        m_drone_mission_description_handcuff_coat_random => Handcuff key and coat are placed on the same map. At the start, you are moved to a random location. The direction is displayed on the compass.
        m_drone_mission_title_handcuff_coat_random_find => Find the handcuff key and coat at random locations!
        m_drone_mission_description_handcuff_coat_random_find => Handcuff key and coat are placed on the same map. At the start, you are moved to a random location. The direction is not displayed.
        m_drone_mission_final_description => The coat, handcuff key, and vibrator remote are all placed on the same map. You will first be moved to a random position. Take off the handcuffs before going to get the coat. No direction is shown.
        m_drone_mission_handcuff_vibe_title => Pick up the handcuff key while wearing a vibrator and flashing!
        m_drone_mission_handcuff_vibe_description => The handcuff key is placed somewhere on the same map. Go get it while wearing both a vibrator and handcuffs. Its direction is shown on your compass.
        m_stage_fashion_shop => Clothes Shop
        m_stage_conbini => Convenience Store
        m_stage_mansion => Apartment
        m_stage_park => Central Park
        m_stage_residence => Residential Area
        m_stage_shopping_mall => Shopping Mall
        m_stage_station_front => Downtown
        m_hair_bob => Bob
        m_hair_career => Career Style
        m_hair_elelong => Elegant Long
        m_hair_fairy_twin => Fairy Twin
        m_hair_fuwa_bob => Fluffy Bob
        m_hair_half_twin => Half Twin
        m_hair_long => Long
        m_hair_medium => Medium
        m_hair_llynn => Flipped Long
        m_hair_mafu => Scarf Hair
        m_hair_middle_bob => Middle Bob
        m_hair_pony => Ponytail
        m_hair_princess_wave => Princess Wave
        m_hair_semi_long => Semi-Long
        m_hair_short => Short
        m_hair_short_twin => Short Twin
        m_hair_side_bob => Side Bob
        m_hair_straght_bob => Straight Bob
        m_hair_tecno => Techno Cut
        m_hair_wave_pony => Wave Pony
        m_hair_wolf => Wolf Cut
        m_hair_straightLong => Straight Long
        m_hair_waveLong => Wavy Long
        m_item_bodypaint => Body Paint Kit
        m_item_bodypaint_reusable => Body Paint Kit (Infinite)
        m_item_drone => Drone
        m_item_futa_inverse => Futanari Reversal Pill
        m_item_futa => Futanari Pill
        m_item_handcuff_key => Handcuff Key
        m_item_invisible_inverse => Invisibility Reversal Potion
        m_item_invisible => Invisibility Potion
        m_item_invisible_reusable => Invisibility Potion (Infinite)
        m_item_water => Water
        m_item_description_bodypaint => For a fixed duration, Suspicion builds more slowly. The effect weakens over time and ends after a set period.
        m_item_description_bodypaint_reusable => Its basic function is the same as the 'Body Paint Kit', it can be used repeatedly, but the RP multiplier remains fixed at 0.
        m_item_description_reusable2 => Can be used repeatedly. If used even once outside of your home, the mission will not be considered complete until you return home and you cannot earn RP.
        m_item_description_drone => Drone Missions become available.
        m_item_description_invisible => You become invisible to others, but the RP multiplier decreases by 100 %. The effect ends upon contact with someone.
        m_item_description_invisible_reusable => The basic function is same as the 'Invisibility Potion', it can be used repeatedly, but the RP multiplier remains fixed at 0.
        m_mission_area_expose => Be in {0} while flashing
        m_mission_area_naked => Be in {0} while naked
        m_mission_blind_expose => While flashing and blindfolded, move {0} m
        m_mission_blind_naked => While naked and blindfolded, move {0} m
        m_mission_bukkake => Splash semen on a person
        m_mission_buy_machine_expose => Use a vending machine while flashing
        m_mission_buy_machine_naked => Use a vending machine while naked
        m_mission_regi_expose => Flash while being served at a register
        m_mission_crouch_naked => While naked and crouching, move {0} m
        m_mission_ecstasy_near_npc => Climax near someone while flashing
        m_mission_elevator_move_with_out_coat => Leave your coat outside the elevator, then use the elevator to move.
        m_mission_escalator_expose => Move on an escalator while flashing
        m_mission_escalator_naked => Move on an escalator while naked
        m_mission_expose_in_sight => Flash while being watched
        m_mission_intercom_expose => Press the intercom and wait at the door while flashing
        m_mission_intercom_naked => Press the intercom and wait at the door while naked
        m_mission_rise_strange_expose => Show off your flashed parts
        m_mission_rise_strange_naked => Show off your naked body
        m_mission_handcuff_map => In a place where people pass by, handcuff yourself to an object while flashing or naked
        m_mission_handcuff_move => While flashing or naked and handcuffed, move {0} m
        m_mission_handcuff_near_npc => While flashing or naked and handcuffed, stay near someone
        m_mission_elevator_with_npc_expose => Ride the elevator with someone while flashing
        m_mission_elevator_with_npc_naked => Ride the elevator with someone while naked
        m_mission_lighting_expose => Step into the light while flashing
        m_mission_lighting_naked => Step into the light while naked
        m_mission_mall_cloor_dist_coat => Move {0} floors away from your coat
        m_mission_move_coat => Move {0} m away from your coat
        m_mission_move_coat_fashion_shop => Leave your coat in the fitting room and move {0} m away from it
        m_mission_move_coat_vertical => Move {0} floors away from your coat
        m_mission_naked => Get naked in a place where people pass by
        m_mission_near_npc_expose => Flash near someone
        m_mission_near_npc_naked => Get naked near someone
        m_mission_open_front => Flash your front in a place where people pass by
        m_mission_open_hip => Flash your butt in a place where people pass by
        m_mission_open_upper => Flash your upper body in a place where people pass by
        m_mission_pee => Urinate in a place where people pass by
        m_mission_pose => Perform a pose action while flashing or naked in a place where people pass by
        m_mission_put_coat_in_area_mansion => Leave your clothes outside the apartment and go to the top floor
        m_mission_m_mission_put_coat_in_area_station_front => Leave your clothes outside the footbridge and cross the bridge
        m_mission_sit_expose => Sit on a chair while flashing
        m_mission_sit_naked => Sit on a chair while naked
        m_mission_stop_elevator_naked => Leave your clothes outside the elevator and, while naked, stop at every floor
        m_mission_toilet_naked => Leave your coat outside the restroom and stay in the stall with the door open
        m_mission_dildo_expose => Use a dildo while flashing in a place where people pass by
        m_mission_dildo_naked => Use a dildo while naked in a place where people pass by
        m_mission_washer => Wash your clothes in the washing machine
        m_mission_vibe_expose => While flashing, set the vibrator to High/Random and move {0} m
        m_mission_vibe_naked => While naked, set the vibrator to High/Random and move {0} m
        m_rank_0 => Amateur
        m_rank_1 => Pervert
        m_rank_2 => Kinky
        m_rank_3 => Nympho
        m_rank_4 => Stranger
        m_rank_5 => Flasher
        m_rank_6 => Flash Fiend
        m_rank_7 => Apex Flasher
        m_rank_release_category_function => Feature Unlocked
        m_rank_release_category_area => Area
        m_rank_release_category_coat_action => Outfit Action
        m_rank_release_body_custom => Unlock locked body customization
        m_rank_release_body_paint => Body paint now on sale
        m_rank_release_open_hip => Flash your butt
        m_rank_release_open_upper => Flash upper body
        m_rank_release_naked => Get naked
        m_rank_release_dildo => Dildos now on sale
        m_rank_release_drone => Drones available for purchase and use
        m_rank_release_daytime => Daytime Flash
        m_rank_release_eye_mask => Eye masks now on sale
        m_rank_release_facial_custom => Facial customization unlocked
        m_rank_release_futanari => Futanari pills now on sale
        m_rank_release_handcuff => Handcuff items now on sale
        m_rank_release_piston_machine => Piston machines now on sale
        m_rank_release_reinforce1 => Rank 1 equipment upgrade
        m_rank_release_reinforce2 => Rank 2 equipment upgrade
        m_rank_release_reinforce3 => Rank 3 equipment upgrade
        m_rank_release_time_stop => Time-stop skill can be learned
        m_rank_release_vibe => Vibrator items now on sale
        ui_reinforce_dash => Movement speed while dashing
        ui_reinforce_dokidoki => Heart rate
        ui_reinforce_ecstasy => Ecstasy gauge gain
        ui_reinforce_mission_gauge => Mission gauge gain
        ui_reinforce_bareyasusa => Detection risk
        ui_reinforce_rp_bonus => RP Mult.
        ui_reinforce_slow_gauge => Slow Gauge Consumption
        ui_reinforce_stamina => Max Stamina
        ui_reinforce_stanima_regene => Stamina Recovery Speed
        ui_skill_auto_bareta_slow => Ah, I'm finished.
        ui_skill_auto_bareta_slow_description => When you get caught, Slow Motion triggers automatically. As long as you still have Slow Time remaining, you can savor the moment; if less than one second remains, no slow motion is applied.
        ui_skill_auto_slow => Auto Slow
        ui_skill_auto_slow_description => When you are about to be caught, Slow Motion triggers automatically. If less than one second of Slow Time remains, no slow motion occurs.
        ui_skill_cant_dash => Walker
        ui_skill_cant_dash_description => Dashing is disabled while flashing, but RP multiplier increases.
        ui_skill_continue_mission => Mission Cannot Be Interrupted
        ui_skill_continue_mission_description => If you no longer meet mission conditions, progress resets, but the RP multiplier increases.
        ui_skill_disable_hide_costume => Remove Casual Clothes Restrictions
        ui_skill_disable_hide_costume_description => Removes Casual Clothes restrictions, but reduces RP multiplier.
        ui_skill_fix_fps => Fixed first-person view
        ui_skill_fix_tps => Fixed third-person view
        ui_skill_fix_fps_description => Locks the view to first-person but increases the RP multiplier.
        ui_skill_fix_tps_description => Locks the view to third-person but increases the RP multiplier.
        ui_skill_hide_strange_ui => Hard Mode
        ui_skill_hide_strange_ui_description => The Suspicion gauge is hidden, so you can’t see how close your Flash is to being busted, but the RP multiplier increases.
        ui_skill_costume_num => Costume Slot Expansion
        ui_skill_costume_num_description => The maximum number of costume slots becomes {0}.
        ui_skill_npc_direct => Show NPC Direction
        ui_skill_npc_direct_description => Shows the direction of NPCs whose Suspicion is rising, but the RP multiplier decreases.
        ui_skill_perspective => X-Ray Vision
        ui_skill_perspective_description => Eye-mask gear no longer darkens vision, but the RP boost from eye masks is lost and eye-mask missions cannot be completed.
        ui_skill_slow => Slow Motion
        ui_skill_slow_description => Press the skill button to enter Slow Motion; the Slow Gauge is consumed and the effect ends at 0. Max continuous slow: {0} s.
        ui_skill_stamina => Stamina Boost
        ui_skill_stamina_description => Increase maximum Stamina by +{0} %.
        ui_skill_time_stop_description => Gain the ability to stop time. While active, missions will not be considered complete, and RP cannot be earned until you return home.
        ui_skill_pose_description_interact_ecstasy => Press “Interact” to cycle motions; the Ecstasy Gauge rises.
        ui_skill_pose_description_interact => Press “Interact” to cycle motions.
        ui_skill_pose_description_ecstasy => The Ecstasy Gauge rises.
        ui_skill_pose_description_pee => You will urinate when you have Urge to Urinate.
        log_no_pee => No Urge to Urinate
        ui_skill_pose_description_futanari => Usable only when futanari; the Ecstasy Gauge rises.
        ui_skill_pose_onaniNormal => Masturbate
        ui_skill_pose_ahegao => Ahegao Double Peace
        ui_skill_pose_tit_rotate => Nipple Play
        ui_skill_pose_dogeza => Dogeza
        ui_skill_pose_tintin => Penis
        ui_skill_pose_ganimata_hip => Bowlegged All-Fours
        ui_skill_pose_koshi_heko => Hip Thrust
        ui_skill_pose_ganimata_walk => Spread-Leg Walk
        ui_skill_pose_high_gre => High-Leg Pose
        ui_skill_pose_hip_shake => Butt Wiggle
        ui_skill_pose_i_balance => I-Split Balance
        ui_skill_pose_kaikyaku_fella => Spread-Leg Blowjob
        ui_skill_pose_onani_mitsuashi => Three-Leg Masturbation
        ui_skill_pose_onani_aomuke => Supine Masturbation
        ui_skill_pose_ne_katakashi_age => Supine One-Leg Raise
        ui_skill_pose_arm_kuri => Clit Play
        ui_skill_pose_onani_yotsuashi => Doggy Masturbation
        ui_skill_pose_pee_dog => Dog Pee
        ui_skill_pose_pee_kaikyaku => Spread-Leg Urination
        ui_skill_pose_pee_stand => Standing Pee
        ui_skill_pose_sikosiko => Stroking
        ui_skill_pose_tebura => Hand Bra
        ui_skill_pose_wakimise_crouch => Armpit Squat
        ui_title_new_game => New Game
        ui_title_continue => Continue
        ui_title_graphics => Graphics Settings
        ui_title_language => Language / 言語
        ui_title_exit => Quit Game
        ui_select_language_title => Select Language
        ui_GraphicsOption_title => Graphics Settings
        ui_GraphicsOption_aspect => Aspect Ratio
        ui_GraphicsOption_resolution => Resolution
        ui_GraphicsOption_screen_mode => Display Mode
        ui_GraphicsOption_quality => Graphics Quality
        ui_GraphicsOptionf_frame_rate => Max Frame Rate
        ui_GraphicsOption_brightness => Brightness
        ui_GraphicsOption_contrast => Contrast
        ui_GraphicsOption_ColorTemerature => Color Temperature
        ui_GraphicsOption_fov_fps => FOV (First-Person)
        ui_GraphicsOption_fov_tps => FOV (Third-Person)
        ui_GraphicsOption_vSync => VSync
        ui_GraphicsOption_aa => Anti-Aliasing
        ui_GraphicsOption_ssao => SSAO
        ui_GraphicsOption_SSR => SSR
        ui_GraphicsOption_bloom => Bloom
        ui_GraphicsOption_bloomStrength => Bloom Intensity
        ui_GraphicsOption_bloomBiasDark => Bloom Threshold (Dark Stage)
        ui_GraphicsOption_bloomBiasLight => Bloom Threshold (Bright Stage)
        ui_GraphicsOption_motion_blur => Motion Blur
        ui_GraphicsOption_reset_fov => Reset FOV
        ui_GraphicsOption_mask_by_vSync => VSync is enabled
        ui_GraphicsOption_high => High
        ui_GraphicsOption_middle => Medium
        ui_GraphicsOption_low => Low
        ui_GraphicsOption_window => Windowed
        ui_GraphicsOption_fullscreen => Fullscreen
        ui_InGame_RemainSlowTime => Remaining Slow Time
        ui_InGame_BringRp => RP Held
        ui_InGame_Ring_manual => Release Ring Shortcut Key : assign to shortcut<n>Click : execute without assigning
        ui_InGame_unlocking => Unlocking...
        ui_InGame_motion_speed => Motion Speed
        ui_InGame_WalkSpeed => Walking Speed
        ui_Achievement_Title => Achievements
        ui_Achievement_progress => Progress
        ui_Achievement_reward => Rewards
        ui_ApplyGraphicsCountDown_description => Apply settings?<n>If you do not press Yes, the original settings will be restored automatically.
        ui_status_Title => Manaka
        ui_status_BaseInfo => Basic Info
        ui_status_statistics => Statistics
        ui_status_totalEffect => Total Effects
        ui_status_SoloEffect => Individual Effects
        ui_status_BaseBareyasusa => Base Detection Risk
        ui_status_difficulty => Difficulty
        ui_status_totalPlayTime => Total Play Time
        ui_status_TotalExposeTime => Total Flash Time
        ui_status_costumeRate => Costume Completion Rate
        ui_status_missionRate => Mission Completion Rate
        ui_status_missionCount => Missions Completed
        ui_status_totalEarnRp => Total RP Gained
        ui_status_totalUseRp => Total RP Spent
        ui_status_TotalLostRp => Total RP Lost
        ui_status_MaxRpBonus => Max RP Multiplier
        ui_status_averageMissionMaxRpBonus => Average Max RP Multiplier (All Missions)
        ui_status_HighScore => High Score
        ui_status_GameOverCount => Times Game Over
        ui_status_EcstasyCount => Climaxes
        ui_ChooseDildo_Title => Where will you place the dildo?
        ui_ChooseDildo_floor => Floor
        ui_ChooseDildo_Wall => Object in front
        ui_chooseHandcuff_Title => Where will you cuff yourself?
        ui_chooseHandcuff_back => Behind the body
        ui_ChooseTimer_Title => Set timer to how many minutes?
        ui_ChooseTimer_Set => Set
        ui_ColorSetting_hsv => Specify via HSV
        ui_Closet_Title => Closet
        ui_Closet_tab_body => Body
        ui_Closet_tab_face => Face
        ui_Closet_tab_hair => Hair
        ui_Closet_tab_paint => Paint
        ui_Closet_tab_loadout => Loadout
        ui_Closet_tab_facial => Expression
        ui_Closet_ResetBody => Reset: Body
        ui_Closet_bodyDescription => Extreme values cause motion or outfit misalignment.
        ui_Closet_bodyDescription2 => Some settings may cause the first-person view to become misaligned.  
        ui_Closet_body_height => Height
        ui_Closet_body_head => Head
        ui_Closet_body_neckLength => Neck Length
        ui_Closet_body_NeckWidth => Neck Thickness
        ui_Closet_body_Breast => Boobs
        ui_Closet_body_bigBoobs => Huge Boobs
        ui_Closet_body_superBoobs => Ultra Boobs
        ui_Closet_body_Tit => Nipples
        ui_Closet_body_butt => Butt
        ui_Closet_body_Waist => Waist
        ui_Closet_body_legInterval => Foot Spacing
        ui_Closet_body_hutomomo => Thighs
        ui_Closet_body_hutomomoLength => Thigh Length
        ui_Closet_body_lowerLeg => Shins
        ui_Closet_body_LowerLegLength => lower leg length
        ui_Closet_body_upperBody => Upper Body
        ui_Closet_body_UpperBodyLength => Upper Body Length
        ui_Closet_body_upperBody2 => Upper Body 2
        ui_Closet_body_shoulderInterval => Shoulder Width
        ui_Closet_body_arm => Arms
        ui_Closet_body_inmou_amount => Pubic Hair Amount
        ui_Closet_body_inmouKosa => Pubic Hair Density
        ui_Closet_body_InmouColor => Pubic Hair Color
        ui_Closet_body_SkinColor => Skin Color
        ui_Closet_body_skinColorDetail => Skin Tone Detail
        ui_Closet_body_SkilColorFree => Skin Tone (Detail)
        ui_Closet_body_futa_size => Penis Size
        ui_Closet_body_tama_size => Testicle Size
        ui_Closet_body_futaShape => Shaft Shape
        ui_Closet_body_KitouSize => Glans Size
        ui_Closet_body_futaWidth => Shaft Thickness
        ui_Closet_face_reset => Reset: Face
        ui_Closet_face_test => Expression Test
        ui_Closet_face_loli => Baby Face
        ui_Closet_face_eyeSize => Eye Size
        ui_Closet_face_eyeDegree => Eye Angle
        ui_Closet_face_EyeInterval => Eye Spacing
        ui_Closet_face_EyeHeight => Eye Height
        ui_Closet_face_EyeDark => Eye Bags
        ui_Closet_face_pupilSize => Pupil Size
        ui_Closet_face_EyeColor => Eye Color
        ui_Closet_face_OddEye => Heterochromia
        ui_Closet_face_LeftEyeColor => Left Eye Color
        ui_Closet_face_MatsugeThick => Eyelash Thickness
        ui_Closet_face_MatsugeLength => Eyelash Length
        ui_Closet_face_MatsugeColor => Eyelash Color
        ui_Closet_face_MayuThick => Eyebrow Thickness
        ui_Closet_face_MayuColor => Eyebrow Color
        ui_Closet_face_DogMayu => Dog Brows
        ui_Closet_face_Hoo => Cheek Puffiness
        ui_Closet_face_TinLength => Chin Length
        ui_Closet_face_ElfEar => Elf Ears
        ui_Closet_face_Yaeba => Fang
        ui_Closet_hair_style => Hairstyle
        ui_Closet_hair_texture => Color Pattern
        ui_Closet_hair_color => Hair Color
        ui_Closet_hair_shade => Shadow Tint
        ui_Closet_facial_1 => Heart Rate: Minimum
        ui_Closet_facial_2 => Heart Rate: Low
        ui_Closet_facial_3 => Heart Rate: Medium
        ui_Closet_facial_4 => Heart Rate: High
        ui_Closet_facial_5 => At Climax
        ui_Closet_facial_6 => Post-Climax
        ui_Closet_facial_type_1 => Tension 1
        ui_Closet_facial_type_2 => Tension 2
        ui_Closet_facial_type_3 => Excitement 1
        ui_Closet_facial_type_4 => Excitement 2
        ui_Closet_facial_type_5 => Climax 1
        ui_Closet_facial_type_6 => Post-Climax 1
        ui_Closet_facial_type_7 => Smile
        ui_Closet_facial_type_8 => Excitement 3
        ui_Closet_facial_type_9 => Excitement 4
        ui_Closet_facial_type_10 => Excitement 5
        ui_Closet_facial_type_11 => Excitement 6
        ui_Closet_facial_type_12 => Climax 2
        ui_common_register => Register
        ui_common_equip => Equip
        ui_drone_title => Drone
        ui_drone_tab1 => Drone Mission
        ui_drone_tab2 => Random Mission
        ui_drone_start => Start
        ui_drone_interrupt => Abort
        ui_drone_stagebonus => Stage Bonus
        ui_drone_clears => Clears
        ui_drone_dronemission => Drone Mission
        ui_drone_summery => Mission Overview
        ui_drone_startCondition => Start Conditions
        ui_drone_ClearCondition => Clear Conditions
        ui_drone_AchieveCount => Times Completed
        ui_drone_CantArea => Unavailable in this area
        ui_drone_cantHome => Unavailable at home
        ui_drone_onlyHome => Only usable at home
        ui_randomMission_setting => Random Mission Setup
        ui_randomMission_map => Map
        ui_randomMission_missionNum => Number of Missions
        ui_randomMission_StartCondition => Start Conditions
        ui_randomMission_PerspectiveOff => X-ray Skill OFF
        ui_randomMission_rule => Rules
        ui_randomMission_description => Missions are selected randomly and clearing them all completes the challenge.<n>You must clear missions one by one in order.<n>Harder missions appear later.<n><n>Until cleared, the following are disabled:<n>- Fast Travel<n>- Time Wait<n>- Area Change
        ui_FastTravel_Title => Fast Travel
        ui_FastTravel_go => Move
        ui_Inventory_Title => Inventory
        ui_Mission_Title => Mission
        ui_Skill_Title => Skill / Pose
        ui_rank_Title => Rank
        ui_ingameMenu_killTile => Kill Time
        ui_common_option => Options
        ui_Inventory_tab1 => Items
        ui_Inventory_tab2 => Adult Toy
        ui_Inventory_tab3 => Costume
        ui_Inventory_tab4 => My set
        ui_Inventory_drop => Drop
        ui_Inventory_use => Use
        ui_killTime_wait => Waiting
        ui_saveSlot_Title => Select Save Slot
        ui_missionFaled_Normal => Flash Failed
        ui_missionFaled_GoHome => Return Home
        ui_mission_MaxRpMult => Max RP Mult.
        ui_option_title => Options
        ui_option_category_sound => Audio
        ui_option_category_ui => UI
        ui_option_category_gameSystem => Game System
        ui_option_category_control => Controls
        ui_option_category_keySetting => Key Settings
        ui_option_gayeSystem_reset => Reset Spawn Ratios
        ui_option_gayeSystem_futa => Disable Futanari
        ui_option_gayeSystem_pee => Disable Urination
        ui_option_gayeSystem_sex => Disable Sex
        ui_option_gayeSystem_interactAreaEffect => Hide Interact Area Effects
        ui_option_gayeSystem_redWallEffect => Hide effects in impassable areas
        ui_option_gayeSystem_maleRate => Male Spawn Ratio
        ui_option_gayeSystem_olderRate => Elderly Spawn Ratio
        ui_option_gayeSystem_blackCensor => Genital Black Mosaic
        ui_option_gayeSystem_blackCensorSize => Black Mosaic Size
        ui_option_gayeSystem_vibeState => Show Vibrator Status
        ui_option_gayeSystem_MallNpcScale => Daytime Shopping Mall NPC Multiplier
        ui_option_resetAll => Reset All
        ui_option_control_mouseXReverce => Invert Mouse X Axis
        ui_option_control_mouseYReverce => Invert Mouse Y Axis
        ui_option_control_GamePadXReverce => Invert Gamepad X Axis
        ui_option_control_GamePadYReverce => Invert Gamepad Y Axis
        ui_option_control_WheelReverse => Invert Wheel
        ui_option_control_MouserSensitive => Mouse Sensitivity
        ui_option_control_WheelSensitive => Wheel Sensitivity
        ui_option_control_GamePadSensitive => Gamepad Camera Sensitivity
        ui_option_sound_master => Master Volume
        ui_option_sound_se => SE
        ui_option_sound_ui => UI
        ui_option_sound_bgm => BGM
        ui_option_sound_vibe => Vibrator Volume Multiplier
        ui_option_sound_heartBeat => Heartbeat Volume Multiplier
        ui_option_ui_FaceCamera => Show Face Camera
        ui_option_ui_BodyCamera => Show Body Camera
        ui_option_ui_MessageLog => Show Message Log
        ui_option_ui_MessageLogRing => Show Ring Menu Change Log
        ui_option_ui_playerInfo => Show Player Info
        ui_option_ui_ecstasyGauge => Show Ecstasy Gauge
        ui_option_ui_missionPanel => Show Mission Panel
        ui_option_ui_rightDown => Show Bottom Right Info
        ui_option_ui_interactPanel => Show Interact Panel
        ui_option_ui_HeartBeatEffect => Show Heartbeat Fullscreen Effect
        ui_option_ui_HeartBeatEffectAlpha => Heartbeat Screen Overall Effect Intensity
        ui_option_ui_Manual => Controls Help
        ui_option_ui_compass => Show Compass
        ui_option_keySetting_save => Save
        ui_option_keySetting_reset => Reset
        ui_option_keySetting_move_forward => Move: Forward
        ui_option_keySetting_move_backward => Move: Backward
        ui_option_keySetting_move_left => Move: Left
        ui_option_keySetting_move_right => Move: Right
        ui_option_keySetting_move => Move
        ui_option_keySetting_openCoat => Take off coat
        ui_option_keySetting_closeCoat => Put on coat
        ui_option_keySetting_openHip => Flash butt
        ui_option_keySetting_dash => Run
        ui_option_keySetting_crouch => Crouch
        ui_option_keySetting_interact => Interact
        ui_option_keySetting_changeView => View Switch
        ui_option_keySetting_camera => Camera Rotation
        ui_option_keySetting_slow => Skill: Slow
        ui_option_keySetting_padShift => Shift Button
        ui_option_keySetting_systemMenu => System Menu
        ui_option_keySetting_menu => Menu
        ui_ingame_manual_pose_ring_menu => Pose Ring Menu (Hold): {0}, {1}, {2}
        ui_ingame_manual_ring_menu_hold => Ring Menu (Hold): {0}, {1}, {2}, {3}
        ui_ingame_manual_ring_menu => Common Ring Menu (Hold): {0}
        ui_option_keySetting_commonRingMenu => Common Ring Menu (Hold)
        ui_option_keySetting_skillRingMenu => Pose Ring Menu (Hold)
        ui_option_keySetting_skillRingMenu1 => Pose Ring Menu 1 (Hold)
        ui_option_keySetting_skillRingMenu2 => Pose Ring Menu 2 (Hold)
        ui_option_keySetting_skillRingMenu3 => Pose Ring Menu 3 (Hold)
        ui_option_keySetting_RingMenu => Ring Menu
        ui_option_keySetting_RingMenu1 => Ring Menu 1
        ui_option_keySetting_RingMenu2 => Ring Menu 2
        ui_option_keySetting_RingMenu3 => Ring Menu 3
        ui_option_keySetting_RingMenu4 => Ring Menu 4
        ui_option_keySetting_RingMenu5 => Ring Menu 5
        ui_option_keySetting_RingMenu6 => Ring Menu 6
        ui_option_keySetting_RingMenu7 => Ring Menu 7
        ui_option_keySetting_RingMenu8 => Ring Menu 8
        ui_option_keySetting_motionSpeedUp => Motion Speed: Increase
        ui_option_keySetting_motionSpeedDown => Motion Speed: Decrease
        ui_option_keySetting_walkSpeedUp => Walk Speed: Increase
        ui_option_keySetting_walkSpeedDown => Walk Speed: Decrease
        ui_option_keySetting_MoveForwardOneHand => Move Forward (One-Hand)
        ui_pcMenu_Title => PC Menu
        ui_pcMenu_collectAll => Collect all placed objects
        ui_OnlineShop_TItle => Online Shop
        ui_reinforce_Title => Equipment Upgrade
        ui_SkillChange_Title => Toggle Skill
        ui_ChangeDifficult_Title => Change Difficulty
        ui_common_purchase => Purchase
        ui_shop_mask_achievement => Unlock via Achievement
        ui_shop_purchased_mark => Purchased
        ui_reinforce_rank => Upgrade Rank
        ui_reinforce_rank1 => Rank 1
        ui_reinforce_rank2 => Rank 2
        ui_reinforce_rank3 => Rank 3
        ui_reinforce_cost => Upgrade Cost
        ui_reinforce_content => Upgrade Details
        ui_reinforce_reinforce => Upgrade
        ui_reinforce_rank1_description => One effect will be granted<n>A Rank 1 effect will be granted<n>A negative effect may also be granted
        ui_reinforce_rank2_description => Two effects will be granted<n>Rank 1~2 effects will be granted<n>A negative effect may also be granted
        ui_reinforce_rank3_description => Three effects will be granted<n>Rank 1~3 effects will be granted<n>A negative effect may also be granted
        ui_rank_current => Current Flasher Rank
        ui_rank_list => Flasher Rank List
        ui_rank_ReleaseContent => Unlockable Contents
        ui_result_title => Flash Result
        ui_result_completedMission => Missions Completed
        ui_result_EarnRp => RP Earned :
        ui_result_highScore => High Score :
        ui_result_CurrnetRp => RP Held :
        ui_result_exp => Flash EXP :
        ui_result_NewRecord => New Record!
        ui_loadGame_title => Load Game
        ui_SelectDifficulty_title => Select Difficulty
        ui_SexOption_title => Select Fetish
        ui_SexOption_futa => Enable Futanari
        ui_SexOption_pee => Enable Urination
        ui_SexOption_sex => Enable Sex
        ui_SexOption_description => Can be changed anytime in Options
        ui_skill_learned => Learned
        ui_skill_tab1 => Skill
        ui_skill_tab2 => Pose
        ui_systemMenu_Title => Pausing
        ui_systemMenu_resume => Return to Game
        ui_systemMenu_goTitle => Return to Title
        ui_tutorial_title => Tutorial
        ui_Tutorial_Description_bareyasusa => Flash Risk affects how quickly Suspicion rises and is calculated as follows:<n>Flash Risk = Base Flash Risk × Lighting Factor<n><n>Base Flash Risk varies according to what you are wearing.<n>The Lighting Factor caps at 100 %.<n><n>Base Flash Risk range: 10 % – 300 %<n>The higher the Base Flash Risk, the greater the RP multiplier.
        ui_Tutorial_Description_bodyPaint => Using a Body Paint Kit covers your body with painted-on clothing.<n>While the paint is active, your Flash is much harder to detect.<n>However, the paint fades over time and the effect gradually disappears.
        ui_Tutorial_Description_control1 => You can review the basic controls in Key Settings.<n>Open System Menu (Esc) → Options → Key Settings.
        ui_Tutorial_Description_control2 => Fixed actions with disabled key customization <n>Esc key<n> Open system menu<n> Any UI<n> Back, Close, Cancel<n><n>Mouse wheel<n> Adjust camera distance<n><n>Wheel drag<n> Adjust camera height<n><n>Right-click drag<n> Inventory, Online shop<n> Camera rotation<n> Closet<n> Character rotation<n> Auto-run active<n> Change path  
        ui_Tutorial_Description_control3 => Hold the Ring Menu key to open the ring menu.<n>Select an action with the mouse, then release the key to assign it.<n>Tap the Ring Menu key to perform the assigned action.<n>Alternatively, click an action while the ring is open to execute it immediately without assigning it.
        ui_Tutorial_Description_discomfort => Even without Flashing, suspicious behavior or wearing strange costumes will make people uneasy and raise Suspicion.<n>The Unease gauge is shown in gray and will never reach the MAX Suspicion threshold.
        ui_Tutorial_Description_fasttravel => Instantly travel to a different map.<n>Can only be used near a portal.
        ui_Tutorial_Description_handcuff => There are three handcuff-type items:<n> Handcuffs, Keyed Handcuffs, and Timer Handcuffs.<n>While cuffed, you can attempt to unlock them by holding the Interact button.<n><n>Handcuffs and Timer Handcuffs can also be placed on objects directly in front of you; if an obstacle is at close range and the right height, they can be attached anywhere.<n>While in Keyed Handcuffs you can drop the handcuff key from your inventory.<n><n>Timer Handcuffs reset their timer to 0 when you return home.<n><n>In handcuff-related missions, the faster you pick the lock or the further the unlock progress, the slower the mission gauge will grow.
        ui_Tutorial_Description_hideCostume => Some costumes hide Flashable body parts; these are called Casual Clothes.<n>The parts hidden correspond to the way each outfit looks.<n><n><n>Casual Clothes come with several restrictions:<n>They can only be put on near your coat, although you can take them off anywhere.<n>Wearing Casual Clothes imposes limits on certain missions, resetting their progress to 0. You can retry by putting the coat back on.<n>You cannot take on Drone Missions while wearing Casual Clothes.
        ui_Tutorial_Description_home1 => My home features an entrance, a PC, and a closet.
        ui_Tutorial_Description_home2 => Go to the entrance to go outside and start a mission.
        ui_Tutorial_Description_home3 => The PC lets you save, shop, configure skills, and more.
        ui_Tutorial_Description_home4 => Use the Closet to customize your character’s appearance.
        ui_Tutorial_Description_HowToPlay1 => Flash to complete missions and earn Flash Points (RP)!<n>Spend RP to unlock costumes and sexy poses.
        ui_Tutorial_Description_HowToPlay2 => Earn RP and return home safely to gain Flash EXP and raise your Flash Rank.<n>Higher Flash Ranks unlock new stages, missions, and more.<n>The kinkier the costume or pose, the higher the Flash Rank required to unlock it.
        ui_Tutorial_Description_invisible => Drink an Invisibility Potion to become invisible, making your Flash impossible to detect.<n>Be careful—accidentally bumping into someone will dispel the invisibility.
        ui_Tutorial_Description_pee => You can drink water from your inventory to increase your Urge to Urinate.<n>When the Urge reaches its maximum, you will automatically urinate.<n>You can also urinate early by performing a urination-related action before it maxes out.<n>After urinating, the Urge resets to zero.<n>If someone sees you urinating, their Suspicion gauge will rise.<n>Water can be obtained from vending machines.
        ui_Tutorial_Description_pistonMachine => There are three types of Piston Machines:<n>   Penis Piston Machine, Anal Piston Machine, and Vaginal Piston Machine.<n><n>These are controlled by the Piston Machine Switch.<n>You can use the Piston Switch from the ring menu.<n>The RP multiplier changes according to the intensity of the Piston Machine.
        ui_Tutorial_Description_reinforce => You can pay RP to grant special effects to equipment.<n>Special effects are selected at random and saved when equipment is enhanced.<n>The granted effects are broadly classified into three categories:<n>・Positive Effects<n>・Negative Effects<n>・Neutral Effects<n><n>Some effects have lower and upper limits:<n>　Base Detection Rate: 10% ~ 300%<n>　Handcuff Unlock Speed: Lower Limit -80%<n><n>The following effects have no positive variants:<n>　RP Multiplier<n>　Mission Gauge Increase Rate
        ui_Tutorial_Description_rule1 => Flash without getting caught!<n>If someone sees you while flashing, their Suspicion rises.<n>When the Suspicion gauge reaches MAX, you’re caught and it’s GAME OVER.
        ui_Tutorial_Description_rule2 => Getting home safely is part of the flash.<n>GAME OVER means the RP you earned and the missions you cleared will be lost.<n>If you make it home safely, the RP you earned and the completed missions will be confirmed, and your flash experience points will increase.
        ui_Tutorial_Description_timeStop => Time Stop can be triggered from the Ring Menu.<n>Everything freezes; mission gauges do not fill and RP multiplier is forced to 0.<n>If anyone’s suspicion is at MAX, the skill fails.
        ui_Tutorial_Description_vibe => There are three Vibrator items:<n> Vibrator, Nipple Rotor, Clit Rotor.<n>Controlled by the Vibrator Switch via Ring Menu or dropped from inventory.<n>RP multiplier changes with vibration strength.
        ui_Tutorial_Description_dildo => Placing a dildo lets you masturbate there.<n>Use from inventory or Ring Menu.<n>It can also be attached to nearby objects at suitable height.<n>Moving maps or waiting with a dildo placed discards it—remember to pick it up.
        ui_result_releaseContent => Unlocked Contents
        log_not_equip_vibe => Vibrator-type item is not equipped.
        ui_tutorial_controlPad_shiftPressing => While holding the Shift button
        ui_tutorial_controlPad_cameraUp => Camera Up
        ui_tutorial_controlPad_cameraDown => Camera Down
        ui_tutorial_controlPad_cameraDistance => Camera Distance
        ui_common_color => Color
        ui_drone_record => Record
        ui_buff_kind_reduceFootStepSound => Suspicion increase from footsteps
        title => Secret Flasher Manaka
        ui_Tutorial_ControlOneHanded => Recommended One-Handed Operation Settings<n>By assigning ‘Move Forward (One-Handed)’ to left click in key settings, you can move forward with a left click and change direction with a right click.<n><n>Right-click drag<n>　Moving Forward (One-Handed)<n>　　Change course<n><n>Mouse wheel<n>　Moving Forward (One-Handed)<n>　　Adjust walking speed,
        ui_misison_achiving => Completing
        ui_randomMission_all => All
        ui_RandomMission_orderMask => Unlocked after clearing the previous stage
        ui_RandomMission_completeMask => Unlocked after clearing all stages
        ui_tutoriarl_Trial => That’s all for the demo!
        ui_tutoriarl_TrialDescription => Continue in the full version!<n><n>You can keep playing, but RP and EXP have caps.<n>Your save data will carry over to the full version.
        ui_option_DescriptionFuta => Hide the Futanari feature. Missions can be completed without clearing any futanari-related missions.
        ui_option_DescriptionPee => Hide the urination feature. Missions can be completed without clearing urination-related missions.
        ui_option_DescriptionSex => When Sex is enabled, you can learn an exclusive skill in the late game.
        ui_option_DescriptionSelfCamera => Turning it off reduces processing load
        ui_option_DescriptionSound => In addition to SE volume control, you can further set individual multipliers.
        ui_option_DescriptionBlack => The genital area will be rendered black. Please note that there’s a non-zero chance the black mosaic may be removed due to unexpected bugs.
        ui_option_DescriptionVibeState => Display vibrator and piston machine intensity as 5×5 px icons in the bottom-left corner of the screen for users who want to link this state with something outside the game. \nOFF, Low, Medium, and High in black, blue, green, red respectively, with the vibrator on the left and the piston machine on the right.
        ui_cosplayOption_type => Style
        ui_cosplayOption_material => Material
        ui_cosplayOption_blink => Blinking
        ui_cosplayOption_zipper => Zipper
        ui_cosplayOption_removeCenter => Center Removal
        ui_cosplayOption_shape => Shape
        ui_cosplayOption_eye => Eye
        ui_cosplayOption_showHair => Show Hair
        ui_cosplayOption_position => Position
        ui_cosplayOption_button => Button Open/Close
        ui_cosplayOption_shirtsIn => Shirt Tucked In
        ui_tutorial_title_Stamina => Stamina
        ui_tutorial_title_StaminaDescription => Dashing while flashing consumes stamina.<n>It doesn’t consume stamina when not flashing.
        m_skill_exibitionism => Exhibitionist Urge
        ui_skill_description_exibitionism => You won’t get a GAMEOVER if you’re seen. While active, missions will not be considered complete until you return home, and you cannot earn RP.
        ui_rpPerSec => {0}RP<n>/sec
        ui_customize_onlyFuta => Can only be set when in Futanari
        ui_optionControl_InversePadCameraUpDown => Invert gamepad camera up/down movement
        ui_optionUi_missionPanelSize => Mission Panel Size
        ui_randomMission_conditionExhibitionism => Desire to be seen Skill OFF
        ui_option_keyConfig_ringLeft => Move ring menu to the left
        ui_option_keyConfig_ringRight => Move ring menu to the right
        log_portal_cant_move_naked_externalCoat => If the coat is outside, you cannot go outside（can be retrieved by the PC）
        ui_dringWater => Drink Water
        ui_NoWater => No water
        ui_tutorial_Compass => The compass displays directions to your home and other stages. If you get lost, check the compass.
        m_item_bodypaint_Washer => Body Paint Cleaning Kit
        m_item_bodypaint_WasherDescription => Wash off body paint
        log_noAchieveMissionMode => The mission will not be considered complete until you return home.
        ui_interact_sexNpc => Negotiate
        ui_interactMenu_sex => Have Sex
        ui_interactMenu_sexCancel => You should go back
        ui_manual_cumShot_0 => Ejaculation : {0}
        ui_manual_cumShot => Ejaculation
        sex_Taii => Sex Position
        sex_positionAdjust => Position Adjustment
        sex_maleVisible => Show penetrator character
        sex_exit => Finish
        sex_moveYoko => Horizontal Movement
        sex_moveZengo => Forward/Backward Movement
        sex_moveJouge => Up/Down Movement
        sex_rotate => Rotation
        sex_hantoumei => Semi-Transparent
        sex_onlySao => Penetrator only
        m_skill_sex => Seduction
        ui_randomMission_conditionSex => Seduction Skill OFF
        mSex_name_standBack => Standing Doggy Style
        mSex_name_Kijoui => Woman on top
        ui_option_futanariFemalClimax => Female Orgasm OFF During Futanari Climax
        ui_option_control_longPressTimeScale => Button long-press recognition time
        ui_skillTitle_DokodemoHideCostume => Casual Outfit Anywhere
        ui_skillDescription_DokodemoHideCostume => Enables changing into casual outfit anywhere without being near the coat, but reduces RP multiplier.
        log_cantTakeOffViewInDrone => Cannot be removed during vibrator-related drone missions.
        ui_SystemMenu_emergencyEscape => Emergency Escape
        ui_SystemMenu_emergencyEscape_description => This is the emergency escape feature for when a bug prevents you from returning home.<n>The mission will not be considered complete, and you will not earn any RP.<n>Execute it?
        ui_keyConfig_uiBack => UI: Back
        ui_keyConfig_escToCancel => Awaiting key input<n>Press Esc to cancel
        ui_keyConfig_leftClickToCancel => Awaiting key input<n>Press left click to cancel
        ui_keyConfig_inventoryItem => Inventory : Items
        ui_keyConfig_inventoryAdultGoods => Inventory : Adult Toy
        ui_keyConfig_inventoryCosplay => Inventory : Costumes
        ui_keyConfig_inventoryMySet => Inventory : My Set
        ui_noEyeMask => No eye mask
        ui_ingame_manual_viewChange => View Switch : {0}
        ui_skill_description_sex => Being seen will no longer cause a GAMEOVER. While active, missions will not be considered complete until you return home, and you cannot earn RP.<n>You can have sex with certain male NPCs whose suspicion level has reached MAX.<n><n>Changing certain body customization options may cause animations to misalign.<n>Height, thigh length, lower leg length
        ui_option_GameSystem_title_ekitaiDisable => Disable Liquid Representation
        ui_option_GameSystem_description_ekitaiDisable => Turn off the visual representation of urination, ejaculation, and squirting.
        ui_nameEdit_title => Rename
        ui_reinforce_reinforceOption => Settings
        ui_reinforce_reinforceOption_Title => Enhancement Settings: {0}
        ui_reinforce_reinforceOption_slotFix => Lock Slot {0}
        ui_reinforce_reinforceOption_onlyNegative => Negative Effects Only
        ui_reinforce_reinforceOption_costScale => Enhancement Cost Multiplier: x{0}
        ui_reinforce_rank1_descriptionOnlyNegative => One effect will be granted.<n>Only <color=red>Negative Effects</color> of Rank 1 will be granted.
        ui_reinforce_rank2_descriptionOnlyNegative => Two effects will be granted.<n>Only <color=red>Negative Effects</color> of Rank 1~2 will be granted.<n>
        ui_reinforce_rank3_descriptionOnlyNegative => Three effects will be granted.<n>Only <color=red>Negative Effects</color> of Rank 1~3 will be granted.
        m_skill_autoAddMoisture => Frequent Urination
        m_skill_autoAddMoisture_description => Urge to urinate continuously increases
        ui_cosplayPartOption_hide => Hidden
        ui_Tutorial_Description_pistonMachine2 => There are two types of Piston Machines:<n>   Anal Piston Machine, and Vaginal Piston Machine.<n><n>These are controlled by the Piston Machine Switch.<n>You can use the Piston Switch from the ring menu.<n>The RP multiplier changes according to the intensity of the Piston Machine.
        m_mission_m_mission_put_coat_in_area_station_front2 => Place the coat outside the pedestrian bridge and stand on top of the pedestrian bridge
        log_cantReachDildo => Cannot reach the dildo
        ui_caution_NoAchieveMissionMode => If used, the mission will not be considered complete until you return home.<n>Is it okay?
        ui_difficulty_description_easy2 => For players who are not good at games.<n>The speed at which flash is noticed is slower.<n>Stamina increases and recovers faster.<n>Fewer NPCs appear.
        m_skill_NoReinforceEffect_Title => Equipment and enhancement effects disabled
        m_skill_NoReinforceEffect_description => Disable costume equipment and enhancement effects, and toy enhancement effects.
        ui_cosplayCurrentEquipCategory => Equipped
        m_mission_put_coat_in_area_mansion2 => Place the coat outside the apartment and go to the top floor
        m_mission_stop_elevator_naked2 => Leave the coat outside the elevator and stop at every floor naked.
        m_item_description_invisible2 => You become invisible to others, but the RP multiplier is multiplied by x0.2. The effect ends if you come into contact with others.

DropItem(itemtype = DropItemType, stage = StageName, x = PositionX, y =
PositionY, z = PositionZ[, rx = RotationX, ry = RotationY, rz = RotationZ, rw
= RotationW][, compass = CompassIconVisible])  
DropItem(itemtype = DropItemType, position = Position[, compass =
CompassIconVisible])  

returns: String or Number or null

  * The return value is the DropItemType or null if the function fails.
  * For dildos, that can have multiple dropped instances, it may also return a number instead to differentiate between the different instances.
  * Either way, you can use the return value to pick the dropped item back up.

CollectItem(itemtype = DropItemType[, stage = StageName, x = PositionX, y =
PositionY, z = PositionZ])  
CollectItem(itemtype = DropItemType[, position = Position])  
CollectItem(itemtype = DropItemType)  

returns: Boolean

  * If you specify a position (including stage) you can choose which item to pick up, if there are multiple of the same base type.
  * If not, you can use the return value of the DropItem function, that might also be a Number if it isn't the first instance.

SetVibrator(VibratorStrength)  
SetVibrator(VibratorStrength)  

returns: null

  * If used with a string, the eligible values are: Off, Low, High, Random

SetPiston(PistonStrength)  
SetPiston(PistonStrength)  

returns: null

  * If used with a string, the eligible values are: Off, Low, Medium, High, Random

LockHandcuffs(handcuffstype = HandcuffsType[, attachtoobject =
AttachToObject][, duration = DurationInSeconds])  

returns: null

  * HandcuffsType must be one of the following values: Handcuff, KeyHandcuff, TimerHandcuff

UnlockHandcuffs()  

returns: null

EquipCosplay(CosplayNameKey1[, CosplayNameKey2]...)  
EquipCosplay(ListOfCosplayNameKeys1[, ListOfCosplayNameKeys2]...)  

returns: null

UnequipCosplay(CosplayNameKey1[, CosplayNameKey2]...)  
UnequipCosplay(ListOfCosplayNameKeys1[, ListOfCosplayNameKeys2]...)  

returns: null

UnequipAllCosplay()  

returns: null

OwnCosplay(owns = NewOwn, CosplayNameKey1[, CosplayNameKey2]...)  
OwnCosplay(owns = NewOwn, ListOfCosplayNameKeys1[, ListOfCosplayNameKeys2]...)  

returns: null

EquipAdultToy(AdultToyName1[, AdultToyName2]...)  
EquipAdultToy(ListOfAdultToyNames1[, ListOfAdultToyNames2]...)  

returns: null

UnequipAdultToy(AdultToyName1[, AdultToyName2]...)  
UnequipAdultToy(ListOfAdultToyNames1[, ListOfAdultToyNames2]...)  

returns: null

SetPlayerPosition([x = PositionX, y = PositionY, z = PositionZ][, rx =
RotationX, ry = RotationY, rz = RotationZ, rw = RotationW])  
SetPlayerPosition([position = Position][, rotation = Rotation])  

returns: null

SetStage([StageType][, daytime = DayTime])  

returns: null

SetCamera([pitch = Pitch][, yaw = Yaw][, lock = LockCamera])  

returns: null

  * Set direction of the current camera (first or third person)
  * Pitch and yaw values are in degrees
  * For first person camera there is no yaw because it is tied to the player model
  * Set "lock=true" to prevent player-induced rotation. Don't forget to unlock it later by calling the function with lock=false.
  * The game might change the lock state by itself in certain events

SetAction(Action)  

returns: null

SetFutanari(FutanariActive)  

returns: null

SetSkill(Skill, Enabled)  

returns: null

SetPlayerData(DataName, DataValue)  
SetPlayerData(DataName, DataValue)  
SetPlayerData(DataName, DataValue)  
SetPlayerData("BodyPaintTypeDict", BodyPaintType, Active)  
SetPlayerData("HairCustomizeDataDict", HairType, DataName, DataValue)  

returns: null

SetSkillShortcut(Slot, ActionIndex)  

returns: null

  * Slot indices range from 0 to 7

Index| Action| Index| Action  
---|---|---|---  
0| Auto Run| 16| Penis  
1| Chase| 17| Supine One-Leg Raise  
2| Dildo| 18| Armpit Squat  
3| Vibrator Switch| 19| High-Leg Pose  
4| Handcuffs| 20| Hip Thrust  
5| Piston Machine Switch| 21| Masturbate  
6| Time Stop| 22| Three-Leg Masturbation  
7| Urination| 23| Supine Masturbation  
8| Spread-Leg Blowjob| 24| Doggy Masturbation  
9| I-Split Balance| 25| Nipple Play  
10| Dogeza| 26| Clit Play  
11| Butt Wiggle| 27| Stroking  
12| Hand Bra| 28| Spread-Leg Urination  
13| Ahegao Double Peace| 29| Standing Pee  
14| Bowlegged All-Fours| 30| Dog Pee  
15| Spread-Leg Walk  
  
GetSkillShortcut(Slot)  

returns: Number

GetRandomPosition(minRange)  

returns: List

  * Uses the games StageRandomPositionManager which provides a random position out of a set list of positions for the current stage.
  * If there is no valid position returns null.

AddCurrentEarnRP(RPValue)  

returns: Number newRP

  * add to RP currently earned in this outing

SetCurrentEarnRP(RPValue)  

returns: Number newRP

GetCurrentEarnRP()  

returns: Number currentRP

AddCurrentRP(RPValue)  

returns: Number newRP

  * add to total RP currently held
  * don't confuse with AddCurrentEarnRP

SetCurrentRP(RPValue)  

returns: Number newRP

GetCurrentRP()  

returns: Number currentRP

SetEcstasy(Value)  

returns: Number newValue

AddEcstasy(Value)  

returns: Number newValue

GetEcstasy()  

returns: Number Value

SetStamina(Value)  

returns: Number newValue

  * You can get the maximum stamina with the _state.StaminaMax variable.

AddStamina(Value)  

returns: Number newValue

GetStamina()  

returns: Number Value

SetMoisture(Value)  

returns: Number newValue

  * Set the fullness of the players bladder.

AddMoisture(Value)  

returns: Number newValue

GetMoisture()  

returns: Number Value

SetItemCount(Item, newCount)  

returns: Number newCount

AddItemCount(Item, addCount)  

returns: Number newCount

GetItemCount(Item)  

returns: Number Count

CanGameOver([Value])  

returns: Boolean

  * If set to false, prevents a game over when found by NPCs
  * You can get the ID of the NPC that found the player (even if no game over is triggered) with _state.FoundNPC 
    * It triggers only once per NPC and is only valid for one frame
    * _state.FoundNPC is -1 if not triggered

TriggerGameOver()  

returns: null

  * Also works if you have skills that prevent getting caught
  * Condition and _stage.GameOver will stay true until you leave your house again.

PlaySoundEffect(SoundEffectName[, volume = Volume][, x = PositionX, y =
PositionY, z = PositionZ])  

returns: null

  * SoundEffectName must be one of the following values defined here. (Not all might work)
  * Volume is between 0 and 1
  * If you don't need 3D sound you can omit the x,y,z values

SetStageRankLimit(StageType, Rank)  

returns: null

  * Change the rank limit to disable all portals to that stage
  * Fast Travel will also be disabled.
  * If you pass a Rank value of less than 0, the rank requirement will be reset to the default.

GetStageRankLimit(StageType)  

returns: Number

SetPortalEnabled(StageType, Enabled)  

returns: null

  * Enabled/disables the portal to the specified stage
  * You can still fast travel to the stage though.
  * Resets when you travel to another stage.

GetAllWaypoints()

returns: List

  * Get a list of the current stage's waypoints for chairs, vending machines etc.
  * Each item contains _Type_ , _RouteInteractType_ and _Position_ values, so you can identify them and use in an NPC object

SetSexPosition(SexPosition)

returns: null

  * SexPosition must be one of the two positions: StandBack, Kijoui

DeactivateSex()

returns: null

TriggerSexOrgasm()

returns: null

  * Implicitly also sets ecstacy to 1, otherwise it wouldn't work unless ecstacy is already high enough.

SetSexMenu([canfinish = CanFinish][, canposition = CanChangePosition])
SetSexMenu([canfinish = CanFinish][, canposition = CanChangeToPosition])

returns: null

  * canfinish: En-/Disable "Finish" Button
  * canposition: 
    * If Boolean, enables/disables all position buttons
    * As a list, the keys should be the integer indices (zero-based) with boolean values
    * The easiest way would be a CreateList(true,false), or similar, that implicitly creates the indices.

### Additional Game Functions

  * Functions for features added by the mod that aren't part of the original game

ShowBlackscreen([color = Color][, delay = Delay][, fadein = FadeIn][, duration
= Duration][, fadeout = FadeOut])

returns: null

GetSnapshotData(ImageReference)

returns: List

  * Returns the meta data of a snapshot taken by the camera feature.
  * Has the same structure as the _state global variable, minus some values like mission status and owned cosplay, because those aren't visible in an image.

GetAllSnapshots([deleted = ListDeleted][, hidden = ListHidden])

returns: List

  * Returns a list of references to all snapshots
  * By default deleted and hidden snapshots are excluded. Set _deleted_ and/or _hidden_ to true to include them in the list.

DeleteSnapshot(ImageReference)

returns: null

  * Flags the image as deleted so it will be deleted on the next program start if it isn't referenced in a save

GetImageReference(FilePath)

returns: null

  * FilePath is relative to your projects folder

### Graphics

SetGraphicsOption(Option, Value) SetGraphicsOption(Option, Value)

returns: null

  * Values are Numbers with the exception of the "MotionBlur" option, which expects a Boolean.
  * The values get rounded to an Integer, because the game uses those, but you can pass floating point numbers.
  * List of options: 
    * MotionBlur
    * Brightness
    * Contrast
    * ColorTemperature
    * BloomStrength
    * BloomBiasDark
    * BloomBiasLight
    * CameraAngle1st
    * CameraAngle3rd

GetGraphicsOption(Option)

returns: Number or Boolean

### Audio

StopAudio(AudioInstanceID[, FadeOutTime])

returns: null

  * Stops an audio source that has been started with Audio.Play()

### Math

#### Standard

Random([[minInclusive], maxExclusive])

returns: Number

  * If no parameters are provided the function return a value in the interval [0,1) 

RandomInt([[minInclusive], maxExclusive])

returns: integer Number

  * If no parameters are provided the function return either 0 or 1

Sin(Angle)  

returns: Number

Cos(Angle)  

returns: Number

Tan(Angle)  

returns: Number

Asin(Value)  

returns: Number

Acos(Value)  

returns: Number

Atan(Value)  

returns: Number

Floor(Value)  

returns: Number

Ceil(Value)  

returns: Number

Sign(Value)  

returns: Number

Abs(Value)  

returns: Number

Trunc(Value)  

returns: Number

Round(Value)  

returns: Number

LogN(Value)  

returns: Number

Log2(Value)  

returns: Number

Log10(Value)  

returns: Number

Min(Value1[, Value2]...)  

returns: Number

Max(Value1[, Value2]...)  

returns: Number

#### Vector

Vector(X, Y[, Z])  

returns: List

  * Creates a list with x, y, z items

Quaternion(RX, RY, RZ, RW)  

returns: List

  * Creates a list with rx, ry, rz, rw items

Vector3Length(Vector)  

returns: Number

Vector3SqrLength(Vector)  

returns: Number

Vector3Add(Vector1, Vector2)  

returns: List

Vector3Sub(Vector1, Vector2)  

returns: List

Vector3Scale(Vector, Scalar)  
Vector3Scale(Scalar, Vector)  

returns: List

Vector3Dot(Vector1, Vector2)  

returns: Number

Vector3Cross(Vector1, Vector2)  

returns: List

Vector3Rotate(Quaternion, Vector)  

returns: List

Vector3Distance(Vector1, Vector2)  

returns: Number

### String

Length(String)

returns: Number

Lower(String)

returns: String

  * Convert a string to lower case

Upper(String)

returns: String

  * Convert a string to upper case

Find(SubString, String)

returns: Number

  * Indices are zero-based.
  * Returns -1 if substring is not found.

SubString(String, start = Start[, length = Length]) SubString(String, start =
Start, end = End) SubString(String, end = End[, length = Length])

returns: String

  * Indices are zero-based.
  * Result includes character at _end_

    
    
    s = "Hello World"
    Log(Length(s)) 11
    Log(SubString(s, start=4)) o World
    Log(SubString(s, start=4, length=5)) o Wor
    Log(SubString(s, start=4, end=9)) o Worl
    Log(SubString(s, end=9)) Hello Worl
    Log(SubString(s, end=3, length=3)) ell

Format(FormatString[, UnnamedParameter1]...) Format(FormatString[,
ListOfUnnamedParameters])

returns: String

  

Examples:

    
    
    Log(Format("Number with 3 decimals: {0:F3}",1/7))
    list = CreateList("cow",4,"legs")
    Log(Format("A {0} has {1} {2}.",list))
    
    Output:
    Number with 3 decimals: 0.143
    A cow has 4 legs.

ToNumber(S)

returns: Number or null

  * Returns null if the string can not be converted to a number.

### Files

  * **All paths are relative to your project directory**

FileExists(Path)

returns: Boolean

GetFiles([Path][, subfolders = SearchSubfolders])

returns: List

  * Get a list of all files.
  * If no path is specified, it defaults to the projects root directory.

GetFileExtension(Path)

returns: String

  * Returns the extension (including the period) or an empty string (if the file has no extension).

## Objects

