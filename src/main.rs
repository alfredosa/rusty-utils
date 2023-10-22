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
struct Cli {
    /// The pattern to look for
    #[arg(short, long)]
    command: String,
    #[arg(short, long)]
    input: std::path::PathBuf,
    #[arg(short, long)]
    output: std::path::PathBuf,
    #[arg(long)]
    outformat: String,
}



fn main() {
    
    let args = Cli::parse();

    match args.command.as_str() {
        "convertvideo" => {
            let video = Video::new(args.input.to_str().unwrap().to_string());
            video.convert_video(args.output.to_str().unwrap(), args.outformat.as_str());
        },
        _ => {
            println!("Unknown command {}", args.command);
        }

    }
}