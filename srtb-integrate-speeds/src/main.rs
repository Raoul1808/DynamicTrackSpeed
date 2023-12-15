use std::{fs, io};

use rfd::FileDialog;
use srtb_integrate_speeds::RawSrtbFile;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

macro_rules! str_err {
    ($e:expr) => {
        Err($e.into())
    };
}

fn program_flow() -> Result<()> {
    println!("Select a chart to open");
    let file = FileDialog::new()
        .add_filter("Spin Rhythm Track Bundle", &["srtb"])
        .pick_file();
    let srtb_file = match file {
        Some(f) => f,
        None => return str_err!("Please select a file"),
    };
    let chart_contents = fs::read_to_string(&srtb_file)?;
    let chart: RawSrtbFile = serde_json::from_str(&chart_contents)?;
    println!("{:#?}", chart);
    Ok(())
}

fn main() {
    match program_flow() {
        Ok(()) => println!("Program finished executing without errors."),
        Err(e) => println!("{e}"),
    }
    println!("Press ENTER to exit");
    io::stdin()
        .read_line(&mut String::new())
        .expect("failed to read from stdin");
}
