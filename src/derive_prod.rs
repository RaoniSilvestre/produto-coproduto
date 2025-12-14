// // TIRAR DUVIDA SEGUNDA SOBRE MELHOR IMPLEMENTAÇÂO
//
// pub trait DeriveProduct {
//     type Product<A, B>;
//
//     fn derive_product<A, B, C, F, G>(f: F, g: G) -> impl Fn(C) -> Self::Product<A, B>
//     where
//         C: Clone,
//         F: Fn(C) -> A,
//         G: Fn(C) -> B;
//
//     fn first<A: Clone, B>(p: Self::Product<A, B>) -> A;
//     fn second<A, B: Clone>(p: Self::Product<A, B>) -> B;
// }
//
// struct RustTypes;
//
// impl DeriveProduct for RustTypes {
//     type Product<A, B> = (A, B);
//
//     fn derive_product<A, B, C, F, G>(f: F, g: G) -> impl Fn(C) -> Self::Product<A, B>
//     where
//         C: Clone,
//         F: Fn(C) -> A,
//         G: Fn(C) -> B,
//     {
//         move |c: C| (f(c.clone()), g(c))
//     }
//
//     fn first<A: Clone, B>(p: Self::Product<A, B>) -> A {
//         p.0
//     }
//
//     fn second<A, B: Clone>(p: Self::Product<A, B>) -> B {
//         p.1
//     }
// }
//
// #[cfg(test)]
// mod deriving_prod {
//     use crate::derive_prod::{DeriveProduct, RustTypes};
//
//     #[derive(Clone)]
//     struct User {
//         name: String,
//         age: u8,
//     }
//
//     fn projecao_esquerda(user: User) -> String {
//         user.name
//     }
//
//     fn projecao_direita(user: User) -> u8 {
//         user.age
//     }
//
//     #[test]
//     fn criando_produto_do_nada() {
//         let u = User {
//             name: String::from("Valdisgleis"),
//             age: 30,
//         };
//
//         let get_product = RustTypes::derive_product(projecao_esquerda, projecao_direita);
//
//         let produto = get_product(u);
//
//         assert_eq!(
//             String::from("Valdisgleis"),
//             RustTypes::first(produto.clone())
//         );
//
//         assert_eq!(30, RustTypes::second(produto));
//     }
// }
//
// pub trait Produto {
//     type First;
//     type Second;
//
//     fn derive<C, F, G>(f: F, g: G, c: C) -> Self
//     where
//         C: Clone,
//         F: Fn(C) -> Self::First,
//         G: Fn(C) -> Self::Second,
//         Self: Sized;
//
//     fn projecao_esquerda(&self) -> Self::First;
//     fn projecao_direita(&self) -> Self::Second;
// }
//
// #[derive(Clone)]
// pub struct User {
//     name: String,
//     age: u8,
// }
//
// impl Produto for User {
//     type First = String;
//     type Second = u8;
//
//     fn derive<C, F, G>(f: F, g: G, c: C) -> Self
//     where
//         C: Clone,
//         F: Fn(C) -> Self::First,
//         G: Fn(C) -> Self::Second,
//         Self: Sized,
//     {
//         User {
//             name: f(c.clone()),
//             age: g(c.clone()),
//         }
//     }
//
//     fn projecao_esquerda(&self) -> Self::First {
//         self.name.clone()
//     }
//
//     fn projecao_direita(&self) -> Self::Second {
//         self.age
//     }
// }
//
// // Possivelmente a melhor implementação até então
// fn derive_product<A, B, C, F, G>(f: F, g: G) -> impl Fn(C) -> (A, B)
// where
//     C: Clone,
//     F: Fn(C) -> A,
//     G: Fn(C) -> B,
// {
//     move |c: C| (f(c.clone()), g(c))
// }
//
// #[test]
// fn deriving() {
//     fn projecao_esquerda(user: User) -> String {
//         user.name
//     }
//
//     fn projecao_direita(user: User) -> u8 {
//         user.age
//     }
//
//     let derived_produto = derive_product(projecao_esquerda, projecao_direita);
//
//     let f = String::from("Valdisgleis");
//     let s = 10;
//
//     let (first, second) = derived_produto(User {
//         name: f.clone(),
//         age: s,
//     });
//
//     assert_eq!(first, f);
//     assert_eq!(second, s);
// }

pub trait DeriveProduct {
    type Left;
    type Right;

    fn derive(s: Self) -> (Self::Left, Self::Right);
}

#[cfg(test)]
mod test {
    use crate::derive_prod::DeriveProduct;

    #[test]
    fn utilizando_derive_product() {
        #[derive(Debug, PartialEq)]
        struct Id(i32);
        #[derive(Debug, PartialEq)]
        struct Email(String);
        #[derive(Debug, PartialEq)]
        struct Name(String);

        struct User {
            id: Id,
            name: Name,
            email: Email,
        }

        impl DeriveProduct for User {
            type Left = (Id, Name);
            type Right = Email;
            fn derive(s: Self) -> (Self::Left, Self::Right) {
                ((s.id, s.name), s.email)
            }
        }

        let u = User {
            id: Id(10),
            name: Name("Valdisgleis".to_string()),
            email: Email("valdisgleis@gmail.com".to_string()),
        };

        let ((id, nome), email) = User::derive(u);

        assert_eq!(id, Id(10));
        assert_eq!(nome, Name("Valdisgleis".to_string()));
        assert_eq!(email, Email("valdisgleis@gmail.com".to_string()));
    }
}
