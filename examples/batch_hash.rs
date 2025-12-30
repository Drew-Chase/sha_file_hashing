use sha_file_hashing::Hashable;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file1> [file2] [file3] ...", args[0]);
        eprintln!("\nExample:");
        eprintln!("  {} file1.txt file2.pdf file3.bin", args[0]);
        std::process::exit(1);
    }

    println!("Computing SHA-1 hashes for {} files...\n", args.len() - 1);

    let mut success_count = 0;
    let mut error_count = 0;

    for file_path in &args[1..] {
        let path = PathBuf::from(file_path);

        print!("{:<50} ", file_path);

        match path.hash() {
            Ok(hash) => {
                println!("{}", hash);
                success_count += 1;
            }
            Err(e) => {
                println!("ERROR: {}", e);
                error_count += 1;
            }
        }
    }

    println!("\n{} successful, {} errors", success_count, error_count);

    Ok(())
}
