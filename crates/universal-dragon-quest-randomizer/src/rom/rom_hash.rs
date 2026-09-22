use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RomHash([u8; 32]);

impl RomHash {
    pub fn from_bytes(data: &[u8]) -> Self {
        let digest = Sha256::digest(data);

        let mut hash = [0u8; 32];
        hash.copy_from_slice(&digest);

        Self(hash)
    }

    pub const fn from_raw(hash: [u8; 32]) -> Self {
        Self(hash)
    }

    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}
