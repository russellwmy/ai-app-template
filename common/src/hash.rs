use base64ct::{Base64, Encoding};
use sha2::{Digest, Sha256};

pub fn hash(data: &[u8]) -> String {
    // create a Sha256 object
    let mut hasher = Sha256::new();

    // write input message
    hasher.update(data);

    // read hash digest and consume hasher
    let hash = hasher.finalize();

    Base64::encode_string(&hash)
}
