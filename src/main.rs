use walkdir::WalkDir;
use regex::Regex;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn search_in_file(file_path: &Path, search_pattern: &Regex) -> io::Result<()> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if search_pattern.is_match(&line) {
            println!(
                "Match found in file {} at line {}: {}",
                file_path.display(),
                index + 1,
                line
            );
        }
    }

    Ok(())
}

fn search_recursively(dir: &Path, pattern: &str) -> io::Result<()> {
    let search_pattern = Regex::new(pattern).expect("Invalid regex pattern");

    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        if path.is_file() {
            if let Err(e) = search_in_file(path, &search_pattern) {
                eprintln!("Failed to read file {}: {}", path.display(), e);
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let mut dir = String::new();
    let mut pattern = String::new();

    println!("Enter the directory path to search in:");
    io::stdout().flush()?; // Ensure the prompt is displayed
    io::stdin().read_line(&mut dir)?;
    let dir = dir.trim(); // Remove any trailing newline characters

    println!("Enter the search string:");
    io::stdout().flush()?; // Ensure the prompt is displayed
    io::stdin().read_line(&mut pattern)?;
    let pattern = pattern.trim(); // Remove any trailing newline characters

    search_recursively(Path::new(dir), pattern)?;

    Ok(())
}
