mod merkle_tree;
mod verify_able;
mod util;
use std::iter::successors;



fn main() {
    let powers_of_10 = successors(Some(1_u16), |n| n.checked_mul(10));
    assert_eq!(powers_of_10.collect::<Vec<_>>(), &[1, 10, 100, 1_000, 10_000]);
    println!("Hello, world!");
    let str_list = vec![String::from("1"), String::from("2"), String::from("1"), String::from("2"), String::from("3")];
    // let str_list = vec![String::from("1")];
    let cks: Vec<&[String]> = str_list.chunks(2).collect();
    println!("Debug: {:?}", cks);
    let a = merkle_tree::from_list(str_list);
    println!("Debug: {:?}", a);


// create a Sha256 object
    // let mut hasher = Sha256::new();

    // write input message
    // hasher.input_str("hello world");

    // read hash digest
    // let hex = hasher.result_str();
    // println!("{}", hex);
    // let x = merkle_tree::from_list(hex);
    // let y = get_sig(&x);
    // println!("{}", y);
    // match x {
    //     MerkleTree::Leaf(x) => println!("{}", x),
    //     MerkleTree::MtNode{sig, right: _, left: _ } => print!("{}", sig)
    // }
}
