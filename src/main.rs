use clap::{command, Arg, Command};

mod file_handler;

fn main() {
    let matches = command!()
        .about("Rust bookmark manager CLI")
        .subcommand(
            Command::new("create")
                .about("Create bookmark entry")
                .arg(Arg::new("url").help("URL to bookmark").required(true)),
        )
        .subcommand(Command::new("list").about("List bookmarks"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("create") {
        match matches.get_one::<String>("url") {
            Some(url) => {
                file_handler::save_bookmark(url.clone());
            }
            None => {
                println!("NOne");
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("list") {
        file_handler::list_bookmark();
    }
}
