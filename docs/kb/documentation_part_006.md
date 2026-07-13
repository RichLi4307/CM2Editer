<!-- source: docs\documentation.html -->
<!-- part: 6 -->
<!-- line_range: 3782-4683 -->

### Tips

    
    
    faster
    if value == 0
    	...
    slower
    if (value == 0)
    	...
    
    
    faster
    list["0"] = "test"
    slower
    list[0] = "test"
    
    
    faster
    list.arg1 = "test"
    slower
    list["arg1"] = "test"

### Example Implementations

    
    
    Implementation of a function to execute code after a delay
    
    thread = CreateThread("main")
    
    main:
    	Log("DelayExample: Init")
    	Delay(_this, "delayed1", 5)
    	Delay(_this, "delayed2", 8)
    	Delay("delayed_function", 6)
    	_this.Goto("next")
    	
    	next:
    		Log("DelayExample: Next Label")		
    	delayed1:
    		Log("DelayExample: Delay 1")
    	delayed2:
    		Log("DelayExample: Delay 2")
    
    delayed_function:
    	Log("DelayExample: Delayed Function")
    
    Parameters: (thread, methodname, delay) or (functionname, delay)
    Delay:	
    	if delaymanager == null
    		Global("delaymanager", CreateThread("delaymanager"))
    	if GetType(_args[0]) == "Object"
    		delaymanager.AddMethod(thread = _args[0], method = _args[1], delay = _args[2])
    	else
    		delaymanager.AddMethod(thread = null, method = _args[0], delay = _args[1])
    
    delaymanager:
    	listeners = CreateList()
    	
    	AddMethod:
    		index = 0
    		while listeners[index] != null
    			index+=1
    	
    		newlistener = CreateListener("ListenerMethods", thread = thread, method = method, delay = delay, index = index)
    		listeners[index] = newlistener
    		
    	ListenerMethods:
    		delay -= _timediff
    		if delay <= 0
    			if thread != null
    				CallMethod(thread, method)
    			else
    				CallFunction(method)
    			listeners[index] = null

### Game Constants

The values here are a list of the constants as they are defined in the games
code. Not all will work when used and might even cause errors.

#### SoundEffect

  * NoType
  * FootStepHeel
  * FootStepHeelJog
  * ClothesSound
  * ClothesDrop
  * HeartBeat
  * FootStepHeelCrouch
  * Kuchu
  * Shiofuki
  * DokinHigh
  * DokinLow
  * Ok
  * Close
  * BuyMachine1
  * BuyMachine2
  * Drink
  * PeeNormal
  * Equip
  * SleepBed
  * Failed
  * Picking
  * Unlock
  * AttachHandcuffs
  * RankUp
  * SuburbsDaytime
  * SyburbsNight
  * MyRoom
  * KillTime
  * BaibuLow
  * BaibuHigh
  * Barefoot
  * FootStepSneaker
  * BraAttach
  * BraDetach
  * PantsDetach
  * VibratorSwitch
  * Breath1
  * Breath2
  * SlowIn
  * SlowOut
  * Gogogo
  * Reinforce
  * CommonEquip
  * ShoppingMallBgm
  * Piston
  * PutDildo
  * CollectAll
  * Buy
  * UiSelect
  * UiDecide
  * UiCancel
  * UiSlideOpen
  * UiSlideClose
  * OpenDoor
  * CloseDoor
  * Pinpon
  * CarIdling
  * CarStart
  * CarStop
  * CarPass
  * BgmParkDayTime
  * BgmParkNight
  * FootStepGrass
  * FootStepGravel
  * ToiletDoorOpen
  * ToiletDoorClose
  * ToiletFutaOpen
  * ToiletFutaClose
  * FootStepMetal
  * MentalAlert
  * Drone
  * ShopDoor
  * ShoppingMallNight
  * ElevatorMove
  * ElevatorOpen
  * ElevatorClose
  * ElevatorBell
  * Regi
  * MansionPostReceiveOpen
  * MansionPostReceiveClose
  * ToiletPeeWater
  * ToiletHandWash
  * ToiletLock
  * ToiletUnlock
  * ToiletWater
  * BgmFashionShop
  * CurtainOpen
  * CurtainClose
  * WasherOpen
  * WasherClose
  * WasherStart
  * WasherLoop
  * WasherBeep
  * Syasei
  * TimeStop
  * FootStepSneakerJog
  * Rotor
  * UICompleteMission

#### StageType

  * None
  * Apart
  * Convenience
  * FashionShop
  * Residence
  * ShoppingMall
  * StationFront
  * Park
  * Mansion

Other values that aren't used in the game but defined are:

  * TokyoStreet
  * Suburbs
  * Street
  * City
  * BarberShop
  * Laundry
  * Underpass

