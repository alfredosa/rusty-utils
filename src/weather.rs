use std::process::{Command};
use std::error::Error;
use std::process::Stdio;


pub fn get_weather(city: String) -> Result<(), Box<dyn Error>> {
    let url = format!("http://wttr.in/{}", city);
    let output = Command::new("curl")
        .arg(url)
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute process");   
    let stdout = String::from_utf8(output.stdout).unwrap();
    println!("{}", stdout);

    Ok(())
}