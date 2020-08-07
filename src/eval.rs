use crate::id;
use crate::machine::*;
use std::collections::BTreeSet;

/// We'd expect, say, a Database to implement Eval<Ast>. This is defining how to evaluate Facts
/// in the context of some Database.
trait Eval<T>: Reducer<T> {
    fn eval(&self) -> Iterator<T>;
}

pub type OrdSet<T> = BTreeSet<T>;
pub type FactSet = OrdSet<Fact>;

impl<F: Into<Fact>, T: Into<OrdSet<F>>> From<T> for FactSet {}

fn eval<F: Into<FactSet>>(facts: F) -> FactSet {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_eval() {
        assert_eq!(eval([fact(id::get("ent"), id::get("atr"),)]), [fact()])
    }
}
