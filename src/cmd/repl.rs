use crate::db;
use crate::id;
use crate::lang::*;
use crate::store::fact;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use Expr::*;

fn eval(db: &mut db::State, exp: Expr) -> Expr {
    return match exp {
        Seq(box Debug(x), box Ident(name)) if x == "id" => id::get(&name).into(),
        Debug(x) if x == "db" => db.eav.iter().collect(),
        Debug(x) => {
            db.toggle(&id::get("debug"), &id::get(&x));
            db.for_entity(&id::get("debug")).into()
        }
        Ref(id) => eval(db, db.for_entity(&id).into()),

        Ident(x) => eval(db, db.all(&id::get(&x), &id::get("id")).into()),

        Seq(box a, box Nil) => eval(db, a),
        Seq(box Ident(e), box Ident(a)) => {
            let eid = id::get(&e);
            let aid = id::get(&a);
            db.all(&eid, &aid).into()
        }

        Many(box a, box b) => Many(eval(db, a).into(), eval(db, b).into()),

        Op(box Ident(a), op, box Ident(b)) if op == "=" => {
            let aid = id::get(&a);
            let bid = id::get(&b);
            let alias = id::get("alias");
            db.set(&aid, &alias, bid);
            db.set(&bid, &alias, aid);
            Nil
        }
        Op(box a, op, box b) if op == "=" => Op(a.into(), op, b.into()),

        Op(box Int(a), op, box Int(b)) if op == "+" => Int(a + b),
        _ => exp,
    };
}

fn prettify(depth: u32, db: &db::State, exp: &Expr) -> Expr {
    return match (depth, exp) {
        (_, Op(box Ident(x), o, box Ref(id))) if o == "=" && x == "id" => op(ident(x), o, Ref(*id)),
        (_, Ref(id)) => Expr::from(db.all(id, &id::get("alias")))
            .map(|s| eq(s.clone(), *id))
            .or(Ref(*id)),
        (0, Many(box a, box b)) => Many(
            prettify(depth + 1, db, a).into(),
            prettify(depth, db, b).into(),
        ),
        _ => exp.map(|e| prettify(depth + 1, db, e)),
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
                let debug = id::get("debug");
                let expr: Expr = line.parse().unwrap();
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

        rl.save_history(".session").unwrap();
    }
}
