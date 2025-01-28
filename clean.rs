use std::{fs, path::Path};

fn main() {
    clean_generated_files();
}

fn clean_generated_files() {
    // Specify the directory to be cleaned
    let paths_to_clean = vec![
        "generated" // This will remove the entire `generated` directory
    ];

    for path in paths_to_clean {
        if Path::new(path).exists() {
            match fs::remove_dir_all(path) {
                Ok(_) => println!("Cleaned directory: {}", path),
                Err(err) => eprintln!("Error cleaning directory {}: {}", path, err),
            }
        }
    }
}
