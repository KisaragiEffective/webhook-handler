use crate::ApiKey;

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
            ApiKey::new(x)
        }
    }
}

impl AsRef<[u8]> for ApiKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}