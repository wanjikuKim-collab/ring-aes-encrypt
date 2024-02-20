use std::fs::File;
use std::io::{Error, ErrorKind};

pub struct MP4Reader {
    path: String,
    data: Vec<u8>,
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

        let mut data = Vec::new();
        match file.read_to_end(&mut data) {
            Ok(_) => Ok(Self { path, data }),
            Err(err) => Err(MP4ReaderError::IoError(err)),
        }
    }
}

#[derive(Debug)]
pub enum MP4ReaderError {
    FileNotFound(String),
    PermissionDenied(String),
    IoError(std::io::Error),
}
