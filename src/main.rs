mod merkleTree;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

use crate::merkleTree::{MerkleTree, get_sig};





fn main() {
    println!("Hello, world!");
// create a Sha256 object
    let mut hasher = Sha256::new();

    // write input message
    hasher.input_str("hello world");

    // read hash digest
    let hex = hasher.result_str();
    println!("{}", hex);
    let x = merkleTree::from_list(hex);
    let y = get_sig(&x);
    println!("{}", y);
    match x {
        MerkleTree::Leaf(x) => println!("{}", x),
        MerkleTree::MtNode{sig, right: _, left: _ } => print!("{}", sig)
    }
}
