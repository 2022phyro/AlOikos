use aloikos_web::authentication::jwt::{JwtAccessToken, JwtRefreshToken, Token, Blacklist};
use aloikos_web::config::CONFIG;

#[tokio::main]
async fn main() {
    println!("🚀 JWT Token System Test Suite");
    println!("=============================================================");
    
    // Display configuration
    print_config();
    
    // Test user ID for demonstrations
    let test_user_id = "user123".to_string();
    
    println!("\n📋 Testing JWT Token Creation...");
    
    // Test 1: Create Access Token
    println!("\n1️⃣  Creating Access Token...");
    let access_token = JwtAccessToken::new(test_user_id.clone());
    println!("✅ Access token created: {}", truncate_token(&access_token.token));
    
    // Test 2: Create Refresh Token
    println!("\n2️⃣  Creating Refresh Token...");
    let refresh_token = JwtRefreshToken::new(test_user_id.clone());
    println!("✅ Refresh token created: {}", truncate_token(&refresh_token.token));
    
    println!("\n🔍 Testing Token Verification...");
    
    // Test 3: Verify Access Token
    println!("\n3️⃣  Verifying Access Token...");
    match access_token.verify() {
        Ok(token_data) => {
            println!("✅ Access token verification successful!");
            println!("   Subject: {}", token_data.claims.sub);
            println!("   Token Type: {}", token_data.claims.token_type);
            println!("   JWT ID: {}", token_data.claims.jti);
            println!("   Expires: {}", format_timestamp(token_data.claims.exp));
        }
        Err(e) => println!("❌ Access token verification failed: {}", e)
    }
    
    // Test 4: Verify Refresh Token
    println!("\n4️⃣  Verifying Refresh Token...");
    match refresh_token.verify() {
        Ok(token_data) => {
            println!("✅ Refresh token verification successful!");
            println!("   Subject: {}", token_data.claims.sub);
            println!("   Token Type: {}", token_data.claims.token_type);
            println!("   JWT ID: {}", token_data.claims.jti);
            println!("   Expires: {}", format_timestamp(token_data.claims.exp));
        }
        Err(e) => println!("❌ Refresh token verification failed: {}", e)
    }
    
    println!("\n🔒 Testing Blacklist Functionality...");
    
    // Test 5: Extract JWT IDs
    println!("\n5️⃣  Extracting JWT IDs...");
    if let Some(access_jti) = access_token.jti() {
        println!("✅ Access token JTI: {}", access_jti);
    } else {
        println!("❌ Failed to extract access token JTI");
    }
    
    if let Some(refresh_jti) = refresh_token.jti() {
        println!("✅ Refresh token JTI: {}", refresh_jti);
    } else {
        println!("❌ Failed to extract refresh token JTI");
    }
    
    // Test 6: Check blacklist status (before blacklisting)
    println!("\n6️⃣  Checking blacklist status (before blacklisting)...");
    if let Some(jti) = access_token.jti() {
        match access_token.check_blacklist(jti).await {
            Ok(()) => println!("✅ Access token is not blacklisted"),
            Err(e) => println!("⚠️  Blacklist check result: {}", e),
        }
    }
    
    // Test 7: Full verification (before blacklisting)
    println!("\n7️⃣  Full verification (before blacklisting)...");
    match access_token.full_verify().await {
        Ok(token_data) => {
            println!("✅ Full access token verification successful!");
            println!("   Subject: {}", token_data.claims.sub);
        }
        Err(e) => println!("❌ Full access token verification failed: {}", e)
    }
    
    match refresh_token.full_verify().await {
        Ok(token_data) => {
            println!("✅ Full refresh token verification successful!");
            println!("   Subject: {}", token_data.claims.sub);
        }
        Err(e) => println!("❌ Full refresh token verification failed: {}", e)
    }
    
    // Test 8: Blacklist tokens
    println!("\n8️⃣  Blacklisting tokens...");
    match access_token.blacklist().await {
        Ok(()) => println!("✅ Access token successfully blacklisted"),
        Err(e) => println!("❌ Failed to blacklist access token: {}", e)
    }
    
    match refresh_token.blacklist().await {
        Ok(()) => println!("✅ Refresh token successfully blacklisted"),
        Err(e) => println!("❌ Failed to blacklist refresh token: {}", e)
    }
    
    // Test 9: Check blacklist status (after blacklisting)
    println!("\n9️⃣  Checking blacklist status (after blacklisting)...");
    if let Some(jti) = access_token.jti() {
        match access_token.check_blacklist(jti).await {
            Ok(()) => println!("⚠️  Access token is not blacklisted (unexpected)"),
            Err(e) => println!("✅ Access token blacklist check: {}", e),
        }
    }
    
    // Test 10: Full verification (after blacklisting)
    println!("\n🔟 Full verification (after blacklisting)...");
    match access_token.full_verify().await {
        Ok(_) => println!("⚠️  Full access token verification passed (unexpected)"),
        Err(e) => println!("✅ Full access token verification correctly failed: {}", e)
    }
    
    match refresh_token.full_verify().await {
        Ok(_) => println!("⚠️  Full refresh token verification passed (unexpected)"),
        Err(e) => println!("✅ Full refresh token verification correctly failed: {}", e)      
    }
    
    println!("\n🧪 Testing Token Recreation and Import...");
    
    // Test 11: Create new tokens and test from_token
    println!("\n1️⃣1️⃣ Testing token recreation from string...");
    let new_access = JwtAccessToken::new("user456".to_string());
    let token_string = new_access.token.clone();
    let imported_access = JwtAccessToken::from_token(token_string);
    
    match imported_access.verify() {
        Ok(token_data) => {
            println!("✅ Imported access token verification successful!");
            println!("   Subject: {}", token_data.claims.sub);
        }
        Err(e) => println!("❌ Imported access token verification failed: {}", e)
    }
    
    println!("\n🎉 Test Suite Complete!");
    println!("{}", "=".repeat(50));
}

fn print_config() {
    println!("\n⚙️  Configuration Settings:");
    println!("   JWT Algorithm: {}", CONFIG.jwt_algorithm);
    println!("   JWT Issuer: {}", CONFIG.jwt_issuer);
    println!("   Access Duration: {} seconds", CONFIG.jwt_access_duration);
    println!("   Refresh Duration: {} seconds", CONFIG.jwt_refresh_duration);
    println!("   Redis URL: {}", CONFIG.redis_url);
}

fn truncate_token(token: &str) -> String {
    if token.len() > 50 {
        format!("{}...", &token[..50])
    } else {
        token.to_string()
    }
}

fn format_timestamp(timestamp: usize) -> String {
    use chrono::{DateTime};
    match DateTime::from_timestamp(timestamp as i64, 0) {
        Some(dt) => dt.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
        None => "Invalid timestamp".to_string(),
    }
}
