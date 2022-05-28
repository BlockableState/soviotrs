use crate::{
    merkle_tree::MerkleTree,
    util::DG,
};

pub trait Signed<T> {
    fn signature(&self) -> String;
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
