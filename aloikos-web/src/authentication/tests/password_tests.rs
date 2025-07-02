
use super::super::password::*; // Import everything from the parent module
use super::super::errors::{PasswordError};
#[test]
    fn test_hash_and_verify_success() {
        let password = "mySecurePassword123!";
        let hashed_password = hash_password(password).expect("Failed to hash password");
        let is_valid = verify_password(password, &hashed_password).expect("Failed to verify password");
        assert!(is_valid, "Password should be valid after hashing and verification");
    }

    #[test]
    fn test_verify_incorrect_password() {
        let original_password = "mySecurePassword123!";
        let wrong_password = "wrongPassword";
        let hashed_password = hash_password(original_password).expect("Failed to hash password");
        let result = verify_password(wrong_password, &hashed_password);
        assert!(matches!(result, Err(PasswordError::VerificationFailed(_))), "Expected VerificationFailed error for wrong password");  }

    #[test]
    fn test_verify_malformed_hash() {
        let password = "test_password";
        let malformed_hash = "not-a-valid-argon2-hash-string";
        let result = verify_password(password, malformed_hash);
        assert!(result.is_err(), "Verification should fail for a malformed hash string");
        // Optionally, check the specific error type
        if let Err(PasswordError::InvalidHashFormat(_)) = result {
            // Expected error type
        } else {
            panic!("Expected InvalidHashFormat error");
        }
    }

    #[test]
    fn test_hash_output_format() {
        let password = "anotherPassword456";
        let hashed_password = hash_password(password).expect("Failed to hash password");

        assert!(hashed_password.starts_with("$argon2"), "Hash should start with $argon2");
        assert!(hashed_password.len() > 60, "Hash should be reasonably long");
    }

    #[test]
    fn test_different_hashes_for_same_password() {
        let password = "myPassword";
        let hash1 = hash_password(password).expect("Failed to hash password 1");
        let hash2 = hash_password(password).expect("Failed to hash password 2");

        assert_ne!(hash1, hash2, "Hashing the same password should produce different hashes (due to random salt)");
        assert!(verify_password(password, &hash1).expect("Verify hash1 failed"), "Hash1 should verify");
        assert!(verify_password(password, &hash2).expect("Verify hash2 failed"), "Hash2 should verify");
    }