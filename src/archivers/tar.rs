use std::{
    ffi::CStr,
    fs::{File, Metadata},
    os::unix::fs::MetadataExt,
    path::PathBuf,
    time::SystemTime,
};

use itertools::Itertools;

use crate::{
    archivers::archiver::Archiver,
    compressed::CompressedData,
    compressors::compressor::Compressor,
    error::{CompressionError, CompressionResult},
    utils::{create_bytes_buffer, read_file},
};

const BLOCK_SIZE: usize = 512;

#[derive(PartialEq)]
enum FileTypeFlag {
    RegularFile = 0,
    Link = 1,
    Sym = 2,
    CharacterSpecial = 3,
    BlockSpecial = 4,
    Directory = 5,
    FIFOSpecial = 6,
    Content = 7,
}

pub struct Tar {}

impl Tar {
    fn get_checksum(bytes: &[u8]) -> u64 {
        bytes.iter().map(|&b| b as u64).sum::<u64>()
    }

    fn get_type_flag(m: &Metadata) -> FileTypeFlag {
        if m.is_dir() {
            FileTypeFlag::Directory
        } else if m.is_symlink() {
            FileTypeFlag::Link
        } else {
            FileTypeFlag::RegularFile
        }
    }

    fn make_header(file: &PathBuf) -> CompressionResult<Vec<u8>> {
        // let bytes = read_file(file)?;
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
        let type_flag = Tar::get_type_flag(&metadata);
        let link_name = if type_flag != FileTypeFlag::Link {
            [0u8; 100]
        } else {
            /* TODO */
            [0u8; 100]
        };
        let type_flag_bytes = type_flag as u8;
        let magic = [117_u8, 115, 116, 97, 114];
        let version = [0_u8, 0];
        // TODO: Actually do this dog.
        let uname = [0u8; 32];
        let gname = [0u8; 32];
        // TODO: Can't find where this is defined in the specs but it do be in there but like where it be dog?
        let dev_major = [0u8; 8];
        let dev_minor = [0u8; 8];
        let prefix = [0u8; 155];
        let mut header = vec![
            filename.to_vec(),
            mode.to_vec(),
            uid.to_vec(),
            gid.to_vec(),
            size.to_vec(),
            modified_time.to_vec(),
            vec![0u8; 8], // Check sum
            vec![type_flag_bytes],
            link_name.to_vec(),
            magic.to_vec(),
            version.to_vec(),
            uname.to_vec(),
            gname.to_vec(),
            dev_major.to_vec(),
            dev_minor.to_vec(),
            prefix.to_vec(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

        let check_sum = Tar::get_checksum(&header).to_ne_bytes();
        for i in 148..156 {
            header[i] = check_sum[i - 148];
        }

        Ok(header)
    }

    fn make_blocks(file: &PathBuf) -> CompressionResult<Vec<u8>> {
        let header = Self::make_header(file)?;
        let bytes = read_file(file)?;
        let full_thing = vec![
            header,
            bytes
                .chunks(BLOCK_SIZE)
                .map(|c| {
                    if c.len() == BLOCK_SIZE {
                        c.to_vec()
                    } else {
                        create_bytes_buffer(c, BLOCK_SIZE)
                    }
                })
                .flatten()
                .collect::<Vec<_>>(),
        ];
        Ok(full_thing.into_iter().flatten().collect())
    }
}

impl Archiver for Tar {
    fn archive(files: &Vec<std::path::PathBuf>) -> CompressionResult<CompressedData> {
        let bytes = files
            .iter()
            .filter_map(|fp| Tar::make_blocks(fp).ok())
            .flatten()
            .collect();
        Ok(CompressedData::new(bytes))
    }
}
