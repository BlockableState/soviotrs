use crate::{
    merkle_tree::MerkleTree,
    util::DG, block_chain::{Block, BlockChain},
};

pub trait Signed<T> {
    fn signature(&self) -> String;
}

impl<T: DG + Clone> Signed<T> for Block<T> {
    fn signature(&self) -> String {
        return self.sig.clone();
    }
}

impl<T: DG + Clone> Signed<T> for BlockChain<T> {
    fn signature(&self) -> String {
        match self {
            BlockChain::Nil => String::new(),
            BlockChain::Cons { head, tail: _ } => head.sig.clone()
        }
    }
}

impl<T: DG + Clone> Signed<T> for MerkleTree<T> {
    fn signature(&self) -> String {
        match self {
            MerkleTree::Leaf(x) => Signed::<T>::signature(x),
            MerkleTree::MtNode {
                _hash_type: _p,
                sig,
                left: _,
                right: _,
            } => sig.clone(),
        }
    }
}

impl<T: DG> Signed<T> for String {
    fn signature(&self) -> String {
        return T::hash(self);
    }
}
