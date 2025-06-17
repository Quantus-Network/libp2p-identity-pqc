# libp2p-identity-pqc 🛡️

A **Post-Quantum Cryptography (PQC)** enabled fork of `libp2p-identity` that provides quantum-resistant cryptographic identity support for libp2p networks.

## 🌟 Features

- **🛡️ Post-Quantum Security**: Supports ML-DSA-87 (Dilithium Level 5) signatures
- **🔑 Multi-Algorithm Support**: Ed25519, RSA, ECDSA, Secp256k1, and Dilithium
- **🔒 Drop-in Replacement**: Compatible with existing libp2p-identity APIs
- **📦 Protocol Buffer Support**: Seamless serialization/deserialization
- **🎯 Peer ID Generation**: Quantum-resistant peer identification
- **📊 Comprehensive Logging**: Emoji-enhanced debug logging for key operations

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
libp2p-identity = { package = "libp2p-identity", path = "../libp2p-identity-pqc" }
# or from git
libp2p-identity = { git = "https://github.com/Quantus-Network/libp2p-identity-pqc", package = "libp2p-identity" }
```

### Basic Usage

```rust
use libp2p_identity::{Keypair, KeyType};

// Generate a quantum-resistant keypair
let keypair = Keypair::generate_dilithium();
println!("Generated keypair: {:?}", keypair.key_type());

// Sign a message
let message = b"Hello, Post-Quantum World!";
let signature = keypair.sign(message)?;

// Verify signature
let public_key = keypair.public();
let is_valid = public_key.verify(message, &signature);
assert!(is_valid);

// Generate PeerId
let peer_id = public_key.to_peer_id();
println!("Peer ID: {}", peer_id);
```

## 🔐 Supported Key Types

| Algorithm | Emoji | Security Level | Quantum Resistant |
|-----------|-------|----------------|-------------------|
| **Dilithium** | 🛡️ | Level 5 | ✅ **YES** |
| Ed25519 | 🔑 | Classical | ❌ No |
| RSA | 🗝️ | Classical | ❌ No |
| ECDSA | 🔒 | Classical | ❌ No |
| Secp256k1 | 🔐 | Classical | ❌ No |

## 🛡️ Post-Quantum Cryptography

This crate implements **ML-DSA-87** (the NIST-standardized version of Dilithium Level 5), providing:

- **Security Level**: Equivalent to AES-256
- **Signature Size**: ~4,627 bytes
- **Public Key Size**: ~2,592 bytes
- **Quantum Resistance**: Secure against both classical and quantum attacks

### Why Post-Quantum?

With the advent of quantum computers, traditional cryptographic algorithms (RSA, ECDSA, etc.) will become vulnerable. Post-quantum cryptography provides security against both classical and quantum attacks, ensuring long-term security for your libp2p networks.

## 📚 API Documentation

### Key Generation

```rust
// Post-Quantum (recommended for new applications)
let pq_keypair = Keypair::generate_dilithium();

// Classical algorithms (for compatibility)
let ed25519_keypair = Keypair::generate_ed25519();
let ecdsa_keypair = Keypair::generate_ecdsa();
let secp256k1_keypair = Keypair::generate_secp256k1();
```

### Key Loading

```rust
// Load from bytes
let keypair = Keypair::ed25519_from_bytes(secret_bytes)?;

// Load RSA from PKCS8
let rsa_keypair = Keypair::rsa_from_pkcs8(&mut pkcs8_der)?;

// Load Secp256k1 from DER
let secp_keypair = Keypair::secp256k1_from_der(&mut der_bytes)?;
```

### Serialization

```rust
// Encode keypair
let encoded = keypair.to_protobuf_encoding()?;

// Decode keypair
let decoded = Keypair::from_protobuf_encoding(&encoded)?;

// Encode public key
let pub_encoded = public_key.encode_protobuf();

// Decode public key
let pub_decoded = PublicKey::try_decode_protobuf(&pub_encoded)?;
```

### Key Type Detection

```rust
match keypair.key_type() {
    KeyType::Dilithium => println!("🛡️  Quantum-resistant!"),
    KeyType::Ed25519 => println!("🔑 Classical Ed25519"),
    KeyType::RSA => println!("🗝️  Classical RSA"),
    KeyType::Ecdsa => println!("🔒 Classical ECDSA"),
    KeyType::Secp256k1 => println!("🔐 Classical Secp256k1"),
}
```

## 🔧 Cargo Features

| Feature | Description | Default |
|---------|-------------|---------|
| `dilithium` | Post-Quantum Dilithium signatures | ✅ |
| `ed25519` | Ed25519 signatures | ❌ |
| `rsa` | RSA signatures | ❌ |
| `ecdsa` | ECDSA signatures | ❌ |
| `secp256k1` | Secp256k1 signatures | ❌ |
| `peerid` | PeerId generation support | ✅ |
| `rand` | Random key generation | ✅ |

### Custom Feature Selection

```toml
[dependencies]
libp2p-identity = {
    package = "libp2p-identity",
    path = "../libp2p-identity-pqc",
    default-features = false,
    features = ["dilithium", "ed25519", "peerid"]
}
```

## 🧪 Testing

Run the test suite:

```bash
# All tests
cargo test

