
use base64::encode;

pub fn hash(data: &[u8]) -> String {
    encode(data)
}