use crate::data::{A, E, V};
use std::collections::BTreeSet;

pub struct Db {
    eav: BTreeSet<(E, A, V)>,
}

pub trait Database {
    fn get(e: E, a: A) -> V;
    fn set(e: E, a: A, v: V);
}
