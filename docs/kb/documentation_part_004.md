<!-- source: docs\documentation.html -->
<!-- part: 4 -->
<!-- line_range: 2640-3606 -->

### Common

  * Objects are automatically destroyed when their reference count is zero

#### Methods

FunctionExists(FunctionName)  

returns: Boolean

  * Checks if an object has a function
  * Only works for functions defined by the engine or extensions, not those defined in scripts.

### List

#### Constructors

CreateList([Name1 = Value1][, Name2 = Value2]...) or  
CreateList([Value1][, Value2]...)  
or a combination of both with named and unnamed parameters

returns: List

  * Lists are a collection of values
  * Values can be accessed in different ways 
        
        list1 = CreateList()
        list1["name"] = "Player"
        Log(list1.name)

  * All indices are converted to strings
  * For integer indices there are additional functions
    * The list can still have other indices

Copy(ListToCopy[, deepCopy])  

returns: List

  
CreateListFromJson(file = FilePath)  

returns: List

#### Methods

Insert([index = InsertAt][, Value1][, Value2]...)  

returns: null

  * If index is not defined, values will be appended at the end

Remove([index = RemoveFrom][, count = Count])  

returns: null

  * If index is not defined, values will be removed from the end.
  * If count is not defined, count will be 1.
  * Values after the removed ones will be moved to the free indices

Count()  

returns: Number

  * If there is a gap in the indices starting from 0, the return value will only be the index of the gap

Contains(Value)  

returns: Boolean

  * Returns true if the list contains Value, otherwise false.

IndexOf(Value)  

returns: String or null

  * Returns the key of the first occurrence of Value in the list if present, otherwise null.
  * The keys are always strings even for items that have been set with a number as index because they are converted and stored as strings internally.

Keys([IncludeAll])  

returns: List

  * Returns a new list containing the keys of the list.
  * By default organized number indices are **not included** because you can just use a loop to iterate them. If you need them, pass true as parameter.
  * The indices of the returned list start with 0 so you can iterate over them. 
        
        list = CreateList(key1 = "1", key2 = "2")
        keys = list.Keys()
        i = 0
        while i < keys.Count()
        	Log(keys[i])
        	i+=1
        
        Output:
        key1
        key2

### Thread

#### Constructors

CreateThread(labelName[, Named_Or_Unnamed_Parameter]...)  

returns: Object

  * When creating a new thread their code is run once.
  * After changing the label with Goto, the code of the specified label is run once in the next frame.
  * Use Listeners to wait for conditions and call Goto to progress to the next label.
  * When the label is changed the old context gets destroyed and references to Objects deleted.

#### Methods

Goto(labelName[, Named_Or_Unnamed_Parameter]...)  

returns: null

    
    
    thread = CreateThread("m1")
    
    m1:
    	Log("start timer")
    	listener = CreateListener("l", waittime = 5)
    	l:
    		if duration == null
    			duration = 0
    		duration = duration + _timediff
    		if duration>=waittime
    			thread.Goto("m2")
    m2:
    	Log("5 seconds have passed")

GetLabel()  

returns: String

  
Calling user-defined functions:  

    
    
    test = CreateThread("m")
    test.Output() Output: Hello World!
    
    m:
    	text = "Hello World!"
    	Output:
    		Log(text)
    

### Listener

#### Constructors

CreateListener(labelName[, Named_Or_Unnamed_Parameter]...)  
CreateListenerLocal(labelName[, Named_Or_Unnamed_Parameter]...)  

returns: Object

  * Listeners are similar to Threads, but their code is run every frame
  * Variables keep their values for the next frame
  * There are two functions to create a listener: CreateListener and CreateListenerLocal
  * CreateListener creates the listener within the scope where the code is defined.
  * CreateListenerLocal creates the listener within the scope where you called the function. 
    * That makes it easier to create shared code without needing to pass all needed variables.
    * But when the parent scope gets destroyed so will the listener. Even if there is still a reference.

    
    
    thread = CreateThread("m1")
    
    m1:
    	localvariable = "local1"
    	listener = CreateListener("l", nextcheckpoint = "m2")
    	l:
    		Log(localvariable)
    		thread.Goto(nextcheckpoint)
    m2:
    	localvariable = "local2"
    	listener = CreateListener("lglobal", nextcheckpoint = "m3")
    m3:
    	localvariable = "local3"
    	listener = CreateListenerLocal("lglobal", nextcheckpoint = "m4")
    m4:
    	thread = null
    
    lglobal:
    	Log(localvariable)
    	thread.Goto(nextcheckpoint)
    
    Output:
    local1
    null
    local3

