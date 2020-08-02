use std::fmt;
use std::iter::FromIterator;
use uuid::Uuid;

pub type Id = uuid::Uuid;
pub type OrdSet<A> = std::collections::BTreeSet<A>;

#[macro_export]
macro_rules! ordset {
    ( $($x:expr), *) => {
        {
        let set = &mut crate::data::OrdSet::new();
        $(
            set.insert($x);
        )*
        (*set).clone()
    }
    };
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ExprError {
    InvalidCharacter(char),
    NotImplemented,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Expr {
    Nil,
    Debug(String),
    Ident(String),
    Many(Box<Expr>, Box<Expr>),
    Seq(Box<Expr>, Box<Expr>),
    Op(Box<Expr>, String, Box<Expr>),
    Failure(ExprError),
    Not(Box<Expr>),
    Value(V),
}

use Expr::*;

impl Expr {
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

    pub fn or<T: Into<Expr>>(self, v: T) -> Self {
        if self == Nil {
            v.into()
        } else {
            self
        }
    }

    pub fn map<F>(&self, f: F) -> Self
    where
        E: Clone,
        F: Fn(&Self) -> Self,
    {
        return match self {
            Op(box a, op, box b) => Op(f(a).into(), op.clone(), f(b).into()),
            Many(box a, box b) => Many(f(a).into(), f(b).into()),
            Seq(box a, box b) => Seq(f(a).into(), f(b).into()),
            _ => (*self).clone(),
        };
    }
}

pub type E = Id;
pub type A = Id;
#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum V {
    Start,
    Ref(Id),
    Bool(bool),
    Int(i32),
    Str(String),
    End,
}

impl From<V> for Expr {
    fn from(v: V) -> Expr {
        Value(v)
    }
}

impl From<&V> for Expr {
    fn from(v: &V) -> Expr {
        Value(v.clone())
    }
}

impl From<Id> for Expr {
    fn from(id: Id) -> Expr {
        Value(id.into())
    }
}

impl From<Fact> for Expr {
    fn from(f: Fact) -> Expr {
        eq(f.entity(), f.value())
    }
}

impl<T> From<Vec<T>> for Expr
where
    T: Clone + Into<Expr>,
{
    fn from(exps: Vec<T>) -> Expr {
        match exps.as_slice() {
            [] => Nil,
            [x] => (*x).clone().into(),
            _ => exps.into_iter().collect(),
        }
    }
}
impl<'a, T: Into<Expr>> FromIterator<T> for Expr {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut i = iter.into_iter();

        let mut exp = match i.next() {
            Some(v) => v.into(),
            None => Nil,
        };

        for v in i {
            exp = Many(exp.into(), v.into().into());
        }
        exp
    }
}

impl From<&str> for V {
    fn from(s: &str) -> Self {
        V::Str(s.to_owned())
    }
}

impl From<uuid::Uuid> for V {
    fn from(id: uuid::Uuid) -> Self {
        V::Ref(id)
    }
}

impl From<i32> for V {
    fn from(x: i32) -> Self {
        V::Int(x)
    }
}

impl From<bool> for V {
    fn from(x: bool) -> Self {
        V::Bool(x)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct Fact(pub E, pub A, pub V);

impl Fact {
    pub fn entity(&self) -> E {
        self.0
    }

    pub fn attr(&self) -> A {
        self.1
    }

    pub fn value(&self) -> &V {
        &self.2
    }

    pub fn eav(self) -> (E, A, V) {
        let Fact(e, a, v) = self;
        (e, a, v)
    }

    pub fn aev(self) -> (A, E, V) {
        let Fact(e, a, v) = self;
        (a, e, v)
    }

    pub fn ave(self) -> (A, V, E) {
        let Fact(e, a, v) = self;
        (a, v, e)
    }
}

pub fn fact<T: Into<V>>(e: E, a: A, v: T) -> Fact {
    Fact(e, a, v.into())
}

pub fn ident(name: &str) -> Expr {
    Ident(name.to_string())
}

pub fn two(a: Expr, b: Expr) -> Expr {
    Seq(Box::new(a), Box::new(b))
}

pub fn op<T, V>(a: T, op: &str, b: V) -> Expr
where
    T: Into<Expr>,
    V: Into<Expr>,
{
    Op(Box::new(a.into()), op.to_string(), Box::new(b.into()))
}

pub fn eq<T, V>(a: T, b: V) -> Expr
where
    T: Into<Expr>,
    V: Into<Expr>,
{
    op(a, "=", b)
}

impl From<(Expr, Expr)> for Expr {
    fn from((a, b): (Expr, Expr)) -> Expr {
        two(a, b)
    }
}

impl From<&str> for Expr {
    fn from(s: &str) -> Expr {
        Ident(s.to_string())
    }
}

impl fmt::Display for V {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use V::*;

        match self {
            Start | End => write!(f, ""),
            Ref(id) => write!(f, "{}", id),
            Bool(x) => write!(f, "{}", x),
            Int(n) => write!(f, "{}", n),
            Str(s) => write!(f, "{:?}", s),
        }
    }
}

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
            Nil => write!(f, "()"),
            Debug(x) => write!(f, "/{}", x),
            Ident(x) => write!(f, "{}", x),
            Many(a, b) => write!(f, "{}, {}", a, b),
            Seq(a, b) => write!(f, "{} {}", a, b),
            Op(a, op, b) => write!(f, "{} {} {}", a, op, b),
            Not(x) => write!(f, "! {}", x),
            Failure(x) => write!(f, "(Failure: {})", x),
            Value(v) => write!(f, "{}", v),
        }
    }
}

pub fn id(st: &String) -> Id {
    uuid::Uuid::parse_str(st).unwrap()
}

pub trait Edge {
    fn set<T: Into<V>>(&self, value: T) -> Fact;
}

impl Edge for (String, String) {
    fn set<T: Into<V>>(&self, v: T) -> Fact {
        Fact(id(&self.0), id(&self.1), v.into())
    }
}

impl Edge for (Uuid, Uuid) {
    fn set<T: Into<V>>(&self, v: T) -> Fact {
        Fact(self.0, self.1, v.into())
    }
}
