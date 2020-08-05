// TODO(jeff): This file is WIP

/// My thinking here is to break apart the Expr enum into modular pieces.
/// Each piece, like Seq<T>, implements Node. Node allows for walking the tree.
/// I think we could have different traits for each node type.
pub trait Collection<T> {
    fn walk<F>(&mut self, f: &mut F)
    where
        F: FnMut(&mut T);
}
