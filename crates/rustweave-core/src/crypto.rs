// SPDX-License-Identifier: MIT OR Apache-2.0
//! Hashing, message authentication, key derivation, and symmetric encryption helpers.

use aes_gcm::aead::consts::U12;
use aes_gcm::aead::{Aead, Payload};
use aes_gcm::aes::Aes192;
use aes_gcm::{Aes128Gcm, Aes256Gcm, AesGcm, KeyInit, Nonce};
use hkdf::Hkdf;
use hmac::{Hmac, Mac};
use md5::Digest;
use rand::rngs::OsRng;
use rand::RngCore;
use sha1::Sha1;
use sha2::{Sha256, Sha384, Sha512};
use subtle::ConstantTimeEq;

/// AES-GCM with a 192-bit key and 96-bit nonce.
type Aes192Gcm = AesGcm<Aes192, U12>;

use crate::error::DwError;
use crate::number::Number;
use crate::value::Value;

/// Converts a content argument to bytes.
///
/// Strings use their UTF-8 bytes, binaries are used directly, and numbers become a single
/// byte: truncate toward zero, then take the value modulo 256.
pub(crate) fn bytes(value: &Value, label: &str) -> Result<Vec<u8>, DwError> {
    match value.unwrap_formatted() {
        Value::String(text) => Ok(text.as_bytes().to_vec()),
        Value::Binary(bytes) => Ok(bytes.clone()),
        Value::Number(number) => {
            let truncated = if number.is_negative() {
                number.ceil()
            } else {
                number.floor()
            };
            let mut remainder = truncated
                .rem(&Number::from_i64(256))
                .ok_or_else(|| DwError::new("invalid numeric content"))?;
            if remainder.is_negative() {
                remainder = remainder.add(&Number::from_i64(256));
            }
            let byte = remainder
                .to_i64()
                .ok_or_else(|| DwError::new("invalid numeric content"))?;
            Ok(vec![byte as u8])
        }
        other => Err(DwError::new(format!(
            "{label} must be a String or Binary, found {}",
            other.type_name()
        ))),
    }
}

