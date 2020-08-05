use crate::lang::node;

// TODO(jeff): This file is WIP

impl<T> node::Collection<T> for Vec<T> {
    fn walk<F: FnMut(&mut T)>(&mut self, f: &mut F) {
        match self {
            Seq::Li(x, box nxt) => {
                f(x);
                nxt.walk(f)
            }
            Seq::End => {}
        }
    }
}

impl<T> std::iter::FromIterator<T> for Seq<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut seq = Seq::End;
        for item in iter {
            seq = Seq::Li(item, Box::new(seq));
        }

        seq
    }
}

impl<T> From<Vec<T>> for Seq<T> {
    fn from(vec: Vec<T>) -> Self {
        vec.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walk_test() {
        vec![]
    }
}
