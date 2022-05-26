use crypto::digest::Digest;
use crypto::sha2::Sha256;


pub enum MerkleTree<'a> {
    Leaf(String),
    MtNode {
        sig: String,
        left: &'a MerkleTree<'a>,
        right: &'a MerkleTree<'a>
    }
}

fn hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(input);
    hasher.result_str()
}

pub fn from_list<'a>(s: String) -> MerkleTree<'a> {
    return MerkleTree::Leaf(s);
}

pub fn get_sig(t: &MerkleTree) -> String {
    match t {
        MerkleTree::Leaf(x) => hash(x),
        MerkleTree::MtNode{sig, right: _, left: _ } => String::from(sig)
    }
}

fn goUp<'a>(xs: [MerkleTree<'a>; 12]) -> [MerkleTree<'a>; 12]  {
    return xs;
}




