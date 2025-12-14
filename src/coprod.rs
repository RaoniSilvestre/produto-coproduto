use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Sum<A: Debug + PartialEq, B: Debug + PartialEq> {
    Left(A),
    Right(B),
}

impl<A: Debug + PartialEq, B: Debug + PartialEq> Sum<A, B> {
    pub fn left(a: A) -> Self {
        Self::Left(a)
    }

    pub fn right(b: B) -> Self {
        Self::Right(b)
    }
}

pub trait SumMorfism<
    A: Debug + PartialEq,
    B: Debug + PartialEq,
    C: Debug + PartialEq,
    D: Debug + PartialEq,
>
{
    fn apply(s: Sum<A, B>) -> Sum<C, D>;
}

#[cfg(test)]
mod coprod_tests {
    use std::fmt::Debug;

    use crate::coprod::{Sum, SumMorfism};

    pub struct Swap;

    impl<A: Debug + PartialEq, B: Debug + PartialEq> SumMorfism<A, B, B, A> for Swap {
        fn apply(s: Sum<A, B>) -> Sum<B, A> {
            match s {
                Sum::Left(l) => Sum::Right(l),
                Sum::Right(r) => Sum::Left(r),
            }
        }
    }

    #[test]
    fn test_swap_left_to_right() {
        let input: Sum<i32, &str> = Sum::Left(10);

        let result = Swap::apply(input);

        assert_eq!(result, Sum::Right(10));
    }
}
