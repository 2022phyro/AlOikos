use authentication::jwt::{JwtAccessToken, JwtRefreshToken, Token};
fn main() {
   let user_id = "user123".to_string();
   let access_token = JwtAccessToken::new(user_id.clone());
   let refresh_token = JwtRefreshToken::new(user_id);
    println!("Access Token: {}", access_token.token);
    println!("Refresh Token: {}", refresh_token.token);
    match access_token.verify() {
        Ok(claims) => println!("Access Token Claims: {:?}", claims),
        Err(e) => println!("Access Token Verification Error: {}", e),
    }
}
