use crypto::digest::Digest;
use crypto::sha2::Sha256;

#[derive(Clone, Debug)]
pub struct DGSha256 {}
// type DGSha256 = ();

pub trait DG {
    fn hash(input: &str) -> String;
}

impl DG for DGSha256 {
    fn hash(input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.input_str(input);
        hasher.result_str()
    }
}
