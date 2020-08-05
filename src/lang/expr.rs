use crate::id::Id;
use crate::store::{Fact, A, E, V};
use std::iter::FromIterator;
use Expr::*;

pub mod seq;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Expr {
    Nil,
    Ref(Id),
    Bool(bool),
    Int(i32),
    Str(String),
    Debug(String),
    Ident(String),
    Many(Box<Expr>, Box<Expr>),
    Seq(Box<Expr>, Box<Expr>),
    Op(Box<Expr>, String, Box<Expr>),
    Failure(ExprError),
    Not(Box<Expr>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ExprError {
    InvalidCharacter(char),
    NotImplemented,
}

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

    // pub fn walk<F>(&mut self, f: F)
    // where
    //     F: FnMut(&mut Self),
    // {
    //     match self {
    //         Op(box a, op, box b) =>
    //     }
    // }

    pub fn map<F>(&self, f: F) -> Self
    where
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

pub fn eq<T, U>(a: T, b: U) -> Expr
where
    T: Into<Expr>,
    U: Into<Expr>,
{
    op(a, "=", b)
}

impl<T, U> From<(T, U)> for Expr
where
    T: Into<Expr>,
    U: Into<Expr>,
{
    fn from((a, b): (T, U)) -> Self {
        two(a.into(), b.into())
    }
}

impl From<&str> for Expr {
    fn from(s: &str) -> Expr {
        Ident(s.to_string())
    }
}

impl From<V> for Expr {
    fn from(v: V) -> Expr {
        match v {
            V::Start | V::End => Nil,
            V::Ident(x) => Ident(x),
            V::Bool(x) => Bool(x),
            V::Int(x) => Int(x),
            V::Str(x) => Str(x),
            V::Ref(x) => Ref(x),
        }
    }
}

impl From<&V> for Expr {
    fn from(v: &V) -> Expr {
        (*v).clone().into()
    }
}

impl From<Id> for Expr {
    fn from(id: Id) -> Expr {
        Ref(id.into())
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

impl From<&(E, A, V)> for Expr {
    fn from((e, a, v): &(E, A, V)) -> Self {
        eq((*e, *a), v)
    }
}
