use sha_file_hashing::Hashable;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "Cargo.toml".to_string());

    println!("Opening file: {}", file_path);

    // Open file handle
    let file = File::open(&file_path)?;
    println!("File opened successfully\n");

    // Compute hash using the file handle
    println!("Computing hash...");
    let hash = file.hash()?;
    println!("SHA-1: {}", hash);

    // Reopen file for validation (since we consumed the first handle)
    let file = File::open(&file_path)?;

    println!("\nValidating hash...");
    let is_valid = file.validate(&hash)?;
    println!("Valid: {}", is_valid);

    Ok(())
}
