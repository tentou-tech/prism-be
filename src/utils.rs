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
            98, 101, 45, 105, 100, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 205, 6, 179,
            231, 3, 76, 200, 5, 158, 119, 60, 31, 63, 96, 150, 188, 97, 174, 136, 110, 210, 53,
            222, 106, 162, 120, 205, 160, 74, 152, 63, 136, 39, 204, 149, 31, 49, 186, 254, 163,
            105, 119, 70, 216, 240, 206, 234, 70, 36, 49, 211, 189, 196, 150, 223, 54, 204, 117,
            62, 134, 88, 243, 213, 15, 4, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 2, 188, 25, 216, 87,
            50, 192, 25, 253, 30, 72, 103, 228, 148, 202, 20, 60, 85, 72, 64, 48, 185, 102, 46,
            240, 145, 16, 218, 72, 252, 250, 114, 14, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let payload_base64 = payload.to_base64();
        println!("payload_base64: {}", payload_base64);

        let key = SigningKey::new_cosmos_adr36();
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
