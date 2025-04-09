use bm::{
    argc::argc_app,
    file_handler::{init_bookmark, list_bookmark, save_bookmark},
};

fn main() {
    let matches = argc_app();

    if matches.subcommand_matches("init").is_some() {
        init_bookmark();
    }

    if let Some(matches) = matches.subcommand_matches("create") {
        match matches.get_one::<String>("url") {
            Some(url) => {
                save_bookmark(url.clone());
                println!("Bookmark added.");
            }
            None => {
                println!("None");
            }
        }
    }

    if matches.subcommand_matches("list").is_some() {
        list_bookmark();
    }
}
