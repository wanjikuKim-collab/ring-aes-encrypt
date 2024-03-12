use std::fs::File;
use std::io::{BufReader, Read, ErrorKind};

#[derive(Debug)]
pub struct MP4Reader {
    pub path: String,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub enum MP4ReaderError {
    FileNotFound(String),
    PermissionDenied(String),
    IoError(std::io::Error),
}

impl MP4Reader {
    pub fn read_file(path: &str) -> Result<Self, MP4ReaderError> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(err) => match err.kind() {
                ErrorKind::NotFound => return Err(MP4ReaderError::FileNotFound(path.to_owned())),
                ErrorKind::PermissionDenied => return Err(MP4ReaderError::PermissionDenied(path.to_owned())),
                _ => return Err(MP4ReaderError::IoError(err)),
            },
        };

         // reading file into a BufReader
        let mut reader = BufReader::new(&mut file);

        let mut data = Vec::new();
        match reader.read_to_end(&mut data) {
            Ok(_) => Ok(Self { 
                path: path.to_owned(), 
                data: data 
            }),
            Err(err) => Err(MP4ReaderError::IoError(err)),
        }
    }
}


