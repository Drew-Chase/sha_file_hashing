use sha_file_hashing::Hashable;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get file path from command line or use default
    let file_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "Cargo.toml".to_string());

    let path = Path::new(&file_path);

    // Check if file exists
    if !path.exists() {
        eprintln!("Error: File '{}' does not exist", file_path);
        std::process::exit(1);
    }

    // Compute the SHA-1 hash
    println!("Computing SHA-1 hash for: {}", file_path);
    let hash = path.hash()?;
    println!("SHA-1: {}", hash);

    // Validate the hash
    println!("\nValidating hash...");
    let is_valid = path.validate(&hash)?;
    println!("Valid: {}", is_valid);

    Ok(())
}
