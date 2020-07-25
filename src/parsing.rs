use crate::data::*;
use Expr::*;

// Experimenting. Broken.

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ParseError {
    InvalidCharacter(char),
    NotImplemented,
}

pub type Ast = Expr<ParseError>;

fn expr(cur: Expr<ParseError>, ch: char) -> Expr<ParseError> {
    match ch {
        ',' => match cur {
            Nil => Nil,
            _ => Many(cur.into(), Nil.into()),
        },

        '\n' | '\r' => match cur {
            Nil => Nil,
            Seq(a, b) if a.is_seq() => Seq(a, b),
            _ => ((cur, Nil).into(), Nil).into(),
        },

        ' ' | '\t' => match cur {
            Nil => Nil,
            Ident(_) => (cur, Nil).into(),
            Seq(a, exp) => (*a, expr(*exp, ch)).into(),
            _ => Failure(ParseError::NotImplemented),
        },

        'a'..='z' | 'A'..='Z' | '_' => match cur {
            Nil => Expr::Ident(ch.to_string()),
            Ident(x) => Ident(x + &ch.to_string()),
            Seq(a, exp) => (*a, expr(*exp, ch)).into(),
            _ => Failure(ParseError::NotImplemented),
        },

        _ => Failure(ParseError::InvalidCharacter(ch)),
    }
}

impl std::str::FromStr for Expr<ParseError> {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut exp = Expr::Nil;
        for ch in s.chars() {
            exp = expr(exp, ch);
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
    }
}
