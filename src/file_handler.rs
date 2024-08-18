use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
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

    let file = File::open(file_path).expect("File not found");

    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{:?}", line.expect("Error reading content"));
    }
}