### EventListener

#### Constructors

CreateEventListener(LabelName, EventName[, Named_Or_Unnamed_Parameter]...)  
CreateEventListenerLocal(LabelName, EventName[,
Named_Or_Unnamed_Parameter]...)  

returns: Object

  * Similar to listener but only runs when an event is set with SetEvent
  * The associated label is executed instantly after setting an event.
  * That means that it can be run during the execution of another mission and other variables might not be up to date.
  * The local variable __eventdata_ contains the data that is supplied by a single SetEvent.
  * The local variable __eventname_ contains the name of the event (helpful in case you use the same code for multiple events)

### MissionPanel

#### Constructors

CreateMissionPanel()  

returns: Object

#### Methods

SetText(Text)  

returns: null

  
SetRPText(Text)  

returns: null

  
SetVisible(Visible)  

returns: null

  
SetGaugeVisible(Visible)  

returns: null

  
SetGaugeProgress(Progress)  

returns: null

  
GetText()  

returns: String

  
GetRPText()  

returns: String

  
GetVisible()  

returns: Boolean

  
GetGaugeVisible()  

returns: Boolean

  
GetGaugeProgress()  

returns: Number

### MissionMenuItem

#### Constructors

CreateMissionMenuItem()  

returns: Object

  * Creates an item for the Mission Menu (the one when you press Tab and click Missions)

#### Methods

SetText(Text)  

returns: null

  
GetText()  

returns: String

  
SetRPText(Text)  

returns: null

  
GetRPText()  

returns: String

  
SetCleared(ClearedMarker)  

returns: null

  
GetCleared()  

returns: Boolean

  
SetMark(Mark)  

returns: null

  
GetMark()  

returns: Boolean

  
SetClears(Clears)  
SetClears(Clears)  

returns: null

  
GetClears()  

returns: String

  
SetMaxRP(MaxRP)  
SetMaxRP(MaxRP)  

returns: null

  
GetMaxRP()  

returns: String

  
AutoColor(Value)  

returns: Boolean

  * If set to true, SetMaxRP (with a Number parameter) also changes the background and text color according to the games color scheme.

SetBackgroundColor(Color1, Color2)  

returns: null

  * There are two colors because it is a gradient
  * The values used by the base game are: 
    * White: Color(1.000, 1.000, 1.000, 1.000), Color(0.358, 0.358, 0.358, 1.000)
    * Green: Color(0.495, 1.000, 0.521, 1.000), Color(0.000, 0.434, 0.024, 1.000)
    * Blue: Color(0.429, 0.644, 1.000, 1.000), Color(0.000, 0.379, 1.000, 1.000)
    * Purple: Color(0.788, 0.458, 1.000, 1.000), Color(0.607, 0.000, 1.000, 1.000)
    * Gold: Color(1.000, 0.751, 0.325, 1.000), Color(0.604, 0.378, 0.000, 1.000)
    * Red: Color(1.000, 0.306, 0.306, 1.000), Color(0.651, 0.000, 0.000, 1.000)

SetMaxRPColor(Color1, Color2)  

returns: null

  * There are two colors because it is a gradient

SetStages(Stage1,[Stage2]...)  
SetStages(Stages)  

returns: null

### Area

#### Constructors

CreateArea(type = "sphere", stage = StageName, x = CenterX, y = CenterY, z =
CenterZ, r = Radius[, outline = OutlineVisible][, compass =
CompassIconVisible])  
CreateArea(type = "sphere", stage = StageName, x = CenterX, y = CenterY, z =
CenterZ, r = Radius[, outline = OutlineVisible][, compass =
CompassIconFilename])  
CreateArea(type = "cylinder", stage = StageName, x = CenterX, y = CenterY, z =
CenterZ, r = Radius, h = Height[, outline = OutlineVisible][, compass =
CompassIconVisible])  
CreateArea(type = "cylinder", stage = StageName, x = CenterX, y = CenterY, z =
CenterZ, r = Radius, h = Height[, outline = OutlineVisible][, compass =
CompassIconFilename])  
CreateArea(type = "cuboid", stage = StageName, x1 = StartX, y1 = StartY, z1 =
StartZ, x2 = EndX, y2 = EndY, z2 = EndZ, w = Width, h = Height[, outline =
OutlineVisible][, compass = CompassIconVisible])  
CreateArea(type = "cuboid", stage = StageName, x1 = StartX, y1 = StartY, z1 =
StartZ, x2 = EndX, y2 = EndY, z2 = EndZ, w = Width, h = Height[, outline =
OutlineVisible][, compass = CompassIconFilename])  

