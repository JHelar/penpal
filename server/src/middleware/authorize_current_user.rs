use biscuit::jwa::{ContentEncryptionAlgorithm, EncryptionOptions, KeyManagementAlgorithm};
use biscuit::jwe::Compact;
use biscuit::jwk::JWK;
use biscuit::Empty;
use cookie::Cookie;
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Clone)]
pub struct CurrentUser {
    pub id: uuid::Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    name: String,
    email: String,
    picture: String,
    sub: String,
    ppid: uuid::Uuid,
}

pub fn authorize_current_user(cookie_str: &str) -> Option<CurrentUser> {
    for cookie in Cookie::split_parse(cookie_str) {
        if let Some(cookie) = cookie.ok() {
            if cookie.name() == "next-auth.session-token" {
                let token = cookie.value();
                let key: JWK<Empty> = JWK::new_octet_key(&vec![0; 256 / 8], Default::default());

                let compact = Compact::<Vec<u8>, Empty>::new_encrypted(&token);
                let res = compact.decrypt(
                    &key,
                    KeyManagementAlgorithm::A256GCMKW,
                    ContentEncryptionAlgorithm::A256GCM,
                );

                println!("res: {:?}", res);

                // let payload: Vec<u8> = compact.part(1).unwrap();
                // println!("payload: {:?}", payload);

                return None;
            }
        }
    }
    None
}
