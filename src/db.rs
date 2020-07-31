use crate::data::{Fact, OrdSet, A, E, V};

#[derive(Debug)]
pub struct State {
    pub eav: OrdSet<(E, A, V)>,
    pub aev: OrdSet<(A, E, V)>,
    pub ave: OrdSet<(A, V, E)>,
}

impl State {
    pub fn new() -> Self {
        State {
            eav: OrdSet::new(),
            aev: OrdSet::new(),
            ave: OrdSet::new(),
        }
    }

    pub fn add(&mut self, fact: Fact) {
        self.eav.insert(fact.clone().eav());
        self.aev.insert(fact.clone().aev());
        self.ave.insert(fact.ave());
    }

    pub fn add_all(&mut self, facts: OrdSet<Fact>) {
        for fact in facts {
            self.add(fact);
        }
    }

    pub fn find(&self, e: &E, a: &A) -> impl Iterator<Item = &(E, A, V)> {
        let start = (*e, *a, V::Start);
        let end = (*e, *a, V::End);

        self.eav.range(start..end)
    }

    pub fn get(&self, e: &E, a: &A) -> Vec<&V> {
        self.find(e, a).map(|(_, _, v)| v).collect()
    }

    pub fn set<T: Into<V>>(&mut self, e: E, a: A, v: T) {
        self.add(Fact(e, a, v.into()));
    }

    pub fn update(&self, _e: E) {}

    pub fn bootstrap(&mut self) -> &mut Self {
        self.add_all(crate::bootstrap::bootstrap());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reg::get;

    #[test]
    fn add_and_get_test() {
        let db = &mut State::new();
        db.set(get("a"), get("b"), 1);

        assert_eq!(db.get(&get("a"), &get("b")), vec![&1.into()]);
    }

    #[test]
    fn bootstrap_test() {
        let db = &mut State::new();
        db.bootstrap();
        println!("{:?}", db);
        assert_eq!(db.get(&get("name"), &get("name")), vec![&"name".into()])
    }
}