/// Renders bytes as lowercase hexadecimal text.
fn hex_lower(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

fn digest_bytes(content: &[u8], algorithm: &str) -> Option<Vec<u8>> {
    match algorithm {
        "MD5" => Some(md5::Md5::digest(content).to_vec()),
        "SHA-1" | "SHA1" => Some(Sha1::digest(content).to_vec()),
        "SHA-256" => Some(Sha256::digest(content).to_vec()),
        "SHA-384" => Some(Sha384::digest(content).to_vec()),
        "SHA-512" => Some(Sha512::digest(content).to_vec()),
        _ => None,
    }
}

fn hmac_bytes(key: &[u8], content: &[u8], algorithm: &str) -> Option<Vec<u8>> {
    fn finish<M: Mac>(mut mac: M, content: &[u8]) -> Vec<u8> {
        mac.update(content);
        mac.finalize().into_bytes().to_vec()
    }
    match algorithm {
        "HmacSHA1" => Some(finish(
            <Hmac<Sha1> as Mac>::new_from_slice(key).ok()?,
            content,
        )),
        "HmacSHA256" => Some(finish(
            <Hmac<Sha256> as Mac>::new_from_slice(key).ok()?,
            content,
        )),
        "HmacSHA384" => Some(finish(
            <Hmac<Sha384> as Mac>::new_from_slice(key).ok()?,
            content,
        )),
        "HmacSHA512" => Some(finish(
            <Hmac<Sha512> as Mac>::new_from_slice(key).ok()?,
            content,
        )),
        _ => None,
    }
}

/// Computes an MD5 digest and returns lowercase hexadecimal text.
pub fn md5(content: &Value) -> Result<Value, DwError> {
    let content = bytes(content, "content")?;
    Ok(Value::String(hex_lower(
        &digest_bytes(&content, "MD5").unwrap_or_default(),
    )))
}

/// Computes a SHA-1 digest and returns lowercase hexadecimal text.
pub fn sha1(content: &Value) -> Result<Value, DwError> {
    let content = bytes(content, "content")?;
    Ok(Value::String(hex_lower(
        &digest_bytes(&content, "SHA-1").unwrap_or_default(),
    )))
}

/// Computes a digest and returns its raw bytes.
pub fn hash_with(content: &Value, algorithm: &str) -> Result<Value, DwError> {
    let content = bytes(content, "content")?;
    let digest = digest_bytes(&content, algorithm)
        .ok_or_else(|| DwError::new(format!("unknown digest algorithm '{algorithm}'")))?;
    Ok(Value::Binary(digest))
}

/// Computes an HMAC and returns lowercase hexadecimal text.
pub fn hmac_with(key: &Value, content: &Value, algorithm: &str) -> Result<Value, DwError> {
    let key = bytes(key, "key")?;
    let content = bytes(content, "content")?;
    let mac = hmac_bytes(&key, &content, algorithm)
        .ok_or_else(|| DwError::new(format!("unknown HMAC algorithm '{algorithm}'")))?;
    Ok(Value::String(hex_lower(&mac)))
}

/// Computes an HMAC and returns its raw bytes.
pub fn hmac_binary(key: &Value, content: &Value, algorithm: &str) -> Result<Value, DwError> {
    let key = bytes(key, "key")?;
    let content = bytes(content, "content")?;
    let mac = hmac_bytes(&key, &content, algorithm)
        .ok_or_else(|| DwError::new(format!("unknown HMAC algorithm '{algorithm}'")))?;
    Ok(Value::Binary(mac))
}

fn aad_bytes(aad: Option<&Value>) -> Result<Vec<u8>, DwError> {
    match aad {
        Some(value) if !value.is_null() => bytes(value, "aad"),
        _ => Ok(Vec::new()),
    }
}

fn validate_iv(iv: &Value) -> Result<Vec<u8>, DwError> {
    let iv = bytes(iv, "iv")?;
    if iv.len() != 12 {
        return Err(DwError::new("iv must be exactly 12 bytes"));
    }
    Ok(iv)
}

/// Encrypts content with AES-GCM, appending the 16-byte tag.
pub fn aes_gcm_encrypt(
    content: &Value,
    key: &Value,
    iv: &Value,
    aad: Option<&Value>,
) -> Result<Value, DwError> {
    let content = bytes(content, "content")?;
    let key = bytes(key, "key")?;
    let iv = validate_iv(iv)?;
    let aad = aad_bytes(aad)?;
    let nonce = Nonce::from_slice(&iv);
    let payload = Payload {
        msg: &content,
        aad: &aad,
    };
    let encrypted = match key.len() {
        16 => Aes128Gcm::new_from_slice(&key)
            .map_err(|_| DwError::new("invalid AES key"))?
            .encrypt(nonce, payload),
        24 => Aes192Gcm::new_from_slice(&key)
            .map_err(|_| DwError::new("invalid AES key"))?
            .encrypt(nonce, payload),
        32 => Aes256Gcm::new_from_slice(&key)
            .map_err(|_| DwError::new("invalid AES key"))?
            .encrypt(nonce, payload),
        _ => return Err(DwError::new("AES key must be 16, 24, or 32 bytes")),
    };
    encrypted
        .map(Value::Binary)
        .map_err(|_| DwError::new("encryption failed"))
}

/// Decrypts ciphertext with its appended AES-GCM tag.
pub fn aes_gcm_decrypt(
    content: &Value,
    key: &Value,
    iv: &Value,
    aad: Option<&Value>,
) -> Result<Value, DwError> {
    let content = bytes(content, "content")?;
    if content.len() < 16 {
        return Err(DwError::new("ciphertext must include a 16-byte tag"));
    }
    let key = bytes(key, "key")?;
    let iv = validate_iv(iv)?;
    let aad = aad_bytes(aad)?;
    let nonce = Nonce::from_slice(&iv);
    let payload = Payload {
        msg: &content,
        aad: &aad,
    };
    let decrypted = match key.len() {
        16 => Aes128Gcm::new_from_slice(&key)
            .map_err(|_| DwError::new("invalid AES key"))?
            .decrypt(nonce, payload),
        24 => Aes192Gcm::new_from_slice(&key)
            .map_err(|_| DwError::new("invalid AES key"))?
            .decrypt(nonce, payload),
        32 => Aes256Gcm::new_from_slice(&key)
            .map_err(|_| DwError::new("invalid AES key"))?
            .decrypt(nonce, payload),
        _ => return Err(DwError::new("AES key must be 16, 24, or 32 bytes")),
    };
    decrypted
        .map(Value::Binary)
        .map_err(|_| DwError::new("decryption failed"))
}

/// Produces secure random bytes of the requested length.
pub fn random_bytes(length: i64) -> Result<Value, DwError> {
    if length < 0 {
        return Err(DwError::new("randomBytes length must not be negative"));
    }
    let length = usize::try_from(length).map_err(|_| DwError::new("length is too large"))?;
    let mut buffer = vec![0u8; length];
    OsRng.fill_bytes(&mut buffer);
    Ok(Value::Binary(buffer))
}

/// Derives key material with HKDF using HMAC-SHA-256.
pub fn hkdf_sha256(
    ikm: &Value,
    salt: Option<&Value>,
    info: Option<&Value>,
    length: i64,
) -> Result<Value, DwError> {
    let ikm = bytes(ikm, "ikm")?;
    let salt = match salt {
        Some(value) if !value.is_null() => bytes(value, "salt")?,
        _ => vec![0u8; 32],
    };
    let info = match info {
        Some(value) if !value.is_null() => bytes(value, "info")?,
        _ => Vec::new(),
    };
    if !(0..=8160).contains(&length) {
        return Err(DwError::new("hkdf length must be between 0 and 8160"));
    }
    let length = usize::try_from(length).map_err(|_| DwError::new("length is too large"))?;
    let hkdf = Hkdf::<Sha256>::new(Some(&salt), &ikm);
    let mut output = vec![0u8; length];
    hkdf.expand(&info, &mut output)
        .map_err(|_| DwError::new("hkdf expansion failed"))?;
    Ok(Value::Binary(output))
}

/// Derives key material with PBKDF2 using HMAC-SHA-256.
pub fn pbkdf2_sha256(
    password: &Value,
    salt: &Value,
    iterations: i64,
    length: i64,
) -> Result<Value, DwError> {
    let password = bytes(password, "password")?;
    let salt = bytes(salt, "salt")?;
    if iterations <= 0 || iterations > u32::MAX as i64 {
        return Err(DwError::new("iterations must be a positive integer"));
    }
    if length <= 0 {
        return Err(DwError::new("length must be a positive integer"));
    }
    let length = usize::try_from(length).map_err(|_| DwError::new("length is too large"))?;
    let mut output = vec![0u8; length];
    pbkdf2::pbkdf2_hmac::<Sha256>(&password, &salt, iterations as u32, &mut output);
    Ok(Value::Binary(output))
}

/// Compares two byte sequences without an early exit on the first difference.
pub fn constant_time_equals(left: &Value, right: &Value) -> Result<Value, DwError> {
    let left = bytes(left, "left")?;
    let right = bytes(right, "right")?;
    if left.len() != right.len() {
        return Ok(Value::Bool(false));
    }
    let equal = left.ct_eq(right.as_slice()).unwrap_u8() == 1;
    Ok(Value::Bool(equal))
}
