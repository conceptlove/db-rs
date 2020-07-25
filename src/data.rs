pub use im::ordset;
use std::fmt;
use std::iter::FromIterator;
use uuid::Uuid;

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Copy, Clone)]
pub enum Id {
    Id(uuid::Uuid),
    Hash(u64),
}

pub type OrdSet<A> = im::OrdSet<A>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Expr<E> {
    Nil,
    Ident(String),
    Many(Box<Expr<E>>, Box<Expr<E>>),
    Seq(Box<Expr<E>>, Box<Expr<E>>),
    Op(Box<Expr<E>>, String, Box<Expr<E>>),
    Failure(E),
    Not(Box<Expr<E>>),
    Value(V),
}

use Expr::*;

impl<E> Expr<E> {
    // pub fn map<F>(&self, f: F) -> Self
    // where
    //     F: Fn(&Self) -> Self,
    // {
    //     match self {
    //         Nil => Nil,
    //         Many(a, b) => Many(a.map(f).into(), a.map(f).into()),
    //         _ => f(self),
    //     }
    // }

    pub fn is_seq(&self) -> bool {
        match self {
            Seq(_, _) => true,
            _ => false,
        }
    }

    pub fn is_ident(&self) -> bool {
        match self {
            Ident(_) => true,
            _ => false,
        }
    }
}

pub type E = Id;
pub type A = Id;
#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum V {
    Start,
    Ref(Id),
    Int(u32),
    Str(String),
    End,
}

impl<E> From<V> for Expr<E> {
    fn from(v: V) -> Expr<E> {
        Value(v)
    }
}

impl<E> FromIterator<V> for Expr<E> {
    fn from_iter<I: IntoIterator<Item = V>>(iter: I) -> Self {
        let mut exp = Nil;
        for v in iter {
            exp = Many(exp.into(), Value(v).into());
        }
        exp
    }
}

#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct Fact(pub E, pub A, pub V);

impl Fact {
    pub fn entity(&self) -> E {
        self.0
    }

    pub fn attr(&self) -> A {
        self.1
    }

    pub fn value(&self) -> V {
        self.2.clone()
    }
}

pub fn ident<E>(name: &str) -> Expr<E> {
    Ident(name.to_string())
}

pub fn two<E>(a: Expr<E>, b: Expr<E>) -> Expr<E> {
    Seq(Box::new(a), Box::new(b))
}

impl<E> From<(Expr<E>, Expr<E>)> for Expr<E> {
    fn from((a, b): (Expr<E>, Expr<E>)) -> Expr<E> {
        two(a, b)
    }
}

impl<E> From<&str> for Expr<E> {
    fn from(s: &str) -> Expr<E> {
        Ident(s.to_string())
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Id::*;

        match self {
            Id(uuid) => write!(f, "{}", uuid),
            Hash(n) => write!(f, "@{:x}", n),
        }
    }
}

impl fmt::Display for V {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use V::*;

        match self {
            Start | End => write!(f, ""),
            Ref(id) => write!(f, "{}", id),
            Int(n) => write!(f, "{}", n),
            Str(s) => write!(f, "{:?}", s),
        }
    }
}

impl fmt::Display for crate::parsing::ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::parsing::ParseError::*;

        match self {
            InvalidCharacter(ch) => write!(f, "Invalid character: {:?}", ch),
            NotImplemented => write!(f, "Not yet implemented"),
        }
    }
}

impl fmt::Display for crate::parsing::Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Nil => write!(f, ""),
            Ident(x) => write!(f, "{}", x),
            Many(a, b) => write!(f, "{} {}", a, b),
            Seq(a, b) => write!(f, "{}, {}", a, b),
            Op(a, op, b) => write!(f, "{} {} {}", a, op, b),
            Not(x) => write!(f, "! {}", x),
            Failure(x) => write!(f, "(Failure: {})", x),
            Value(v) => write!(f, "{}", v),
        }
    }
}

pub fn id(st: &String) -> Id {
    let uuid = uuid::Uuid::parse_str(st).unwrap();
    Id::Id(uuid)
}

impl std::str::FromStr for Id {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Id, Self::Err> {
        uuid::Uuid::parse_str(s).map(|x| Id::Id(x))
    }
}

pub trait Edge {
    fn set(&self, v: V) -> Fact;
}

impl Edge for (String, String) {
    fn set(&self, v: V) -> Fact {
        Fact(id(&self.0), id(&self.1), v)
    }
}

impl Edge for (Uuid, Uuid) {
    fn set(&self, v: V) -> Fact {
        Fact(Id::Id(self.0), Id::Id(self.1), v)
    }
}
