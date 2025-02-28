use serde::{Deserialize, Serialize};

/// A Groth16 proof object, similar to how SnarkJS exports it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnarkjsGroth16Proof {
    /// A point in G1
    pub pi_a: [String; 2],
    /// A point in G2
    pub pi_b: [[String; 2]; 2],
    /// A point in G1
    pub pi_c: [String; 2],
    /// Protocol name, should be "groth16"
    pub protocol: String,
    /// Curve name, should be "bls12381" or "bn254 / bn128 / altbn128"
    pub curve: String,
}

impl std::fmt::Display for SnarkjsGroth16Proof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

/// Public signals object, similar to how SnarkJS exports it.
///
/// Each signal is a string that should be parsed into a `BigInt`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnarkjsPublicInputs(pub Vec<String>);

impl std::fmt::Display for SnarkjsPublicInputs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnarkjsOutput {
    pub proof: SnarkjsGroth16Proof,
    pub public_signals: SnarkjsPublicInputs,
}

impl std::fmt::Display for SnarkjsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Proof: {}\nPublic Signals: {}",
            serde_json::to_string_pretty(&self.proof).unwrap(),
            serde_json::to_string_pretty(&self.public_signals).unwrap()
        )
    }
}

/// Executes the following command:
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
        .args([
            "g16v", // short for "groth16 verify"
            verification_key_path,
            proof_path,
            public_signals_path,
        ])
        .output()
}
