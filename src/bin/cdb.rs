use clap::{App, Arg, SubCommand};
use conceptdb::cmd;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("ConceptDB")
        .version("0.1")
        .author("Jeff Peterson <jeff@yak.sh>")
        .about("Command-line access to the conceptual space.")
        .subcommand(SubCommand::with_name("repl").about("Starts the ConceptDb REPL"))
        .subcommand(SubCommand::with_name("ui").about("Starts the ConceptDb UI"))
        .subcommand(
            SubCommand::with_name("id")
                .about("Gets the id for the given argument")
                .arg(Arg::with_name("name")),
        )
        .get_matches();

    if let Some(sub) = matches.subcommand_name() {
        match (sub, matches.subcommand_matches(sub)) {
            ("repl", _) => cmd::repl::run(),
            ("ui", _) => cmd::ui::run(),
            ("id", Some(m)) => cmd::id::run(m.value_of("name")),
            (cmd, _) => {
                eprintln!("Command not found: {}", cmd);
                Ok(())
            }
        }
    } else {
        eprintln!("Missing subcommand.");
        Ok(())
    }
}
