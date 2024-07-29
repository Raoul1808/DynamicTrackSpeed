using Newtonsoft.Json;

namespace DynamicTrackSpeed
{
    // I stole this from SpinCore. Sorry Prog and Pink :(
    public static class CustomChartHelper
    {
        /// <summary>
        /// Attempts to get miscellaneous data from a chart file
        /// </summary>
        /// <param name="customFile">The file to read from</param>
        /// <param name="key">The key used to identify the data</param>
        /// <param name="data">The acquired data</param>
        /// <typeparam name="T">The type of the data object</typeparam>
        /// <returns>True if data was found</returns>
        public static bool TryGetCustomData<T>(IMultiAssetSaveFile customFile, string key, out T data) {
            if (!customFile.HasJsonValueForKey(key)) {
                data = default;

                return false;
            }
        
            data = JsonConvert.DeserializeObject<T>(customFile.GetLargeStringOrJson(key).Value);

            return data != null;
        }
    }
}
