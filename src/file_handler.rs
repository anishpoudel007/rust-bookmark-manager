use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
};

const SAVE_FILE: &str = "./bm.txt";

fn file_exists() -> bool {
    Path::new(SAVE_FILE).exists()
}

fn ensure_initialized() -> bool {
    if !file_exists() {
        eprintln!("No file initialized. Please run `bm init` command.");
        return false;
    }
    true
}

pub fn save_bookmark(content: String) -> bool {
    if !ensure_initialized() {
        return false;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(SAVE_FILE)
        .expect("Failed to open or create the file");

    let content = format!("{}\n", content);

    file.write_all(content.as_bytes()).is_ok()
}

pub fn list_bookmark() {
    if !ensure_initialized() {
        return;
    }

    let file = File::open(SAVE_FILE).expect("File not found");

    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{:?}", line.expect("Error reading content"));
    }
}

pub fn init_bookmark() {
    if file_exists() {
        match fs::canonicalize(SAVE_FILE) {
            Ok(path) => println!("File already exists at\n{}", path.to_string_lossy()),
            Err(err) => eprintln!("Failed to get file path: {}", err),
        }
    } else {
        match fs::write(SAVE_FILE, "") {
            Ok(_) => match fs::canonicalize(SAVE_FILE) {
                Ok(path) => println!(
                    "Bookmark manager initialized at\n{}",
                    path.to_string_lossy()
                ),
                Err(err) => eprintln!("Failed to get file path: {}", err),
            },
            Err(err) => eprintln!("Failed to create file : {}", err),
        }
    }
}
