use std::{fmt::Write, fs, path::Path};

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

pub fn integrate(srtb: &Path, speeds: &Path, diff_key: &str) -> Result<(), String> {
    println!("Reading file contents");
    let chart_contents = fs::read_to_string(srtb).map_err(|e| e.to_string())?;
    let speeds_contents = fs::read_to_string(speeds).map_err(|e| e.to_string())?;

    println!("Converting speeds");
    let speeds = speeds_to_json(&speeds_contents)?;
    let speeds_json = serde_json::to_string(&speeds).map_err(|e| e.to_string())?;

    println!("Integrating to srtb");
    let mut chart: RawSrtbFile =
        serde_json::from_str(&chart_contents).map_err(|e| e.to_string())?;
    if let Some(value) = chart
        .large_string_values_container
        .values
        .iter_mut()
        .find(|v| v.key == diff_key)
    {
        value.val.clone_from(&speeds_json);
    } else {
        chart
            .large_string_values_container
            .values
            .push(LargeStringValue {
                key: diff_key.to_string(),
                val: speeds_json.clone(),
            });
    }
    let chart = serde_json::to_string(&chart).map_err(|e| e.to_string())?;

    println!("Integration complete! Please select where you would like to save your file");
    let file = rfd::FileDialog::new()
        .add_filter("Spin Rhythm Track Bundle", &["srtb"])
        .save_file();
    let dest_file = file.ok_or("no destination file selected")?;
    fs::write(dest_file, chart).map_err(|e| e.to_string())?;
    println!("All done!");
    Ok(())
}

pub fn extract(file: &Path, diff_key: &str) -> Result<(), String> {
    println!("Checking for speeds data");
    let srtb_contents = fs::read_to_string(file).map_err(|e| e.to_string())?;
    let chart: RawSrtbFile = serde_json::from_str(&srtb_contents).map_err(|e| e.to_string())?;

    if let Some(value) = chart
        .large_string_values_container
        .values
        .iter()
        .find(|v| v.key == diff_key)
    {
        println!("Found speeds data. Converting");
        let speeds: SpeedTriggersData =
            serde_json::from_str(&value.val).map_err(|e| e.to_string())?;
        let speeds = json_to_speeds(&speeds);

        println!(
            "Conversion done! Please select where you would like to save the resulting speeds file"
        );
        let file = rfd::FileDialog::new()
            .add_filter("Speed Triggers file", &["speeds"])
            .save_file();
        let file = file.ok_or("no destination file selected")?;
        let file = file.with_extension("speeds");
        fs::write(file, speeds).map_err(|e| e.to_string())?;
        println!("All done!");
    } else {
        println!("No speeds data found.");
    }
    Ok(())
}

pub fn remove(file: &Path, diff_key: &str) -> Result<(), String> {
    println!("Checking for speeds data");
    let srtb_contents = fs::read_to_string(file).map_err(|e| e.to_string())?;
    let mut chart: RawSrtbFile = serde_json::from_str(&srtb_contents).map_err(|e| e.to_string())?;

    if let Some((index, _)) = chart
        .large_string_values_container
        .values
        .iter()
        .enumerate()
        .find(|(_, v)| v.key == diff_key)
    {
        println!("Found speeds data. Removing");
        chart.large_string_values_container.values.remove(index);
        let chart_contents = serde_json::to_string(&chart).map_err(|e| e.to_string())?;
        println!("Removed! Please select a saving location");
        let file = rfd::FileDialog::new()
            .add_filter("Spin Rhythm Track Bundle", &["srtb"])
            .save_file();
        let file = file.ok_or("no destination file selected")?;
        let file = file.with_extension("srtb");
        fs::write(file, chart_contents).map_err(|e| e.to_string())?;
        println!("All done!");
    } else {
        println!("No speeds data found.");
    }
    Ok(())
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