returns: Object

  * The parameter "compass" can be either a Boolean or a String 
    * The default value is True
    * If you pass a string containing the path of an image (relative to your projects folder) you can set the compass icon to a custom image.

#### Methods

SetVisible(Visible)  

returns: null

  
SetColor(Red, Green, Blue[, Alpha])  
SetColor(Color)  

returns: null

  
SetOutline(Visible)  

returns: null

  
SetCompass(Visible)  

returns: null

  
Inside()  
Inside(stage = StageName, x = PositionX, y = PositionY, z = PositionZ)  
Inside(Position)  

returns: Boolean

  
Distance()  
Distance(stage = StageName, x = PositionX, y = PositionY, z = PositionZ)  
Distance(Position)  

returns: Number

### Zone

#### Constructors

CreateZone([Area1][, Area2]...)  
CreateZone([ListOfAreas])  

returns: Object

#### Methods

SetVisible(Visible)  

returns: null

  
SetColor(Red, Green, Blue[, Alpha])  
SetColor(Color)  

returns: null

  
Inside()  
Inside(stage = StageName, x = PositionX, y = PositionY, z = PositionZ)  
Inside(Position)  

returns: Boolean

  * If no position is specified the player position is used.

DistanceToLastPosition()  
DistanceToLastPosition(stage = StageName, x = PositionX, y = PositionY, z =
PositionZ)  
DistanceToLastPosition(Position)  

returns: Number

  * Returns the distance to the last area of the zone the player has been in.
  * **The last area is only updated when "Inside()" is called.**

DistanceToNearest()  
DistanceToNearest(stage = StageName, x = PositionX, y = PositionY, z =
PositionZ)  
DistanceToNearest(Position)  

returns: Number

### Condition

#### Constructors

CreateCondition(Condition[, id = ID])  

returns: Object

  * Conditions use a short and simple syntax to create more complex combinations of the player states that can also be accessed with the _state global variable.
  * For a list of all condition types see Appendix Conditions
  * To combine multiple condition types with a logical AND use square brackets around a comma-separated list
  * To combine multiple condition types with a logical OR use round brackets around a comma-separated list
  * To negate a value put an exclamation mark in front of it
  * There is no XOR, but you could recreate the effect by combining the other operators

Examples:

    
    
    Creates a condition where you have to flash your front and back, but not be completely naked.
    Square brackets means that all of the condition must be true, and the exclamation mark negates Exposed_All, so it must be false to satisfy the overall requirement.
    condition1 = CreateCondition("[Exposed_Front,Exposed_Hip,!Exposed_All]")
    
    Creates a condition where you have to be naked, use an activated clitoris rotor and do an action with a wall-mounted dildo.
    condition2 = CreateCondition("[Exposed_All,AdultToy_KuriRotor,(VibrationLow,VibrationHigh,VibrationRandom),(Action_UseDildoWallPussy1,Action_UseDildoWallAnal1,Action_UseDildoWallFella1)]")
    

  * There are conditions for Actions, Cosplay, Missions etc. that use a prefix followed by the name or id as it is used by the game.
  * To get a list of those press Ctrl+F9 while in the game and check the BepInEx console window.
  

  * Certain conditions can be compared to numbers with operators >, >=, <, <=, ==, !=
  

  * You can also set an ID when creating a condition and use it as a subcondition with the condition type "Subcondition" in another.
  * This also works for ItemConditions.
  * You should use prepared conditions whenever you can and the condition is more complex than a single comparison of a value in the _state variable.
  * Avoid creating conditions in listeners and instead pass them to the listener when you create it, for more performance.

#### Methods

Check()  

returns: Boolean

### ItemCondition

#### Constructors

CreateItemCondition(itemtype = ItemType, zone = Zone[, id = ID])  
CreateItemCondition(itemtype = ItemType, area = Area[, id = ID])  

returns: Object

#### Methods

Check()  

returns: Boolean

### InteractArea

#### Constructors

CreateInteractArea(stage = StageName, x = CenterX, y = CenterY, z = CenterZ, r
= Radius, text = Text[, options = OptionTexts])  

