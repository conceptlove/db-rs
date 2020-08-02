use crate::field::*;
use crate::store::Fact;

const ID: Field = Field {
    name: "id",
    desc: "An entity's id.",
};

const NAME: Field = Field {
    name: "name",
    desc: "An entity's name.",
};

const DESC: Field = Field {
    name: "desc",
    desc: "A short description of an entity.",
};

pub fn bootstrap() -> Vec<Fact> {
    let mut facts = vec![];

    facts.append(&mut ID.into());
    facts.append(&mut NAME.into());
    facts.append(&mut DESC.into());

    facts

    // vec![
    //     fact(name, alias, ident("name")),
    //     fact(name, name, "name"),
    //     fact(name, id, name),
    //     fact(name, desc, "A node's name."),
    //     fact(id, id, id),
    //     fact(id, alias, ident("id")),
    //     fact(id, name, "id"),
    //     fact(id, desc, "The UUID for a node."),
    //     fact(desc, id, desc),
    //     fact(desc, alias, ident("desc")),
    //     fact(desc, name, "desc"),
    //     fact(desc, desc, "A short description of a node."),
    // ]
}
