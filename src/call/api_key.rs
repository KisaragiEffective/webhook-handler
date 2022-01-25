use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error;

pub struct ApiKey(pub(in crate::call) [u8; 64]);

impl ApiKey {
    /// 暗号学的に安全な乱数を使用した新しいインスタンスの生成
    /// 暗号学的に安全な乱数を生成することで、予測不可能なアクセストークンが生成されることが保証される。
    fn new() -> Self {
        let mut csp_rng = ChaCha20Rng::from_entropy();
        let mut data = [0u8; 64];
        csp_rng.fill_bytes(&mut data);
        ApiKey(data)
    }

    fn as_base64(&self) -> String {
        base64::encode(self)
    }
}

impl <'de> Deserialize<'de> for ApiKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        String::deserialize(deserializer).and_then(|a| base64::decode(a).map_err(|f| D::Error::custom(f))).map(|a| ApiKey::from(a))
    }
}

impl Serialize for ApiKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        base64::encode(self).serialize(serializer)
    }
}

impl AsRef<[u8]> for ApiKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<Vec<u8>> for ApiKey {
    fn from(a: Vec<u8>) -> Self {
        if a.len() != 64 {
            panic!("ApiKey bytes != 64")
        } else {
            let mut x = [0u8; 64];
            let mut p = 0usize;
            for m in a {
                x[p] = m;
                p += 1;
            }
            ApiKey(x)
        }
    }
}