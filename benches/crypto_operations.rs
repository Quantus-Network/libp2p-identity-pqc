// Copyright 2023 Protocol Labs.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

//! Benchmarks for cryptographic operations in libp2p-identity-pqc
//!
//! This benchmark suite compares the performance of different cryptographic
//! algorithms, with special focus on Post-Quantum Cryptography (Dilithium)
//! compared to classical algorithms.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use libp2p_identity::{KeyType, Keypair, PublicKey};

const MESSAGE_SIZES: &[usize] = &[32, 64, 128, 256, 512, 1024, 2048, 4096];
const SMALL_MESSAGE: &[u8] = b"Hello, libp2p world!";
const MEDIUM_MESSAGE: &[u8] = &[0u8; 256];
const LARGE_MESSAGE: &[u8] = &[0u8; 4096];

// Key generation benchmarks
fn bench_key_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("key_generation");

    // Dilithium (Post-Quantum)
    group.bench_function("🛡️ dilithium", |b| {
        b.iter(|| {
            black_box(Keypair::generate_dilithium());
        })
    });

    // Classical algorithms for comparison
    #[cfg(feature = "ed25519")]
    group.bench_function("🔑 ed25519", |b| {
        b.iter(|| {
            black_box(Keypair::generate_ed25519());
        })
    });

    #[cfg(feature = "ecdsa")]
    group.bench_function("🔒 ecdsa", |b| {
        b.iter(|| {
            black_box(Keypair::generate_ecdsa());
        })
    });

    #[cfg(feature = "secp256k1")]
    group.bench_function("🔐 secp256k1", |b| {
        b.iter(|| {
            black_box(Keypair::generate_secp256k1());
        })
    });

    group.finish();
}

// Signing benchmarks
fn bench_signing(c: &mut Criterion) {
    let mut group = c.benchmark_group("signing");

    let dilithium_keypair = Keypair::generate_dilithium();

    #[cfg(feature = "ed25519")]
    let ed25519_keypair = Keypair::generate_ed25519();

    #[cfg(feature = "ecdsa")]
    let ecdsa_keypair = Keypair::generate_ecdsa();

    #[cfg(feature = "secp256k1")]
    let secp256k1_keypair = Keypair::generate_secp256k1();

    // Small message signing
    group.bench_function("🛡️ dilithium_small", |b| {
        b.iter(|| {
            black_box(dilithium_keypair.sign(SMALL_MESSAGE).unwrap());
        })
    });

    #[cfg(feature = "ed25519")]
    group.bench_function("🔑 ed25519_small", |b| {
        b.iter(|| {
            black_box(ed25519_keypair.sign(SMALL_MESSAGE).unwrap());
        })
    });

    #[cfg(feature = "ecdsa")]
    group.bench_function("🔒 ecdsa_small", |b| {
        b.iter(|| {
            black_box(ecdsa_keypair.sign(SMALL_MESSAGE).unwrap());
        })
    });

    #[cfg(feature = "secp256k1")]
    group.bench_function("🔐 secp256k1_small", |b| {
        b.iter(|| {
            black_box(secp256k1_keypair.sign(SMALL_MESSAGE).unwrap());
        })
    });

    // Medium message signing
    group.bench_function("🛡️ dilithium_medium", |b| {
        b.iter(|| {
            black_box(dilithium_keypair.sign(MEDIUM_MESSAGE).unwrap());
        })
    });

    #[cfg(feature = "ed25519")]
    group.bench_function("🔑 ed25519_medium", |b| {
        b.iter(|| {
            black_box(ed25519_keypair.sign(MEDIUM_MESSAGE).unwrap());
        })
    });

    // Large message signing
    group.bench_function("🛡️ dilithium_large", |b| {
        b.iter(|| {
            black_box(dilithium_keypair.sign(LARGE_MESSAGE).unwrap());
        })
    });

    #[cfg(feature = "ed25519")]
    group.bench_function("🔑 ed25519_large", |b| {
        b.iter(|| {
            black_box(ed25519_keypair.sign(LARGE_MESSAGE).unwrap());
        })
    });

    group.finish();
}

