use ark_circom::{CircomCircuit, CircomConfig};
use std::path::Path;

use crate::snarkjs::*;

mod snarkjs;

mod core;
use core::*;

/// Proves a circuit with an existing witness and prover key.
///
/// The witness path can be either a JSON or binary file.
pub fn prove_with_existing_witness(
    r1cs_path: impl AsRef<Path>,
    wtns_path: impl AsRef<Path>,
    pkey_path: impl AsRef<Path>,
) -> SnarkjsOutput {
    // if wtns path ends with JSON, use `load_witness_json`, otherwise, use `load_witness`
    let wtns = if wtns_path.as_ref().ends_with(".json") {
        load_witness_json(wtns_path).expect("could not load witness JSON")
    } else {
        load_witness(wtns_path).expect("could not load witness")
    };

    let proving_key = load_proving_key(pkey_path).expect("could not load prover key");
    let r1cs = load_r1cs(r1cs_path).expect("could not load R1CS");

    // construct the circuit with explicit witness
    let circom = CircomCircuit {
        r1cs,
        witness: Some(wtns),
    };

    let public_inputs = circom
        .get_public_inputs()
        .expect("could not get public inputs, is witness computed?");
    let proof = prove_circuit(circom, &proving_key).unwrap();
    debug_assert!(
        matches!(verify(&proof, &public_inputs, &proving_key), Ok(true)),
        "proof is not accepted"
    );

    let snarkjs_proof = SnarkjsGroth16Proof::from(&proof);
    let snarkjs_public_inputs = SnarkjsPublicInputs::from_arkworks(public_inputs);

    SnarkjsOutput {
        proof: snarkjs_proof,
        public_signals: snarkjs_public_inputs,
    }
}

/// Proves a circuit with an existing witness and prover key.
///
/// The witness path can be either a JSON or binary file.
pub fn prove_with_setup(
    r1cs_path: impl AsRef<Path>,
    wasm_path: impl AsRef<Path>,
    pkey_path: impl AsRef<Path>,
    inputs: Vec<(impl ToString, impl Into<num_bigint::BigInt>)>,
) -> SnarkjsOutput {
    // if wtns path ends with JSON, use `load_witness_json`, otherwise, use `load_witness`
    let config = CircomConfig::new(wasm_path, r1cs_path).expect("could not create config");

    // construct the circuit with explicit witness
    let circom = compute_witness(config, inputs).expect("could not compute witness");

    let proving_key = load_proving_key(pkey_path).expect("could not load prover key");
    let public_inputs = circom
        .get_public_inputs()
        .expect("could not get public inputs, is witness computed?");
    let proof = prove_circuit(circom, &proving_key).expect("could not prove");
    debug_assert!(
        matches!(verify(&proof, &public_inputs, &proving_key), Ok(true)),
        "proof is not accepted"
    );

    let snarkjs_proof = SnarkjsGroth16Proof::from(&proof);
    let snarkjs_public_inputs = SnarkjsPublicInputs::from_arkworks(public_inputs);

    SnarkjsOutput {
        proof: snarkjs_proof,
        public_signals: snarkjs_public_inputs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CIRCUIT: &str = "mul30";

    fn check_snarkjs_output(snarkjs_out: &SnarkjsOutput, circuit_name: &str) -> eyre::Result<()> {
        std::fs::write(
            &format!("tests/res/arkworks_{}_proof.json", circuit_name),
            serde_json::to_string_pretty(&snarkjs_out.proof).unwrap(),
        )?;
        std::fs::write(
            &format!("tests/res/arkworks_{}_public.json", circuit_name),
            serde_json::to_string_pretty(&snarkjs_out.public_signals).unwrap(),
        )?;

        let output = snarkjs_verify_groth16(
            &format!("tests/res/{}_groth16_vkey.json", circuit_name),
            &format!("tests/res/arkworks_{}_public.json", circuit_name),
            &format!("tests/res/arkworks_{}_proof.json", circuit_name),
        )?;
        assert!(output.status.success());

        Ok(())
    }

    #[tokio::test]
    async fn test_arkworks_mul3_without_witness() -> eyre::Result<()> {
        let wasm_path = format!("tests/res/{}.wasm", CIRCUIT);
        let r1cs_path = format!("tests/res/{}.r1cs", CIRCUIT);
        let pkey_path = format!("tests/res/{}_groth16.zkey", CIRCUIT);

        // you can push same input few times, if its an array
        let inputs = vec![("in", 2), ("in", 4), ("in", 10)];

        let snarkjs_out = prove_with_setup(r1cs_path, wasm_path, pkey_path, inputs);
        check_snarkjs_output(&snarkjs_out, CIRCUIT)
    }

    #[tokio::test]
    async fn test_arkworks_mul3_with_witness() -> eyre::Result<()> {
        let wtns_path = format!("tests/res/{}.wtns", CIRCUIT);
        let r1cs_path = format!("tests/res/{}.r1cs", CIRCUIT);
        let pkey_path = format!("tests/res/{}_groth16.zkey", CIRCUIT);

        let snarkjs_out = prove_with_existing_witness(r1cs_path, wtns_path, pkey_path);
        check_snarkjs_output(&snarkjs_out, CIRCUIT)
    }
}
