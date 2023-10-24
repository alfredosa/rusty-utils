use crate::prelude::*;

use std::fs::Metadata;
use std::io;

pub fn get_file_name_and_metadata(file_path: &String) -> (String, io::Result<Metadata>, u64) {
    let file_name = fs::canonicalize(&file_path)
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let metadata = fs::metadata(&file_path).unwrap();
    let file_size = metadata.len();

    (file_name, Ok(metadata), file_size)
}