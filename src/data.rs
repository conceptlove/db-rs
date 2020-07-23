use im::OrdSet;

pub enum Id {
    Id(uuid::Uuid),
    ContentId(u64),
}

/// The idea that a value could either exist or reference a variable.
pub enum Value<T> {
    One(T),
    Many(OrdSet<T>),
    Var(String),
}

#[derive(Hash)]
pub enum Expr {
    Token(String),
    Many(Vec<Box<Expr>>),
    Op(String, Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
}

pub type E = uuid::Uuid;
pub type A = String;

#[derive(Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum V {
    Int(u32),
    Str(String),
}

#[derive(Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Fact(E, A, V);

impl Fact {
    fn entity_id(Fact(e, _, _): Self) -> E {
        e
    }
}

pub fn token(tok: String) -> Expr {
    Expr::Token(tok)
}
