use crate::{merkle_tree::MerkleTree, signed::Signed, util::DG};

pub trait Verifiable {
    fn verify(&self) -> bool;
    fn verify_all(&self) -> bool;

}

impl<T: DG + Clone> Verifiable for MerkleTree<T> {
    fn verify(&self) -> bool {
        match self {
            MerkleTree::Leaf(_) => true,
            MerkleTree::MtNode {
                _hash_type: _p,
                sig,
                left,
                right,
            } => {
                let v_sig = Signed::<T>::signature( &(left.as_ref().signature() + &right.as_ref().signature()));
                println!("sig compare: {} {}", v_sig.clone(), sig.clone());
                return sig.clone() == v_sig;
            }
        }
    }

    fn verify_all(&self) -> bool {
        match self {
            MerkleTree::Leaf(_) => self.verify(),
            MerkleTree::MtNode {
                _hash_type: _p,
                sig: _,
                left,
                right,
            } => {
                return self.verify() && left.verify_all() && right.verify_all() ;
            }
        }
    }
}
