use sha_file_hashing::{hash_file_from_path, validate_file_from_path, Hashable, SHAError};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Helper function to create a temporary file with content
fn create_temp_file(dir: &TempDir, name: &str, content: &[u8]) -> PathBuf {
    let file_path = dir.path().join(name);
    let mut file = File::create(&file_path).unwrap();
    file.write_all(content).unwrap();
    file_path
}

#[test]
fn test_hash_file_from_path() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_file(&temp_dir, "test.txt", b"Hello, World!");

    let hash = hash_file_from_path(&file_path).unwrap();
    // SHA-1 hash of "Hello, World!"
    assert_eq!(hash, "0a0a9f2a6772942557ab5355d76af442f8f65e01");
}

#[test]
fn test_hash_empty_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_file(&temp_dir, "empty.txt", b"");

    let hash = hash_file_from_path(&file_path).unwrap();
    // SHA-1 hash of empty string
    assert_eq!(hash, "da39a3ee5e6b4b0d3255bfef95601890afd80709");
}

#[test]
fn test_hash_large_file() {
    let temp_dir = TempDir::new().unwrap();
    // Create a file larger than the buffer size (8192 bytes)
    let content = vec![b'A'; 20000];
    let file_path = create_temp_file(&temp_dir, "large.txt", &content);

    let hash = hash_file_from_path(&file_path).unwrap();
    assert!(!hash.is_empty());
    assert_eq!(hash.len(), 40); // SHA-1 produces 40 hex characters
}

#[test]
fn test_validate_file_from_path_success() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_file(&temp_dir, "test.txt", b"Hello, World!");

    let hash = "0a0a9f2a6772942557ab5355d76af442f8f65e01";
    let is_valid = validate_file_from_path(&file_path, hash).unwrap();
    assert!(is_valid);
}

#[test]
fn test_validate_file_from_path_case_insensitive() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_file(&temp_dir, "test.txt", b"Hello, World!");

    // Test with uppercase hash
    let hash_upper = "0A0A9F2A6772942557AB5355D76AF442F8F65E01";
    let is_valid = validate_file_from_path(&file_path, hash_upper).unwrap();
    assert!(is_valid);

    // Test with mixed case
    let hash_mixed = "0a0A9f2A6772942557aB5355d76AF442f8f65E01";
    let is_valid = validate_file_from_path(&file_path, hash_mixed).unwrap();
    assert!(is_valid);
}

#[test]
fn test_validate_file_from_path_failure() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_file(&temp_dir, "test.txt", b"Hello, World!");

    let wrong_hash = "0000000000000000000000000000000000000000";
    let is_valid = validate_file_from_path(&file_path, wrong_hash).unwrap();
    assert!(!is_valid);
}

#[test]
fn test_hash_nonexistent_file() {
    let result = hash_file_from_path("nonexistent_file.txt");
    assert!(result.is_err());

    if let Err(SHAError::IO(e)) = result {
        assert_eq!(e.kind(), std::io::ErrorKind::NotFound);
    } else {
        panic!("Expected IO error");
    }
}

#[test]
fn test_validate_nonexistent_file() {
    let result = validate_file_from_path("nonexistent_file.txt", "abc123");
    assert!(result.is_err());
}

#[test]
fn test_path_hashable_trait() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_file(&temp_dir, "test.txt", b"Test content");
    let path = Path::new(&file_path);

    let hash = path.hash().unwrap();
    assert!(!hash.is_empty());
    assert_eq!(hash.len(), 40);

    let is_valid = path.validate(&hash).unwrap();
    assert!(is_valid);
}

#[test]
fn test_pathbuf_hashable_trait() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_file(&temp_dir, "test.txt", b"Test content");

    let hash = file_path.hash().unwrap();
    assert!(!hash.is_empty());
    assert_eq!(hash.len(), 40);

    let is_valid = file_path.validate(&hash).unwrap();
    assert!(is_valid);
}

#[test]
fn test_file_hashable_trait() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_file(&temp_dir, "test.txt", b"File handle test");

    let file = File::open(&file_path).unwrap();
    let hash = file.hash().unwrap();
    assert!(!hash.is_empty());
    assert_eq!(hash.len(), 40);

    let file = File::open(&file_path).unwrap();
    let is_valid = file.validate(&hash).unwrap();
    assert!(is_valid);
}

#[test]
fn test_binary_file() {
    let temp_dir = TempDir::new().unwrap();
    let binary_content = vec![0x00, 0x01, 0x02, 0xFF, 0xFE, 0xFD];
    let file_path = create_temp_file(&temp_dir, "binary.bin", &binary_content);

    let hash = hash_file_from_path(&file_path).unwrap();
    assert!(!hash.is_empty());

    let is_valid = validate_file_from_path(&file_path, &hash).unwrap();
    assert!(is_valid);
}

#[test]
fn test_consistent_hashing() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_file(&temp_dir, "test.txt", b"Consistency test");

    // Hash the same file multiple times
    let hash1 = hash_file_from_path(&file_path).unwrap();
    let hash2 = hash_file_from_path(&file_path).unwrap();
    let hash3 = hash_file_from_path(&file_path).unwrap();

    assert_eq!(hash1, hash2);
    assert_eq!(hash2, hash3);
}

#[test]
fn test_different_files_different_hashes() {
    let temp_dir = TempDir::new().unwrap();
    let file1 = create_temp_file(&temp_dir, "file1.txt", b"Content 1");
    let file2 = create_temp_file(&temp_dir, "file2.txt", b"Content 2");

    let hash1 = hash_file_from_path(&file1).unwrap();
    let hash2 = hash_file_from_path(&file2).unwrap();

    assert_ne!(hash1, hash2);
}