returns: Object

  * "options" (if defined) must be a list of strings with indices starting from 0.

#### Methods

Check()  

returns: Boolean or Number or null

  * If the InteractArea was **not** created with options, return value will be a boolean indication wether it was interacted with this frame.
  * If the InteractArea has defined options, the return value will be the index of the choosen option or **null, if not interacted with**.

### Text

#### Constructors

CreateText()  

returns: Object

#### Methods

SetFace([color = FaceColor][, dilate = FaceDilate])  

returns: null

  
SetOutline([color = OutlineColor][, width = OutlineWidth])  

returns: null

  
SetUnderlay([color = UnderlayColor][, dilate = UnderlayDilate][, softness =
UnderlaySoftness][, offsetx = UnderlayOffsetX][, offsety = UnderlayOffsetY])  

returns: null

  
SetAnchor(AnchorX, AnchorY)  

returns: null

  * Values range from 0 to 1
  * The origin is on the bottom left

SetAlignment(Alignment)  

returns: null

  * Alignment Values: TopLeft, Top, TopRight, Right, BottomRight, Bottom, BottomLeft, Left, Center

SetSize(Size)  

returns: null

  
SetWidth(Width)  

returns: null

  
SetFrontLayer(InFrontlayer)  

returns: null

  * Text in the front layer is displayed over a blackscreen

Clear()  

returns: null

  * Clear display queue
  * Especially useful when there is text that doesn't expire

Add([text = Text][, delay = Delay][, fadein = FadeIn][, fadeout = FadeOut][,
duration = Duration]  

[, facecolor = FaceColor][, facedilate = FaceDilate]  

[, outlinecolor = OutlineColor][, outlinewidth = OutlineWidth]  

[, underlaycolor = UnderlayColor][, underlaydilate = UnderlayDilate][,
underlaysoftness = UnderlaySoftness][, underlayoffsetx = UnderlayOffsetX][,
underlayoffsety = UnderlayOffsetY]  

[, size = Size][, alignment = Alignment][, anchorx = AnchorX][, anchory =
AnchorY][, width = Width]  

[ListOfNamedParameters])  

returns: null

  * Adds a text to the display queue
  * A duration < 0 means infinite and the text is only removed when Clear() is called.

### Messenger Chat

#### Constructors

CreateMessengerChat(Title)  
CreateMessengerChat(Title[, icontext = IconText][, icontextcolor =
IconTextColor][, iconcolor = IconColor])  
CreateMessengerChat(Title[, iconfilename = IconFilename][, iconcolor =
IconBorderColor])  

returns: Object

  * Default IconColor is blue. Default IconTextColor is white.
  * IconText should be something short or the text becomes unreadable small.

#### Methods

Add(Text, Orientation[, user = Username[, usercolor = UsernameColor]][, silent
= SilentAdd])  

returns: null

  * Orientation must be either "Left" or "Right"
  * Username can be defined to create group chats
  * Set "silent" to true to add lines without triggering the new messages notification. (Can be useful when you want to load a saved progress.)

Clear()  

returns: null

  
SetButtons(Caption1[, Caption2]... [, ids = ButtonIds])  
SetButtons(Captions[, ids = ButtonIds])  

returns: null

  * Takes unnamed string parameters or a list of strings and creates buttons for the Chat
  * With the list "ids" you can specify custom return values for the "Clicked" method

Clicked()  

returns: null or Number or Value

  * If no button was pressed last frame, returns null
  * If a button was pressed and "ids" was set, returns the respective value
  * Otherwise returns the number of the button (numbers start at 0)

Opened()  

returns: Boolean

  * Returns true if the chat is currently visible.

### Audio

#### Constructors

CreateAudio(FilePath)  

returns: Object

  * FilePath is relative to your projects folder
  * Supported file formats are WAVE (.wav), Ogg Vorbis (.ogg) and MP3 (.mp3)

#### Methods

Play([volume = Volume][, x = PositionX, y = PositionY, z = PositionZ])  

returns: Number

  * Volume is between 0 and 1
  * If you don't need 3D sound you can omit the x,y,z values
  * Returns an id which can be used for functions like StopAudio

Length()  

returns: Number

  * Returns the audio clip's length in seconds

### Gallery

#### Constructors

CreateGallery([CallbackFunction][,Condition][,Area][,Zone])  

