mod video;

mod prelude {
    pub use std::env;
    pub use std::fs;
    pub use crate::video::*;
    pub use clap::Parser;
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
}



fn main() {
    
    let args = Cli::parse();

    match args.command.as_str() {
        "convertvideo" => {
            let video = Video::new(args.input.unwrap().to_str().unwrap().to_string());
            let _ = video.convert_video(args.output.unwrap().to_str().unwrap());
        },
        "forecast" => {
            println!("Forecasting");
        },
        _ => {
            println!("Unknown command {}", args.command);
        }
    }
}