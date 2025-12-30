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
    /// ```no_run
    /// use sha_file_hashing::Hashable;
    /// use std::path::Path;
    ///
    /// let path = Path::new("example.txt");
    /// match path.hash() {
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
    ///   the hash value to be validated.
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
    /// ```no_run
    /// use sha_file_hashing::Hashable;
    /// use std::path::Path;
    ///
    /// let path = Path::new("myfile.txt");
    /// match path.validate("da39a3ee5e6b4b0d3255bfef95601890afd80709") {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_hash_file_basic() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"Hello, World!").unwrap();
        temp_file.flush().unwrap();

        let file = File::open(temp_file.path()).unwrap();
        let hash = hash_file(file).unwrap();

        assert_eq!(hash, "0a0a9f2a6772942557ab5355d76af442f8f65e01");
    }

    #[test]
    fn test_hash_file_empty() {
        let temp_file = NamedTempFile::new().unwrap();

        let file = File::open(temp_file.path()).unwrap();
        let hash = hash_file(file).unwrap();

        assert_eq!(hash, "da39a3ee5e6b4b0d3255bfef95601890afd80709");
    }

    #[test]
    fn test_validate_file_correct_hash() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"test data").unwrap();
        temp_file.flush().unwrap();

        let file = File::open(temp_file.path()).unwrap();
        let is_valid = validate_file(file, "f48dd853820860816c75d54d0f584dc863327a7c");

        assert!(is_valid);
    }

    #[test]
    fn test_validate_file_incorrect_hash() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"test data").unwrap();
        temp_file.flush().unwrap();

        let file = File::open(temp_file.path()).unwrap();
        let is_valid = validate_file(file, "0000000000000000000000000000000000000000");

        assert!(!is_valid);
    }

    #[test]
    fn test_validate_file_case_insensitive() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"test data").unwrap();
        temp_file.flush().unwrap();

        let file = File::open(temp_file.path()).unwrap();
        let is_valid = validate_file(file, "F48DD853820860816C75D54D0F584DC863327A7C");

        assert!(is_valid);
    }

    #[test]
    fn test_hash_file_from_path_success() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"path test").unwrap();
        temp_file.flush().unwrap();

        let hash = hash_file_from_path(temp_file.path()).unwrap();
        assert_eq!(40, hash.len());
    }

    #[test]
    fn test_hash_file_from_path_not_found() {
        let result = hash_file_from_path("nonexistent_file_12345.txt");
        assert!(result.is_err());

        match result {
            Err(SHAError::IO(e)) => assert_eq!(e.kind(), std::io::ErrorKind::NotFound),
            _ => panic!("Expected IO error"),
        }
    }

    #[test]
    fn test_validate_file_from_path_success() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"validation test").unwrap();
        temp_file.flush().unwrap();

        let hash = hash_file_from_path(temp_file.path()).unwrap();
        let is_valid = validate_file_from_path(temp_file.path(), &hash).unwrap();

        assert!(is_valid);
    }

    #[test]
    fn test_validate_file_from_path_file_not_found() {
        let result = validate_file_from_path("nonexistent_file_12345.txt", "abc123");
        assert!(result.is_err());
    }

    #[test]
    fn test_path_hashable() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"trait test").unwrap();
        temp_file.flush().unwrap();

        let path = temp_file.path();
        let hash = path.hash().unwrap();

        assert_eq!(hash.len(), 40);
        assert!(path.validate(&hash).unwrap());
    }

    #[test]
    fn test_pathbuf_hashable() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"pathbuf test").unwrap();
        temp_file.flush().unwrap();

        let pathbuf = temp_file.path().to_path_buf();
        let hash = pathbuf.hash().unwrap();

        assert_eq!(hash.len(), 40);
        assert!(pathbuf.validate(&hash).unwrap());
    }

    #[test]
    fn test_file_hashable() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"file trait test").unwrap();
        temp_file.flush().unwrap();

        let file = File::open(temp_file.path()).unwrap();
        let hash = file.hash().unwrap();

        let file = File::open(temp_file.path()).unwrap();
        assert!(file.validate(&hash).unwrap());
    }

    #[test]
    fn test_sha_error_display() {
        let error = SHAError::FailedValidation("test.txt".to_string());
        assert_eq!(error.to_string(), "SHA validation failed for file: test.txt");
    }

    #[test]
    fn test_large_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let large_data = vec![b'A'; 20000];
        temp_file.write_all(&large_data).unwrap();
        temp_file.flush().unwrap();

        let file = File::open(temp_file.path()).unwrap();
        let hash = hash_file(file).unwrap();

        assert_eq!(hash.len(), 40);

        let file = File::open(temp_file.path()).unwrap();
        assert!(validate_file(file, &hash));
    }

    #[test]
    fn test_binary_data() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let binary_data = vec![0x00, 0x01, 0x02, 0xFF, 0xFE, 0xFD];
        temp_file.write_all(&binary_data).unwrap();
        temp_file.flush().unwrap();

        let file = File::open(temp_file.path()).unwrap();
        let hash = hash_file(file).unwrap();

        let file = File::open(temp_file.path()).unwrap();
        assert!(validate_file(file, &hash));
    }
}
