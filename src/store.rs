pub use crate::id::{id, Id};
use std::fmt;

pub type E = Id;
pub type A = Id;

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum V {
    Start,
    Ident(String),
    Ref(Id),
    Bool(bool),
    Int(i32),
    Str(String),
    End,
}

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct Fact(pub E, pub A, pub V);

pub struct Query();

trait Store {
    fn new() -> Self;
    fn adding(&mut self, item: Fact);
    fn removing(&mut self, item: Fact);
    fn querying<T>(&self, q: Query) -> T
    where
        T: Iterator<Item = Fact>;

    fn add<T: Into<Fact>>(&mut self, item: T) -> &mut Self {
        self.adding(item.into());
        self
    }

    fn remove<T: Into<Fact>>(&mut self, item: T) -> &mut Self {
        self.removing(item.into());
        self
    }

    fn query<Q, T>(&self, q: Q) -> T
    where
        Q: Into<Query>,
        T: Iterator<Item = Fact>,
    {
        self.querying(q.into())
    }
}

pub fn fact<T: Into<V>>(e: E, a: A, v: T) -> Fact {
    Fact(e, a, v.into())
}

pub fn ident(s: &str) -> V {
    V::Ident(s.to_string())
}

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

impl fmt::Display for V {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use V::*;

        match self {
            Start | End => write!(f, ""),
            Ident(x) => write!(f, "{}", x),
            Ref(x) => write!(f, "{}", x),
            Bool(x) => write!(f, "{}", x),
            Int(x) => write!(f, "{}", x),
            Str(s) => write!(f, "{:?}", s),
        }
    }
}

pub trait Edge {
    fn set<T: Into<V>>(&self, value: T) -> Fact;
}

impl Edge for (String, String) {
    fn set<T: Into<V>>(&self, v: T) -> Fact {
        Fact(id(&self.0), id(&self.1), v.into())
    }
}

impl Edge for (Id, Id) {
    fn set<T: Into<V>>(&self, v: T) -> Fact {
        Fact(self.0, self.1, v.into())
    }
}
