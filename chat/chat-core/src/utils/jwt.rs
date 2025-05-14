use crate::User;
use serde_json::Value;

use josekit::{
    JoseError,
    jws::{
        EdDSA, JwsHeader,
        alg::eddsa::{EddsaJwsSigner, EddsaJwsVerifier},
    },
    jwt::{self, JwtPayload},
};

#[allow(unused)]
const JWT_DURATION: u64 = 60 * 60 * 24 * 7;
#[allow(unused)]
const JWT_ISS: &str = "chat-server";
#[allow(unused)]
const JWT_AUD: &str = "chat_web";

#[allow(unused)]
pub struct EncodingKey(EddsaJwsSigner);

#[allow(unused)]
pub struct DecodingKey(EddsaJwsVerifier);

#[allow(unused)]
impl EncodingKey {
    pub fn load(pem: &str) -> Result<Self, JoseError> {
        let private_key = pem.as_bytes();
        let signer = EdDSA.signer_from_pem(private_key)?;
        Ok(Self(signer))
    }

    pub fn sign(&self, user: impl Into<User>) -> Result<String, JoseError> {
        let mut header = JwsHeader::new();
        header.set_token_type("JWT");
        let mut payload = JwtPayload::new();
        let user_json = serde_json::to_string(&user.into()).unwrap();
        payload.set_claim("custom", Some(Value::String(user_json)));
        payload.set_issuer(JWT_ISS);
        payload.set_audience(vec![JWT_AUD]);

        jwt::encode_with_signer(&payload, &header, &self.0)
    }
}

#[allow(unused)]
impl DecodingKey {
    pub fn load(pem: &str) -> Result<Self, JoseError> {
        let verifier = EdDSA.verifier_from_pem(pem)?;
        Ok(Self(verifier))
    }

    pub fn verify(&self, token: &str) -> Result<User, JoseError> {
        let (payload, _) = jwt::decode_with_verifier(token, &self.0)?;
        let user_json = payload.claim("custom").unwrap().as_str().unwrap();
        let user: User = serde_json::from_str(user_json).unwrap();
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::User;
    use anyhow::Result;

    #[tokio::test]
    async fn jwt_sign_verify_should_work() -> Result<()> {
        let encoding_pem = include_str!("../../fixtures/encoding.pem");
        let decoding_pem = include_str!("../../fixtures/decoding.pem");
        let ek = EncodingKey::load(encoding_pem)?;
        let dk = DecodingKey::load(decoding_pem)?;

        let user = User::new(1, "test", "test");

        let token = ek.sign(user.clone())?;
        let user2 = dk.verify(&token)?;

        assert_eq!(user, user2);
        Ok(())
    }
}
