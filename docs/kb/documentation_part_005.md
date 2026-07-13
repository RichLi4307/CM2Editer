<!-- source: docs\documentation.html -->
<!-- part: 5 -->
<!-- line_range: 3607-3781 -->

  * This section is for people writing their own mods (as dll) and wanting to use or extend Custom Mission 2 capabilities.
  * You should probably set at least a SoftDependency to guarantee that the Custom Missions Plugin is initialized before yours. 
        
        ...
        [BepInPlugin(MyPluginInfo.PLUGIN_GUID, MyPluginInfo.PLUGIN_NAME, MyPluginInfo.PLUGIN_VERSION)]
        [BepInDependency("SFM_custom_mission_v2", BepInDependency.DependencyFlags.SoftDependency)]
        public class Plugin : BasePlugin
        {
        ...

  * To check if the Custom Mission mod is loaded with the right version: 
        
        if (IL2CPPChainloader.Instance.Plugins.ContainsKey("SFM_custom_mission_v2") && (IL2CPPChainloader.Instance.Plugins["SFM_custom_mission_v2"].Metadata.Version >= new SemanticVersioning.Version(2,2,0)))
        {
        	...
        }

### Creating a settings menu

  * You can create an entry for your mod inside the phone UI that Custom Mission 2 creates 
        
        using SFM_custom_mission_v2.Script;
        ...
        	static ProgramSettings Settings;
            static float RValue;
            static float GValue;
            static float BValue;
            static bool active;
        
        	void CreateSettingsMenu()
        	{
        		Settings = new ProgramSettings();
        		Settings.title = new LocalizedString("My Addon Name");
        		Settings.active = active;
        		Settings.OnChange += this.HandleOnChange;
        
        		Settings.Add(new ProgramSettings.Setting("ColorR", new LocalizedString("Red"), RValue, 0, 255));
        		Settings.Add(new ProgramSettings.Setting("ColorG", new LocalizedString("Green"), GValue, 0, 255));
        		Settings.Add(new ProgramSettings.Setting("ColorB", new LocalizedString("Blue"), BValue, 0, 255));
        	}
        	
        	void HandleOnChange()
        	{
        		RValue = (float)Settings["ColorR"].Value.ToFloat();
        		GValue = (float)Settings["ColorG"].Value.ToFloat();
        		BValue = (float)Settings["ColorB"].Value.ToFloat();
        		active = Settings.active;
        		...
        	}

  * To set a language setting specific value for a LocalizedString you have to set them separately for each language 
        
        using ExposureUnnoticed2.Scripts.Base;
        ...
        
        var title = new LocalizedString("DefaultTitle");
        title[Localizer.Language.En] = "Title In English";
        title[Localizer.Language.Ja] = "Title In Japanese";
        ...
        

  * You have to handle loading and saving your settings yourself. You can use the BepInEx ConfigFile class for that. 
        
        using BepInEx.Configuration;
        ...
        
        	private static ConfigEntry<bool> Config_Set;
        	private static ConfigEntry<float> Config_R;
        	private static ConfigEntry<float> Config_G;
        	private static ConfigEntry<float> Config_B;
        
        	public override void Load()
        	{
        		...
        
        		Config_Set = Config.Bind("General", "ColorSet", false); 
        		Config_R = Config.Bind("General", "ColorR", 0.0f); 
        		Config_G = Config.Bind("General", "ColorG", 0.0f);
        		Config_B = Config.Bind("General", "ColorB", 0.0f);
        		...
        	}

### Extending the script language

  * To create your own functions that can be used in a script use _Engine.RegisterFunction_. 
    * Functions must have a _ProgramVariables_ and _ProgramThreadBase_ parameter (though the thread is seldomly used) and return a _ProgramValue_.
  * To create new methods for objects use _ProgramObject.RegisterFunction_. 
    * Additionally to _ProgramVariables_ and _ProgramThreadBase_ there is also a _ProgramObject_ parameter pointing to the instance for which the method was called.
  * _ProgramVariables_ contains all parameters that were passed by the script. 
    * Unnamed parameters can be accessed from the "_args" item, that is a list.
  * You can not create overloaded variants for functions. You have to decide on the basis of which parameters are in the _ProgramVariables_.
  

  * New object classes can be created by inheriting from _ProgramObject_. 
    * If you reference other objects, you can increase their reference counter (_IncRef_ method of a _ProgramObject_) to prevent them from getting destroyed.
    * Don't forget to override _OnDestroy_ and call _DecRef_ in it if you do.
    * _OnDestroy_ is called by the engine when an object isn't referenced anymore and shall be destroyed.
  * To easily create methods for your own objects that can be called in a script use the [EngineFunction] attribute. You can also set a differing name for the script through that.
    
    						
    using SFM_custom_mission_v2.Objects;
    using SFM_custom_mission_v2.Script;
    using UnityEngine;
    ...
    
    public static class Extensions
    {
    	public static void Initialize()
    	{
    		Engine.RegisterFunction("CreateGameObject", CreateGameObject);
    		ProgramObject.RegisterFunction(typeof(Object_NPC), "GameObject", GameObject);
    	}
    	public static ProgramValue CreateGameObject(ProgramVariables parameters, ProgramThreadBase thread)
    	{
    		return new ProgramValue(new Object_GameObject());
    	}
    	public static ProgramValue GameObject(ProgramObject obj, ProgramVariables parameters, ProgramThreadBase thread)
    	{
    		if (obj is Object_NPC)
    		{
    			var npc = (obj as Object_NPC).npc;
    			if (npc != null)
    				return new ProgramValue(new Object_GameObject((obj as Object_NPC).npc.GameObject));
    		}
    		return ProgramValue.Null;
    	}
    }
    public class Object_GameObject : ProgramObject
    {
    	public GameObject gameObject;
    	public bool removeOnDestroy { private set; get; }
    	public Object_GameObject(GameObject _gameObject)
    	{
    		gameObject = _gameObject;
    		removeOnDestroy = false; //used as wrapper, don't destroy GameObject when wrapper is destroyed
    	}
    	public Object_GameObject()
    	{
    		gameObject = new GameObject();
    		removeOnDestroy = true;
    	}
    	public override void OnDestroy()
    	{
    		if (removeOnDestroy)
    			GameObject.DestroyImmediate(gameObject);
    	}
    	[EngineFunction]
    	public ProgramValue SetPosition(ProgramVariables parameters, ProgramThreadBase thread)
    	{
    		Vector3 p = FunctionsMath.VectorFromList(parameters.GetVariable("_args")[0].ToList());
    		gameObject.transform.position = p;
    		return ProgramValue.Null;
    	}
    	...
    }

## Appendix

