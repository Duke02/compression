use std::{os::unix::fs::MetadataExt, path::PathBuf, time::SystemTime};

use crate::{
    archivers::archiver::Archiver,
    compressed::CompressedData,
    compressors::compressor::Compressor,
    error::{CompressionError, CompressionResult},
    utils::{create_bytes_buffer, read_file},
};

const BLOCK_SIZE: usize = 512;

pub struct Tar {}

impl Tar {
    fn get_checksum(bytes: &[u8]) -> u64 {
        bytes.iter().map(|&b| b as u64).sum::<u64>()
    }

    fn make_blocks(file: &PathBuf) -> CompressionResult<Vec<u8>> {
        let bytes = read_file(file)?;
        let filename = create_bytes_buffer(
            &file
                .as_os_str()
                .to_str()
                .ok_or(CompressionError::OptionIsNone)?
                .to_string()
                .as_bytes(),
            100,
        );

        // NOTE: Only Unix supported.
        let metadata = file.metadata()?;
        let mode = create_bytes_buffer(&metadata.mode().to_ne_bytes(), 8);
        let uid = create_bytes_buffer(&metadata.uid().to_ne_bytes(), 8);
        let gid = create_bytes_buffer(&metadata.gid().to_ne_bytes(), 8);
        let size = create_bytes_buffer(&metadata.size().to_ne_bytes(), 12);
        let modified_time = create_bytes_buffer(
            &metadata
                .modified()?
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_secs()
                .to_ne_bytes(),
            12,
        );


        Ok(vec![])
    }
}

impl Archiver for Tar {
    fn archive(files: &Vec<std::path::PathBuf>) -> CompressionResult<CompressedData> {}
}
