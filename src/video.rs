
use crate::prelude::*;
extern crate ffmpeg_next as ffmpeg;

use ffmpeg::format::{input, output};
use ffmpeg::media::Type;
use ffmpeg::codec;
use std::env;
use std::path::Path;


pub struct Video {
    file_path: String,
    file_name: String,
    file_size: u64,
}

impl Video {
    pub fn new(file_path: String) -> Self {
        let file_name = fs::canonicalize(&file_path).unwrap().file_name().unwrap().to_str().unwrap().to_string();
        let metadata = fs::metadata(&file_path).unwrap();
        let file_size = metadata.len();
        
        if metadata.is_dir() {
            panic!("{} is a directory", file_path);
        }

        println!("properties: {} {} bytes", file_name, file_size);

        Self {
            file_path,
            file_name,
            file_size,
        }
    }
    pub fn convert_video(&self, output_file: &str, output_format: &str) -> Result<(), ffmpeg::Error>{
        //convert file to output_format read file
        let input_file = input(&Path::new(&self.file_path)).unwrap();
        let output_file = input(output_file).unwrap();

        ffmpeg::init()?;
        // Iterate through the input streams
        for (index, stream) in input_file.streams().enumerate() {
            if stream.codec().medium() == Type::Video {
                // Add a video stream to the output
                let mut output_stream = output_file.add_stream(codec::encoder(ffmpeg::codec::Id::H264, 30, 640, 480)?);

                // Copy codec parameters from the input
                output_stream.set_parameters(stream.codecpar())?;

                // Open the codec for the output stream
                output_stream.open(None)?;
            }
        }

        // Initialize the output file
        output.write_header()?;

        // Process the packets from the input and write them to the output
        let mut packet = ffmpeg::util::frame::Video::empty();

        while input.read_packet(&mut packet)? {
            if let Some(mut output_stream) = output.streams().get_mut(packet.stream_index as usize) {
                packet.rescale(output_stream.time_base());
                packet.set_stream(output_stream.index());

                packet.write_interleaved(output)?;
            }
        }

        output.write_trailer()?;
        println!("Conversion complete. Output file: {}", output_file);
        Ok(())

        }
}