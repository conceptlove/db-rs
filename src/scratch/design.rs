use futures::{future::Future, stream::Stream};

#[derive(Ord)]
pub struct Ent(pub uuid::Uuid);
#[derive(Ord)]
pub struct Atr(pub uuid::Uuid);
#[derive(Ord)]
pub struct Txn(pub uuid::Uuid);

pub struct Val<T> {
    pub Not(T),
    pub Seq(T, Box<Val<T>>),
}

trait Fact<V: Ord> {
    fn ent() -> &Ent;
    fn atr() -> &Atr;
    fn val() -> &V;
}

trait FactSet<V: Ord> {
    fn facts() -> Vec<Fact<V>>;
}

pub type Query<T> = Range<Fact<T>>;

trait Store<T: Ord> {
    type Results: Iterator<(Txn, Fact<T>)>;

    fn add<F: FactSet<T>>(txn: Txn, facts: F) -> Self;
    fn query<Q: Query<T>>(q: Q) -> Self::Results;
    fn redact(txn: Txn) -> FactSet<T>;
}

trait Storage<T: Ord> {
    type Results: Stream<Item = (Txn, Fact<T>)>;

    fn add<F: FactSet<T>>(txn: Txn, facts: F) -> Future<Output=Self>;
    fn query<Q: Query<T>>(query: Q) -> Self::Results;
    fn redact(txn: Txn) -> Self::Results;
}