returns: Object

  * The Gallery object is used to display and select from a filtered list of your photos.
  * Should be triggered from a button in the Messenger to make sure the player is actually looking at the smart phone.
  * The callback function gets called for each image in the gallery. 
    * It gets passed a _data_ variable that contains the meta data of the image
    * If you want to show the image in the gallery set __result_ to true
  * You can also use Conditions and Areas or Zones
    * All must be true to add an image to the gallery

#### Methods

Show([multiselect = MultiSelect])  

returns: null

  * By default multiselect is off.

Confirmed()  

returns: Boolean

  * Returns true when selections was confirmed

GetSelection()  

returns: List

  * Returns a List of the selected image references

### Snapshot

#### Constructors

CreateSnapshot(position = Position, direction = Direction[, width = Width,
height = Height][, fov = FieldOfView])  

returns: Object

  * Creates a camera to take a snapshot
  * You have to wait for the next frame to get the game to render before you can save the image
  * Also see Additional Game Functions for additional snapshot functions
  * Destroy the object by removing all references after you've saved the snapshot. Otherwise it will impact performance.

#### Methods

Save([hidden = Hidden])  

returns: String

  * Returns the image reference
  * Hidden images do not show up in the gallery but you can use them in the Messenger

### NPC

#### Constructors

CreateNPC(AvatarType, position = Position, rotation = Rotation[, body =
BodyIndex][, hair = HairIndex][, face = FaceIndex][, size = Size])  
CreateNPC(ID)  

returns: Object

  * Creates an NPC of the specified type at the position
  * By default NPC stand still, but react to the player
  * There is a different number of available clothes (BodyIndex) for each AvatarType
  * The NPC gets removed when you change the stage, but the NPC object is still there and you can respawn them
  * A size around 1 is normal.
  * Depending on the type there are different numbers of bodies/clothes, hair styles and faces available:  AvatarType| # Bodies| # Hairs| # Faces  
---|---|---|---  
NewFemale| 11| 10| 11  
NewMale| 5| 6| 5  
PreviousFemale| 15| 7| 1  
PreviousMale| 12| 5| 1  
NewOba| 2| 1| 1  
NewOji| 3| 1| 1  
  

  * Use the contructor with ID to connect to an existing NPC like the ones created by the game
  * Not all functions are available for NPCs created by the game itself. Like changing the route, stopping or respawning.

#### Methods

IsAlive()  

returns: Boolean

  * Returns if the NPC does still exist

Respawn([position = Position][, rotation = Rotation])  

returns: null

  * Respawn the NPC with the same appearance and position
  * You can specify the position/rotation to change it from the original

Warp([position = Position][, rotation = Rotation])  

returns: null

  * Immediately change the position of the NPC

AddWaypoint(Position[, Rotation][, last = Last])  
AddWaypoint(WaypointIndex[, last = Last])  

returns: null

  * Lets the NPC walk to a position.
  * If there are already points added before, the NPC will pass them in order
  * If you add a waypoint with "last = true" the NPC will despawn after reaching that point
  * You can also add some of the waypoints that are connected to things like chairs or vending machines by using their index as parameter 
    * For a list see GetAllWaypoints function

ClearWaypoints()  

returns: null

  * Removes all waypoints and lets the NPC stop

Stopped([NewStopped])  

returns: Boolean or null

  * Stops the NPC without removing waypoints or lets them walk again
  * Returns if NPC is stopped or null if it is not alive

Finished()  

returns: Boolean or null

  * Returns if the NPC reached the end of its path or null if it is not alive

GetID()  

returns: Number

  * Returns the ID of the NPC
  * The ID will change if they are respawned

GetType()  

returns: String

  
Strangeness([NewStrangeness])  

returns: Number or null

  * If parameter is set, changes the NPC's alertness (called strangeness in the game code)
  * Returns the NPC's current alertness

SkipStrangeness([NewSkip])  

returns: Boolean or null

  * If parameter is set, changes if the NPC reacts to your exposure
  * Returns if NPC holds a smartphone or null if it is not alive

Smartphone([NewSmartphone])  

returns: Boolean or null

  * If parameter is set, changes if the NPC holds a smartphone or not
  * Returns if NPC holds a smartphone or null if it is not alive

Headset([NewHeadset])  

returns: Boolean or null

  * If parameter is set, changes if the NPC wears a headset or not
  * Returns if NPC wears a headset or null if it is not alive
  * **AvatarTypes _NewOba_ and _NewOji_ have no headsets**

