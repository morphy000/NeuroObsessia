use sha2::{Digest, Sha256};

pub const MASTER_KEY_NORMALIZED: &str = "NEUROOBSESSIA2026KEYPASS";
pub const LICENSE_SECRET: &str = "NeuroObsessia#2026#GlobalLicenseSecret";

/// Checks if a key matches either the universal master key or a generated valid license key.
pub fn verify_key(key: &str) -> bool {
    let k: String = key.chars().filter(|c| c.is_ascii_alphanumeric()).collect();
    let k = k.to_uppercase();
    if k.len() != 24 {
        return false;
    }

    // 1. Universal Master Key for NeuroObsessia
    if k == MASTER_KEY_NORMALIZED {
        return true;
    }

    // 2. Cryptographic signature check for generated keys
    let mut hasher = Sha256::new();
    hasher.update(&k[..12]);
    hasher.update(LICENSE_SECRET.as_bytes());
    let expected = hex::encode(hasher.finalize()).to_uppercase();
    let expected = &expected[..12];

    let mut result = 0u8;
    for (x, y) in k[12..].chars().zip(expected.chars()) {
        result |= (x as u8) ^ (y as u8);
    }
    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mint_key(nonce: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(nonce.as_bytes());
        hasher.update(LICENSE_SECRET.as_bytes());
        let sig = hex::encode(hasher.finalize()).to_uppercase();
        format!("{}{}", nonce, &sig[..12])
    }

    #[test]
    fn accepts_master_key() {
        assert!(verify_key("NEUROOBSESSIA-2026-KEY-PASS"));
        assert!(verify_key("NEUROOBSESSIA2026KEYPASS"));
        assert!(verify_key("neuroobsessia-2026-key-pass"));
    }

    #[test]
    fn accepts_minted_key() {
        let key = mint_key("NEURO1234567");
        assert!(verify_key(&key));
    }

    #[test]
    fn rejects_garbage() {
        assert!(!verify_key("not-a-key"));
        assert!(!verify_key(""));
        assert!(!verify_key("NEUROOBSESSIA-WRONG-KEY"));
    }
}