#### Action

  * None
  * OldOnaniNormal
  * OldGanimataWalk
  * Pinpon
  * ConbiniTakeGoods
  * CrouchCry
  * EatMedicine
  * SadHandcuffAtMap
  * SwitchTimeStop
  * SwitchPistonMachine
  * PickingCoat
  * Pick
  * Drop
  * ChangeClothes
  * DroppingClothes
  * HandOver
  * InsertAnalPlug
  * ExtractAnalPlug
  * CommonEquip
  * IntoWasher
  * TakeFromWasher
  * UseBuyMachine
  * DrinkWater
  * PeeNormal
  * TakeOffPants
  * TakeOnPants
  * TakeOffBra
  * TakeOnBra
  * Sad
  * AttachHandcuffs
  * PutHandcuffsOnMap
  * HandcuffsAtMap
  * UnlockHandcuffsAtMap
  * AttachEyeMask
  * SwitchVibrator
  * PickUpItem
  * SitDown
  * StandUp
  * PutDildoFloor
  * PutDildoWall
  * UseDildoFloorPussy1
  * UseDildoFloorPussy2
  * UseDildoFloorPussy3
  * UseDildoFloorPussy4
  * UseDildoFloorPussy5
  * UseDildoFloorAnal1
  * UseDildoFloorAnal2
  * UseDildoFloorAnal3
  * UseDildoFloorAnal4
  * UseDildoFloorAnal5
  * UseDildoFloorFella1
  * UseDildoFloorFella2
  * UseDildoFloorFella3
  * UseDildoFloorFella4
  * UseDildoFloorFella5
  * UseDildoWallPussy1
  * UseDildoWallPussy2
  * UseDildoWallPussy3
  * UseDildoWallPussy4
  * UseDildoWallPussy5
  * UseDildoWallAnal1
  * UseDildoWallAnal2
  * UseDildoWallAnal3
  * UseDildoWallAnal4
  * UseDildoWallAnal5
  * UseDildoWallFella1
  * UseDildoWallFella2
  * UseDildoWallFella3
  * UseDildoWallFella4
  * UseDildoWallFella5
  * UseDildoFloorWaitPussy
  * UseDildoFloorWaitAnal
  * UseDildoFloorWaitFella
  * UseDildoWallWaitPussy
  * UseDildoWallWaitAnal
  * UseDildoWallWaitFella
  * UseDildoFloorPussyEcstasyA
  * UseDildoFloorAnalEcstasyA
  * UseDildoFloorFellaEcstasyA
  * UseDildoWallPussyEcstasyA
  * UseDildoWallAnalEcstasyA
  * UseDildoWallFellaEcstasyA
  * PickDildo
  * SitDildo
  * SitDildoPut
  * SitDildoPick
  * SitDildoMoveAnal
  * SitDildoMovePussy
  * PickDildoWall
  * GanimataWalk
  * AhegaoDoublePiece
  * HipShake
  * GanimataHip
  * KaikyakuFella
  * Dogeza
  * DogTintin
  * IBalance
  * WakimiseCrouch
  * MituasiOnani
  * Tebura
  * PeeKaikyaku
  * PeeDog
  * ChikubiRotate
  * OnaniYotuashi
  * OnaniNeGanimata
  * OnaniNormal
  * NeKataashiage
  * OnaniArmKuri
  * OnaniSikoru
  * GanimataKoshiHeko
  * Haigure
  * PeeStand
  * DogezaUpHead
  * PoseEnd
  * SexStandBack
  * SexKijoui

#### AdultToy

  * AnalPlug
  * Vibrator
  * EyeMask
  * Handcuff
  * KeyHandcuff
  * TimerHandcuff
  * TitRotor
  * KuriRotor
  * PistonFuta
  * PistonAnal
  * PistonPussy

#### Skill

  * None
  * Mental
  * Stamina
  * CoatLevel
  * Flasher
  * Raper
  * ContinueMission
  * Slow
  * NpcDirect
  * Sneak
  * AutoSlow
  * FixFps
  * MaxAccessoryNum
  * AutoBaretaSlow
  * HideStrangeUi
  * NoFastTravel
  * Perspective
  * MyPace
  * TimeStop
  * CantDash
  * DisableHideCostume
  * FixTps
  * Exhibitionism
  * Sex
  * AutoAddMoisture
  * NoReinforceEffect
  * GanimataWalk
  * AhegaoDoublePiece
  * HipShake
  * GanimataHip
  * KaikyakuFella
  * Dogeza
  * DogTintin
  * IBalance
  * WakimiseCrouch
  * MituasiOnani
  * Tebura
  * PeeKaikyaku
  * PeeDog
  * ChikubiRotate
  * OnaniYotuashi
  * OnaniNeGanimata
  * OnaniNormal
  * NeKataashiage
  * OnaniArmKuri
  * Sikoru
  * GanimataKoshiHeko
  * Haigure
  * PeeStand

