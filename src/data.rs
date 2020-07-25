pub use im::ordset;
use uuid::Uuid;

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Copy, Clone)]
pub enum Id {
    Id(uuid::Uuid),
    ContentId(u64),
}

pub type OrdSet<A> = im::OrdSet<A>;

/// The idea that a value could either exist or reference a variable.
pub enum Value<T> {
    One(T),
    Many(OrdSet<T>),
    Var(String),
}

#[derive(Hash)]
pub enum Expr {
    Token(String),
    Many(Vec<Box<Expr>>),
    Op(String, Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
}

pub type E = Id;
pub type A = Id;
#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum V {
    Start,
    Id(Id),
    Int(u32),
    Str(String),
    End,
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

pub fn id(st: &String) -> Id {
    let uuid = uuid::Uuid::parse_str(st).unwrap();
    Id::Id(uuid)
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

// fn fact(value: (String, String, V)) -> Fact {
//     uui
// }

pub fn token(tok: String) -> Expr {
    Expr::Token(tok)
}
