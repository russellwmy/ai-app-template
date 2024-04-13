use sha2::{Digest, Sha256};

pub fn hash_value(data: &[u8]) -> String {
    let result = Sha256::digest(data);
    hex::encode(result)
}
