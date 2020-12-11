// To generate text fixture:
// cargo run --example issue > examples/vc.jsonld

fn main() {
    let key_str = include_str!("../tests/ed25519-2020-10-18.json");
    let key: ssi::jwk::JWK = serde_json::from_str(key_str).unwrap();
    let vc = serde_json::json!({
        "@context": ["https://www.w3.org/2018/credentials/v1"],
        "type": "VerifiableCredential",
        "issuer": key.to_did().unwrap(),
        "issuanceDate": ssi::ldp::now_ms(),
        "credentialSubject": {
            "id": "urn:uuid:".to_string() + &uuid::Uuid::new_v4().to_string()
        }
    });
    let mut vc: ssi::vc::Credential = serde_json::from_value(vc).unwrap();
    let mut proof_options = ssi::vc::LinkedDataProofOptions::default();
    let verification_method = key.to_verification_method().unwrap();
    proof_options.verification_method = Some(verification_method);
    let proof = vc.generate_proof(&key, &proof_options).unwrap();
    vc.add_proof(proof);
    let result = vc.verify(None);
    if result.errors.len() > 0 {
        panic!("verify failed: {:#?}", result);
    }
    let stdout_writer = std::io::BufWriter::new(std::io::stdout());
    serde_json::to_writer_pretty(stdout_writer, &vc).unwrap();
}