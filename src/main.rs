use std::fs;
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
    let dir = "."; // Replace with the desired directory path
    let string_to_find = "your_search_string"; // Replace with the string to search

    search_string(dir, string_to_find)?;

    Ok(())
}