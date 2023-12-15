use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawSrtbFile {
    pub unity_object_values_container: UnityObjectValuesContainer,   
    pub large_string_values_container: LargeStringValuesContainer,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UnityObjectValuesContainer {
    pub values: Vec<UnityObjectValue>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UnityObjectValue {
    pub key: String,
    pub json_key: String,
    pub full_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LargeStringValuesContainer {
    pub values: Vec<LargeStringValue>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LargeStringValue {
    pub key: String,
    pub val: String,
}

fn main() {
    let chart_contents = fs::read_to_string("/home/mew/.local/share/Steam/steamapps/compatdata/1058830/pfx/drive_c/users/steamuser/AppData/LocalLow/Super Spin Digital/Spin Rhythm XD/Custom/Jeff.srtb").expect("should be able to read from file");
    let chart: RawSrtbFile = serde_json::from_str(&chart_contents).unwrap();
    println!("{:#?}", chart);
}
