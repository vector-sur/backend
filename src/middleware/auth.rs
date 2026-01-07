use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use async_trait::async_trait;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};

// JWT Secret - In production, load this from environment variables
const JWT_SECRET: &[u8] = b"your-secret-key-change-this-in-production";

// Token expiration time (24 hours)
const TOKEN_EXPIRATION_HOURS: i64 = 24;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // Subject (user id)
    pub username: String, // Username
    pub exp: i64,        // Expiration time
    pub iat: i64,        // Issued at
}

impl Claims {
    pub fn new(user_id: i32, username: String) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(TOKEN_EXPIRATION_HOURS);
        
        Self {
            sub: user_id.to_string(),
            username,
            exp: exp.timestamp(),
            iat: now.timestamp(),
        }
    }
}

// Create a JWT token
pub fn create_token(user_id: i32, username: String) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(user_id, username);
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
}

// Validate and decode a JWT token
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

// Auth error type
pub enum AuthError {
    InvalidToken,
    MissingToken,
    TokenExpired,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing token"),
            AuthError::TokenExpired => (StatusCode::UNAUTHORIZED, "Token expired"),
        };
        (status, message).into_response()
    }
}

// Extractor for authenticated requests
#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts<'life0, 'life1, 'async_trait>(
        parts: &'life0 mut Parts,
        _state: &'life1 S,
    ) -> Result<Self, Self::Rejection>
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::MissingToken)?;

        // Validate the token
        validate_token(bearer.token()).map_err(|err| {
            match err.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
                _ => AuthError::InvalidToken,
            }
        })
    }
}

// Password hashing utilities using bcrypt
// 
// Bcrypt automatically handles salt generation and embedding:
// - Cost factor: 12 (DEFAULT_COST) - number of hashing rounds (2^12 = 4096 iterations)
// - Salt: 22 random characters, automatically generated per password
// - Hash format: $2b$12$[22-char-salt][31-char-hash]
// 
// Example hash: $2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYqNRx8QVVW
//   - $2b$ = bcrypt algorithm identifier
//   - 12 = cost factor
//   - Next 22 chars = salt (e.g., "LQv3c1yqBWVHxkd0LHAkCO")
//   - Remaining 31 chars = actual hash
//
// Security features:
// - Each password gets a unique salt (prevents rainbow table attacks)
// - Slow hashing algorithm (prevents brute force attacks)
// - Salt is stored with the hash (no separate storage needed)

/// Hash a plaintext password using bcrypt with automatic salt generation
/// 
/// # Arguments
/// * `password` - The plaintext password to hash
/// 
/// # Returns
/// * `Ok(String)` - The hashed password with embedded salt
/// * `Err(BcryptError)` - If hashing fails
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
}

/// Verify a plaintext password against a bcrypt hash
/// 
/// # Arguments
/// * `password` - The plaintext password to verify
/// * `hash` - The bcrypt hash (with embedded salt) to verify against
/// 
/// # Returns
/// * `Ok(true)` - Password matches the hash
/// * `Ok(false)` - Password does not match
/// * `Err(BcryptError)` - If verification fails
pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(password, hash)
}
