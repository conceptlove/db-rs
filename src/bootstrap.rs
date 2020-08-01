use crate::data::*;
// use crate::field::*;
use crate::{ordset, reg};
use reg::get;

// const NAME: Field = Field {
//     name: "name",
//     desc: "An entity's name.",
// };

pub fn bootstrap() -> OrdSet<Fact> {
    let name = get("name");
    let id = get("id");
    let desc = get("desc");
    let alias = get("alias");

    ordset![
        (name, alias).set("name"),
        (name, name).set("name"),
        (name, id).set(name),
        (name, desc).set("A node's name."),
        (id, id).set(id),
        (id, alias).set("id"),
        (id, name).set("id"),
        (id, desc).set("The UUID for a node.")
    ]
}
