use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub fn hash(input:&str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(input);
    hasher.result_str()
}