// Verification benchmarks
fn bench_verification(c: &mut Criterion) {
    let mut group = c.benchmark_group("verification");

    let dilithium_keypair = Keypair::generate_dilithium();
    let dilithium_pubkey = dilithium_keypair.public();
    let dilithium_signature = dilithium_keypair.sign(SMALL_MESSAGE).unwrap();

    #[cfg(feature = "ed25519")]
    let ed25519_keypair = Keypair::generate_ed25519();
    #[cfg(feature = "ed25519")]
    let ed25519_pubkey = ed25519_keypair.public();
    #[cfg(feature = "ed25519")]
    let ed25519_signature = ed25519_keypair.sign(SMALL_MESSAGE).unwrap();

    #[cfg(feature = "ecdsa")]
    let ecdsa_keypair = Keypair::generate_ecdsa();
    #[cfg(feature = "ecdsa")]
    let ecdsa_pubkey = ecdsa_keypair.public();
    #[cfg(feature = "ecdsa")]
    let ecdsa_signature = ecdsa_keypair.sign(SMALL_MESSAGE).unwrap();

    #[cfg(feature = "secp256k1")]
    let secp256k1_keypair = Keypair::generate_secp256k1();
    #[cfg(feature = "secp256k1")]
    let secp256k1_pubkey = secp256k1_keypair.public();
    #[cfg(feature = "secp256k1")]
    let secp256k1_signature = secp256k1_keypair.sign(SMALL_MESSAGE).unwrap();

    group.bench_function("🛡️ dilithium", |b| {
        b.iter(|| {
            black_box(dilithium_pubkey.verify(SMALL_MESSAGE, &dilithium_signature));
        })
    });

    #[cfg(feature = "ed25519")]
    group.bench_function("🔑 ed25519", |b| {
        b.iter(|| {
            black_box(ed25519_pubkey.verify(SMALL_MESSAGE, &ed25519_signature));
        })
    });

    #[cfg(feature = "ecdsa")]
    group.bench_function("🔒 ecdsa", |b| {
        b.iter(|| {
            black_box(ecdsa_pubkey.verify(SMALL_MESSAGE, &ecdsa_signature));
        })
    });

    #[cfg(feature = "secp256k1")]
    group.bench_function("🔐 secp256k1", |b| {
        b.iter(|| {
            black_box(secp256k1_pubkey.verify(SMALL_MESSAGE, &secp256k1_signature));
        })
    });

    group.finish();
}

