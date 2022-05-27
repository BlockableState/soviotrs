use std::{iter::{successors}};
use crate::util::{hash};


#[derive(Clone, Debug)]
pub enum MerkleTree {
    Leaf(String),
    MtNode {
        sig: String,
        left: Box<MerkleTree>,
        right: Box<MerkleTree>
    },
}


pub fn from_list(s: Vec<String>) -> Option<MerkleTree> {
    let xs: Vec<MerkleTree> = s.iter().map(|x| MerkleTree::Leaf(x.clone())).collect();
    let levels = (xs.len() as f32).log2().ceil() as usize ;
    let x = successors(Some(xs), go_up);
    let y = x.skip(levels).next().and_then(|x| x.iter().next().map(MerkleTree:: clone));
    return y
}

fn go_up<'a>(xs:&Vec<MerkleTree>) -> Option<Vec<MerkleTree>>  {
    let ps: Vec<MerkleTree> = xs.chunks(2).map(|xs| {
        return xs.get(0).
            zip(xs.get(1).or(Some(&MerkleTree::Leaf(String::new()))))
            .and_then(|(x, y)| Some(merge(x, y)))
            .unwrap();
    }).collect();
    return Some(ps);
}

pub fn get_sig(t: & MerkleTree) -> String {
    match t {
        MerkleTree::Leaf(x) => hash(&x),
        MerkleTree::MtNode{sig, right: _, left: _ } => String::from(sig)
    }
}

pub fn merge<'a>(t:  &MerkleTree, u: &MerkleTree) -> MerkleTree {
    return MerkleTree::MtNode {
        sig: hash(&(get_sig(&t) + &get_sig(&u))),
        left: Box::new(t.clone()),
        right: Box::new(u.clone())
    };
}




