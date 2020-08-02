use crate::data::*;
use crate::db;
use crate::reg;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use Expr::*;

fn eval(db: &mut db::State, exp: Expr) -> Expr {
    return match exp {
        Debug(x) => {
            db.toggle(&reg::get("debug"), &reg::get(&x));
            db.for_entity(&reg::get("debug")).into()
        }
        Value(V::Ref(id)) => eval(db, db.for_entity(&id).into()),

        Ident(x) => eval(db, db.all(&reg::get(&x), &reg::get("id")).into()),

        Seq(box a, box Nil) => eval(db, a),
        Seq(box Ident(e), box Ident(a)) => {
            let eid = reg::get(&e);
            let aid = reg::get(&a);
            db.all(&eid, &aid).into()
        }

        Many(box a, box b) => Many(eval(db, a).into(), eval(db, b).into()),

        Op(box Ident(a), op, box Ident(b)) if op == "=" => {
            let aid = reg::get(&a);
            let bid = reg::get(&b);
            let alias = reg::get("alias");
            db.set(&aid, &alias, bid);
            db.set(&bid, &alias, aid);
            Nil
        }
        Op(box a, op, box b) if op == "=" => Op(a.into(), op, b.into()),

        Op(box Value(V::Int(a)), op, box Value(V::Int(b))) if op == "+" => Value(V::Int(a + b)),
        _ => exp,
    };
}

fn prettify(db: &db::State, exp: &Expr) -> Expr {
    return match exp {
        Value(V::Ref(id)) => Expr::from(db.all(id, &reg::get("alias"))).or(V::Ref(*id)),
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
                let debug = reg::get("debug");
                let expr: Expr = line.parse().unwrap();
                let evaled = eval(db, expr.clone());
                let pretty = prettify(db, &evaled);

                if db.has(fact(debug, reg::get("inspect"), true)) {
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

        rl.save_history(".session").unwrap();
    }
}
