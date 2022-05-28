use crate::{merkle_tree::MerkleTree, signed::Signed, util::DG, block_chain::{Block, BlockChain}};

pub trait Verifiable {
    fn verify(&self) -> bool;

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
                return sig.clone() == v_sig && left.verify() && right.verify();
            }
        }
    }
}


impl<T: DG + Clone> Verifiable for Block<T> {
    fn verify(&self) -> bool {
        return self.tx_root.verify();
    }
}

impl<T: DG + Clone> Verifiable for BlockChain<T> {
    fn verify(&self) -> bool {
        match self {
            BlockChain::Cons { head, tail } => {
                return tail.signature() == head.prev_hash && head.verify() && tail.verify();
            }
            BlockChain::Nil => {
                return true;
            }
        }
    }
}
