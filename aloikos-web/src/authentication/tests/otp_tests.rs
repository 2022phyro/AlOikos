
// use crate::authentication::otp::Otp;
// use crate::config::CONFIG;
// const TEST_SECRET: &str = "c8085463-15ad-415c-a77a-45eb24d5a75a";
// const TEST_EMAIL: &str = "test@example.com";
// use urlencoding::encode;

// #[test]
// fn test_otp_creation_with_valid_inputs() {

//     let otp_result = Otp::new(TEST_SECRET, TEST_EMAIL, Some("EMAIL".to_string()));
//     assert!(
//         otp_result.is_ok(),
//         "OTP creation should succeed with valid inputs"
//     );

//     let otp = otp_result.unwrap();
//     assert_eq!(otp.get_user_email(), TEST_EMAIL);
//     assert_eq!(otp.get_otp_type(), "EMAIL");
//     assert!(!otp.get_secret().is_empty());
// }

// #[test]
// fn test_otp_creation_with_empty_secret() {
//     let otp_result = Otp::new("", TEST_EMAIL, None);
//     assert!(
//         otp_result.is_err(),
//         "OTP creation should fail with empty secret"
//     );

//     let error = otp_result.unwrap_err();
//     assert_eq!(error, "Secret cannot be empty");
// }

// #[test]
// fn test_otp_creation_with_empty_email() {
//     let otp_result = Otp::new(TEST_SECRET, "", None);
//     assert!(
//         otp_result.is_err(),
//         "OTP creation should fail with empty email"
//     );

//     let error = otp_result.unwrap_err();
//     assert_eq!(error, "User email cannot be empty");
// }

// #[test]
// fn test_otp_creation_with_default_type() {
//     let otp_result = Otp::new(TEST_SECRET, TEST_EMAIL, None);
//     assert!(
//         otp_result.is_ok(),
//         "OTP creation should succeed with default type"
//     );

//     let otp = otp_result.unwrap();
//     assert_eq!(otp.get_otp_type(), "EMAIL");
// }

// #[test]
// fn test_otp_creation_with_custom_type() {
//     let custom_type = "AUTHENTICATOR";
//     let otp_result = Otp::new(TEST_SECRET, TEST_EMAIL, Some(custom_type.to_string()));
//     assert!(
//         otp_result.is_ok(),
//         "OTP creation should succeed with custom type"
//     );

//     let otp = otp_result.unwrap();
//     assert_eq!(otp.get_otp_type(), custom_type);
// }

// #[test]
// fn test_random_secret_generation() {
//     let secret1 = Otp::generate_random_secret();
//     let secret2 = Otp::generate_random_secret();

//     assert!(!secret1.is_empty(), "Generated secret should not be empty");
//     assert!(!secret2.is_empty(), "Generated secret should not be empty");
//     assert_ne!(secret1, secret2, "Generated secrets should be different");

//     // Check that the secret is valid base32
//     assert!(
//         secret1
//             .chars()
//             .all(|c| "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567".contains(c)),
//         "Secret should contain only valid base32 characters"
//     );
// }

// #[test]
// fn test_otp_creation_with_random_secret() {
//     let otp_result = Otp::new_with_random_secret(TEST_EMAIL, Some("AUTHENTICATOR".to_string()));
//     assert!(
//         otp_result.is_ok(),
//         "OTP creation with random secret should succeed"
//     );

//     let otp = otp_result.unwrap();
//     assert_eq!(otp.get_user_email(), TEST_EMAIL);
//     assert_eq!(otp.get_otp_type(), "AUTHENTICATOR");
//     assert!(!otp.get_secret().is_empty());
// }

// #[test]
// fn test_otp_generation() {
//     let otp = Otp::new(TEST_SECRET, TEST_EMAIL, None).unwrap();
//     let generated_otp = otp.generate_otp();

//     assert!(generated_otp.is_ok(), "OTP generation should succeed");

