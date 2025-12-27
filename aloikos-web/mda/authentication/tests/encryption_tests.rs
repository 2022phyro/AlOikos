use crate::authentication::encryption::{encrypt_string, decrypt_string};

#[test]
fn test_encrypt_decrypt_roundtrip() {
    let original_text = "Hello, this is a secret message!";
    
    // Encrypt the text
    let encrypted = encrypt_string(original_text).expect("Encryption should succeed");
    assert!(!encrypted.is_empty(), "Encrypted text should not be empty");
    assert_ne!(encrypted, original_text, "Encrypted text should be different from original");
    
    // Decrypt the text
    let decrypted = decrypt_string(&encrypted).expect("Decryption should succeed");
    assert_eq!(decrypted, original_text, "Decrypted text should match original");
}

#[test]
fn test_encrypt_different_outputs() {
    let text = "Same input text";
    
    let encrypted1 = encrypt_string(text).expect("First encryption should succeed");
    let encrypted2 = encrypt_string(text).expect("Second encryption should succeed");
    
    // Should be different due to random nonces
    assert_ne!(encrypted1, encrypted2, "Multiple encryptions should produce different outputs");
    
    // But both should decrypt to the same original text
    let decrypted1 = decrypt_string(&encrypted1).expect("First decryption should succeed");
    let decrypted2 = decrypt_string(&encrypted2).expect("Second decryption should succeed");
    
    assert_eq!(decrypted1, text);
    assert_eq!(decrypted2, text);
}

#[test]
fn test_decrypt_invalid_data() {
    let invalid_base64 = "this is not valid base64!";
    let result = decrypt_string(invalid_base64);
    assert!(result.is_err(), "Should fail with invalid base64");
}

#[test]
fn test_decrypt_too_short() {
    use base64::{Engine as _, engine::general_purpose};
    let too_short = general_purpose::STANDARD.encode("short");
    let result = decrypt_string(&too_short);
    assert!(result.is_err(), "Should fail with data too short");
}

#[test]
fn test_encrypt_empty_string() {
    let empty = "";
    let encrypted = encrypt_string(empty).expect("Should encrypt empty string");
    let decrypted = decrypt_string(&encrypted).expect("Should decrypt empty string");
    assert_eq!(decrypted, empty);
}

#[test]
fn test_encrypt_unicode() {
    let unicode_text = "Hello 🌍! 日本語 العربية";
    let encrypted = encrypt_string(unicode_text).expect("Should encrypt unicode");
    let decrypted = decrypt_string(&encrypted).expect("Should decrypt unicode");
    assert_eq!(decrypted, unicode_text);
}

#[test]
fn test_large_text() {
    let large_text = "A".repeat(10000);
    let encrypted = encrypt_string(&large_text).expect("Should encrypt large text");
    let decrypted = decrypt_string(&encrypted).expect("Should decrypt large text");
    assert_eq!(decrypted, large_text);
}
