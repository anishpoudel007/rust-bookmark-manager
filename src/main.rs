use clap::{command, Arg, Command};

mod file_handler;

fn main() {
    let matches = command!()
        .about("This program generates greeting card for the user")
        .subcommand(
            Command::new("create")
                .about("Create subcommand")
                .arg(Arg::new("url").help("URL to bookmark").required(true)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("create") {
        match matches.get_one::<String>("url") {
            Some(url) => {
                println!("URL is {}", url);
                file_handler::save_bookmark(url.clone());
            }
            None => {
                println!("NOne");
            }
        }
    }
}
