use std::path::PathBuf;

use crate::{compressed::CompressedData, error::CompressionResult};



pub trait Archiver {
    fn archive(files: &Vec<PathBuf>) -> CompressionResult<CompressedData>;
}