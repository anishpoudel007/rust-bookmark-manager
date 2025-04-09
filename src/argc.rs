use clap::{command, Arg, ArgMatches, Command};

pub fn argc_app() -> ArgMatches {
    command!()
        .about("Rust bookmark manager CLI")
        .subcommand(Command::new("init").about("Initialize bookmark manager."))
        .subcommand(
            Command::new("create")
                .about("Create bookmark entry")
                .alias("add")
                .arg(Arg::new("url").help("URL to bookmark").required(true)),
        )
        .subcommand(Command::new("list").about("List bookmarks").alias("ls"))
        .get_matches()
}
