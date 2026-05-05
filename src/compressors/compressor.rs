use std::path::PathBuf;

use crate::{compressed::CompressedData, error::CompressionResult, utils::read_file};



pub trait Compressor {
    fn compress_files(files: &Vec<PathBuf>) -> CompressionResult<CompressedData> {
        let bytes: Vec<u8> = files.iter().filter_map(|fp| read_file(fp).ok()).flatten().collect();

        Self::compress(&bytes)
    }

    fn compress_file(file: &PathBuf) -> CompressionResult<CompressedData> {
        let bytes = read_file(file)?;
        Self::compress(&bytes)
    }

    fn compress_data(data: &CompressedData) -> CompressionResult<CompressedData> {
        Self::compress(&data.data)
    }

    fn compress(bytes: &Vec<u8>) -> CompressionResult<CompressedData>;
}