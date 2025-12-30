use sha1::{Digest, Sha1};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

pub trait Hashable {
    fn hash(&self) -> Result<String, SHAError>;
    fn validate(&self, hash: impl AsRef<str>) -> Result<bool, SHAError>;
}

impl Hashable for Path {
    fn hash(&self) -> Result<String, SHAError> {
        hash_file_from_path(self)
    }

    fn validate(&self, hash: impl AsRef<str>) -> Result<bool, SHAError> {
        validate_file_from_path(self, hash)
    }
}

impl Hashable for File {
    fn hash(&self) -> Result<String, SHAError> {
        hash_file(self.try_clone()?)
    }

    fn validate(&self, hash: impl AsRef<str>) -> Result<bool, SHAError> {
        if let Ok(file) = self.try_clone() {
            Ok(validate_file(file, hash))
        } else {
            Err(SHAError::IO(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to clone file",
            )))
        }
    }
}

impl Hashable for PathBuf {
    fn hash(&self) -> Result<String, SHAError> {
        hash_file_from_path(self)
    }
    fn validate(&self, hash: impl AsRef<str>) -> Result<bool, SHAError> {
        validate_file_from_path(self, hash)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SHAError {
    #[error("SHA validation failed for file: {0}")]
    FailedValidation(String),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}

pub fn validate_file_from_path(
    path: impl AsRef<Path>,
    hash: impl AsRef<str>,
) -> Result<bool, SHAError> {
    let Ok(file) = File::open(path.as_ref()) else {
        return Err(SHAError::IO(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found",
        )));
    };
    Ok(validate_file(file, hash))
}

pub fn hash_file_from_path(path: impl AsRef<Path>) -> Result<String, SHAError> {
    let path = path.as_ref();
    if !path.exists() {
        Err(SHAError::IO(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found",
        )))
    } else {
        let file = File::open(path)?;
        let computed = hash_file(file)?;
        Ok(computed)
    }
}

pub fn validate_file(file: File, hash: impl AsRef<str>) -> bool {
    let mut reader = BufReader::new(file);
    let mut hasher = Sha1::new();
    let mut buffer = [0u8; 8192];

    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => hasher.update(&buffer[..n]),
            Err(_) => return false,
        }
    }

    let result = hasher.finalize();
    let computed: String = result.iter().map(|b| format!("{:02x}", b)).collect();
    computed.eq_ignore_ascii_case(hash.as_ref())
}

pub fn hash_file(file: File) -> Result<String, SHAError> {
    let mut reader = BufReader::new(file);
    let mut hasher = Sha1::new();
    let mut buffer = [0u8; 8192];

    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => hasher.update(&buffer[..n]),
            Err(e) => return Err(SHAError::IO(e)),
        }
    }

    let result = hasher.finalize();
    let computed: String = result.iter().map(|b| format!("{:02x}", b)).collect();
    Ok(computed)
}
