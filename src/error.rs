use std::{alloc::System, error::Error, path::PathBuf, time::SystemTimeError};



#[derive(Debug)]
pub enum CompressionError {
    Generic(String),
    FileNotFound(PathBuf),
    IoError(std::io::Error),
    NotFinished,
    OptionIsNone,
    SystemTimeError(SystemTimeError),
}

pub type CompressionResult<T> = Result<T, CompressionError>;

impl std::fmt::Display for CompressionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompressionError::Generic(s) => write!(f, "Generic Error: {s}"),
            CompressionError::FileNotFound(path_buf) => write!(f, "FileNotFound: {path_buf:?}"),
            CompressionError::IoError(error) => write!(f, "IO Error: {error:?}"),
            CompressionError::NotFinished => write!(f, "Feature not finished!"),
            CompressionError::OptionIsNone => write!(f, "Option is None."),
            CompressionError::SystemTimeError(system_time_error) => write!(f, "SystemTimeError: {system_time_error}"),
        }
    }
}

impl Error for CompressionError {}

impl From<std::io::Error> for CompressionError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<SystemTimeError> for CompressionError {
    fn from(value: SystemTimeError) -> Self {
        Self::SystemTimeError(value)
    }
}