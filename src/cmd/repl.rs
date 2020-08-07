use crate::{db, eval::*, id, lang::*, store::fact};
use rustyline::{error::ReadlineError, Editor};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = Editor::<()>::new();

    if rl.load_history(".session").is_err() {
        println!("Creating new session...");
    }

    let db = &mut db::State::new();
    db.bootstrap();

    loop {
        match rl.readline("> ") {
            Ok(line) => {
                let debug = id::get("debug");
                let expr: Expr = line.parse()?;
                let evaled = eval(db, expr.clone());
                let pretty = prettify(0, db, &evaled);

                if db.has(fact(debug, id::get("inspect"), true)) {
                    println!(
                        "\nParsed: {:?}\n\nEvaled: {:?}\n\nPretty: {:?}\n",
                        expr, evaled, pretty
                    );
                }

                println!("\n{}\n", pretty);

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

        rl.save_history(".session")?;
    }

    Ok(())
}
