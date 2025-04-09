use bm::{
    argc::argc_app,
    file_handler::{init_bookmark, list_bookmark, save_bookmark},
};

fn handle_create(matches: &clap::ArgMatches) {
    if let Some(url) = matches.get_one::<String>("url") {
        let is_saved = save_bookmark(url.clone());
        if is_saved {
            println!("Bookmark added.");
        } else {
            println!("Bookmark could not be added.");
        }
    } else {
        println!("URL is required to create a bookmark.");
    }
}

fn main() {
    let matches = argc_app();

    match matches.subcommand() {
        Some(("init", _)) => init_bookmark(),
        Some(("create", sub_matches)) => handle_create(sub_matches),
        Some(("list", _)) => list_bookmark(),
        _ => eprintln!("Invalid command. Use `init`, `create`, or `list`."),
    }
}
