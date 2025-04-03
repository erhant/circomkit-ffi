use std::{ffi::OsStr, path::Path};

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
#[serde(rename_all = "camelCase")]
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
pub fn snarkjs_verify_groth16(
    verification_key_path: impl AsRef<OsStr>,
    proof_path: impl AsRef<OsStr>,
    public_signals_path: impl AsRef<OsStr>,
) -> std::io::Result<std::process::Output> {
    std::process::Command::new("snarkjs")
        .args([
            OsStr::new("g16v"), // short for "groth16 verify"
            verification_key_path.as_ref(),
            public_signals_path.as_ref(),
            proof_path.as_ref(),
        ])
        .output()
}

/// Checks the output of `snarkjs` by verifying the proof.
///
/// - If the verification fails, this function will panic.
/// - The proof is saved as `{prefix}_{circuit_name}_proof.json`
/// - The public signals are saved as `{prefix}_{circuit_name}_public.json`
/// - The verification key expected from the disk as `groth16_vkey.json`
pub fn check_snarkjs_output(
    snarkjs_out: &SnarkjsOutput,
    dir: &Path,
    circuit_name: &str,
    prefix: &str,
) -> eyre::Result<()> {
    let proof_output_path = dir
        .join(format!("{}_{}_proof", prefix, circuit_name))
        .with_extension("json");
    let public_output_path = dir
        .join(format!("{}_{}_public", prefix, circuit_name))
        .with_extension("json");
    let vkey_path = dir.join("groth16_vkey").with_extension("json");

    std::fs::write(
        &proof_output_path,
        serde_json::to_string_pretty(&snarkjs_out.proof).unwrap(),
    )?;
    std::fs::write(
        &public_output_path,
        serde_json::to_string_pretty(&snarkjs_out.public_signals).unwrap(),
    )?;
    let output = snarkjs_verify_groth16(&vkey_path, &proof_output_path, &public_output_path)?;
    assert!(output.status.success());

    Ok(())
}
