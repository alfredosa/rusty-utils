mod video;
mod weather;
mod utils;
mod drive_backup;

mod prelude {
    pub use std::env;
    pub use std::fs;
    pub use crate::video::*;
    pub use clap::Parser;
    pub use crate::weather::*;
    pub use crate::utils::*;
    pub use crate::drive_backup::*;
}

use prelude::*;


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[clap(name = "Rust CLI", version = "1.0", author = "Rust CLI")]
#[command(author, about, version)]
struct Cli {
    /// Command to execute
    #[arg(short, long)]
    command: String,

    /// input file path
    #[arg(short, long)]
    input: Option<std::path::PathBuf>,

    /// output file path
    #[arg(short, long)]
    output: Option<std::path::PathBuf>,

    /// forecast city if forecast command was called
    #[arg(long)]
    city: Option<String>,
}



fn main() {
    
    let args = Cli::parse();

    match args.command.as_str() {
        "convertvideo" => {
            let video = Video::new(args.input.unwrap().to_str().unwrap().to_string());
            let _ = video.convert_video(args.output.unwrap().to_str().unwrap());
        },
        "forecast" => {
            get_weather(args.city.unwrap_or("Oslo".to_string())).expect("Failed to get weather");
        },
        "backup" => {
            let drive_backup = DriveBackup::new(args.input.unwrap().to_str().unwrap().to_string());
            let _ = drive_backup.backup();
            // google drive api call
        },
        _ => {
            println!("Unknown command {}", args.command);
        }
    }
}