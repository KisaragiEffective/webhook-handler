use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

struct ApiKey(pub(in crate::call) [u8; 64]);

impl ApiKey {
    /// 暗号学的に安全な乱数を使用した新しいインスタンスの生成
    /// 暗号学的に安全な乱数を生成することで、予測不可能なアクセストークンが生成されることが保証される。
    fn new() -> Self {
        let mut csp_rng = ChaCha20Rng::from_entropy();
        let mut data = [0u8; 64];
        csp_rng.fill_bytes(&mut data);
        ApiKey(data)
    }
}
