pub struct Pair<A, B> {
    left: A,
    right: B,
}

impl<A: Clone, B: Clone> Pair<A, B> {
    pub fn left(&self) -> A {
        self.left.clone()
    }

    pub fn right(&self) -> B {
        self.right.clone()
    }

    pub fn new(left: A, right: B) -> Self {
        Self { left, right }
    }
}

pub trait PairMorfism<A, B, C, D> {
    fn apply(a: Pair<A, B>) -> Pair<C, D>;
}

#[cfg(test)]
mod ordered_pair_tests {
    pub struct Revert;

    impl<A: Clone, B: Clone> PairMorfism<A, B, B, A> for Revert {
        fn apply(a: Pair<A, B>) -> Pair<B, A> {
            let r = a.right;
            let l = a.left;

            Pair::new(r, l)
        }
    }

    use crate::prod::{Pair, PairMorfism};

    #[test]
    fn create_pair() {
        let pair = Pair::new(2, 3);

        assert_eq!(pair.left, 2);
        assert_eq!(pair.right, 3);
    }

    #[test]
    fn revert_morf() {
        let pair = Pair::new("50", 10);

        let reverted = Revert::apply(pair);

        assert_eq!(reverted.left, 10);
        assert_eq!(reverted.right, "50");
    }
}
