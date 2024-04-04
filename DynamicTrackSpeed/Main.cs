using System.Collections.Generic;
using System.Globalization;
using System.IO;
using System.Linq;
using BepInEx;
using BepInEx.Logging;
using HarmonyLib;

namespace DynamicTrackSpeed
{
    [BepInPlugin(Guid, Name, Version)]
    public class Main : BaseUnityPlugin
    {
        public const string Guid = "srxd.raoul1808.dynamictrackspeed";
        public const string Name = "Dynamic Track Speed";
        public const string Version = "1.3.0";

        private static ManualLogSource _logger;
        private static CultureInfo _culture;

        private void Awake()
        {
            _logger = Logger;
            Logger.LogMessage("Hello from Dynamic Track Speed!");
            _culture = new CultureInfo("en-US");
            Harmony harmony = new Harmony(Guid);
            harmony.PatchAll(typeof(QuickPatches));
            Logger.LogMessage("Patched methods: " + harmony.GetPatchedMethods().Count());
        }

        public static void Log(object msg) => _logger.LogMessage(msg);

        internal class QuickPatches
        {
            private static TrackData.DifficultyType[] _validDiffs =
            {
                TrackData.DifficultyType.Easy,
                TrackData.DifficultyType.Normal,
                TrackData.DifficultyType.Hard,
                TrackData.DifficultyType.Expert,
                TrackData.DifficultyType.XD
            };

            struct SpeedTrigger
            {
                public float Time { get; set; }
                public float SpeedMultiplier { get; set; }
                public bool InterpolateToNextTrigger { get; set; }
            }

            struct SpeedTriggersMetadata
            {
                public List<SpeedTrigger> Triggers { get; set; }
            }

            private static List<SpeedTrigger> TriggersFromSrtb(PlayableTrackData trackData, string diff)
            {
                var files = new List<IMultiAssetSaveFile>();
                trackData.GetCustomFiles(files);
                var file = files.First();
                if (file is null) return null;
                if (CustomChartHelper.TryGetCustomData(file, "SpeedHelper_SpeedTriggers_" + diff, out SpeedTriggersMetadata diffData))
                    return diffData.Triggers;
                return CustomChartHelper.TryGetCustomData(file, "SpeedHelper_SpeedTriggers",
                    out SpeedTriggersMetadata data) ? data.Triggers : null;
            }

            private static List<SpeedTrigger> TriggersFromSpeedsFile(string speedsPath)
            {
                var speeds = new List<SpeedTrigger>();
                foreach (string line in File.ReadAllLines(speedsPath))
                {
                    if (line.StartsWith("#"))
                        continue;
                    var elems = line.Split(' '); 
                    var trigger = new SpeedTrigger();
                    if (elems.Length < 2) continue;
                    if (float.TryParse(elems[0], NumberStyles.Float, _culture, out float time) && float.TryParse(elems[1], NumberStyles.Float, _culture, out float speed))
                    {
                        trigger.Time = time;
                        trigger.SpeedMultiplier = speed;
                    }
                    else continue;

                    if (elems.Length >= 3 && bool.TryParse(elems[2], out bool interpolate))
                    {
                        trigger.InterpolateToNextTrigger = interpolate;
                    }

                    speeds.Add(trigger);
                }

                return speeds;
            }
            
            [HarmonyPatch(typeof(SplineTrackData.DataToGenerate), MethodType.Constructor, typeof(PlayableTrackData)), HarmonyPostfix]
            public static void AddTrackSpeedsToSpline(SplineTrackData.DataToGenerate __instance, PlayableTrackData trackData)
            {
                if (trackData.TrackDataList.Count != 1)
                    return;
                var data = trackData.TrackDataList[0];
                string customPath = data.CustomFile?.FilePath;
                if (string.IsNullOrEmpty(customPath))
                    return;
                if (!_validDiffs.Contains(trackData.Difficulty))
                    return;
                string diffStr = trackData.Difficulty.ToString().ToUpper();
                string speedsFilename = Path.GetFileNameWithoutExtension(customPath) + ".speeds";
                string diffSpeedsFilename = Path.GetFileNameWithoutExtension(customPath) + "_" + diffStr + ".speeds";
                string customsDirectory = Directory.GetParent(customPath)?.FullName;
                if (string.IsNullOrEmpty(customsDirectory))
                    return;
                string speedsPath = Path.Combine(customsDirectory, speedsFilename);
                string diffSpeedsPath = Path.Combine(customsDirectory, diffSpeedsFilename);
                bool loadedFromSpeeds = true;
                List<SpeedTrigger> triggers;
                
                // Loading order: diff-speeds > speeds > diff-srtb > srtb
                // diff-speeds
                if (File.Exists(diffSpeedsPath))
                    triggers = TriggersFromSpeedsFile(diffSpeedsPath);
                // global speeds
                else if (File.Exists(speedsPath))
                    triggers = TriggersFromSpeedsFile(speedsPath);
                // diff-srtb and srtb
                else
                {
                    triggers = TriggersFromSrtb(trackData, diffStr);
                    loadedFromSpeeds = false;
                }

                if (triggers == null || triggers.Count <= 0) return;

                float initialSpeed = __instance.trackSpeeds[0].speed;
                
                // __instance.trackSpeeds.Clear();
                foreach (var trigger in triggers)
                {
                    __instance.trackSpeeds.Add(new TrackSpeedAtTime
                    {
                        time = trigger.Time,
                        speed = trigger.SpeedMultiplier * initialSpeed,
                        interpolateToNextSpeed = trigger.InterpolateToNextTrigger,
                    });
                }

                if (__instance.trackTurns.Count == 1)
                    __instance.trackTurns.Add(new SplineRenderer.TrackTurnAndContext());

                string msg = loadedFromSpeeds
                    ? $"Applied {triggers.Count} triggers from file {speedsFilename}"
                    : $"Applied {triggers.Count} triggers from embedded data";
                Log(msg);
            }
        }
    }
}
