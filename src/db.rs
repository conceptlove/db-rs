use crate::id;
use crate::store::{fact, Fact, A, E, V};

pub type OrdSet<A> = std::collections::BTreeSet<A>;

#[derive(Debug)]
pub struct State {
    pub eav: OrdSet<(E, A, V)>,
    pub aev: OrdSet<(A, E, V)>,
    pub ave: OrdSet<(A, V, E)>,
}

impl State {
    // TODO(jeff): Move the convenience methods to a trait.
    // TODO(jeff): Add type Query(Range<E>, Range<A>, Range<V>)

    pub fn new() -> Self {
        State {
            eav: OrdSet::new(),
            aev: OrdSet::new(),
            ave: OrdSet::new(),
        }
    }

    pub fn add<T: Into<Fact>>(&mut self, item: T) {
        let fact = item.into();
        self.eav.insert(fact.clone().eav());
        self.aev.insert(fact.clone().aev());
        self.ave.insert(fact.ave());
    }

    pub fn add_all<T: IntoIterator<Item = Fact>>(&mut self, facts: T) {
        for fact in facts {
            self.add(fact);
        }
    }

    pub fn remove<T: Into<Fact>>(&mut self, item: T) {
        let fact = item.into();

        self.eav.remove(&fact.clone().eav());
        self.aev.remove(&fact.clone().aev());
        self.ave.remove(&fact.ave());
    }

    pub fn find(&self, e: &E, a: &A) -> impl Iterator<Item = &(E, A, V)> {
        let start = (*e, *a, V::Start);
        let end = (*e, *a, V::End);

        self.eav.range(start..end)
    }

    pub fn all(&self, e: &E, a: &A) -> Vec<&V> {
        self.find(e, a).map(|(_, _, v)| v).collect()
    }

    pub fn for_entity(&self, e: &E) -> Vec<Fact> {
        let start = (*e, id::FIRST, V::Start);
        let end = (*e, id::LAST, V::End);

        self.eav
            .range(start..end)
            .cloned()
            .map(|(e, a, v)| Fact(e, a, v))
            .collect()
    }

    pub fn set<T: Into<V>>(&mut self, e: &E, a: &A, v: T) {
        self.add(Fact(*e, *a, v.into()));
    }

    pub fn has<T: Into<Fact>>(&self, f: T) -> bool {
        self.eav.contains(&f.into().eav())
    }

    pub fn toggle(&mut self, e: &E, a: &A) {
        if self.has(fact(*e, *a, true)) {
            self.remove(fact(*e, *a, true));
            self.add(fact(*e, *a, false));
        } else {
            self.remove(fact(*e, *a, false));
            self.add(fact(*e, *a, true));
        }
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

    #[test]
    fn add_and_get_test() {
        let db = &mut State::new();
        db.set(&id::get("a"), &id::get("b"), 1);

        assert_eq!(db.all(&id::get("a"), &id::get("b")), vec![&1.into()]);
    }

    #[test]
    fn bootstrap_test() {
        let db = &mut State::new();
        db.bootstrap();
        println!("{:?}", db);
        assert_eq!(
            db.all(&id::get("name"), &id::get("name")),
            vec![&"name".into()]
        )
    }
}
