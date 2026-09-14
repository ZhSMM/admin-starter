//! 密码 hash / verify
//!
//! 用 bcrypt。bcrypt 的 cost 越高越慢,生产建议 10~12。

use bcrypt::{hash, verify, DEFAULT_COST};

use crate::error::{AppError, AppResult};

/// 用指定 cost 把明文密码 hash 成 bcrypt 字符串。
pub fn hash_password(plain: &str, cost: u32) -> AppResult<String> {
    let cost = if cost == 0 { DEFAULT_COST } else { cost };
    Ok(hash(plain, cost)?)
}

/// 校验明文密码和 hash 是否一致。
pub fn verify_password(plain: &str, hashed: &str) -> AppResult<bool> {
    // 截掉可能的多余空白
    Ok(verify(plain.trim(), hashed)?)
}

/// 校验失败时直接返回 401,简化上层写法。
pub fn ensure_password(plain: &str, hashed: &str) -> AppResult<()> {
    if verify_password(plain, hashed)? {
        Ok(())
    } else {
        Err(AppError::Unauthorized)
    }
}
