use aloikos_web::authentication::encryption::{encrypt_string, decrypt_string};

fn main() {
    println!("🔐 Encryption/Decryption Demo");
    println!("{}", "=".repeat(50));
    
    // Test 1: Basic encryption/decryption
    println!("\n1️⃣ Basic Encryption/Decryption Test");
    let original_message = "This is a secret message that needs to be encrypted!";
    println!("Original: {}", original_message);
    
    match encrypt_string(original_message) {
        Ok(encrypted) => {
            println!("✅ Encrypted: {}", encrypted);
            println!("   Length: {} characters", encrypted.len());
            
            match decrypt_string(&encrypted) {
                Ok(decrypted) => {
                    println!("✅ Decrypted: {}", decrypted);
                    if decrypted == original_message {
                        println!("🎉 Round-trip successful!");
                    } else {
                        println!("❌ Decryption mismatch!");
                    }
                }
                Err(e) => println!("❌ Decryption failed: {}", e),
            }
        }
        Err(e) => println!("❌ Encryption failed: {}", e),
    }
    
    // Test 2: Multiple encryptions of same text
    println!("\n2️⃣ Multiple Encryptions Test");
    let text = "Same input text";
    println!("Input: {}", text);
    
    for i in 1..=3 {
        match encrypt_string(text) {
            Ok(encrypted) => {
                println!("Encryption {}: {}", i, encrypted);
                match decrypt_string(&encrypted) {
                    Ok(decrypted) => {
                        if decrypted == text {
                            println!("   ✅ Decryption {} successful", i);
                        } else {
                            println!("   ❌ Decryption {} failed", i);
                        }
                    }
                    Err(e) => println!("   ❌ Decryption {} error: {}", i, e),
                }
            }
            Err(e) => println!("❌ Encryption {} failed: {}", i, e),
        }
    }
    
    // Test 3: Different data types
    println!("\n3️⃣ Different Data Types Test");
    let test_cases = vec![
        ("Empty string", ""),
        ("Simple text", "Hello World"),
        ("Unicode text", "Hello 🌍! 日本語 العربية"),
        ("JSON-like", r#"{"name": "John", "age": 30}"#),
        ("Numbers", "12345.67890"),
        ("Special chars", "!@#$%^&*()_+-=[]{}|;:,.<>?"),
    ];
    
    for (name, text) in test_cases {
        println!("\nTesting {}: {}", name, text);
        match encrypt_string(text) {
            Ok(encrypted) => {
                match decrypt_string(&encrypted) {
                    Ok(decrypted) => {
                        if decrypted == text {
                            println!("   ✅ Success");
                        } else {
                            println!("   ❌ Mismatch");
                        }
                    }
                    Err(e) => println!("   ❌ Decryption error: {}", e),
                }
            }
            Err(e) => println!("   ❌ Encryption error: {}", e),
        }
    }
    
    // Test 4: Error handling
    println!("\n4️⃣ Error Handling Test");
    println!("Testing invalid base64...");
    match decrypt_string("invalid_base64_string!") {
        Ok(_) => println!("   ⚠️ Should have failed"),
        Err(e) => println!("   ✅ Correctly failed: {}", e),
    }
    
    println!("Testing too short data...");
    match decrypt_string("dGVzdA==") { // "test" in base64 - too short
        Ok(_) => println!("   ⚠️ Should have failed"),
        Err(e) => println!("   ✅ Correctly failed: {}", e),
    }
    
    println!("\n🎉 Encryption Demo Complete!");
}
