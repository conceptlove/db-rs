#![feature(or_patterns)]
#![feature(box_patterns)]

pub mod bootstrap;
pub mod color;
pub mod db;
pub mod field;
pub mod lang;
pub mod machine;
pub mod store;
pub mod cmd {
    pub mod repl;
}

use clap::{App, SubCommand};

fn main() {
    let matches = App::new("ConceptDB")
        .version("0.1")
        .author("Jeff Peterson <jeff@yak.sh>")
        .about("Command-line access to the conceptual space.")
        .subcommand(SubCommand::with_name("repl").about("Starts the ConceptDb REPL"))
        .get_matches();

    match matches.subcommand_name() {
        Some("repl") => crate::cmd::repl::run(),
        Some(cmd) => eprintln!("Command not found: {}", cmd),
        None => eprintln!("Missing subcommand."),
    }
}
