use ark_bn254::Bn254;
use ark_ff::Field;
use ark_groth16::Proof;
use eyre::Context;
use serde::Serialize;

#[derive(Serialize)]
struct SnarkjsProof {
    pi_a: [String; 2],
    pi_b: [[String; 2]; 2],
    pi_c: [String; 2],
    protocol: String,
}

/// Exports proof as a JSON object.
pub fn export_proof(proof: &Proof<Bn254>) -> eyre::Result<String> {
    let obj = SnarkjsProof {
        pi_a: [proof.a.x.to_string(), proof.a.y.to_string()],
        pi_b: [
            [proof.b.x.c0.to_string(), proof.b.x.c1.to_string()],
            [proof.b.y.c0.to_string(), proof.b.y.c1.to_string()],
        ],
        pi_c: [proof.c.x.to_string(), proof.c.y.to_string()],
        protocol: "groth16".to_string(),
    };

    serde_json::to_string(&obj).wrap_err("could not serialize proof")
}

/// Exports public signals as a JSON array of string bigints.
pub fn export_public_signals<F: Field>(pubs: &Vec<F>) -> Result<String, serde_json::Error> {
    let signal_strings = pubs.iter().map(|s| s.to_string()).collect::<Vec<String>>();

    serde_json::to_string(&signal_strings)
}
