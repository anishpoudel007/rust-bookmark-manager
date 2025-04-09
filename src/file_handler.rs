use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
};

const SAVE_FILE: &str = "./bm.txt";

pub fn save_bookmark(content: String) -> bool {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(SAVE_FILE)
        .expect("Failed to open or create the file");

    let content = format!("{}\n", content);

    file.write_all(content.as_bytes()).is_ok()
}

pub fn list_bookmark() {
    let file = File::open(SAVE_FILE).expect("File not found");

    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{:?}", line.expect("Error reading content"));
    }
}

pub fn init_bookmark() {
    match std::fs::exists(SAVE_FILE) {
        Ok(x) => match x {
            true => println!(
                "File already exists at \n {}",
                fs::canonicalize(Path::new(SAVE_FILE))
                    .unwrap()
                    .to_string_lossy()
            ),
            false => {
                std::fs::write(SAVE_FILE, "").unwrap();
                println!(
                    "Bookmark manager intialized at \n {}",
                    fs::canonicalize(Path::new(SAVE_FILE))
                        .unwrap()
                        .to_string_lossy()
                )
            }
        },
        Err(err) => println!("{:#?}", err),
    }
}
