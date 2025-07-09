use chrono::Utc;
use crate::authentication::jwt::{JwtAccessToken, JwtRefreshToken, Token, Blacklist};

#[tokio::test]
async fn test_token_creation() {
    let auth_change = Utc::now();
    let access = JwtAccessToken::new("user_a".into(), auth_change);
    let refresh = JwtRefreshToken::new("user_a".into(), auth_change);

    assert!(access.verify().is_ok());
    assert!(refresh.verify().is_ok());
}

#[tokio::test]
async fn test_token_jti_extraction() {
    let auth_change = Utc::now();
    let access = JwtAccessToken::new("user_b".into(), auth_change);
    let refresh = JwtRefreshToken::new("user_b".into(), auth_change);

    assert!(access.jti().is_some());
    assert!(refresh.jti().is_some());
}

#[tokio::test]
async fn test_token_blacklist_roundtrip() {
    let auth_change = Utc::now();
    let access = JwtAccessToken::new("user_c".into(), auth_change);
    let refresh = JwtRefreshToken::new("user_c".into(), auth_change);

    let jti_a = access.jti().unwrap();
    let jti_r = refresh.jti().unwrap();

    assert!(access.check_blacklist(jti_a.clone()).await.is_ok());
    assert!(refresh.check_blacklist(jti_r.clone()).await.is_ok());

    access.blacklist().await.unwrap();
    refresh.blacklist().await.unwrap();

    assert!(access.check_blacklist(jti_a).await.is_err());
    assert!(refresh.check_blacklist(jti_r).await.is_err());
}

#[tokio::test]
async fn test_token_full_verify_before_and_after_blacklist() {
    let auth_change = Utc::now();
    let access = JwtAccessToken::new("user_d".into(), auth_change);
    let refresh = JwtRefreshToken::new("user_d".into(), auth_change);

    assert!(access.full_verify().await.is_ok());
    assert!(refresh.full_verify().await.is_ok());

    access.blacklist().await.unwrap();
    refresh.blacklist().await.unwrap();

    assert!(access.full_verify().await.is_err());
    assert!(refresh.full_verify().await.is_err());
}

#[tokio::test]
async fn test_token_import_and_verification() {
    let auth_change = Utc::now();
    let original = JwtAccessToken::new("user_e".into(), auth_change);
    let token_string = original.token.clone();
    let imported = JwtAccessToken::from_token(token_string);

    assert!(imported.verify().is_ok());
}
