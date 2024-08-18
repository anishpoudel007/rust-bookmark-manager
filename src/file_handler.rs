use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

pub fn save_bookmark(content: String) -> bool {
    let file_path = "/home/anishpoudel/delete-me-later/bm.txt";

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .expect("Failed to open or create the file");

    let content = format!("{}\n", content);

    file.write_all(content.as_bytes()).is_ok()
}

pub fn list_bookmark() {
    let file_path = "/home/anishpoudel/delete-me-later/bm.txt";

    let mut file = File::open(file_path).expect("File not found");

    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("Could not read file");

    println!("{}", content);
}
