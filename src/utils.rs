use std::{fs::OpenOptions, io::Read, path::PathBuf};

use crate::error::CompressionError;



pub fn read_file(fp: &PathBuf) -> Result<Vec<u8>, CompressionError> {
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