use libp2p_identity::{KeyType, Keypair};

#[test]
fn test_dilithium_keypair_generation() {
    let keypair = Keypair::generate_dilithium();

    // Test that we can get the public key
    let public_key = keypair.public();

    // Test that the key type is correct
    assert_eq!(public_key.key_type(), KeyType::Dilithium);
    assert_eq!(keypair.key_type(), KeyType::Dilithium);
}

#[test]
fn test_dilithium_sign_and_verify() {
    let keypair = Keypair::generate_dilithium();
    let public_key = keypair.public();

    let message = b"Hello, Post-Quantum World!";

    // Sign the message
    let signature = keypair.sign(message).expect("Signing should succeed");

    // Verify the signature
    let is_valid = public_key.verify(message, &signature);
    assert!(is_valid, "Signature should be valid");

    // Test with wrong message
    let wrong_message = b"Wrong message";
    let is_invalid = public_key.verify(wrong_message, &signature);
    assert!(!is_invalid, "Signature should be invalid for wrong message");
}

#[test]
fn test_dilithium_protobuf_roundtrip() {
    let original_keypair = Keypair::generate_dilithium();

    // Encode to protobuf
    let encoded = original_keypair
        .to_protobuf_encoding()
        .expect("Encoding should succeed");

    // Decode from protobuf
    let decoded_keypair =
        Keypair::from_protobuf_encoding(&encoded).expect("Decoding should succeed");

    // Test that both keypairs produce the same signatures
    let message = b"Test message for roundtrip";
    let original_signature = original_keypair
        .sign(message)
        .expect("Original signing should succeed");
    let decoded_signature = decoded_keypair
        .sign(message)
        .expect("Decoded signing should succeed");

    // Both signatures should be valid when verified with their respective public keys
    let original_public = original_keypair.public();
    let decoded_public = decoded_keypair.public();

    assert!(
        original_public.verify(message, &original_signature),
        "Original signature should be valid"
    );
    assert!(
        decoded_public.verify(message, &decoded_signature),
        "Decoded signature should be valid"
    );

    // Cross-verification should also work since they're the same key
    assert!(
        original_public.verify(message, &decoded_signature),
        "Cross-verification should work"
    );
    assert!(
        decoded_public.verify(message, &original_signature),
        "Cross-verification should work"
    );
}

#[test]
fn test_dilithium_public_key_encoding() {
    let keypair = Keypair::generate_dilithium();
    let public_key = keypair.public();

    // Test protobuf encoding/decoding
    let encoded = public_key.encode_protobuf();
    let decoded = libp2p_identity::PublicKey::try_decode_protobuf(&encoded)
        .expect("Public key decoding should succeed");

    assert_eq!(
        public_key, decoded,
        "Public keys should be equal after roundtrip"
    );
    assert_eq!(
        decoded.key_type(),
        KeyType::Dilithium,
        "Key type should be preserved"
    );
}

#[test]
fn test_dilithium_peer_id_generation() {
    let keypair = Keypair::generate_dilithium();
    let public_key = keypair.public();

    // Generate PeerId from public key
    let peer_id = public_key.to_peer_id();

    // PeerId should be valid
    assert!(
        !peer_id.to_string().is_empty(),
        "PeerId string should not be empty"
    );

    // Test that the same public key generates the same PeerId
    let peer_id2 = public_key.to_peer_id();
    assert_eq!(
        peer_id, peer_id2,
        "Same public key should generate same PeerId"
    );

    // Different keypairs should generate different PeerIds
    let keypair2 = Keypair::generate_dilithium();
    let peer_id3 = keypair2.public().to_peer_id();
    assert_ne!(
        peer_id, peer_id3,
        "Different keypairs should generate different PeerIds"
    );
}
