use crate::data::{Fact, A, E, V};
use im::OrdSet;

pub struct State {
    eav: OrdSet<Fact>,
}

impl State {
    fn add(self, fact: Fact) -> Self {
        self
    }
    fn get(&self, e: E, a: A) {}
    fn set(&self, e: E, a: A, v: V) {}
    fn update(&self, e: E) {}
    fn bootstrap() {}
}
