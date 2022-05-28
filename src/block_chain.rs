use crate::{util::DG, merkle_tree::MerkleTree};

#[derive(Clone, Debug)]
pub struct Block<T: DG + Clone> {
    pub sig: String,
    pub prev_hash: String,
    pub time_stamp : String,
    pub tx_root : MerkleTree<T>,
    pub nonce : String
}


#[derive(Clone, Debug)]
pub enum BlockChain<T: DG+ Clone> {
    Nil,
    Cons {
        head: Block<T>,
        tail: Box<BlockChain<T>>,
    }
}




