use super::expr::*;
use crate::c;
use std::fmt;
use Expr::*;

impl fmt::Display for ExprError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ExprError::*;

        match self {
            InvalidCharacter(ch) => write!(f, "Invalid character: {:?}", ch),
            NotImplemented => write!(f, "Not yet implemented"),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Nil => write!(f, c!(white, "()")),
            Ref(x) => write!(f, c!(yellow, "{}"), x),
            Int(x) => write!(f, c!(blue, "{}"), x),
            Str(x) => write!(f, c!(green, "{:?}"), x),
            Bool(x) => write!(f, c!(blue, "{}"), x),
            Debug(x) => write!(f, "/{}", x),
            Ident(x) => write!(f, "{}", x),
            Many(a, b) => write!(f, "{},\n{}", a, b),
            Seq(a, b) => write!(f, "{} {}", a, b),
            Op(a, op, b) => write!(f, "{} {} {}", a, op, b),
            Not(x) => write!(f, "! {}", x),
            Failure(x) => write!(f, c!(red, "(Failure: {})"), x),
        }
    }
}
