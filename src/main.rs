mod merkle_tree;
mod signed;
mod util;
mod verifiable;
use std::iter::successors;

use crate::{merkle_tree::MerkleTree, util::DGSha256, verifiable::Verifiable};
// use functional::{Functor, control::Functor};

pub trait Generic1 {
    type Type;
}

impl<T> Generic1 for Option<T> {
    type Type = T;
}
pub trait Rebind1<Y>: Generic1 {
    type Type;
}

impl<T, Y> Rebind1<Y> for Option<T> {
    type Type = Option<Y>;
}

pub trait Functor: Generic1 {
    fn fmap<Y, F: Fn(<Self as Generic1>::Type) -> Y>(self, f: F) -> <Self as Rebind1<Y>>::Type
    where
        Self: Rebind1<Y>;
}

// impl<T> Functor for Option<T> {
//     fn fmap<Y, F: Fn(T)->Y>(self, f: F) -> Option<Y> {
//         match self {
//             Some(value) => Some(f(value)),
//             None => None
//         }
//     }
// }

// pub trait Functor: Generic1 {
//     fn fmap<F: Fn(<Self as Generic1>::Type)->Self>(self, f: F) -> Self;
// }

impl<T> Functor for Option<T> {
    fn fmap<Y, F: (Fn(<Option<T> as Generic1>::Type) -> Y)>(
        self,
        f: F,
    ) -> <Option<T> as Rebind1<Y>>::Type {
        match self {
            Some(value) => Some(f(value)),
            None => None,
        }
    }
}

// generic1!(Option);

fn main() {
    let powers_of_10 = successors(Some(1_u16), |n| n.checked_mul(10));
    assert_eq!(
        powers_of_10.collect::<Vec<_>>(),
        &[1, 10, 100, 1_000, 10_000]
    );
    println!("Hello, world!");
    let str_list = vec![
        String::from("1"),
        String::from("2"),
        String::from("1"),
        String::from("2"),
        String::from("3"),
    ];
    // let str_list = vec![String::from("1")];
    let cks: Vec<&[String]> = str_list.chunks(2).collect();
    println!("Debug: {:?}", cks);
    let a: Option<MerkleTree<DGSha256>> = merkle_tree::from_list(str_list);
    println!(
        "Debug: {:?}",
        a.clone().map(|x| x.verify())
    );
    println!( "Debug: verifying head:{:?} all:{:?}", a.clone().map(|x| x.verify()), a.clone().map(|x| x.verify_all()));
    // let a = Some(1);
    // let x = a.fmap(|a|a +1);
    // println!("Debug: {:?}", x);

    // create a Sha256 object
    // let mut hasher = Sha256::new();

    // write input message
    // hasher.input_str("hello world");

    // read hash digest
    // let hex = hasher.result_str();
    // println!("{}", hex);
    // let x = merkle_tree::from_list(hex);
    // let y = get_sig(&x);
    // println!("{}", y);
    // match x {
    //     MerkleTree::Leaf(x) => println!("{}", x),
    //     MerkleTree::MtNode{sig, right: _, left: _ } => print!("{}", sig)
    // }
}
