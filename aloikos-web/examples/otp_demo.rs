use aloikos_web::authentication::otp::{Otp, OtpAction, OtpType};

fn main() {
    println!("🔐 OTP Service Test Demo");
    println!("{}", "=".repeat(50));
    
    // Test 1: Create OTP with random secret
    println!("\n1️⃣ Creating OTP with random secret...");
    match Otp::new("demo@example.com", OtpAction::TwoFaLogin, OtpType::AUTHENTICATOR) {
        Ok(otp) => {
            println!("✅ OTP created successfully!");
            println!("   User: {}", otp.get_user_email());
            println!("   Type: {}", otp.get_otp_type());
            println!("   Secret: {}", otp.get_secret());
            
            // Test 2: Generate OTP code
            println!("\n2️⃣ Generating OTP code...");
            match otp.generate_otp() {
                Ok(code) => {
                    println!("✅ Generated OTP: {}", code);
                    
                    // Test 3: Verify the generated code
                    println!("\n3️⃣ Verifying OTP code...");
                    match otp.verify_otp(&code) {
                        Ok(is_valid) => {
                            if is_valid {
                                println!("✅ OTP verification successful!");
                            } else {
                                println!("⚠️ OTP verification failed (might be timing issue)");
                            }
                        }
                        Err(e) => println!("❌ OTP verification error: {}", e),
                    }
                }
                Err(e) => println!("❌ Failed to generate OTP: {}", e),
            }
            
            // Test 4: Get authenticator URL
            println!("\n4️⃣ Getting authenticator URL...");
            let url = otp.get_url();
            println!("✅ Authenticator URL: {}", url);
            
            // Test 5: Generate QR code
            println!("\n5️⃣ Generating QR code...");
            match otp.qr_code_base64() {
                Ok(qr) => {
                    println!("✅ QR code generated (base64 length: {} chars)", qr.len());
                    println!("   First 50 chars: {}...", &qr[..50.min(qr.len())]);
                }
                Err(e) => println!("❌ Failed to generate QR code: {}", e),
            }
        }
        Err(e) => println!("❌ Failed to create OTP: {}", e),
    }
    
    // Test 6: Error handling
    println!("\n6️⃣ Testing error handling...");

    // Empty email
    match Otp::new("", OtpAction::MakeTransaction, OtpType::EMAIL) {
        Ok(_) => println!("⚠️ Empty email should have failed"),
        Err(e) => println!("✅ Empty email correctly rejected: {}", e),
    }
    
    println!("\n🎉 OTP Service Demo Complete!");
}
