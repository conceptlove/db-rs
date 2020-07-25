use crate::data::{Fact, A, E, V};
use im::OrdSet;

pub struct State {
    pub eav: OrdSet<Fact>,
}

impl State {
    pub fn new() -> Self {
        State { eav: OrdSet::new() }
    }

    pub fn add(&mut self, fact: Fact) {
        self.eav.insert(fact);
    }

    pub fn add_all(&mut self, facts: OrdSet<Fact>) {
        for fact in facts {
            self.add(fact);
        }
    }

    pub fn get(&self, e: E, a: A) -> Vec<V> {
        self.eav
            .range(Fact(e.clone(), a.clone(), V::Start)..Fact(e, a, V::End))
            .map(|f| f.value())
            .collect()
    }

    pub fn set(&mut self, e: E, a: A, v: V) {
        self.add(Fact(e, a, v));
    }
    pub fn update(&self, _e: E) {}

    pub fn bootstrap(&mut self) -> &mut Self {
        self.add_all(crate::bootstrap::bootstrap());
        self
    }
}
