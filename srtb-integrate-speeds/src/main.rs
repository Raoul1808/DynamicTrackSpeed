use std::{fs, io::Write};

use rfd::FileDialog;
use srtb_integrate_speeds::*;

fn integrate_speeds() -> Result<(), String> {
    println!("Select a chart to integrate speeds to");
    let file = FileDialog::new()
        .add_filter("Spin Rhythm Track Bunidle", &["srtb"])
        .pick_file();
    let srtb_file = file.ok_or("Please select a srtb file")?;

    println!("Select a speeds file to integrate");
    let file = FileDialog::new()
        .add_filter("Speed Triggers File", &["speeds"])
        .pick_file();
    let speeds_file = file.ok_or("Please select a speeds file")?;

    println!("Beginning process");
    println!("Reading file contents");
    let chart_contents = fs::read_to_string(srtb_file).map_err(|e| e.to_string())?;
    let speeds_contents = fs::read_to_string(speeds_file).map_err(|e| e.to_string())?;

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
        .find(|v| v.key == "SpeedHelper_SpeedTriggers")
    {
        value.val.clone_from(&speeds_json);
    } else {
        chart
            .large_string_values_container
            .values
            .push(LargeStringValue {
                key: "SpeedHelper_SpeedTriggers".to_string(),
                val: speeds_json.clone(),
            });
    }
    let chart = serde_json::to_string(&chart).map_err(|e| e.to_string())?;

    println!("Integration complete! Please select where you would like to save your file");
    let file = FileDialog::new()
        .add_filter("Spin Rhythm Track Bundle", &["srtb"])
        .save_file();
    let dest_file = file.ok_or("Please select a destination file")?;
    fs::write(dest_file, chart).map_err(|e| e.to_string())?;
    println!("All done!");
    Ok(())
}

fn extract_speeds() -> Result<(), String> {
    println!("Select a chart to extract speeds from");
    let file = FileDialog::new()
        .add_filter("Spin Rhythm Track Bundle", &["srtb"])
        .pick_file();
    let srtb_file = file.ok_or("Please select a srtb file")?;

    println!("Checking for speeds data");
    let srtb_contents = fs::read_to_string(srtb_file).map_err(|e| e.to_string())?;
    let chart: RawSrtbFile = serde_json::from_str(&srtb_contents).map_err(|e| e.to_string())?;

    if let Some(value) = chart
        .large_string_values_container
        .values
        .iter()
        .find(|v| v.key == "SpeedHelper_SpeedTriggers")
    {
        println!("Found speeds data. Converting");
        let speeds: SpeedTriggersData =
            serde_json::from_str(&value.val).map_err(|e| e.to_string())?;
        let speeds = json_to_speeds(&speeds);

        println!(
            "Convertion done! Please select where you would like to save the resulting speeds file"
        );
        let file = FileDialog::new()
            .add_filter("Speed Triggers file", &["speeds"])
            .save_file();
        let file = file.ok_or("Please select a saving location")?;
        let file = file.with_extension("speeds");
        fs::write(file, speeds).map_err(|e| e.to_string())?;
        println!("All done!");
        Ok(())
    } else {
        println!("No speeds data found.");
        Ok(())
    }
}

fn program_flow() -> Result<(), String> {
    println!("Please select a mode");
    println!("1. Integrate speeds into srtb");
    println!("2. Extract speeds from srtb");
    println!("3. Exit");
    print!("> ");
    let mut buf = String::new();
    std::io::stdout().flush().expect("failed to flush stdout");
    std::io::stdin()
        .read_line(&mut buf)
        .expect("failed to read from stdin");
    let opt: i32 = buf.trim_end().parse().expect("not a number");

    match opt {
        1 => integrate_speeds(),
        2 => extract_speeds(),
        3 => Ok(()),
        _ => Err("invalid mode".into()),
    }
}

fn main() {
    match program_flow() {
        Ok(()) => println!("Program finished executing without errors."),
        Err(e) => println!("{e}"),
    }
    #[cfg(target_os = "windows")]
    {
        println!("Press ENTER to exit");
        std::io::stdin()
            .read_line(&mut String::new())
            .expect("failed to read from stdin");
    }
}
