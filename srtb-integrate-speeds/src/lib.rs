use serde::{Deserialize, Serialize};

pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[macro_export]
macro_rules! str_err {
    ($e:expr) => {
        Err($e.into())
    };
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawSrtbFile {
    pub unity_object_values_container: UnityObjectValuesContainer,
    pub large_string_values_container: LargeStringValuesContainer,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnityObjectValuesContainer {
    pub values: Vec<UnityObjectValue>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnityObjectValue {
    pub key: String,
    pub json_key: String,
    pub full_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LargeStringValuesContainer {
    pub values: Vec<LargeStringValue>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LargeStringValue {
    pub key: String,
    pub val: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SpeedTriggersData {
    pub triggers: Vec<SpeedTrigger>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SpeedTrigger {
    pub time: f32,
    pub speed_multiplier: f32,
    pub interpolate_to_next_trigger: bool,
}

pub fn parse_speeds_file(content: String) -> Result<SpeedTriggersData> {
    let mut triggers = Vec::new();
    for line in content.lines().enumerate() {
        let (line_number, line) = line;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let line: Vec<_> = line.split_whitespace().collect();
        if line.is_empty() {
            continue;
        }
        if line.len() < 2 || line.len() > 3 {
            return str_err!(format!(
                "Line {}: expected 2 or 3 values values, found {}",
                line_number,
                line.len()
            ));
        }
        let time = line[0].parse();
        let time: f32 = match time {
            Ok(t) => t,
            Err(_) => {
                return str_err!(format!(
                    "Line {}: time value is not a valid number",
                    line_number
                ))
            }
        };

        let speed = line[1].parse();
        let speed: f32 = match speed {
            Ok(s) => s,
            Err(_) => {
                return str_err!(format!(
                    "Line {}: speed multiplier is not a valid number",
                    line_number
                ))
            }
        };

        let interpolate = if line.len() != 3 {
            true
        } else {
            let interpolate = line[2].parse();
            match interpolate {
                Ok(i) => i,
                Err(_) => {
                    return str_err!(format!(
                        "Line {}: interpolation is not a valid boolean",
                        line_number
                    ))
                }
            }
        };

        let trigger = SpeedTrigger {
            time,
            speed_multiplier: speed,
            interpolate_to_next_trigger: interpolate,
        };
        println!("Created trigger {:?}", trigger);
        triggers.push(trigger);
    }
    let data = SpeedTriggersData { triggers };
    Ok(data)
}
