use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error;
pub struct ApiKey(pub(in crate::call) [u8; 64]);

impl ApiKey {
    /// 暗号学的に安全な乱数を使用した新しいインスタンスの生成
    /// 暗号学的に安全な乱数を生成することで、予測不可能なアクセストークンが生成されることが保証される。
    pub fn generate() -> Self {
        let mut csp_rng = ChaCha20Rng::from_entropy();
        let mut data = [0u8; 64];
        csp_rng.fill_bytes(&mut data);
        ApiKey(data)
    }

    pub(in crate::call) fn new(slice: [u8; 64]) -> Self {
        ApiKey(slice)
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
