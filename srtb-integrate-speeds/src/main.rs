mod cli;

fn main() {
    match cli::program_flow() {
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
