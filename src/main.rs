pub mod data;
pub mod db;
pub mod machine;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    println!("Hello, world!");
    let mut rl = Editor::<()>::new();

    // if rl.load_history("session.txt").is_err() {
    //     println!("Creating new session...");
    // }

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
            }

            Err(ReadlineError::Interrupted) => {
                eprintln!("Exiting...");
                break;
            }

            Err(ReadlineError::Eof) => break,

            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }

        // rl.save_history("session.txt").unwrap();
    }
}
