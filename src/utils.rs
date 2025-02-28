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
    use super::*;

    #[test]
    fn test_parse_signature_bundle() {
        let verifying_key = "AzInFFk+Ht0PA40u/T0L+3qpPk+EuHBq8mqJr974Asg1";
        let signature = "jU9Q9lnY5gAO51dpt+8d7FpngPLlV6S9S/YBM9vve2JHTkxfMvQch1+hq9hdAD8XiJ69JFsaNW3zu3bTmCEOvA==";
        let signature_bundle =
            parse_signature_bundle(verifying_key.to_string(), signature.to_string()).unwrap();
        assert_eq!(signature_bundle.verifying_key.to_string(), verifying_key);
        assert_eq!(signature_bundle.signature.algorithm(), CryptoAlgorithm::Secp256k1);
    }
}
