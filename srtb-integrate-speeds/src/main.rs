use std::fs;

use rfd::FileDialog;
use srtb_integrate_speeds::*;

fn program_flow() -> Result<()> {
    println!("Select a chart to open");
    let file = FileDialog::new()
        .add_filter("Spin Rhythm Track Bundle", &["srtb"])
        .pick_file();
    let srtb_file = match file {
        Some(f) => f,
        None => return str_err!("Please select a srtb file"),
    };
    let file = FileDialog::new()
        .add_filter("Speed Triggers File", &["speeds"])
        .pick_file();
    let speeds_file = match file {
        Some(f) => f,
        None => return str_err!("Please select a speeds file"),
    };
    let chart_contents = fs::read_to_string(srtb_file)?;
    let mut chart: RawSrtbFile = serde_json::from_str(&chart_contents)?;
    let speeds_content = fs::read_to_string(speeds_file)?;
    let speeds = parse_speeds_file(speeds_content)?;
    let speeds_json = serde_json::to_string(&speeds)?;
    if let Some(value) = chart
        .large_string_values_container
        .values
        .iter_mut()
        .find(|v| v.key == "SpeedHelper_SpeedTriggers")
    {
        value.val = speeds_json.clone();
    } else {
        chart
            .large_string_values_container
            .values
            .push(LargeStringValue {
                key: "SpeedHelper_SpeedTriggers".to_string(),
                val: speeds_json.clone(),
            });
    }
    let chart = serde_json::to_string(&chart)?;
    println!("Chart and speeds combined! Select which file the contents should be saved to");
    let file = FileDialog::new()
        .add_filter("Spin Rhythm Track Bundle", &["srtb"])
        .save_file();
    let dest_file = match file {
        Some(f) => f,
        None => return str_err!("Please select a destination file"),
    };
    fs::write(dest_file, chart)?;
    println!("All done!");
    Ok(())
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
