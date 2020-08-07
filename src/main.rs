#![feature(or_patterns)]
#![feature(box_patterns)]

pub mod bootstrap;
pub mod color;
pub mod db;
pub mod field;
pub mod id;
pub mod lang;
pub mod machine;
pub mod store;
pub mod cmd {
    pub mod repl;
    pub mod ui;
}

use clap::{App, SubCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("ConceptDB")
        .version("0.1")
        .author("Jeff Peterson <jeff@yak.sh>")
        .about("Command-line access to the conceptual space.")
        .subcommand(SubCommand::with_name("repl").about("Starts the ConceptDb REPL"))
        .subcommand(SubCommand::with_name("ui").about("Starts the ConceptDb UI"))
        .get_matches();

    match matches.subcommand_name() {
        Some("repl") => crate::cmd::repl::run(),
        Some("ui") => crate::cmd::ui::run(),
        Some(cmd) => {
            eprintln!("Command not found: {}", cmd);
            Ok(())
        }
        None => {
            eprintln!("Missing subcommand.");
            Ok(())
        }
    }
}
