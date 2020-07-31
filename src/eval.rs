use crate::machine::*;
use crate::parsing::Ast;

/// We'd expect, say, a Database to implement Eval<Ast>. This is defining how to evaluate Ast
/// in the context of some Database.
trait Eval<T>: Reducer<T> {
    fn eval(&self) -> Iterator<T>;
}
