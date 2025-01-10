use ark_bn254::Fr;
use ark_circom::CircomCircuit;

use circomkit_ffi::arkworks::*;
use circomkit_ffi::SnarkjsProof;
use circomkit_ffi::SnarkjsPublicSignals;

/// While there is no await within the test, it still requires Tokio runtime due to
/// internals of Arkworks.
#[tokio::test]
async fn test_arkworks_multiplier_3() -> eyre::Result<()> {
    let wasm_path = "tests/res/mul3.wasm";
    let r1cs_path = "tests/res/mul3.r1cs";
    let pkey_path = "tests/res/mul3_groth16.zkey";

    // you can push same input few times, if its an array
    let inputs = vec![("in", 2), ("in", 4), ("in", 10)];

    let config = load_circom_config(wasm_path, r1cs_path)?;
    let prover_key = load_prover_key(pkey_path)?;

    let circom: CircomCircuit<ark_ff::Fp<ark_ff::MontBackend<ark_bn254::FrConfig, 4>, 4>> =
        compute_witness::<Fr>(config, inputs)?;
    println!("Witness computed: {:#?}", circom.witness);
    let public_signals = circom.get_public_inputs().ok_or(eyre::eyre!(
        "could not get public inputs, is witness computed?"
    ))?;
    let proof = prove_circuit(circom, &prover_key)?;

    let snarkjs_proof = SnarkjsProof::from(&proof);
    println!(
        "Proof: {}",
        serde_json::to_string_pretty(&snarkjs_proof).unwrap()
    );

    let snarkjs_public_signals = SnarkjsPublicSignals::from_arkworks_vec(public_signals);
    println!(
        "Public Signals: {}",
        serde_json::to_string_pretty(&snarkjs_public_signals).unwrap()
    );

    Ok(())
}

/// While there is no await within the test, it still requires Tokio runtime due to
/// internals of Arkworks.
#[tokio::test]
async fn test_arkworks_multiplier_3_witness() -> eyre::Result<()> {
    let wtns_path = "tests/res/mul3.wtns.json";
    let r1cs_path = "tests/res/mul3.r1cs";
    let pkey_path = "tests/res/mul3_groth16.zkey";

    // you can push same input few times, if its an array
    let circom = load_circom_with_witness_json(r1cs_path, wtns_path)?;
    let prover_key = load_prover_key(pkey_path)?;

    println!("Witness computed: {:#?}", circom.witness);
    let public_signals = circom.get_public_inputs().ok_or(eyre::eyre!(
        "could not get public inputs, is witness computed?"
    ))?;
    let proof = prove_circuit(circom, &prover_key)?;

    let snarkjs_proof = SnarkjsProof::from(&proof);
    println!(
        "Proof: {}",
        serde_json::to_string_pretty(&snarkjs_proof).unwrap()
    );

    let snarkjs_public_signals = SnarkjsPublicSignals::from_arkworks_vec(public_signals);
    println!(
        "Public Signals: {}",
        serde_json::to_string_pretty(&snarkjs_public_signals).unwrap()
    );

    Ok(())
}
