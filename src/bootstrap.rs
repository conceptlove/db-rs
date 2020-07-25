use crate::data::*;
use crate::reg::NAME;

pub fn bootstrap() -> OrdSet<Fact> {
    ordset![(NAME, NAME).set(V::Str("name".to_owned()))]
}
