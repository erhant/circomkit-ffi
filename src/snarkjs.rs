use serde::{Deserialize, Serialize};

/// A proof object, similar to how SnarkJS exports it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnarkjsProof {
    /// A point in G1
    pub pi_a: [String; 2],
    /// A point in G2
    pub pi_b: [[String; 2]; 2],
    /// A point in G1
    pub pi_c: [String; 2],
    /// Protocol name, should be "groth16"
    pub protocol: String,
    /// Curve name, should be "bls12381" or "bn254"
    pub curve: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnarkjsPublicSignals(pub Vec<String>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnarkjsOutput {
    pub proof: SnarkjsProof,
    pub public_signals: SnarkjsPublicSignals,
}

/// Execute the following command.
///
/// ```sh
/// snarkjs g16v [verification_key.json] [public.json] [proof.json]
/// ```
///
/// If the process fails, this may panic.
///
/// Requires `snarkjs` to be installed globally.
#[inline]
pub fn snarkjs_verify_groth16(
    verification_key_path: &str,
    proof_path: &str,
    public_signals_path: &str,
) -> std::io::Result<std::process::Output> {
    std::process::Command::new("snarkjs")
        .args(&[
            "g16v", // short for "groth16 verify"
            verification_key_path,
            proof_path,
            public_signals_path,
        ])
        .output()
}
