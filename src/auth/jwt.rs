use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use chrono::{Utc, Duration};

#[derive(Serialize,Deserialize,Debug)]
pub struct Claims{
    pub sub: String,
    pub exp: usize,
}

pub fn create_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("SECRET MUST BE SET !!");
    let exp = Utc::now() + Duration::hours(24);

    let claims = Claims{
        sub: user_id.to_string(),
        exp: exp.timestamp() as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