#### Item

  * None
  * Water
  * Dildo
  * InvisiblePotion
  * FutanariPotion
  * FutanariInversePotion
  * BodyPaint
  * InvisibleInversePotion
  * InvisiblePotionReusable
  * BodyPaintReusable
  * BodyPaintWasher
  * HandcuffKey
  * VibeRemocon
  * DroneController
  * DebugEarnRp
  * DebugEarnMental
  * DebugEarnRp2
  * DebugEarnExp
  * DebugEarnExp2

#### DropItemType

  * None
  * Coat
  * Hoodie
  * Basket
  * Pants
  * Bra
  * HandcuffKey
  * VibeRemocon
  * DildoFloor
  * DildoWall

#### Player Data

**String:**

  * CurrentHair 
    * None
    * Short
    * Bob
    * Wolf
    * SideBob
    * FairyTwin
    * HalfTwin
    * ElegantLong
    * PrincesWave
    * Pony
    * MafuHair
    * Lynn
    * Himari
    * Invert
    * Natsu
    * Nekomimi
    * Shu
    * Yuki
    * WavePony
    * LovelyMedium
    * StraightLong
    * WaveLong
    * StraightBob
  

**Integer:**

  * CustomizeArm
  * CustomizeBoobs
  * CustomizeButt
  * CustomizeCheekFat
  * CustomizeChinLength
  * CustomizeEarElf
  * CustomizeEyeBlowColorB
  * CustomizeEyeBlowColorG
  * CustomizeEyeBlowColorR
  * CustomizeEyeDist
  * CustomizeEyeHeight
  * CustomizeEyeLashColorB
  * CustomizeEyeLashColorG
  * CustomizeEyeLashColorR
  * CustomizeEyeLeftPattern
  * CustomizeEyeOpenness
  * CustomizeEyePattern
  * CustomizeEyeSize
  * CustomizeFutanariKitouSize
  * CustomizeFutanariSaoSize
  * CustomizeFutanariStraight
  * CustomizeFutanariTamaSize
  * CustomizeFutanariWidth
  * CustomizeHead
  * CustomizeHeight
  * CustomizeHitomiSize
  * CustomizeHyperBoobs
  * CustomizeInmou1
  * CustomizeInmou2
  * CustomizeInmouColorB
  * CustomizeInmouColorG
  * CustomizeInmouColorR
  * CustomizeLegInterval
  * CustomizeLoliFace
  * CustomizeLowerLeg
  * CustomizeLowerLegLength
  * CustomizeMatsugeLength
  * CustomizeMatsugeThick
  * CustomizeMayuDog
  * CustomizeMayuThick
  * CustomizeNeck
  * CustomizeNeckThick
  * CustomizeNipple
  * CustomizeShoulder
  * CustomizeSkinColor
  * CustomizeSkinColorB
  * CustomizeSkinColorG
  * CustomizeSkinColorR
  * CustomizeSuperBoobs
  * CustomizeTareme
  * CustomizeThighs
  * CustomizeThighsLength
  * CustomizeUpperBody
  * CustomizeUpperBody2
  * CustomizeUpperBodyLength
  * CustomizeUpperBodyLength2
  * CustomizeWaist

**Boolean:**

  * CustomIsSkinColorFree
  * CustomizeAdjustHeight
  * CustomizeEyeDark
  * CustomizeFutanariTamaVisible
  * CustomizeOddEye
  * CustomizeYaeba

**Special:**

  * BodyPaintTypeDict 
    * Inmon
    * Rakugaki
    * TattooDark
    * TattooFlower1
    * TattooFlower2
    * TattooLotus
    * TattooSatanism
    * TattooSlave
    * BodyOil
    * RubberSuit
  * HairCustomizeDataDict 
    * R
    * G
    * B
    * ShadeColor
    * TextureId

#### NPC

**AvatarType**

  * NewFemale
  * NewMale
  * NewOji
  * NewOba
  * PreviousFemale
  * PreviousMale

**FixedType**

  * None
  * Car
  * Talk
  * SitWork
  * SitTalk
  * SitSleep
  * SitBook
  * SitSumaho
  * StandSumaho
  * Pinpon
  * Conbini

### Mod Constants

