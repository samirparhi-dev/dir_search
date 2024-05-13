use std::fs;
use std::io::{self, Read};
use walkdir::WalkDir;

fn search_string(dir: &str, string: &str) -> Result<(), std::io::Error> {
    for entry in WalkDir::new(dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let path = entry.path();
            let contents = fs::read_to_string(path)?;
            if contents.contains(string) {
                println!("Found '{}' in {}", string, path.display());
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    // Prompt the user to enter the directory path
    println!("Enter the directory path:");
    let dir = read_input()?;

    // Prompt the user to enter the search string
    println!("Enter the string to search:");
    let string_to_find = read_input()?;

    // Call the search_string function with user-provided inputs
    search_string(&dir, &string_to_find)?;

    Ok(())
}

fn read_input() -> Result<String, io::Error> {
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;
    let input = String::from_utf8_lossy(&buffer);
    Ok(input.trim().to_string())
}
