use std::path::Path;

use crate::{SnarkjsOutput, SnarkjsProof, SnarkjsPublicSignals};

mod snarkjs;

mod core;
pub use core::*;

pub fn prove_with_witness_with_setup(
    r1cs_path: impl AsRef<Path>,
    wtns_path: impl AsRef<Path>,
    pkey_path: impl AsRef<Path>,
) -> SnarkjsOutput {
    // you can push same input few times, if its an array
    let circom = load_circom_with_witness_json(r1cs_path, wtns_path).unwrap();
    let prover_key = load_prover_key(pkey_path).unwrap();

    // println!("Witness computed: {:#?}", circom.witness);
    let public_signals = circom
        .get_public_inputs()
        .ok_or(eyre::eyre!(
            "could not get public inputs, is witness computed?"
        ))
        .unwrap();
    let proof = prove_circuit(circom, &prover_key).unwrap();

    let snarkjs_proof = SnarkjsProof::from(&proof);
    // println!(
    //     "Proof: {}",
    //     serde_json::to_string_pretty(&snarkjs_proof).unwrap()
    // );

    let snarkjs_public_signals = SnarkjsPublicSignals::from_arkworks(public_signals);
    // println!(
    //     "Public Signals: {}",
    //     serde_json::to_string_pretty(&snarkjs_public_signals).unwrap()
    // );

    SnarkjsOutput {
        proof: snarkjs_proof,
        public_signals: snarkjs_public_signals,
    }
}
