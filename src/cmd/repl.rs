use crate::data::*;
use crate::db;
use crate::parsing;
use crate::reg;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use Expr::*;

fn eval(db: &mut db::State, exp: parsing::Ast) -> parsing::Ast {
    return match exp {
        Value(V::Ref(id)) => eval(db, db.for_entity(&id).into()),

        Ident(x) => eval(db, db.all(&reg::get(&x), &reg::get("binding")).into()),

        Seq(box a, box Nil) => eval(db, a),
        Seq(box Ident(e), box Ident(a)) => {
            let eid = reg::get(&e);
            let aid = reg::get(&a);
            db.all(&eid, &aid).into()
        }

        Op(box Ident(a), op, box Ident(b)) if op == "=" => {
            let aid = reg::get(&a);
            let bid = reg::get(&b);
            let alias = reg::get("alias");
            db.set(aid, alias, bid);
            db.set(bid, alias, aid);
            Nil
        }
        Op(box a, op, box b) if op == "=" => Op(a.into(), op, b.into()),
        _ => exp,
    };
}

fn prettify(db: &db::State, exp: &parsing::Ast) -> parsing::Ast {
    return match exp {
        Value(V::Ref(id)) => db.all(id, &reg::get("binding")).into(),
        _ => exp.map(|e| prettify(db, e)),
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
                let result = eval(db, expr);

                println!("\n{}\n", prettify(db, &result));

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
