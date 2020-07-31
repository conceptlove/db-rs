use crate::data;
use crate::data::*;
use crate::db;
use crate::parsing;
use crate::reg;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use Expr::*;

fn eval(db: &mut db::State, exp: parsing::Ast) -> parsing::Ast {
    return match exp {
        Ident(x) => Op(
            Ident(x.clone()).into(),
            "=".into(),
            Value(data::V::Ref(reg::get(&x))).into(),
        ),
        Seq(a, b) => match (*a, *b) {
            (a, Nil) => eval(db, a),
            (Ident(e), Ident(a)) => {
                let eid = reg::get(&e);
                let aid = reg::get(&a);
                db.get(&eid, &aid).into()
            }
            _ => Failure(parsing::ParseError::NotImplemented),
        },
        Op(a, op, b) => match (*a.clone(), op.as_str(), *b.clone()) {
            // (Ident(id), "=", b) => ,
            _ => Op(a, op, b),
        },
        _ => exp,
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

                println!("\n{}\n", eval(db, expr));

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
