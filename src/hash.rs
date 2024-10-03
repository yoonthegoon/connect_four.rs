use std::hash::{BuildHasher, Hasher};

pub struct ConnectFourHasher {
    hash: u64,
}

impl Hasher for ConnectFourHasher {
    fn finish(&self) -> u64 { self.hash }

    fn write(&mut self, bytes: &[u8]) {
        assert_eq!(bytes.len(), 8, "bytes: {:?}", bytes);
        self.hash = u64::from_ne_bytes(bytes[0..8].try_into().unwrap());
    }
}

#[derive(Default)]
pub struct BuildConnectFourHasher;

impl BuildHasher for BuildConnectFourHasher {
    type Hasher = ConnectFourHasher;

    fn build_hasher(&self) -> Self::Hasher { ConnectFourHasher { hash: 0 } }
}
