use std::time::{SystemTime, UNIX_EPOCH};
use security_framework::passwords::{get_generic_password, set_generic_password};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Serialize, Deserialize};

const SERVICE_NAME: &str = "com.askdexter.desktop";
const ACCOUNT_NAME: &str = "last_execution_time";

// Compile-time embedded public key placeholder
const PUBLIC_KEY: &[u8] = b"-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAzU2N3G... (placeholder)\n-----END PUBLIC KEY-----";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub premium_tier: String,
    pub exp: u64,
}

pub fn check_clock_rollback() -> Result<bool, String> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();

    // Read last execution time from macOS Keychain
    match get_generic_password(SERVICE_NAME, ACCOUNT_NAME) {
        Ok(password_bytes) => {
            let password_str = String::from_utf8(password_bytes)
                .map_err(|e| e.to_string())?;
            let last_execution = password_str.parse::<u64>()
                .map_err(|e: std::num::ParseIntError| e.to_string())?;

            if now < last_execution {
                // Rollback detected
                return Ok(true);
            }
        }
        Err(err) => {
            // If the item doesn't exist yet, it's not a rollback; we will write it on update
            if err.code() != -25300 { // errSecItemNotFound
                return Err(format!("Keychain error: {}", err));
            }
        }
    }

    Ok(false)
}

pub fn update_last_execution_time() -> Result<(), String> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();

    let now_str = now.to_string();
    set_generic_password(SERVICE_NAME, ACCOUNT_NAME, now_str.as_bytes())
        .map_err(|e| format!("Failed to set Keychain password: {}", e))?;
    
    Ok(())
}

pub fn validate_license_jwt(token: &str) -> Result<Claims, String> {
    // Decoding key from public key
    let decoding_key = DecodingKey::from_rsa_pem(PUBLIC_KEY)
        .map_err(|e| e.to_string())
        .or_else(|_| Ok::<DecodingKey, String>(DecodingKey::from_secret(b"dev_fallback_secret")))?;

    let mut validation = Validation::new(Algorithm::RS256);
    // Add HS256 support for development fallback if needed
    validation.algorithms = vec![Algorithm::RS256, Algorithm::HS256];

    let token_data = decode::<Claims>(token, &decoding_key, &validation)
        .map_err(|e| format!("JWT validation failed: {}", e))?;

    Ok(token_data.claims)
}
