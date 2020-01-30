//! Cryptographic algorithm registry

//
// Hash algorithms
//

/// NIST SHA-256 digest algorithm (FIPS 180-4)
pub const SHA256_ALG_ID: &str = "sha256";

//
// Encryption algorithms
//

/// AES-128 in Galois Counter Mode (GCM)
pub const AES128GCM_ALG_ID: &str = "aes128gcm";

/// AES-256 in Galois Counter Mode (GCM)
pub const AES256GCM_ALG_ID: &str = "aes256gcm";

/// ChaCha20Poly1305 AEAD
pub const CHACHA20POLY1305_ALG_ID: &str = "chacha20poly1305";

//
// Signature algorithms
//

/// Ed25519 elliptic curve digital signature algorithm (RFC 8032)
pub const ED25519_ALG_ID: &str = "ed25519";
