pub mod login;
pub mod dogs;
pub mod error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

// The sub field is the token subject (generally username or user id), 
// the iat field will store the unix timestamp of the token generation and 
// the exp field will store the unix timestamp of the token expiration time.
