pub trait Monoid: Clone {
    fn empty() -> Self;
    fn append(&self, other: &Self) -> Self;
}
