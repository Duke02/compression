use std::path::{Path, PathBuf};

use crate::error::CompressionError;



pub trait Compressor {
    fn compress(files: &Vec<PathBuf>) -> Result<PathBuf, CompressionError>;
}