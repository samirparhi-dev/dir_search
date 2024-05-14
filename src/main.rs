use walkdir::WalkDir;
use regex::Regex;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::path::PathBuf;

//For concurrency
use std::sync::mpsc;
use std::thread;

struct SearchResult {
    file_path: String,
    line_number: usize,
    line_content: String,
}
//fn search_inside_file(file_path: &Path, search_pattern: &Regex, sender: mpsc::Sender<SearchResult>)
fn search_inside_file(file_path: &Path, search_pattern: &Regex) -> io::Result<()> {
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
//fn search_folder_recursively(dir: &Path, pattern: &str, sender: mpsc::Sender<SearchResult>)
fn search_folder_recursively(dir: &Path, pattern: &str) -> io::Result<()> {
    let search_pattern = Regex::new(pattern).expect("Invalid regex pattern");

    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        if path.is_file() {
            if let Err(e) = search_inside_file(path, &search_pattern) {
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

    io::stdout().flush()?;

    io::stdin().read_line(&mut dir)?;

    let dir = dir.trim(); 

    let dir_path = PathBuf::from(dir);

    if !dir_path.is_absolute() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Please provide an absolute path for the directory.",
        ));
    }
    println!("Enter the search string:");
    io::stdout().flush()?;
    io::stdin().read_line(&mut pattern)?;
    let pattern = pattern.trim();
    search_folder_recursively(Path::new(dir), pattern)?;

    // Concurrency to be implemented
    // // A channel for sending search results
    // let (sender, receiver) = mpsc::channel();
    // // Spawn a thread to collect search results and print them
    // let handle = thread::spawn(move || {
    //     for result in receiver {
    //         println!(
    //             "Match found in file {} at line {}: {}",
    //             result.file_path,
    //             result.line_number,
    //             result.line_content
    //         );
    //     }
    // });
    // // Start the directory search
    // search_folder_recursively(Path::new(dir), pattern, sender)?;

    // // Wait for the thread to finish
    // handle.join().expect("Failed to join thread");


    Ok(())
}
