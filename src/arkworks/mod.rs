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
    let wtns = if wtns_path.as_ref().to_string_lossy().ends_with(".json") {
        load_witness_json(wtns_path).expect("could not load witness JSON")
    } else {
        load_witness(wtns_path).expect("could not load witness")
    };

    let proving_key = load_proving_key(pkey_path).expect("could not load prover key");
    let mut r1cs = load_r1cs(r1cs_path).expect("could not load R1CS");

    // disable the wire mapping, otherwise you may get out-of-index errors; this is how Arkworks does it
    // for witnesses generated via WASM runtime, see: https://github.com/arkworks-rs/circom-compat/blob/master/src/circom/builder.rs#L82
    r1cs.wire_mapping = None;

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
        verify(&proof, &public_inputs, &proving_key).is_ok_and(|b| b),
        "proof is not accepted"
    );

    let snarkjs_proof = SnarkjsGroth16Proof::from(&proof);
    let snarkjs_public_inputs = SnarkjsPublicInputs::from_arkworks(public_inputs);

    SnarkjsOutput {
        proof: snarkjs_proof,
        public_signals: snarkjs_public_inputs,
    }
}

/// Proves a circuit with an a runtime-computed witness (via WASM) and prover key.
pub fn prove_with_computed_witness(
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

    const CIRCUIT: &str = "multiplier_30";

    #[tokio::test]
    async fn test_arkworks_with_computed_witness() -> eyre::Result<()> {
        let dir = Path::new("example/build").join(CIRCUIT);
        let wasm_path = dir
            .join(format!("{}_js", CIRCUIT))
            .join(CIRCUIT)
            .with_extension("wasm");
        let r1cs_path = dir.join(CIRCUIT).with_extension("r1cs");
        let pkey_path = dir.join("groth16_pkey").with_extension("zkey");

        // you can push same input few times, if its an array
        let inputs = vec![("in", 2); 300]; // TODO: !!!

        let snarkjs_out = prove_with_computed_witness(r1cs_path, wasm_path, pkey_path, inputs);
        check_snarkjs_output(&snarkjs_out, &dir, CIRCUIT, "arkworks")
    }

    #[tokio::test]
    async fn test_arkworks_with_existing_witness() -> eyre::Result<()> {
        let dir = Path::new("example/build").join(CIRCUIT);
        let r1cs_path = dir.join(CIRCUIT).with_extension("r1cs");
        let wtns_path = dir
            .join("default") // input name
            .join("witness")
            .with_extension("wtns");
        let pkey_path = dir.join("groth16_pkey").with_extension("zkey");

        let snarkjs_out = prove_with_existing_witness(r1cs_path, wtns_path, pkey_path);
        check_snarkjs_output(&snarkjs_out, &dir, CIRCUIT, "arkworks")
    }
}
