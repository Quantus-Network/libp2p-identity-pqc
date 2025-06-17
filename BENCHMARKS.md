# Performance Benchmarks - libp2p-identity-pqc

This document provides example benchmark results comparing Dilithium (Post-Quantum) cryptography with classical algorithms in libp2p-identity-pqc.

## Test Environment

- **Hardware**: Apple M1 Pro (ARM64)
- **Rust**: 1.73.0
- **Build**: Release mode with optimizations
- **Date**: Example results (actual performance may vary)

## Running Benchmarks

```bash
# Run all crypto benchmarks
cargo bench --bench crypto_operations

# Run specific categories
cargo bench --bench crypto_operations -- key_generation
cargo bench --bench crypto_operations -- signing
cargo bench --bench crypto_operations -- verification
```

## Benchmark Results

### 🔑 Key Generation Performance

| Algorithm | Time | Relative |
|-----------|------|----------|
| 🛡️ Dilithium (PQC) | 1.2ms | 24x |
| 🔑 Ed25519 | 50μs | 1x (baseline) |
| 🔒 ECDSA | 120μs | 2.4x |
| 🔐 Secp256k1 | 180μs | 3.6x |

### ✍️ Signing Performance

| Algorithm | Small (20B) | Medium (256B) | Large (4KB) |
|-----------|-------------|---------------|-------------|
| 🛡️ Dilithium (PQC) | 480μs | 485μs | 510μs |
| 🔑 Ed25519 | 15μs | 16μs | 18μs |
| 🔒 ECDSA | 95μs | 96μs | 98μs |
| 🔐 Secp256k1 | 75μs | 76μs | 78μs |

### ✅ Verification Performance

| Algorithm | Time | Relative |
|-----------|------|----------|
| 🛡️ Dilithium (PQC) | 180μs | 3.6x |
| 🔑 Ed25519 | 50μs | 1x (baseline) |
| 🔒 ECDSA | 220μs | 4.4x |
| 🔐 Secp256k1 | 160μs | 3.2x |

### 📦 Serialization Performance

#### Keypair Encoding/Decoding

| Algorithm | Encode | Decode | Size |
|-----------|--------|--------|------|
| 🛡️ Dilithium (PQC) | 25μs | 180μs | 4,896B |
| 🔑 Ed25519 | 8μs | 12μs | 64B |
| 🔒 ECDSA | 15μs | 35μs | 121B |
| 🔐 Secp256k1 | 12μs | 18μs | 32B |

#### Public Key Encoding/Decoding

| Algorithm | Encode | Decode | Size |
|-----------|--------|--------|------|
| 🛡️ Dilithium (PQC) | 15μs | 85μs | 2,592B |
| 🔑 Ed25519 | 3μs | 5μs | 32B |
| 🔒 ECDSA | 8μs | 15μs | 91B |
| 🔐 Secp256k1 | 5μs | 8μs | 33B |

### 🆔 PeerId Generation

| Algorithm | Time | Relative |
|-----------|------|----------|
| 🛡️ Dilithium (PQC) | 95μs | 3.2x |
| 🔑 Ed25519 | 30μs | 1x (baseline) |
| 🔒 ECDSA | 45μs | 1.5x |
| 🔐 Secp256k1 | 35μs | 1.2x |

### 📊 Throughput Analysis

#### Dilithium Signing Throughput

| Message Size | Throughput | Notes |
|--------------|------------|-------|
| 32B | 2,083 ops/sec | Typical hash size |
| 256B | 2,062 ops/sec | Small message |
| 1KB | 1,960 ops/sec | Medium message |
| 4KB | 1,869 ops/sec | Large message |

#### Dilithium Verification Throughput

| Message Size | Throughput | Notes |
|--------------|------------|-------|
| 32B | 5,555 ops/sec | ~3x faster than signing |
| 256B | 5,494 ops/sec | Consistent performance |
| 1KB | 5,347 ops/sec | Scales well |
| 4KB | 5,102 ops/sec | Good for large messages |

## 💾 Memory Footprint

### Signature Sizes

| Algorithm | Signature Size | Compression Ratio |
|-----------|----------------|-------------------|
| 🛡️ Dilithium (PQC) | 4,627 bytes | 1x |
| 🔑 Ed25519 | 64 bytes | 72:1 |
| 🔒 ECDSA | 70-72 bytes | 64:1 |
| 🔐 Secp256k1 | 64 bytes | 72:1 |

### Key Sizes

| Algorithm | Private Key | Public Key | Total |
|-----------|-------------|------------|-------|
| 🛡️ Dilithium (PQC) | 4,896B | 2,592B | 7,488B |
| 🔑 Ed25519 | 32B | 32B | 64B |
| 🔒 ECDSA | 32B | 33B | 65B |
| 🔐 Secp256k1 | 32B | 33B | 65B |

## 🎯 Performance Summary

### When to Use Dilithium

✅ **Recommended for:**
- Long-term security requirements (10+ years)
- High-value cryptographic assets
- Compliance with quantum-resistant standards
- Applications where verification is frequent but signing is rare
- Future-proofing against quantum threats

⚠️ **Consider alternatives for:**
- Bandwidth-constrained environments
- High-frequency signing operations
- Memory-limited embedded systems
- Real-time applications requiring microsecond latency

## 🔬 Benchmark Methodology

### Test Configuration

```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
```

### Reproducibility

To reproduce these benchmarks:

```bash
# Ensure release mode
export CARGO_PROFILE_RELEASE_LTO=fat
export CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1

# Run benchmarks
cargo bench --bench crypto_operations

# Generate detailed reports
cargo bench --bench crypto_operations -- --output-format html
```

## 📚 Additional Resources

- [NIST PQC Performance Analysis](https://csrc.nist.gov/Projects/post-quantum-cryptography/post-quantum-cryptography-standardization/evaluation-criteria/security-(evaluation-criteria))
- [Dilithium Specification](https://pq-crystals.org/dilithium/)
- [Post-Quantum Cryptography Performance Studies](https://eprint.iacr.org/2019/1086.pdf)

---

**Note**: These are example benchmark results. Actual performance will vary based on hardware, system configuration, and specific use cases. Always benchmark in your target environment for accurate performance data.