// Throughput benchmarks for different message sizes
fn bench_dilithium_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("dilithium_throughput");

    let keypair = Keypair::generate_dilithium();
    let pubkey = keypair.public();

    for &size in MESSAGE_SIZES {
        let message = vec![0u8; size];
        let signature = keypair.sign(&message).unwrap();

        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(
            BenchmarkId::new("🛡️ sign", size),
            &message,
            |b, msg| {
                b.iter(|| {
                    black_box(keypair.sign(msg).unwrap());
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("🛡️ verify", size),
            &(message.as_slice(), signature.as_slice()),
            |b, (msg, sig)| {
                b.iter(|| {
                    black_box(pubkey.verify(msg, sig));
                })
            },
        );
    }

    group.finish();
}

// Protobuf serialization benchmarks
fn bench_protobuf_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("protobuf_operations");

    let dilithium_keypair = Keypair::generate_dilithium();
    let dilithium_pubkey = dilithium_keypair.public();
    let dilithium_encoded_keypair = dilithium_keypair.to_protobuf_encoding().unwrap();
    let dilithium_encoded_pubkey = dilithium_pubkey.encode_protobuf();

    #[cfg(feature = "ed25519")]
    let ed25519_keypair = Keypair::generate_ed25519();
    #[cfg(feature = "ed25519")]
    let ed25519_pubkey = ed25519_keypair.public();
    #[cfg(feature = "ed25519")]
    let ed25519_encoded_keypair = ed25519_keypair.to_protobuf_encoding().unwrap();
    #[cfg(feature = "ed25519")]
    let ed25519_encoded_pubkey = ed25519_pubkey.encode_protobuf();

    // Keypair encoding
    group.bench_function("🛡️ dilithium_keypair_encode", |b| {
        b.iter(|| {
            black_box(dilithium_keypair.to_protobuf_encoding().unwrap());
        })
    });

    #[cfg(feature = "ed25519")]
    group.bench_function("🔑 ed25519_keypair_encode", |b| {
        b.iter(|| {
            black_box(ed25519_keypair.to_protobuf_encoding().unwrap());
        })
    });

    // Keypair decoding
    group.bench_function("🛡️ dilithium_keypair_decode", |b| {
        b.iter(|| {
            black_box(Keypair::from_protobuf_encoding(&dilithium_encoded_keypair).unwrap());
        })
    });

    #[cfg(feature = "ed25519")]
    group.bench_function("🔑 ed25519_keypair_decode", |b| {
        b.iter(|| {
            black_box(Keypair::from_protobuf_encoding(&ed25519_encoded_keypair).unwrap());
        })
    });

    // Public key encoding
    group.bench_function("🛡️ dilithium_pubkey_encode", |b| {
        b.iter(|| {
            black_box(dilithium_pubkey.encode_protobuf());
        })
    });

    #[cfg(feature = "ed25519")]
    group.bench_function("🔑 ed25519_pubkey_encode", |b| {
        b.iter(|| {
            black_box(ed25519_pubkey.encode_protobuf());
        })
    });

    // Public key decoding
    group.bench_function("🛡️ dilithium_pubkey_decode", |b| {
        b.iter(|| {
            black_box(PublicKey::try_decode_protobuf(&dilithium_encoded_pubkey).unwrap());
        })
    });

    #[cfg(feature = "ed25519")]
    group.bench_function("🔑 ed25519_pubkey_decode", |b| {
        b.iter(|| {
            black_box(PublicKey::try_decode_protobuf(&ed25519_encoded_pubkey).unwrap());
        })
    });

    group.finish();
}

// PeerId generation benchmarks
#[cfg(feature = "peerid")]
fn bench_peer_id_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("peer_id_generation");

    let dilithium_pubkey = Keypair::generate_dilithium().public();

    #[cfg(feature = "ed25519")]
    let ed25519_pubkey = Keypair::generate_ed25519().public();

    #[cfg(feature = "ecdsa")]
    let ecdsa_pubkey = Keypair::generate_ecdsa().public();

    #[cfg(feature = "secp256k1")]
    let secp256k1_pubkey = Keypair::generate_secp256k1().public();

    group.bench_function("🛡️ dilithium", |b| {
        b.iter(|| {
            black_box(dilithium_pubkey.to_peer_id());
        })
    });

    #[cfg(feature = "ed25519")]
    group.bench_function("🔑 ed25519", |b| {
        b.iter(|| {
            black_box(ed25519_pubkey.to_peer_id());
        })
    });

    #[cfg(feature = "ecdsa")]
    group.bench_function("🔒 ecdsa", |b| {
        b.iter(|| {
            black_box(ecdsa_pubkey.to_peer_id());
        })
    });

    #[cfg(feature = "secp256k1")]
    group.bench_function("🔐 secp256k1", |b| {
        b.iter(|| {
            black_box(secp256k1_pubkey.to_peer_id());
        })
    });

    group.finish();
}

// Memory usage comparison (signature and key sizes)
fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");

    // This benchmark measures the time to create and drop different key types
    // as a proxy for memory allocation overhead

    group.bench_function("🛡️ dilithium_memory", |b| {
        b.iter(|| {
            let keypair = Keypair::generate_dilithium();
            let pubkey = keypair.public();
            let signature = keypair.sign(SMALL_MESSAGE).unwrap();
            black_box((keypair, pubkey, signature));
        })
    });

    #[cfg(feature = "ed25519")]
    group.bench_function("🔑 ed25519_memory", |b| {
        b.iter(|| {
            let keypair = Keypair::generate_ed25519();
            let pubkey = keypair.public();
            let signature = keypair.sign(SMALL_MESSAGE).unwrap();
            black_box((keypair, pubkey, signature));
        })
    });

    group.finish();
}

// Combined benchmark groups
criterion_group!(
    crypto_benches,
    bench_key_generation,
    bench_signing,
    bench_verification,
    bench_dilithium_throughput,
    bench_protobuf_operations,
    bench_memory_usage
);

#[cfg(feature = "peerid")]
criterion_group!(peer_id_benches, bench_peer_id_generation);

#[cfg(feature = "peerid")]
criterion_main!(crypto_benches, peer_id_benches);

#[cfg(not(feature = "peerid"))]
criterion_main!(crypto_benches);
