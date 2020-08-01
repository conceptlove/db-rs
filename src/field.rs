use crate::data::{Edge, Expr, Id};
use crate::reg::get;

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
        get(&self.name)
    }
}

impl<E> From<Field> for Expr<E> {
    fn from(f: Field) -> Expr<E> {
        let id = get("id");
        let binding = get("binding");
        let name = get("name");
        let desc = get("desc");

        let fid = f.id();

        vec![
            (fid, id).set(fid),
            (fid, binding).set(f.name),
            (fid, name).set(f.name),
            (fid, desc).set(f.desc),
        ]
        .into()
    }
}
