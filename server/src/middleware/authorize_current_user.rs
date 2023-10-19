use biscuit::jwa::{ContentEncryptionAlgorithm, KeyManagementAlgorithm};
use biscuit::jwe::Compact;
use biscuit::jwk::JWK;
use biscuit::Empty;
use cookie::Cookie;
use hkdf::Hkdf;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::{env, str};

#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub id: uuid::Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
struct JWTPayload {
    email: String,
    exp: i32,
    iat: i32,
    jti: String,
    name: String,
    picture: String,
    ppid: uuid::Uuid,
    sub: String,
}

const KEY_INFO: &[u8; 32] = b"Auth.js Generated Encryption Key";

fn get_derived_encryption_key() -> JWK<Empty> {
    let key_secret = env::var("AUTH_SECRET").expect("Missing AUTH_SECRET");
    let hk = Hkdf::<Sha256>::new(None, key_secret.as_bytes());

    let mut okm = [0u8; 32];
    hk.expand(KEY_INFO, &mut okm).expect("Expanded key");

    JWK::new_octet_key(okm.as_slice(), Default::default())
}

pub fn authorize_current_user(cookie_str: &str) -> Option<CurrentUser> {
    for cookie in Cookie::split_parse(cookie_str) {
        if let Some(cookie) = cookie.ok() {
            if cookie.name() == "next-auth.session-token" {
                let token = cookie.value();
                let key = get_derived_encryption_key();
                let compact = Compact::<Vec<u8>, Empty>::new_encrypted(&token);
                let res = compact.decrypt(
                    &key,
                    KeyManagementAlgorithm::DirectSymmetricKey,
                    ContentEncryptionAlgorithm::A256GCM,
                );

                return match res {
                    Ok(decrypted) => {
                        let jwt_payload = decrypted.payload().ok();
                        if jwt_payload.is_none() {
                            return None;
                        }
                        let jwt_parse_result =
                            serde_json::from_slice::<JWTPayload>(jwt_payload.unwrap());

                        match jwt_parse_result {
                            Ok(jwt) => Some(CurrentUser { id: jwt.ppid }),
                            Err(error) => {
                                println!("authorize_current_user: {:?}", error);
                                None
                            }
                        }
                    }
                    Err(error) => {
                        println!("authorize_current_user: {:?}", error);
                        None
                    }
                };
            }
        }
    }
    None
}
