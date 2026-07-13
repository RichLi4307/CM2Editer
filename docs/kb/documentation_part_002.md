<!-- source: docs\documentation.html -->
<!-- part: 2 -->
<!-- line_range: 278-579 -->

# API

## General

### File Structure

  * Each project has its own folder inside the "CustomMissions2" folder, that itself is in the game's root directory.

#### Code

  * A project can be split into multiple files with the ending ".code".
  * All code files get merged into a single one when they are loaded.  
Probably in alphabetical order, but you shouldn't rely on it and instead make
sure that they can function in arbitrary order.

  * The encoding should be UTF-8 to make sure that string characters are correctly imported into the game.

#### Meta

  * You can include a "meta.json" in your project to specify the title in different languages, add a description and build a simple settings menu.
  * Settings values can be read in code from the _settings global variable.

    
    
    {
    	"title":{
    		"En":"Test",
    		"Ja":"テスト"
    	},
    	"description":{
    		"En":"This is a description"
    	},
    	"settings":[
    		{
    			"type":"Label",
    			"title":"These settings don't do anything"
    		},
    		{
    			"name":"range",
    			"title":"Range Integer",
    			"type":"Integer",
    			"minvalue":0,
    			"maxvalue":100,
    			"default":50
    		},
    		{
    			"name":"range2",
    			"title":"Range Float",
    			"type":"Float",
    			"minvalue":0,
    			"maxvalue":5,
    			"default":5
    		},
    		{
    			"name":"checked",
    			"title":"Boolean Value",
    			"type":"Boolean",
    			"default":true
    		},
    		{
    			"name":"text",
    			"title":"Text Input",
    			"type":"String",
    			"default":""
    		},
    		{
    			"name":"enum",
    			"title":"Options",
    			"type":"Enum",
    			"options":[
    				"Option 1",
    				"Option 2",
    				"Option 3"
    			],
    			"default":0
    		}
    	]
    }

  * _title_ , _description_ and the options for an _Enum_ must be either a normal string or an object with elements named after the language codes. (See also GetLanguage)
  * If you don't specify a translation for a language it will default to the first in the list. (In the example "En")
  * This also works for the settings.

  * _settings_ is an optional array of objects.
  * The _name_ of a setting determines under which name you can access it in code with the global _settings list. (like "_settings.range")
  * The available settings types are: Label, Integer, Float, Boolean, String, Enum
  * _Integer_ and _Float_ create a slider, so you should also specify _minvalue_ and _maxvalue_
  * _Enum_ creates a selector where you can cycle through the options, but the value in _settings will be a number
  * _Boolean_ creates a checkbox, _String_ a one line text input field.
  * _Label_ only adds a description to the settings menu, so it doesn't need a _name_.
  * _default_ specifies the initial value in the respective type.
  * Settings are saved per savegame, when the game is saved.

### Types

  * Number
  * String
  * Boolean
  * List
  * Object

### Built-In Constants

  * true
  * false
  * null

### Built-In Global Variables

  * _state 
    * List with a lot of values like condition states, position, missions, items and cosplay. See Appendix
    * Don't use this to change values, but use the available functions
    * The list only gets updated before each frame. So when you change a value by other means, it will not be reflected in the list immediately.
  * _stagechanged 
    * If stage changed in between this and the last frame
  * _timediff 
    * Ingame time passed since last frame
    * Takes into account when time is slowed or paused
  * _time 
    * Cumulative time with slows and pauses accounted for
  * _save 
    * Write and read the items of this list to create a persistent save state.
    * Objects (aside from Lists) can not be saved
  * _settings 
    * Read the values of the settings that you can change with the smartphone GUI.
    * See Meta on how to set up the menu.
  * _mod 
    * A list that can be filled with data that can be accessed by other projects.
    * Objects (other than lists) are not allowed and will be removed.
    * To access the data by another project see "_mods".
  * _mods 
    * A list of all active projects.
    * Items with a number as index contain the project names (folder names) as strings, so you can iterate over all active projects.
    * Items with project names as indices are lists that contain the data that is also accessible by _mod for the respective project.
    * Objects (other than lists) can not be shared and will be removed.
  * _name 
    * A projects own name as it is used in _mods.

### Built-In Local Variables

  * _this 
    * Reference to the current thread

### Operators

Exponentiation| **  
---|---  
Logical negation| !  
Unary plus and minus| +, -  
Multiplication, division, integer division (floor division), remainder
(modulo)| *, /, //, %  
Addition and subtraction| +, -  
IN| in  
Less than, less than or equal, greater than, greater than or equal| <, <=, >,
>=  
Equal, not equal| ==, !=  
AND| &  
XOR| ^  
OR| |  
Logical AND| &&  
Logical OR| ||  
Assignment| =, +=, -=, *=, /=  
  
The difference between & and && is that & always evaluates both expressions
and && the second only when necessary.  
Same for | and || accordingly. 
    
    
    if (list != null) & (list[0] == 1) crashes if list is not defined
    	Log("Test")
    if (list != null) && (list[0] == 1) works
    	Log("Test")
    if list != null also works, but harder to add an else/elseif if needed
    	if list[0] == 1
    		Log("Test")	

### Control Structures

#### Code indentation

  * Code indentation is used to determine the structure of the code
  * Use a single Tabulator in the lines after labels or If statements to indicate that the code belongs to them

#### labels

  * Labels are used to create custom functions and as entrypoints for threads
  * After the label name follows a colon (":")
  * Unlike Python, If statements and While loops do **not** use colons
  * Labels can be nested

    
    
    Log(welcome_msg(name = "Player"))
    
    welcome_msg:
    	_result = "Welcome "+name
    						

#### Variable scope

  * Functions and threads create their own scope for local variables
  * Functions and threads can access variables defined in a higher scope
  * If you pass a parameter with a name to a function or thread, it hides variables from a higher scope with the same name
  * Multiple threads that use the same label create their own instance of variables, but share the variables from their parent scope

    
    
    name = "Player"
    function1(id=1)
    function1(id=2)
    
    function1:
    	sqr = id*id
    	Log(name+id+": sqr="+sqr)
    
    Output:
    Player1: sqr=1
    Player2: sqr=4

#### if, elseif, else

    
    
    if x == 0
    	Log("zero")
    elseif x < 0
    	Log("less than zero")
    else
    	Log("greater than zero")

#### while

  * You can leave a loop early by using the _break_ statement

    
    
    Log("count down")
    i = 10
    while i>=0
    	Log(i)
    	i = i-1

#### for

  * Iterates over the values in a list
  * To iterate over a number range you can use the Range function to create a temporary list.
  * You can leave a loop early by using the _break_ statement

    
    
    for i in Range(5,10)
    	Log(i)
    
    Output:
    5
    6
    7
    8
    9

### Function Calls

  * Functions are defined with labels.
  * calls are made like: labelname(Parameter1, Parameter2, ...)
  * Parameters can be named or unnamed
  * Unnamed parameters are accessed with the list __args_.
  * A function returns the value of variable _result (if not set _result is null).
  * You can leave a early function with the _return_ statement. If you follow _return_ with an expression, it implicitly sets _result to that value

    
    
    variable1 = function_add(2, 3)
    Log(variable1) Output: 5
    variable2 = function_mult(a=2, b=3)
    Log(variable2) Output: 6
    
    function_add:
    	_result = _args[0]+_args[1]
    
    function_mult:
    	return a * b

## Functions

