use std::fmt::Debug;

use crate::coprod::Sum;

pub trait DeriveSum {
    type Left: Debug + PartialEq;
    type Right: Debug + PartialEq;

    fn derive(s: Self) -> Sum<Self::Left, Self::Right>;
}

#[cfg(test)]
mod test {

    use crate::{coprod::Sum, derive_coprod::DeriveSum};

    #[test]
    fn deriving_coprod() {
        #[derive(Debug, PartialEq)]
        struct Id(i32);
        #[derive(Debug, PartialEq)]
        struct Email(String);
        #[derive(Debug, PartialEq)]
        struct Name(String);

        enum UserIdentifier {
            Id(Id),
            Name(Name),
            Email(Email),
        }

        impl DeriveSum for UserIdentifier {
            type Left = Sum<Id, Name>;
            type Right = Email;

            fn derive(s: Self) -> Sum<Self::Left, Self::Right> {
                match s {
                    Self::Id(id) => Sum::Left(Sum::Left(id)),
                    Self::Name(name) => Sum::Left(Sum::Right(name)),
                    Self::Email(email) => Sum::Right(email),
                }
            }
        }

        let user = UserIdentifier::Email(Email("valdisgleis@gmail.com".to_string()));

        let x = UserIdentifier::derive(user);

        assert_eq!(x, Sum::Right(Email("valdisgleis@gmail.com".to_string())))
    }
}
