use std::{error::Error, path::PathBuf};



#[derive(Debug)]
pub enum CompressionError {
    Generic(String),
    FileNotFound(PathBuf),
    IoError(std::io::Error),
    NotFinished,
}

impl std::fmt::Display for CompressionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompressionError::Generic(s) => write!(f, "Generic Error: {s}"),
            CompressionError::FileNotFound(path_buf) => write!(f, "FileNotFound: {path_buf:?}"),
            CompressionError::IoError(error) => write!(f, "IO Error: {error:?}"),
            CompressionError::NotFinished => write!(f, "Feature not finished!"),
        }
    }
}

impl Error for CompressionError {}

impl From<std::io::Error> for CompressionError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}