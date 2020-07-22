/// The idea that a value could either exist or reference a variable.
pub enum Value<T> {
    Just(T),
    Var(String),
}

pub enum Expr {
    Token(String),
    Many(Vec<Box<Expr>>),
    Op(String, Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
}

pub type E = uuid::Uuid;
pub type A = String;
pub type V = Expr;

pub fn token(tok: String) {
    Expr::Token(tok);
}
