use std::fs;
use std::io;
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

    println!("Enter the directory path:");
    let mut dir = String::new();
    io::stdin().read_line(&mut dir)?;
    let dir = dir.trim();

    println!("Enter the string to search:");
    let mut string_to_find = String::new();
    io::stdin().read_line(&mut string_to_find)?;
    let string_to_find = string_to_find.trim();
    search_string(dir, string_to_find)?;
    Ok(())
}
