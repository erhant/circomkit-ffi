use circomkit_ffi::arkworks::*;
use circomkit_ffi::SnarkjsProof;
use circomkit_ffi::SnarkjsPublicSignals;

/// While there is no await within the test, it still requires Tokio runtime due to
/// internals of Arkworks.
#[tokio::test]
async fn test_arkworks_multiplier_3() {
    let wasm_path = "tests/res/mul3.wasm";
    let r1cs_path = "tests/res/mul3.r1cs";
    let pkey_path = "tests/res/mul3_groth16.zkey";

    // you can push same input few times, if its an array
    let inputs = vec![("in", 1), ("in", 2), ("in", 3)];
    let (proof, public_signals) = prove(wasm_path, r1cs_path, pkey_path, inputs).unwrap();

    let snarkjs_proof = SnarkjsProof::from(&proof);
    println!(
        "Proof: {}",
        serde_json::to_string_pretty(&snarkjs_proof).unwrap()
    );

    let snarkjs_public_signals = SnarkjsPublicSignals::from(&public_signals);
    println!(
        "Public Signals: {}",
        serde_json::to_string_pretty(&snarkjs_public_signals).unwrap()
    );
}