# Dilithium-specific tests
cargo test --test dilithium_test

# With logging (using the log crate)
RUST_LOG=libp2p-identity=debug cargo test -- --nocapture
```

## ⚡ Performance Benchmarks

Comprehensive benchmarks are available to compare Dilithium performance with classical algorithms:

```bash
# Run all crypto benchmarks
cargo bench --bench crypto_operations

# Run specific benchmark groups
cargo bench --bench crypto_operations -- key_generation
cargo bench --bench crypto_operations -- signing
cargo bench --bench crypto_operations -- verification
cargo bench --bench crypto_operations -- dilithium_throughput

# Run PeerId benchmarks
cargo bench --bench peer_id
```

📊 **For detailed benchmark results and analysis, see [BENCHMARKS.md](BENCHMARKS.md)**

### Expected Performance Characteristics

| Operation | Dilithium 🛡️ | Ed25519 🔑 | ECDSA 🔒 | Notes |
|-----------|---------------|-------------|-----------|-------|
| **Key Generation** | ~1-2ms | ~50μs | ~100μs | PQC has higher overhead |
| **Signing** | ~200-500μs | ~10-20μs | ~50-100μs | PQC optimized for security |
| **Verification** | ~100-200μs | ~30-50μs | ~200-300μs | PQC comparable to ECDSA |
| **Signature Size** | ~4,627 bytes | 64 bytes | ~71 bytes | PQC signatures are larger |
| **Public Key Size** | ~2,592 bytes | 32 bytes | ~33 bytes | PQC keys are larger |

### Benchmark Categories

The benchmark suite includes:

- **🔑 Key Generation**: Time to generate new keypairs
- **✍️ Signing**: Time to sign messages of various sizes
- **✅ Verification**: Time to verify signatures
- **📊 Throughput**: Operations per second for different message sizes
- **📦 Serialization**: Protobuf encoding/decoding performance
- **🆔 PeerId Generation**: Time to generate peer identifiers
- **💾 Memory Usage**: Relative memory footprint comparison

### Trade-offs

**Dilithium Advantages:**
- ✅ Quantum-resistant security
- ✅ Strong mathematical foundation
- ✅ NIST standardized (ML-DSA-87)
- ✅ Reasonable verification performance

**Dilithium Considerations:**
- ⚠️ Larger signature and key sizes
- ⚠️ Higher computational overhead for key generation
- ⚠️ Slower signing compared to classical algorithms

## 🔄 Migration Guide

### From libp2p-identity

1. Replace the dependency in `Cargo.toml`
2. No code changes required - it's a drop-in replacement
3. Optionally start using Dilithium for new keypairs

### Gradual Migration

```rust
// Support both classical and post-quantum
fn create_keypair(use_pqc: bool) -> Keypair {
    if use_pqc {
        Keypair::generate_dilithium()  // 🛡️ Quantum-resistant
    } else {
        Keypair::generate_ed25519()    // 🔑 Classical
    }
}
```

## 🔬 Technical Details

### Dilithium Implementation

- **Algorithm**: ML-DSA-87 (NIST FIPS 204)
- **Implementation**: `rusty-crystals-dilithium`
- **Security**: NIST Level 5 (256-bit quantum security)
- **Key Sizes**:
  - Private: 4,896 bytes
  - Public: 2,592 bytes
  - Signature: ~4,627 bytes

## 🤝 Contributing

Contributions are welcome! Please ensure:

1. All tests pass: `cargo test`
2. Code is formatted: `cargo fmt`
3. No clippy warnings: `cargo clippy`
4. Documentation is updated

## 📄 License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

- Based on the original `libp2p-identity` crate
- Post-quantum cryptography implementation via `rusty-crystals-dilithium`
- NIST for standardizing ML-DSA (Dilithium)

## 📚 Further Reading

- [NIST Post-Quantum Cryptography](https://csrc.nist.gov/projects/post-quantum-cryptography)
- [ML-DSA Standard (FIPS 204)](https://csrc.nist.gov/pubs/fips/204/final)
- [libp2p Specifications](https://github.com/libp2p/specs)
- [Quantum Computing Threat Timeline](https://globalriskinstitute.org/publications/quantum-threat-timeline/)

---

**🛡️ Secure your libp2p networks against the quantum future! 🛡️**
