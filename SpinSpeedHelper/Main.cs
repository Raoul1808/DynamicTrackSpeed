﻿using System.Collections.Generic;
using System.IO;
using System.Linq;
using BepInEx;
using BepInEx.Logging;
using ChartHelper;
using HarmonyLib;

namespace SpinSpeedHelper
{
    [BepInPlugin(Guid, Name, Version)]
    [BepInDependency("srxd.raoul1808.charthelper")]
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
            struct CustomData
            {
                public string Name { get; set; }
                public int Wysi { get; set; }
            }

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

            private static List<SpeedTrigger> TriggersFromSrtb(PlayableTrackData trackData)
            {
                var files = new List<IMultiAssetSaveFile>();
                trackData.GetCustomFiles(files);
                var file = files.First();
                if (file is null) return null;
                return CustomChartHelper.TryGetCustomData(file, "SpeedHelper_SpeedTriggers",
                    out SpeedTriggersMetadata data) ? data.Triggers : null;
            }

            private static List<SpeedTrigger> TriggersFromSpeedsFile(string speedsPath)
            {
                var speeds = new List<SpeedTrigger>();
                foreach (string line in File.ReadAllLines(speedsPath))
                {
                    var elems = line.Split(' '); 
                    var trigger = new SpeedTrigger();
                    if (elems.Length < 2) continue;
                    if (float.TryParse(elems[0], out float time) && float.TryParse(elems[1], out float speed))
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
                string speedsFilename = Path.GetFileNameWithoutExtension(customPath) + ".speeds";
                string customsDirectory = Directory.GetParent(customPath)?.FullName;
                if (string.IsNullOrEmpty(customsDirectory))
                    return;
                string speedsPath = Path.Combine(customsDirectory, speedsFilename);
                var triggers = File.Exists(speedsPath)
                    ? TriggersFromSpeedsFile(speedsPath)
                    : TriggersFromSrtb(trackData);

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
                
                Log($"Applied {triggers.Count} triggers from file {speedsFilename}");
            }
        }
    }
}
