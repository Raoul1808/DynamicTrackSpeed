using System.Collections.Generic;
using System.IO;
using System.Linq;
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
        public const string Version = "1.0.0";

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
            [HarmonyPatch(typeof(SplineTrackData.DataToGenerate), MethodType.Constructor, typeof(PlayableTrackData)), HarmonyPostfix]
            public static void AddTrackSpeedsToSpline(SplineTrackData.DataToGenerate __instance, PlayableTrackData trackData)
            {
                if (trackData.TrackDataList.Count != 1)
                    return;
                var data = trackData.TrackDataList[0];
                string customPath = data.CustomFile?.FilePath;
                if (string.IsNullOrEmpty(customPath))
                    return;
                string speedsFilename = Path.GetFileNameWithoutExtension(customPath) + ".speeds";
                string customsDirectory = Directory.GetParent(customPath)?.FullName;
                if (string.IsNullOrEmpty(customsDirectory))
                    return;
                string speedsPath = Path.Combine(customsDirectory, speedsFilename);
                var speeds = new List<(float, float, bool)>();
                if (!File.Exists(speedsPath)) return;
                foreach (string line in File.ReadAllLines(speedsPath))
                {
                    var elems = line.Split(' ');
                    var trigger = (0f, 0f, false);
                    if (elems.Length < 2) continue;
                    if (float.TryParse(elems[0], out float time) && float.TryParse(elems[1], out float speed))
                    {
                        trigger.Item1 = time;
                        trigger.Item2 = speed;
                    }
                    else continue;

                    if (elems.Length >= 3 && bool.TryParse(elems[2], out bool interpolate))
                    {
                        trigger.Item3 = interpolate;
                    }

                    speeds.Add(trigger);
                }

                if (speeds.Count <= 0) return;

                float initialSpeed = __instance.trackSpeeds[0].speed;
                
                // __instance.trackSpeeds.Clear();
                foreach (var trigger in speeds)
                {
                    __instance.trackSpeeds.Add(new TrackSpeedAtTime
                    {
                        time = trigger.Item1,
                        speed = trigger.Item2 * initialSpeed,
                        interpolateToNextSpeed = trigger.Item3,
                    });
                }
                
                if (__instance.trackTurns.Count == 1)
                    __instance.trackTurns.Add(new SplineRenderer.TrackTurnAndContext());
                
                Log($"Applied {speeds.Count} triggers from file {speedsFilename}");
            }
        }
    }
}
