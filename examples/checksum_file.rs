use sha_file_hashing::Hashable;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  Create checksums: {} create <file1> [file2] ...", args[0]);
        eprintln!("  Verify checksums: {} verify <checksum_file>", args[0]);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "create" => create_checksums(&args[2..])?,
        "verify" => verify_checksums(&args[2])?,
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            eprintln!("Use 'create' or 'verify'");
            std::process::exit(1);
        }
    }

    Ok(())
}

fn create_checksums(files: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if files.is_empty() {
        eprintln!("Error: No files specified");
        std::process::exit(1);
    }

    let checksum_file = "checksums.sha1";
    let mut output = File::create(checksum_file)?;

    println!("Creating checksums file: {}\n", checksum_file);

    for file_path in files {
        let path = Path::new(file_path);

        match path.hash() {
            Ok(hash) => {
                writeln!(output, "{}  {}", hash, file_path)?;
                println!("{:<50} OK", file_path);
            }
            Err(e) => {
                eprintln!("{:<50} ERROR: {}", file_path, e);
            }
        }
    }

    println!("\nChecksums written to: {}", checksum_file);
    Ok(())
}

fn verify_checksums(checksum_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(checksum_file)?;
    let reader = BufReader::new(file);

    println!("Verifying checksums from: {}\n", checksum_file);

    let mut total = 0;
    let mut passed = 0;
    let mut failed = 0;
    let mut missing = 0;

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.splitn(2, "  ").collect();

        if parts.len() != 2 {
            eprintln!("Warning: Invalid line format: {}", line);
            continue;
        }

        let expected_hash = parts[0];
        let file_path = parts[1];
        let path = Path::new(file_path);

        total += 1;

        if !path.exists() {
            println!("{:<50} MISSING", file_path);
            missing += 1;
            continue;
        }

        match path.validate(expected_hash) {
            Ok(true) => {
                println!("{:<50} OK", file_path);
                passed += 1;
            }
            Ok(false) => {
                println!("{:<50} FAILED", file_path);
                failed += 1;
            }
            Err(e) => {
                println!("{:<50} ERROR: {}", file_path, e);
                failed += 1;
            }
        }
    }

    println!("\nResults:");
    println!("  Total:   {}", total);
    println!("  Passed:  {}", passed);
    println!("  Failed:  {}", failed);
    println!("  Missing: {}", missing);

    if failed > 0 || missing > 0 {
        std::process::exit(1);
    }

    Ok(())
}
