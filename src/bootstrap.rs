use crate::field::*;
use crate::store::Fact;

pub fn bootstrap() -> Vec<Fact> {
    vec![
        Field {
            name: "id",
            desc: "An entity's id.",
        },
        Field {
            name: "name",
            desc: "An entity's name.",
        },
        Field {
            name: "desc",
            desc: "A short description of an entity.",
        },
    ]
    .iter()
    .cloned()
    .flatten()
    .collect()
}
