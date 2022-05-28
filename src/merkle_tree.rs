use std::iter::successors;

use crate::{signed::Signed, util::DG};
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub enum MerkleTree<T: DG + Clone> {
    Leaf(String),
    MtNode {
        _hash_type: PhantomData<T>, // use to bound hash type
        sig: String,
        left: Box<MerkleTree<T>>,
        right: Box<MerkleTree<T>>,
    },
}

pub fn from_list<T: DG + Clone>(s: Vec<String>) -> Option<MerkleTree<T>> {
    let xs: Vec<MerkleTree<T>> = s.iter().map(|x| MerkleTree::Leaf(x.clone())).collect();
    let levels = (xs.len() as f32).log2().ceil() as usize;
    let x = successors(Some(xs), go_up);
    let y = x
        .skip(levels)
        .next()
        .and_then(|x| x.iter().next().map(MerkleTree::clone));
    return y;
}

fn go_up<'a, T: DG + Clone>(xs: &Vec<MerkleTree<T>>) -> Option<Vec<MerkleTree<T>>> {
    let ps: Vec<MerkleTree<T>> = xs
        .chunks(2)
        .map(|xs| {
            return xs
                .get(0)
                .zip(xs.get(1).or(Some(&MerkleTree::Leaf(String::new()))))
                .and_then(|(x, y)| Some(merge(x, y)))
                .unwrap();
        })
        .collect();
    return Some(ps);
}

pub fn merge<'a, T: DG + Clone>(t: &MerkleTree<T>, u: &MerkleTree<T>) -> MerkleTree<T> {
    return MerkleTree::MtNode {
        _hash_type: PhantomData,
        sig: Signed::<T>::signature(&(t.signature() + &u.signature())),
        left: Box::new(t.clone()),
        right: Box::new(u.clone()),
    };
}
