#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Loc(pub usize, pub usize);
impl Loc {
    #[allow(dead_code)]
    pub fn merge(&self, other: &Loc) {
        use std::cmp::{max, min};
        Loc(min(self.0, other.0), max(self.1, other.1));
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Annot<T> {
    pub value: T,
    pub loc: Loc,
}
impl<T> Annot<T> {
    pub fn new(value: T, loc: Loc) -> Self {
        Self { value, loc }
    }
}