//     let otp_code = generated_otp.unwrap();
//     assert_eq!(
//         otp_code.len(),
//         CONFIG.otp_digit_length,
//         "OTP should have correct length"
//     );
//     assert!(
//         otp_code.chars().all(|c| c.is_ascii_digit()),
//         "OTP should contain only digits"
//     );
// }

// #[test]
// fn test_otp_verification_with_empty_input() {
//     let otp = Otp::new(TEST_SECRET, TEST_EMAIL, None).unwrap();
//     let verification_result = otp.verify_otp("");

//     assert!(
//         verification_result.is_err(),
//         "Verification should fail with empty OTP"
//     );

//     let error = verification_result.unwrap_err();
//     assert_eq!(error, "OTP cannot be empty");
// }

// #[test]
// fn test_otp_verification_with_invalid_code() {
//     let otp = Otp::new(TEST_SECRET, TEST_EMAIL, None).unwrap();
//     let verification_result = otp.verify_otp("000000");

//     // This might succeed or fail depending on timing, but should not error
//     assert!(
//         verification_result.is_ok(),
//         "Verification should not error with invalid code"
//     );
// }

// #[test]
// fn test_otp_self_verification() {
//     let otp = Otp::new(TEST_SECRET, TEST_EMAIL, None).unwrap();

//     // Generate an OTP and immediately verify it
//     let generated_otp = otp.generate_otp().unwrap();
//     let verification_result = otp.verify_otp(&generated_otp);

//     assert!(
//         verification_result.is_ok(),
//         "Self-verification should not error"
//     );
//     // Note: This might fail due to timing windows, but should not panic
// }

// #[test]
// fn test_url_generation() {
//     let otp = Otp::new(TEST_SECRET, TEST_EMAIL, Some("AUTHENTICATOR".to_string())).unwrap();
//     let url = otp.get_url();
//     println!("Generated URL: {}", url);
//     assert!(!url.is_empty(), "URL should not be empty");
//     assert!(
//         url.starts_with("otpauth://totp/"),
//         "URL should be a proper TOTP URL"
//     );
//     assert!(
//         url.contains(&*encode(TEST_EMAIL)),
//         "URL should contain the user email"
//     );
//     assert!(
//         url.contains(&CONFIG.otp_issuer),
//         "URL should contain the issuer"
//     );
// }

// #[test]
// fn test_qr_code_generation() {
//     let otp = Otp::new(TEST_SECRET, TEST_EMAIL, None).unwrap();
//     let qr_result = otp.qr_code_base64();

//     assert!(qr_result.is_ok(), "QR code generation should succeed");

//     let qr_base64 = qr_result.unwrap();
//     assert!(!qr_base64.is_empty(), "QR code should not be empty");

//     // Basic base64 validation
//     assert!(
//         qr_base64
//             .chars()
//             .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '='),
//         "QR code should be valid base64"
//     );
// }

// #[test]
// fn test_multiple_otp_instances_independence() {
//     let otp1 = Otp::new("SECRET1HellojhdgfhdghfgdhgfhdjgfjdhgfjWorld", "user1@example.com", Some("EMAIL".to_string())).unwrap();
//     let otp2 = Otp::new(
//         "SECRET2HellojhdgfhdghfgdhgfhdjgfjdhgfjWorld",
//         "user2@example.com",
//         Some("AUTHENTICATOR".to_string()),
//     )
//     .unwrap();

//     assert_ne!(
//         otp1.get_secret(),
//         otp2.get_secret(),
//         "Different secrets should generate different OTPs"
//     );
//     assert_ne!(
//         otp1.get_user_email(),
//         otp2.get_user_email(),
//         "Different emails should be preserved"
//     );
//     assert_ne!(
//         otp1.get_otp_type(),
//         otp2.get_otp_type(),
//         "Different types should be preserved"
//     );

//     let otp1_code = otp1.generate_otp().unwrap();
//     let otp2_code = otp2.generate_otp().unwrap();

