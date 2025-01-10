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
