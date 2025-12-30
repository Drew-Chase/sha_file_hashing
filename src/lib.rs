#![doc = include_str!("../README.md")]

use sha1::{Digest, Sha1};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

pub trait Hashable {
    /// Computes and returns the hash of the current object's data as a `String`.
    ///
    /// # Returns
    /// - `Ok(String)`: A `String` representation of the computed hash if the operation is successful.
    /// - `Err(SHAError)`: An error of type `SHAError` if the hashing process fails.
    ///
    /// # Errors
    /// This function will return a `SHAError` if there is an issue during the hashing process,
    /// such as invalid input or internal computation errors.
    ///
    /// # Examples
    /// ```rust
    /// let data = MyStruct::new("example data");
    /// match data.hash() {
    ///     Ok(hash) => println!("Hash: {}", hash),
    ///     Err(e) => println!("Error occurred: {:?}", e),
    /// }
    /// ```
    fn hash(&self) -> Result<String, SHAError>;
    ///
    /// Validates the given hash against some internal criteria or expected value.
    ///
    /// # Parameters
    /// - `hash`: An object that can be converted to a string slice (`&str`). This represents
    ///           the hash value to be validated.
    ///
    /// # Returns
    /// - `Ok(true)`: Indicates that the hash is valid and meets the criteria.
    /// - `Ok(false)`: Indicates that the hash is invalid or doesn't meet the criteria.
    /// - `Err(SHAError)`: An error occurred during the validation process, encapsulated in a `SHAError`.
    ///
    /// # Errors
    /// May return a `SHAError` if there is an issue during the validation, such as an invalid
    /// hash format, or other internal processing errors.
    ///
    /// # Example
    /// ```
    /// let validator = SomeValidator::new();
    /// match validator.validate("abc123") {
    ///     Ok(true) => println!("Hash is valid."),
    ///     Ok(false) => println!("Hash is invalid."),
    ///     Err(e) => eprintln!("Validation error: {}", e),
    /// }
    /// ```
    ///
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
