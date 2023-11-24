using System.Linq;
using System.Reflection;
using BepInEx;
using BepInEx.Logging;
using HarmonyLib;

namespace SpinSpeedHelper
{
    [BepInPlugin(Guid, Name, Version)]
    public class Main : BaseUnityPlugin
    {
        public const string Guid = "srxd.raoul1808.spinspeedhelper";
        public const string Name = "Spin Speed Helper";
        public const string Version = "0.1.0";

        private static ManualLogSource _logger;

        private void Awake()
        {
            _logger = Logger;
            Logger.LogMessage("Hi from Spin Speed Helper!");
            Harmony harmony = new Harmony(Guid);
            harmony.PatchAll(typeof(QuickPatches));
            Logger.LogMessage("Patched methods: " + harmony.GetPatchedMethods().Count());
        }

        public static void Log(object msg) => _logger.LogMessage(msg);

        internal class QuickPatches
        {
            [HarmonyPatch(typeof(SplineTrackData.DataToGenerate), nameof(SplineTrackData.DataToGenerate.IsDifferent)), HarmonyPostfix]
            public static void DataToGenerate_IsDifferent_Postfix(SplineTrackData.DataToGenerate __instance)
            {
                if (__instance == null)
                    return;
                __instance.trackSpeeds.Clear();
                for (int i = 0; i < 128; i++)
                {
                    var speed = new TrackSpeedAtTime
                    {
                        time = i,
                        speed = ((i % 2) + 1) * 4,
                        interpolateToNextSpeed = true,
                    };
                    __instance.trackSpeeds.Add(speed);
                }

                Log("Injected " + __instance.trackSpeeds.Count + " track speeds");
            }
        }
    }
}
