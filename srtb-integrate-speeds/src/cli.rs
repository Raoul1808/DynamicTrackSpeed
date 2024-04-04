use std::io::Write;

use rfd::FileDialog;
use srtb_integrate_speeds::*;

fn integrate_speeds(key: &str) -> Result<(), String> {
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
    integrate(&srtb_file, &speeds_file, key)
}

fn extract_speeds(key: &str) -> Result<(), String> {
    println!("Select a chart to extract speeds from");
    let file = FileDialog::new()
        .add_filter("Spin Rhythm Track Bundle", &["srtb"])
        .pick_file();
    let srtb_file = file.ok_or("Please select a srtb file")?;

    extract(&srtb_file, key)
}

fn remove_speeds(key: &str) -> Result<(), String> {
    println!("Select a chart to remove speeds from");
    let file = FileDialog::new()
        .add_filter("Spin Rhythm Track Bundle", &["srtb"])
        .pick_file();
    let srtb_file = file.ok_or("Please select a srtb file")?;

    remove(&srtb_file, key)
}

fn map_num_to_key<'a>(opt: i32) -> Option<&'a str> {
    match opt {
        1 => Some("SpeedHelper_SpeedTriggers_EASY"),
        2 => Some("SpeedHelper_SpeedTriggers_NORMAL"),
        3 => Some("SpeedHelper_SpeedTriggers_HARD"),
        4 => Some("SpeedHelper_SpeedTriggers_EXPERT"),
        5 => Some("SpeedHelper_SpeedTriggers_XD"),
        6 => Some("SpeedHelper_SpeedTriggers_REMIXD"),
        7 => Some("SpeedHelper_SpeedTriggers"),
        _ => None,
    }
}

pub fn program_flow() -> Result<(), String> {
    println!("Please select a mode");
    println!("1. Integrate speeds into srtb");
    println!("2. Extract speeds from srtb");
    println!("3. Remove speeds from srtb");
    println!("4. Exit");
    print!("> ");
    let mut buf = String::new();
    std::io::stdout().flush().expect("failed to flush stdout");
    std::io::stdin()
        .read_line(&mut buf)
        .expect("failed to read from stdin");
    let mode_opt: i32 = buf.trim_end().parse().expect("not a number");

    println!("Please select the target difficulty");
    println!("1. Easy");
    println!("2. Normal");
    println!("3. Hard");
    println!("4. Expert");
    println!("5. XD");
    println!("6. RemiXD");
    println!("7. All (legacy)");
    print!("> ");
    let mut buf = String::new();
    std::io::stdout().flush().expect("failed to flush stdout");
    std::io::stdin()
        .read_line(&mut buf)
        .expect("failed to read from stdin");
    let diff_opt: i32 = buf.trim_end().parse().expect("not a number");

    let lookup_key = map_num_to_key(diff_opt).expect("invalid difficulty");

    match mode_opt {
        1 => integrate_speeds(lookup_key),
        2 => extract_speeds(lookup_key),
        3 => remove_speeds(lookup_key),
        4 => Ok(()),
        _ => Err("invalid mode".into()),
    }
}
