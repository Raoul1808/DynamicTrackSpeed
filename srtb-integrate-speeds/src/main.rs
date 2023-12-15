use std::{fs, io};

use rfd::FileDialog;
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
    println!("Select a chart to open");
    let file = FileDialog::new()
        .add_filter("Spin Rhythm Track Bundle", &["srtb"])
        .pick_file();
    let srtb_file = match file {
        Some(f) => f,
        None => return,
    };
    let chart_contents = fs::read_to_string(&srtb_file).expect("should be able to read from srtb file");
    let chart: RawSrtbFile = serde_json::from_str(&chart_contents).unwrap();
    println!("{:#?}", chart);
    println!("Press ENTER to exit");
    io::stdin()
        .read_line(&mut String::new())
        .expect("failed to read from stdin");
}
