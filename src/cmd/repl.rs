use crate::data;
use crate::data::*;
use crate::db;
use crate::parsing;
use crate::reg;
use data::Id::Id;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use Expr::*;

fn eval(db: &mut db::State, exp: parsing::Ast) -> parsing::Ast {
    return match exp {
        Seq(a, b) => match (*a, *b) {
            (Ident(e), Ident(a)) => db.get(&Id(reg::get(&e)), &Id(reg::get(&a))).into(),
            _ => Failure(parsing::ParseError::NotImplemented),
        },
        _ => Nil,
    };
}

pub fn run() {
    let mut rl = Editor::<()>::new();

    if rl.load_history(".session").is_err() {
        println!("Creating new session...");
    }

    let db = &mut db::State::new();
    db.bootstrap();

    loop {
        match rl.readline("> ") {
            Ok(line) => {
                let expr: parsing::Ast = line.parse().unwrap();

                println!("{}", eval(db, expr));

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

        rl.save_history(".session").unwrap();
    }
}
