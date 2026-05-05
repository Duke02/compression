use std::{fs::OpenOptions, io::Read, path::PathBuf, time::SystemTime};

use crate::error::{CompressionError, CompressionResult};


pub fn posix_time(dt: SystemTime) -> CompressionResult<u64> {
    Ok(dt.duration_since(SystemTime::UNIX_EPOCH)?.as_secs())
}

pub fn create_bytes_buffer(base_data: &[u8], max_size: usize) -> Vec<u8> {
    fn feed_into_buffer(i: usize, data: &[u8]) -> u8 {
        if i < data.len() {
            data[i]
        } else {
            0u8
        }
    }
    (0..max_size).map(|i| feed_into_buffer(i, base_data)).collect()
}


pub fn read_file(fp: &PathBuf) -> CompressionResult<Vec<u8>> {
    let mut file = match OpenOptions::new().read(true).create(false).open(fp) {
        Ok(f) => Ok(f),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => Err(CompressionError::FileNotFound(fp.clone())),
            _ => Err(CompressionError::IoError(e)),
        },
    }?;

    const BUFFER_SIZE: usize = 8192;
    let mut bytes_read = Vec::with_capacity(BUFFER_SIZE);
    let mut buffer = [0u8; BUFFER_SIZE];
    while let Ok(num_read) = file.read(&mut buffer) && (num_read == BUFFER_SIZE) {
        bytes_read.append(&mut buffer.to_vec());
        buffer.fill(0);
    }
    bytes_read.append(&mut buffer.to_vec());

    Ok(bytes_read)
}