Glasses([NewStopped])  

returns: Boolean or null

  * If parameter is set, changes if the NPC wears glasses or not
  * Returns if NPC wears glasses or null if it is not alive
  * **AvatarTypes _NewOba_ , _NewOji_ , _PreviousFemale_ and _PreviousMale_ have no glasses or can not toggle them**

Size([NewSize])  

returns: Number or null

  * If parameter is set, changes the size of the NPC
  * Returns the size of the NPC or null if it is not alive

Speed([NewWalkingSpeed])  

returns: Number or null

  * If parameter is set, changes the walking speed of the NPC
  * **The animation speed does not get adjusted!**
  * Returns the walking speed of the NPC or null if it is not alive

Penis([PenisState])  

returns: Number or null

  * If parameter is set, changes the visibility of the NPCs penis
  * PenisState values can be one of the following integers: 
    * 0 - penis hidden
    * 1 - flaccid penis
    * 2 - erect penis
  * Returns the current penis state of the NPC or null if it is not alive

PenisScale([Scale])  

returns: Number or null

  * If parameter is set, changes the scale of the NPC's penis
  * Returns the current scale of the NPC's penis or null if it is not alive

SeesPlayer()  

returns: Boolean or null

  
SeesFlashing()  

returns: Boolean or null

  
GetPosition()  

returns: List or null

  
GetState()  

returns: String or null

  * The state as per the game is one of those values: Idle, FoundStranger, TurnAround, FadeOut, Stagger, Interact, WaitToken, TraceSexPlayer

ActivateSex()  

returns: null

  * To also change the sex position use the global function SetSexPosition
  * To deactivate you can use the global function DeactivateSex
  * Also works with Seduction skill turned off

TracePlayer([Value])  

returns: Boolean or null

  * Turns following the player on or off using the games built-in system (that's used by the seduction skill)

CanGameOver([Value])  

returns: Boolean

  * If set to false, prevents a game over when found by this NPC
  * You can get the ID of the NPC that found the player (even if no game over is triggered) with _state.FoundNPC 
    * It triggers only once per NPC and is only valid for one frame
    * _state.FoundNPC is -1 if not triggered

PlayAction([ActionName])  

returns: null

  * Plays an animation for the NPC. Some are one time only, others loop and have to be disabled.
  * List of Actions:  ActionName| Notes  
---|---  
NoAction **or** None| reset or cancel action  
HandOver|  
BuyMachine|  
ConbiniCrouch| use ConbiniStand to stand back up, although NoAction also
works, but skips animation  
ConbiniRegiScan| continuous  
ConbiniStand|  
ConbiniTakeGoods|  
FoundStranger| continuous  
HandWash| continuous  
Worry| continuous  
MalePeeing| continuous  

### Input

#### Constructors

CreateInput(Button[, modifier = ModifierButton][, interaction = Interaction])  
CreateInput(Button[, modifier1 = ModifierButton1, modifier2 =
ModifierButton2][, interaction = Interaction])  

returns: Object

  * Creates an object to check for user inputs
  * Button, ModifierButton and Interaction are strings and follow the syntax Unity uses for its InputSystem

Examples:

    
    
    input1 = CreateInput("<Keyboard>/space", interaction="hold(duration=0.5)")
    input2 = CreateInput("<Keyboard>/F", modifier="<Keyboard>/shift")
    listener = CreateListener("waitforinput")
    
    waitforinput:
    	if input1.WasPerformed()
    		Log("Long press [Space]")
    	if input2.WasPerformed()
    		Log("Pressed [Shift]+[F]")

#### Methods

WasPressed()  

returns: Boolean

  * Returns true if the button and modifiers were pressed down this frame.
  * Only triggers when at least one button wasn't pressed before. So it can't trigger again until you release a button.
  * If you have defined an interaction, this function will not account for that. See "WasPerformed()"

WasReleased()  

returns: Boolean

  * Returns true if a button is released and all required buttons were pressed before.

WasPerformed()  

returns: Boolean

  * If you have defined an interaction that, for instance, requires a button to be held, this returns true after the required duration has passed.
  * For more information on other interactions see the Unity documentation.

IsPressed()  

returns: Boolean

  * Returns true if the button and modifiers (if any) are currently all pressed.
  * While "WasPressed" only triggers once, this function will trigger every frame as long as all required buttons are pressed.

## Inter-Mod Tutorial

