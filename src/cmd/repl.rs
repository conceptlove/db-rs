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
        Seq(box a, box Nil) => eval(db, a),
        Seq(box Ident(e), box Ident(a)) => {
            let eid = reg::get(&e);
            let aid = reg::get(&a);
            db.get(&eid, &aid).into()
        }
        Op(box a, op, box b) => match (&a, op.as_str(), &b) {
            // (Ident(id), "=", b) => Nil,
            _ => Op(a.into(), op, b.into()),
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
