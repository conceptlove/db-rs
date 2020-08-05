use crate::lang::*;
use crate::machine::*;
use Expr::*;

impl Reducer<char> for Expr {
    fn update(&self, ch: char) -> Self {
        match (self.clone(), ch) {
            (Nil, _) => ch.into(),
            (Int(n), '0'..='9') => Int(n.update((ch as u8) - 48)),
            (Ident(x), 'a'..='z' | 'A'..='Z' | '_') => Ident(x + &ch.to_string()),
            (Debug(x), 'a'..='z' | 'A'..='Z' | '_') => Debug(x + &ch.to_string()),

            (exp, ',') => Many(exp.into(), Nil.into()),
            (Many(a, box b), _) => Many(a, b.update(ch).into()),

            (Seq(a, b), '\n' | '\r') if a.is_seq() => Seq(a, b),
            (cur, '\n' | '\r') => ((cur, Nil), Nil).into(),

            (exp @ (Ident(_) | Debug(_)), ' ' | '\t') => two(exp, Nil),

            (Seq(box a, box Nil), sym @ ('=' | '+' | '*')) => op(a, &sym.to_string(), Nil),
            (Seq(a, exp), _) => two(*a, exp.update(ch)),

            (a, sym @ ('=' | '+' | '*')) => op(a, &sym.to_string(), Nil),
            (Op(box a, sym, box b), _) => op(a, &sym, b.update(ch)),

            (exp, ' ' | '\t') => exp,
            (Failure(x), _) => Failure(x),
            _ => Failure(ExprError::InvalidCharacter(ch)),
        }
    }
}

impl From<char> for Expr {
    fn from(ch: char) -> Expr {
        match ch {
            'a'..='z' | 'A'..='Z' | '_' => Ident(ch.to_string()),
            '0'..='9' => Int((ch as i32) - 48),
            '=' => op(Nil, &ch.to_string(), Nil),
            '/' => Debug("".to_string()),
            ',' | '.' | '\n' | '\r' | ' ' | '\t' => Nil,

            _ => Nil,
        }
    }
}

impl std::str::FromStr for Expr {
    type Err = ExprError;

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
