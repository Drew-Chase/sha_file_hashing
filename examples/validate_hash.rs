use sha_file_hashing::Hashable;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <file_path> <expected_hash>", args[0]);
        eprintln!("\nExample:");
        eprintln!("  {} myfile.txt da39a3ee5e6b4b0d3255bfef95601890afd80709", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let expected_hash = &args[2];

    let path = Path::new(file_path);

    if !path.exists() {
        eprintln!("Error: File '{}' does not exist", file_path);
        std::process::exit(1);
    }

    // Validate the file against the expected hash
    println!("Validating file: {}", file_path);
    println!("Expected hash: {}", expected_hash);

    match path.validate(expected_hash) {
        Ok(true) => {
            println!("✓ Hash validation successful!");
            println!("The file matches the expected hash.");
            Ok(())
        }
        Ok(false) => {
            println!("✗ Hash validation failed!");
            println!("The file does NOT match the expected hash.");

            // Show actual hash for debugging
            if let Ok(actual_hash) = path.hash() {
                println!("\nActual hash: {}", actual_hash);
            }

            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Error during validation: {}", e);
            std::process::exit(1);
        }
    }
}
