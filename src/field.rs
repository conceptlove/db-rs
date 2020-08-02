use crate::lang::{id, Id};
use crate::store::{fact, ident, Fact};

#[macro_export]
macro_rules! field {
    ( $($name:ident => $value:expr), *) => {
        {
            Field {
                $( $name: $value ), *
            }
        }
    };
}

pub struct Field {
    pub name: &'static str,
    pub desc: &'static str,
}

impl Field {
    fn id(&self) -> Id {
        id::get(&self.name)
    }
}

impl From<Field> for Vec<Fact> {
    fn from(f: Field) -> Vec<Fact> {
        let id = id::get("id");
        let alias = id::get("alias");
        let name = id::get("name");
        let desc = id::get("desc");

        let fid = f.id();

        vec![
            fact(fid, id, fid),
            fact(fid, alias, ident(f.name)),
            fact(fid, name, f.name),
            fact(fid, desc, f.desc),
        ]
    }
}
