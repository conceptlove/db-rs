use crate::Fact::Fact;

// NOTE(jeff): This file is WIP.

// This will likely represent the memory model for storing Facts.

pub trait Block {
    // TODO(jeff): not real
    static SIZE;
    fn raw() -> [u8; SIZE]
}

struct Blob {}

impl Block for Blob {
    static SIZE = 65_535;
    fn raw() {

    }
}

struct Index();

impl Block for Index {

}

pub Enum Block {
    Blob([u8; 4096]),
    Ids([u8])
}

pub struct Page {
    parent: Id,
    blocks: [Block; 32],
}
