use std::path::PathBuf;

use crate::{error::{CompressionError, CompressionResult}, utils::read_file};



pub struct CompressedData {
    pub data: Vec<u8>
}

impl CompressedData {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {data: bytes}
    }

    pub fn from_file(fp: &PathBuf) -> CompressionResult<Self> {
        let bytes = read_file(fp)?;
        Ok(Self::new(bytes))
    }
}