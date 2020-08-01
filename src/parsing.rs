use crate::data::*;
use crate::machine::*;
use Expr::*;

// Experimenting. Broken.

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ParseError {
    InvalidCharacter(char),
    NotImplemented,
}

pub type Ast = Expr<ParseError>;

impl Reducer<char> for Ast {
    fn update(&self, ch: char) -> Self {
        match (self.clone(), ch) {
            (Nil, '0'..='9') => Value(V::Int((ch as i32) - 48)),
            (Value(V::Int(n)), '0'..='9') => Value(V::Int(n.update((ch as u8) - 48))),
            (Nil, 'a'..='z' | 'A'..='Z' | '_') => ch.into(),
            (Ident(x), 'a'..='z' | 'A'..='Z' | '_') => Ident(x + &ch.to_string()),
            (Nil, ',' | '.' | '\n' | '\r' | ' ' | '\t') => Nil,
            (exp, ',') => Many(exp.into(), Nil.into()),

            (Seq(a, b), '\n' | '\r') if a.is_seq() => Seq(a, b),
            (cur, '\n' | '\r') => ((cur, Nil).into(), Nil).into(),

            (exp @ Ident(_), ' ' | '\t') => two(exp, Nil),
            (Seq(a, exp), _) => two(*a, exp.update(ch)),
            (Op(a, op, b), _) => Op(a, op, b.update(ch).into()),

            (Failure(x), _) => Failure(x),
            _ => Failure(ParseError::InvalidCharacter(ch)),
        }
    }
}

impl From<char> for Ast {
    fn from(ch: char) -> Ast {
        match ch {
            'a'..='z' | 'A'..='Z' | '_' => Ident(ch.to_string()),
            '=' => Expr::Op(Nil.into(), ch.to_string(), Nil.into()),
            _ => Nil,
        }
    }
}

impl std::str::FromStr for Expr<ParseError> {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut exp = Expr::Nil;
        for ch in s.chars() {
            exp = exp.update(ch);
        }

        Ok(exp)
    }
}

// fn expr(input: &str) -> IResult<&str, Expr> {
//     let mut cur = Expr::Nil;

//     let token = alpha1;
//     separated_nonempty_list(space1, token)(input)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expr_test() {
        assert_eq!("   ".parse(), Ok(Nil));
        assert_eq!("a".parse(), Ok(ident("a")));
        assert_eq!("a b".parse(), Ok(two(ident("a"), ident("b"))));
        assert_eq!(
            "a = b".parse(),
            Ok(eq(Seq(ident("a").into(), Nil.into()), ident("b")))
        );
    }
}
