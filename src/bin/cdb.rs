use clap::{App, SubCommand};
use conceptdb::cmd;

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
