use Uuid::Uuid;

// TODO(jeff): This file is WIP


// // I'm trying to figure out if we can add a type to Ids... we'd then assume that they will have
// // the properties we expect. Reading their files will have expected properties. Essentially,
// // it'd have to be a transform on the Fact. It substitutes some ids for others.
// //
// pub struct Id<T: Uuid>(T, Uuid);
pub type Id = Uuid;
pub type Var = Id;
pub type Actor = Id;
pub type Page = Id;
pub type Txn = Id;
pub type Ent = Id;
pub type Atr = Id;

pub struct Val {
    Id(Id),
    Int(i64),
    Float(f64),
    Block(Id),
};


// Fact(current_actor, current_page, random_id(), current_page, ID, current_page)

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct EAV(Ent, Atr, Val, Txn);

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct AEV(Atr, Ent, Val, Txn);

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct AVE(Atr, Val, Ent, Txn);

trait Fact {
    fn eav(&self) -> EAV;
    fn aev(&self) -> AEV;
    fn ave(&self) -> AVE;
}

trait Source: Stream<Item = Fact>


/**
 * Switching pages is as easy as pushing a new page id onto the stack. In other words: change the
 * focus to contain the new page id. The system takes care of writing that to a new file.
 * Those systems must be queries. So this means that we have a writer per actor.
 *
 * So, a writer is a system that watches the fact stream, and pattern is compared against every single fact.
 *
 * Facts can cause the current state to change.
 */
