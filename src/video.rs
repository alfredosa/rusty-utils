use crate::prelude::*;

use std::process::{Command, Output};
use std::error::Error;
use std::fmt;
use std::process::Stdio;

// Define a custom error type
#[derive(Debug)]
struct FFmpegError(String);

impl fmt::Display for FFmpegError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FFmpeg error: {}", self.0)
    }
}

impl Error for FFmpegError {}

pub struct Video {
    file_path: String,
    file_name: String,
    file_size: u64,
}


impl Video {
    pub fn new(file_path: String) -> Self {
        match check_requirements() {
            Ok(_) => {
                println!("FFmpeg is installed");
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                eprintln!("Please check the installation or take appropriate action.");
            }
        }
    
        let file_name = fs::canonicalize(&file_path)
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let metadata = fs::metadata(&file_path).unwrap();
        let file_size = metadata.len();
    
        if metadata.is_dir() {
            panic!("{} is a directory", file_path);
        }
    
        // Use format! for improved prints
        let details = format!(
            "file name: {}\nfile size: {}\nfile path: {}",
            file_name,
            file_size,
            file_path
        );
        println!("{}", details);
    
        Self {
            file_path,
            file_name,
            file_size,
        }
    }
    

    pub fn convert_video(&self, output_file: &str) -> Result<(), ()>{
        let mut ffmpeg = Command::new("ffmpeg");
        ffmpeg.args(&[
            "-y",
            "-i",
            &self.file_path,
            "-vf",
            "scale=1280:720",
            "-b:v",
            "1000k",
            output_file,
        ]);
        

        let output: Result<Output, std::io::Error> = ffmpeg.output();
        
        match output {
            Ok(output) => {
                println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
            },
            Err(e) => {
                println!("There was an error: {}", e);
            }
        }
        
        Ok(())

        }

}

pub fn check_requirements() -> Result<(), Box<dyn Error>> {
    // Create a command to check FFmpeg version
    let output = Command::new("ffmpeg")
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match output {
        Ok(exit_status) => {
            if exit_status.success() {
                Ok(())
            } else {
                Err(Box::new(FFmpegError("FFmpeg is not installed".to_string())))
            }
        }
        Err(_) => {
            Err(Box::new(FFmpegError("Error executing FFmpeg command. Please check if FFmpeg is installed.".to_string())))
        }
    }
}