#### Condition Types

  * CoatDropped
  * CoatFrontClosed
  * CoatFrontOpen1
  * CoatFrontOpen2
  * CoatBackClosed
  * CoatBackOpen
  * Blindfolded
  * NoHandcuffs
  * HandcuffsBack
  * HandcuffsObject
  * NormalHandcuffs
  * KeyedHandcuffs
  * TimedHandcuffs
  * Moving
  * Crouching
  * Peeing
  * InLight
  * Sitting
  * Dashing
  * Orgasm
  * Futanari

  * Invisible
  * InOpenToilet
  * Bukkake _(Cumming on NPC)_
  * NearNPC
  * Watched
  * ShowingOff
  * VibrationOff
  * VibrationLow
  * VibrationHigh
  * VibrationRandom
  * PistonOff
  * PistonLow
  * PistonMedium
  * PistonHigh
  * PistonRandom
  * IsDayTime
  * NPCArea
  * Bodypaint
  * FPCamera
  * GameOver

**Can be compared to numbers:**

  * Ecstasy
  * Detection
  * Rank
  * HeartRate
  * Stamina
  * StaminaMax
  * Moisture
  * Item_<Itemname>
  * MissionCompleted_<MissionID>
  * DroneMissionCompleted_<DroneMissionID>
  * MissionCurrCompleted_<MissionID>

**Category Conditions**  
(replace the _<...>_ part with the appropriate value)

  * Action_<Actionname>
  * Cosplay_<CosplayID>
  * OwnsCosplay_<CosplayID>
  * AdultToy_<AdultToy>
  * OwnsAdultToy_<AdultToy>
  * Item_<Itemname>
  * MissionCompleted_<MissionID>
  * DroneMissionCompleted_<DroneMissionID>
  * MissionCurrCompleted_<MissionID>
  * SubCondition_<ConditionID>
  * Skill_<Skillname>
  * Exposed_<Bodypart>
    * None
    * Front
    * Upper
    * HipCrouch
    * Hip
    * All

### Global Variables

#### _state

  * DateTime
  * Blindfolded
  * Peeing
  * Moving
  * Dashing
  * Crouching
  * Sitting
  * InLight
  * Orgasm
  * Bukkake
  * NearNPC
  * ShowingOff
  * Watched
  * Action
  * Futanari
  * Invisible
  * InOpenToilet
  * Bodypaint
  * DayTime
  * NPCArea
  * FirstPerson
  * Ecstasy
  * Detection
  * Rank
  * Vibrator
  * Piston
  * RpBonus
  * HeartRate
  * Stamina
  * StaminaMax
  * Moisture
  * FoundNPC (ID of NPC that found player, -1 otherwise)
  * GameOver
  * Handcuffs 
    * State
    * Type
  * Exposed 
    * None
    * Front
    * Upper
    * HipCrouch
    * Hip
    * All
  * Position 
    * stage
    * x
    * y
    * z
    * rx
    * ry
    * rz
    * rw
    * laststage
  * Camera 
    * stage
    * x
    * y
    * z
    * rx
    * ry
    * rz
    * rw
    * pitch
    * yaw
  * CameraTarget 
    * Face 
      * x
      * y
      * z
    * Body 
      * x
      * y
      * z
    * Crotch 
      * x
      * y
      * z
  * Cosplay 
    * Cosplay names are the keys of the list.
    * If a key is in the list the player owns the piece.
    * If the value is true the player wears the piece currently, false if not.
    * Also has numbered keys starting at 0 with values being the names of currently worn cosplay.
  * Skills 
    * Skill names are the keys of the list. Values are Boolean.
    * Only skills you can activate at the PC.
  * DroppedItems 
    * Numbered list of dropped items. Zero-based indices 
      * Type
      * Position 
        * stage
        * x
        * y
        * z
        * rx
        * ry
        * rz
        * rw
  * AdultToys 
    * Adult toys names are the keys of the list. Values are Boolean.
    * If a key is not in the list the player doesn't own it.
  * Items 
    * Item names are the keys of the list. Values are Numbers.
  * Missions 
    * Contains the number of clears of a mission by its ID.
    * Also contains "Completed" and "Count", indicating the number of different missions cleared and the total number of existing missions.
    * Also has separate lists for each stage. 
          
          stage = "Residence"
          Log(Format("Cleared Missions in \"{0}\": {1}/{2}", stage, _state.Missions[stage].Completed, _state.Missions[stage].Count))

  * CurrMissions 
    * Contains the current progress (0 to 1) on a mission by its ID.
    * Also contains "Completed" and "Count", indicating the number of missions cleared and the total number of existing missions.
    * Also has separate lists for each stage.
  * Coat 
    * Dropped 
      * possible values (Boolean): true, false
    * Front 
      * possible values (String): Closed, Open1, Open2, None
    * Back 
      * possible values (String): Closed, Open, None
  * NPCs 
    * Numbered list of NPcs in the current stage
    * Zero-based indices
    * Each item contains the following values 
      * ID
      * Position 
        * stage
        * x
        * y
        * z
        * rx
        * ry
        * rz
        * rw
      * SeesPlayer
      * SeesFlashing
      * Headset
      * Glasses
      * Smartphone
      * AvatarType
      * FixedType
      * Sitting
      * State