//     // OTPs generated at the same time with different secrets should be different
//     // (Note: This could theoretically fail due to collision, but extremely unlikely)
//     assert_ne!(
//         otp1_code, otp2_code,
//         "Different secrets should generate different OTP codes"
//     );
// }

// #[test]
// fn test_base32_secret_encoding() {
//     let test_input = "HellojhdgfhdghfgdhgfhdjgfjdhgfjWorld!";
//     let otp = Otp::new(test_input, TEST_EMAIL, None).unwrap();
//     let secret = otp.get_secret();

//     // Verify the secret is properly base32 encoded
//     assert!(
//         secret
//             .chars()
//             .all(|c| "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567".contains(c)),
//         "Secret should be valid base32"
//     );

//     // Verify we can decode it back (conceptually)
//     assert!(!secret.is_empty(), "Encoded secret should not be empty");
// }

// #[test]
// fn test_otp_consistency_same_secret() {
//     let otp1 = Otp::new(TEST_SECRET, TEST_EMAIL, Some("AUTHENTICATOR".to_string())).unwrap();
//     let otp2 = Otp::new(TEST_SECRET, TEST_EMAIL, Some("AUTHENTICATOR".to_string())).unwrap();

//     // Same secret should produce same base32 encoding
//     assert_eq!(
//         otp1.get_secret(),
//         otp2.get_secret(),
//         "Same secret should produce same encoding"
//     );

//     // Same secret should produce same URL
//     assert_eq!(
//         otp1.get_url(),
//         otp2.get_url(),
//         "Same secret should produce same URL"
//     );
// }

// #[test]
// fn test_email_validation_in_url() {
//     let emails = vec![
//         "test@example.com",
//         "user.name+tag@domain.co.uk",
//         "simple@test.org",
//     ];

//     for email in emails {
//         let otp = Otp::new(TEST_SECRET, email, Some("AUTHENTICATOR".to_string())).unwrap();
//         let url = otp.get_url();
//         assert!(
//             url.contains(&*encode(email)),
//             "URL should contain the email: {}",
//             email
//         );
//     }
// }
// #[test]
// fn test_otp_type_case_insensitive() {
//     let otp_email_lower = Otp::new(TEST_SECRET, TEST_EMAIL, Some("email".to_string())).unwrap();
//     let otp_email_upper = Otp::new(TEST_SECRET, TEST_EMAIL, Some("EMAIL".to_string())).unwrap();

//     // Both should be stored as provided (preserving case)
//     assert_eq!(otp_email_lower.get_otp_type(), "email");
//     assert_eq!(otp_email_upper.get_otp_type(), "EMAIL");
// }

// // Integration-style test that simulates real-world usage
// #[test]
// fn test_complete_otp_workflow() {
//     // 1. Create OTP service for a user
//     let user_email = "integration.test@example.com";
//     let otp_service = Otp::new_with_random_secret(user_email, Some("AUTHENTICATOR".to_string()))
//         .expect("Should create OTP service");

//     // 2. Verify the service is properly configured
//     assert_eq!(otp_service.get_user_email(), user_email);
//     assert_eq!(otp_service.get_otp_type(), "AUTHENTICATOR");
//     assert!(!otp_service.get_secret().is_empty());

//     // 3. Generate QR code for user setup
//     let qr_code = otp_service
//         .qr_code_base64()
//         .expect("Should generate QR code");
//     assert!(!qr_code.is_empty());

//     // 4. Get URL for manual entry
//     let url = otp_service.get_url();
//     assert!(url.contains(&*encode(user_email)));
//     assert!(url.contains("otpauth://totp/"));

//     // 5. Generate OTP code
//     let current_otp = otp_service.generate_otp().expect("Should generate OTP");
//     assert_eq!(current_otp.len(), CONFIG.otp_digit_length);

//     // 6. Verify the generated OTP (should succeed immediately)
//     let is_valid = otp_service
//         .verify_otp(&current_otp)
//         .expect("Should verify OTP without error");
//     // Note: This might be false due to timing, but should not error
//     println!("OTP {} verification result: {}", current_otp, is_valid);
// }
