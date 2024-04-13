/*
* @author Jakob Grätz, Johannes Schießl | @jakobgraetz, @johannesschiessl
* @edition 10/04/2024 DD/MM/YYYY
* @version v0.0.1
* @description Rust file for authentication utilities.
*/

extern crate argon2;
use argon2::{self, Config, ThreadMode, Variant, Version};

// Hash password
pub fn hash_password (password: String) -> Result<(String, String), argon2::Error> {
    // Generates a cryptographically secure salt string
    let salt = argon2::generate_salt();

    // config for hashing the password
    let config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: 65536,
        time_cost: 10,
        lanes: 4,
        thread_mode: ThreadMode::Parallel,
        secret: &[],
        ad: &[],
        hash_length: 32,
    };

    let hash = argon2::hash_encoded(password.as_bytes(), &salt, &config)?;

    Ok(hash, hex::encode(&salt))
}
// Hash API Key