use std::collections::HashSet;

use prism_client::{Signature, SignatureBundle, VerifyingKey};
use prism_keys::CryptoAlgorithm;
use prism_serde::base64::FromBase64;

// Parse a signature bundle from a verifying key and a signature in hex string
// The verifying key algorithm is CosmosAdr36 and the signature algorithm is Secp256k1
pub fn parse_signature_bundle(
    verifying_key: String,
    signature: String,
) -> anyhow::Result<SignatureBundle> {
    let verifying_key_bytes = Vec::<u8>::from_base64(verifying_key)?;
    let signature_bytes = Vec::<u8>::from_base64(signature)?;

    let verifying_key =
        VerifyingKey::from_algorithm_and_bytes(CryptoAlgorithm::CosmosAdr36, &verifying_key_bytes)?;
    let signature =
        Signature::from_algorithm_and_bytes(CryptoAlgorithm::Secp256k1, &signature_bytes)?;

    Ok(SignatureBundle::new(verifying_key, signature))
}

pub fn parse_cosmos_adr36_verifying_key(verifying_key: String) -> anyhow::Result<VerifyingKey> {
    let verifying_key_bytes = Vec::<u8>::from_base64(verifying_key)?;
    VerifyingKey::from_algorithm_and_bytes(CryptoAlgorithm::CosmosAdr36, &verifying_key_bytes)
}

pub fn remove_duplicates(vec: Vec<String>) -> Vec<String> {
    let set: HashSet<String> = vec.into_iter().collect();
    set.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use prism_keys::SigningKey;
    use prism_serde::base64::ToBase64;

    use super::*;

    #[test]
    fn test_parse_signature_bundle_simple() {
        let message = String::from("123");
        let verifying_key = "AzInFFk+Ht0PA40u/T0L+3qpPk+EuHBq8mqJr974Asg1";
        let signature = "jU9Q9lnY5gAO51dpt+8d7FpngPLlV6S9S/YBM9vve2JHTkxfMvQch1+hq9hdAD8XiJ69JFsaNW3zu3bTmCEOvA==";
        let signature_bundle =
            parse_signature_bundle(verifying_key.to_string(), signature.to_string()).unwrap();
        assert_eq!(signature_bundle.verifying_key.to_string(), verifying_key);
        assert_eq!(signature_bundle.signature.algorithm(), CryptoAlgorithm::Secp256k1);

        signature_bundle
            .verifying_key
            .verify_signature(message, &signature_bundle.signature)
            .unwrap();
    }

    #[test]
    fn test_parse_signature_bundle_with_message() {
        let payload: Vec<u8> = vec![
            6, 0, 0, 0, 0, 0, 0, 0, 116, 105, 101, 110, 110, 118, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0,
            0, 116, 105, 101, 110, 110, 118, 11, 0, 0, 0, 0, 0, 0, 0, 112, 114, 105, 115, 109, 45,
            98, 101, 45, 105, 100, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 130, 161, 91,
            121, 161, 30, 86, 167, 121, 204, 144, 113, 156, 206, 1, 32, 89, 222, 222, 138, 246,
            134, 35, 88, 31, 4, 88, 125, 231, 238, 78, 251, 13, 71, 124, 104, 92, 200, 238, 43,
            141, 72, 189, 240, 209, 245, 35, 74, 90, 163, 4, 8, 142, 152, 135, 203, 51, 211, 76,
            64, 119, 103, 74, 5, 4, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 2, 77, 69, 143, 36, 190, 58,
            50, 58, 85, 22, 62, 119, 148, 28, 48, 123, 102, 250, 91, 71, 6, 203, 96, 244, 120, 155,
            55, 244, 201, 213, 159, 244, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let priv_key = "0SKFp7ehnNwJ2xycIoLc+VM66oKydIWWVGbJigvdvMA=";
        let priv_key_bytes = Vec::<u8>::from_base64(priv_key).unwrap();
        let key =
            SigningKey::from_algorithm_and_bytes(CryptoAlgorithm::CosmosAdr36, &priv_key_bytes)
                .unwrap();
        let verifying_key = key.verifying_key().to_string();
        println!("verifying_key: {}", verifying_key);

        let payload_base64 = payload.to_base64();
        println!("payload_base64: {}", payload_base64);

        let verifying_key = key.verifying_key().to_string();
        println!("verifying_key: {}", verifying_key);
        // let message = String::from("123");

        let signature = key.sign(&payload).unwrap().to_bytes().to_base64();
        println!("signature: {}", signature);

        let signature_bundle =
            parse_signature_bundle(verifying_key.to_string(), signature.to_string()).unwrap();
        assert_eq!(signature_bundle.verifying_key.to_string(), verifying_key);
        assert_eq!(signature_bundle.signature.algorithm(), CryptoAlgorithm::Secp256k1);

        signature_bundle
            .verifying_key
            .verify_signature(payload, &signature_bundle.signature)
            .unwrap();
    }
}
