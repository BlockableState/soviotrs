
use crate::{merkle_tree::{MerkleTree, get_sig}, util::{hash, DG}};
pub trait VerifyAble {
    fn verify(&self) -> bool;
}

impl<T:DG+Clone> VerifyAble for MerkleTree<T> {
    fn verify(&self) -> bool {
        match self {
            MerkleTree::Leaf(_) => true,
            MerkleTree::MtNode {_hash_type: _p, sig, left, right } => 
                hash::<T>(sig) == hash::<T>(&(get_sig(left) + &get_sig(right))),
        }
    }
}