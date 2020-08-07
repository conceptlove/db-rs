#![feature(or_patterns)]
#![feature(box_patterns)]

pub mod bootstrap;
pub mod color;
pub mod db;
pub mod eval;
pub mod field;
pub mod id;
pub mod lang;
pub mod machine;
pub mod store;
pub mod cmd {
    pub mod id;
    pub mod repl;
    pub mod ui;
}
