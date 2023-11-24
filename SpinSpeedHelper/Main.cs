using BepInEx;

namespace SpinSpeedHelper
{
    [BepInPlugin(Guid, Name, Version)]
    public class Main : BaseUnityPlugin
    {
        public const string Guid = "srxd.raoul1808.spinspeedhelper";
        public const string Name = "Spin Speed Helper";
        public const string Version = "0.1.0";

        private void Awake()
        {
            Logger.LogMessage("Hi from Spin Speed Helper!");
        }
    }
}
