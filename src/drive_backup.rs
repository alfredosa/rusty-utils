use crate::prelude::*;

pub struct DriveBackup {
    pub path: String,
}

impl DriveBackup {
    pub fn new(path: String) -> Self {
        Self {
            path,
        }
    }

    pub fn backup(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file_object = get_file_name_and_metadata(&self.path);
        let file_name = file_object.0;
        let metadata = file_object.1.unwrap();
        let file_size = file_object.2;

        println!("file name: {}\nfile size: {}\nfile path: {}", file_name, file_size, self.path);
        if metadata.is_dir() {
            // we dont really want to panic, we need to recursively back up if its a folder
            panic!("{} is a directory", self.path);
        }

        Ok(())
    }
}