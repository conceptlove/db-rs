use crate::data::*;
use crate::ordset;
use crate::reg::get;

pub fn bootstrap() -> OrdSet<Fact> {
    let name = get("name");
    let id = get("id");
    let desc = get("desc");

    ordset![
        (name, name).set("name"),
        (name, id).set(name),
        (name, desc).set("A node's name."),
        (id, id).set(id),
        (id, name).set("id"),
        (id, desc).set("The UUID for a node.")
    ]
}
