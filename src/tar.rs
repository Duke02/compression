use crate::{compressor::Compressor, error::CompressionError, utils::read_file};



struct Tar {

}

impl Tar {

}

impl Compressor for Tar {
    fn compress(files: &Vec<std::path::PathBuf>) -> Result<std::path::PathBuf, CompressionError> {
        let bytes: Vec<_> = files.iter().filter_map(|fp| read_file(fp).ok()).flatten().collect();
        
        if bytes.is_empty() && !files.is_empty() {
            return Err(CompressionError::Generic("Could not read files provided to tar.".to_string()))
        }

        


        Err(CompressionError::NotFinished)
    }
}