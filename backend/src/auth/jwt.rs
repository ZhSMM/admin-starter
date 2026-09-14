//! JWT 签发与校验
//!
//! 设计要点:
//! - `Claims` 里只放最小必要信息(user_id + username),其他信息按需查库
//! - exp 用 unix timestamp,i64,避免时区问题
//! - 密钥从配置读,启动期校验非空

use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::error::{AppError, AppResult};

/// 载荷
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,        // user_id
    pub username: String,
    pub exp: i64,        // 过期时间(unix 秒)
    pub iat: i64,        // 签发时间
}

#[derive(Clone)]
pub struct JwtService {
    encoding: EncodingKey,
    decoding: DecodingKey,
    expire_seconds: i64,
}

impl JwtService {
    pub fn new(cfg: &Config) -> Self {
        Self {
            encoding: EncodingKey::from_secret(cfg.jwt_secret.as_bytes()),
            decoding: DecodingKey::from_secret(cfg.jwt_secret.as_bytes()),
            expire_seconds: cfg.jwt_expire_hours * 3600,
        }
    }

    /// 签发 token
    pub fn issue(&self, user_id: i64, username: &str) -> AppResult<String> {
        let now = Utc::now().timestamp();
        let claims = Claims {
            sub: user_id,
            username: username.to_string(),
            iat: now,
            exp: now + self.expire_seconds,
        };
        Ok(encode(&Header::default(), &claims, &self.encoding)?)
    }

    /// 校验并返回 Claims
    pub fn verify(&self, token: &str) -> AppResult<Claims> {
        let data = decode::<Claims>(token, &self.decoding, &Validation::default())
            .map_err(|_| AppError::Unauthorized)?;
        Ok(data.claims)
    }
}
