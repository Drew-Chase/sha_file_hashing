# SHA File Hashing

A Rust library for computing and validating SHA-1 file hashes with a clean, trait-based API.

## Features

- **Simple API**: Hash and validate files using the `Hashable` trait
- **Multiple Input Types**: Works with `Path`, `PathBuf`, and `File` types
- **Efficient Processing**: Uses buffered reading for memory-efficient hashing of large files
- **Error Handling**: Comprehensive error types using `thiserror`
- **Case-Insensitive Validation**: Hash validation is case-insensitive for convenience

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sha_file_hashing = "0.1.0"
```

## Usage

### Basic File Hashing

```rust,no_run
use sha_file_hashing::Hashable;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("example.txt");

    // Compute hash
    let hash = path.hash()?;
    println!("SHA-1: {}", hash);

    // Validate hash
    let is_valid = path.validate(&hash)?;
    println!("Valid: {}", is_valid);

    Ok(())
}
```

### Using PathBuf

```rust,no_run
use sha_file_hashing::Hashable;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from("documents/file.pdf");
    let hash = path.hash()?;
    println!("Hash: {}", hash);
    Ok(())
}
```

### Using File Handle

```rust,no_run
use sha_file_hashing::Hashable;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("data.bin")?;
    let hash = file.hash()?;

    // Validate with a known hash
    let expected = "da39a3ee5e6b4b0d3255bfef95601890afd80709";
    let is_valid = file.validate(expected)?;

    Ok(())
}
```

### Direct Function API

If you prefer not to use the trait, you can use the functions directly:

```rust,no_run
use sha_file_hashing::{hash_file_from_path, validate_file_from_path};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("myfile.txt");

    // Hash a file
    let hash = hash_file_from_path(path)?;

    // Validate a file
    let is_valid = validate_file_from_path(path, &hash)?;

    Ok(())
}
```

## API Reference

### Trait: `Hashable`

The main trait providing hashing functionality:

- **`hash(&self) -> Result<String, SHAError>`**
  Computes and returns the SHA-1 hash of the file as a hexadecimal string.

- **`validate(&self, hash: impl AsRef<str>) -> Result<bool, SHAError>`**
  Validates whether the file matches the provided hash (case-insensitive).

### Functions

- **`hash_file(file: File) -> Result<String, SHAError>`**
  Computes SHA-1 hash from a `File` handle.

- **`hash_file_from_path(path: impl AsRef<Path>) -> Result<String, SHAError>`**
  Computes SHA-1 hash from a file path.

- **`validate_file(file: File, hash: impl AsRef<str>) -> bool`**
  Validates a file's hash from a `File` handle.

- **`validate_file_from_path(path: impl AsRef<Path>, hash: impl AsRef<str>) -> Result<bool, SHAError>`**
  Validates a file's hash from a file path.

### Error Types

```rust
pub enum SHAError {
    FailedValidation(String),
    IO(std::io::Error),
}
```

- **`FailedValidation`**: Hash validation failed
- **`IO`**: I/O error occurred (file not found, permission denied, etc.)

## Implementation Details

- Uses the `sha1` crate (v0.11.0) for SHA-1 computation
- Processes files in 8KB chunks for memory efficiency
- Suitable for hashing files of any size
- Hash comparison is case-insensitive

## Requirements

- Rust 2024 edition or later

## License

See [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
