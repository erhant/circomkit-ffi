use ark_circom::CircomCircuit;
use std::path::Path;

use crate::snarkjs::{SnarkjsGroth16Proof, SnarkjsOutput, SnarkjsPublicSignals};

mod snarkjs;

mod core;
pub use core::*;

/// Proves a circuit with an existing witness and prover key.
///
/// The witness path can be either a JSON or binary file.
pub fn prove_with_witness_with_setup(
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

    let prover_key = load_prover_key(pkey_path).expect("could not load prover key");
    let r1cs = load_r1cs(r1cs_path).expect("could not load R1CS");

    // construct the circuit with explicit witness
    let circom = CircomCircuit {
        r1cs,
        witness: Some(wtns),
    };

    let public_signals = circom
        .get_public_inputs()
        .expect("could not get public inputs, is witness computed?");
    let proof = prove_circuit(circom, &prover_key).unwrap();

    let snarkjs_proof = SnarkjsGroth16Proof::from(&proof);
    let snarkjs_public_signals = SnarkjsPublicSignals::from_arkworks(public_signals);

    SnarkjsOutput {
        proof: snarkjs_proof,
        public_signals: snarkjs_public_signals,
    }
}
