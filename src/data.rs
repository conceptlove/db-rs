enum Expr {
    Token(str),
    Many(Vec<Expr>),
    Op(str, Expr, Expr),
    Not(Expr),
    Map(HashMap<String, Expr>),
}
