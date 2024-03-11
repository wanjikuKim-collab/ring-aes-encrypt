use std::fs::File;
use std::io::{self,BufReader, Read};

pub struct MP4Reader {
    path: String,
    data: Vec<u8>,
}

impl MP4Reader {
    pub fn read_file(path: &str) -> Result<Self, io::Error> {
        let mut mp4_file = File::open(path).expect("Failed to open file");

        // reading file into a BufReader
        let mut reader = BufReader::new(&mut mp4_file);
        
        //code to read MP4 file in a vector
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).expect("Failed to read file");

        let mp4_reader = MP4Reader{
            path: path,
            data: buffer,
        };

        Ok(Self{
            path: path.to_owned(),
            data: buffer
        })    
    }
}
