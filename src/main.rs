pub mod data;
// pub mod db;
pub mod machine;
pub mod reg;

use rustyline::error::ReadlineError;
use rustyline::Editor;

type State = im::Vector<String>;

enum Msg {
    Token(String),
}

enum Cmd {
    NoOp,
    StdOut(String),
}

fn update(state: &mut State, msg: Msg) -> (&State, Cmd) {
    match msg {
        Msg::Token(name) => match name.as_ref() {
            "id" => {
                let ent = state.last().unwrap();
                let uuid = reg::get(&ent);
                (
                    state,
                    Cmd::StdOut(format!("{} id = {}", ent, format_uuid!(uuid))),
                )
            }

            n => {
                state.push_back(n.to_owned());
                (state, Cmd::NoOp)
            }
        },
    }
}

fn main() {
    println!("Hello, world!");
    let mut rl = Editor::<()>::new();

    // if rl.load_history("session.txt").is_err() {
    //     println!("Creating new session...");
    // }

    let mut state = im::Vector::new();

    loop {
        match rl.readline("> ") {
            Ok(line) => {
                let tokens = line.split_terminator(" ");

                for token in tokens {
                    let (_, cmd) = update(&mut state, Msg::Token(token.to_owned()));
                    // state = state2;

                    match cmd {
                        Cmd::NoOp => (),
                        Cmd::StdOut(x) => println!("{}", x),
                    }
                }

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
