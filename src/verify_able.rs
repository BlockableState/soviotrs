
use crate::{merkle_tree::{MerkleTree, get_sig}, util::hash};
pub trait VerifyAble {
    fn verify(&self) -> bool;
}

impl VerifyAble for MerkleTree {
    fn verify(&self) -> bool {
        match self {
            MerkleTree::Leaf(_) => true,
            MerkleTree::MtNode { sig, left, right } => 
                hash(sig) == hash(&(get_sig(left) + &get_sig(right))),
        }
    }
}