use std::fmt::Write;

use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SpeedTrigger {
    pub time: f32,
    pub speed_multiplier: f32,
    pub interpolate_to_next_trigger: bool,
}

pub fn speeds_to_json(content: &str) -> Result<SpeedTriggersData, String> {
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
            return Err(format!(
                "Line {}: expected 2 or 3 values values, found {}",
                line_number,
                line.len()
            ));
        }
        let time = line[0].parse();
        let time: f32 = match time {
            Ok(t) => t,
            Err(_) => {
                return Err(format!(
                    "Line {}: time value is not a valid number",
                    line_number
                ))
            }
        };

        let speed = line[1].parse();
        let speed: f32 = match speed {
            Ok(s) => s,
            Err(_) => {
                return Err(format!(
                    "Line {}: speed multiplier is not a valid number",
                    line_number
                ))
            }
        };

        let interpolate = if line.len() != 3 {
            false
        } else {
            let interpolate = line[2].parse();
            match interpolate {
                Ok(i) => i,
                Err(_) => {
                    return Err(format!(
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

pub fn json_to_speeds(speeds: &SpeedTriggersData) -> String {
    speeds.triggers.iter().fold(String::new(), |mut output, t| {
        let _ = writeln!(
            output,
            "{} {} {}",
            t.time, t.speed_multiplier, t.interpolate_to_next_trigger
        );
        output
    })
}

#[cfg(test)]
mod test {
    use crate::{json_to_speeds, speeds_to_json, SpeedTrigger, SpeedTriggersData};

    #[test]
    fn test_speeds_to_json() {
        let speeds = r#"
        0 1
        1.5  2    false
        2    1.5  true
        "#;

        let expected_speeds = vec![
            SpeedTrigger {
                time: 0.,
                speed_multiplier: 1.,
                interpolate_to_next_trigger: false,
            },
            SpeedTrigger {
                time: 1.5,
                speed_multiplier: 2.,
                interpolate_to_next_trigger: false,
            },
            SpeedTrigger {
                time: 2.,
                speed_multiplier: 1.5,
                interpolate_to_next_trigger: true,
            },
        ];

        let speeds = speeds_to_json(speeds).unwrap();
        assert_eq!(speeds.triggers, expected_speeds);
    }

    #[test]
    fn struct_to_speeds() {
        let triggers = vec![
            SpeedTrigger {
                time: 0.,
                speed_multiplier: 1.,
                interpolate_to_next_trigger: false,
            },
            SpeedTrigger {
                time: 1.5,
                speed_multiplier: 2.,
                interpolate_to_next_trigger: false,
            },
            SpeedTrigger {
                time: 2.,
                speed_multiplier: 1.5,
                interpolate_to_next_trigger: true,
            },
        ];
        let speeds = SpeedTriggersData { triggers };

        let expected_speeds = "0 1 false\n1.5 2 false\n2 1.5 true\n";

        let speeds = json_to_speeds(&speeds);
        assert_eq!(speeds, expected_speeds);
    }
}
