use circomkit_ffi::arkworks::*;
use circomkit_ffi::snarkjs::*;

/// Creates witness on its own.
#[test]
fn test_arkworks_multiplier_3() -> eyre::Result<()> {
    // due to internals of Arkworks we need tokio runtime even if nothing is async
    tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap()
        .block_on(async {
            let wasm_path = "tests/res/mul3.wasm";
            let r1cs_path = "tests/res/mul3.r1cs";
            let pkey_path = "tests/res/mul3_groth16.zkey";

            // you can push same input few times, if its an array
            let inputs = vec![("in", 2), ("in", 4), ("in", 10)];

            let config = load_circom_config(r1cs_path, wasm_path)?;
            let prover_key = load_prover_key(pkey_path)?;

            let circom = compute_witness::<ark_bn254::Fr>(config, inputs)?;
            // println!("Witness computed: {:#?}", circom.witness);
            let public_signals = circom.get_public_inputs().ok_or(eyre::eyre!(
                "could not get public inputs, is witness computed?"
            ))?;
            let proof = prove_circuit(circom, &prover_key)?;

            let snarkjs_proof = SnarkjsGroth16Proof::from(&proof);
            // println!(
            //     "Proof: {}",
            //     serde_json::to_string_pretty(&snarkjs_proof).unwrap()
            // );

            let snarkjs_public_signals = SnarkjsPublicSignals::from_arkworks(public_signals);
            // println!(
            //     "Public Signals: {}",
            //     serde_json::to_string_pretty(&snarkjs_public_signals).unwrap()
            // );

            std::fs::write(
                "tests/res/arkworks_mul3_proof.json",
                serde_json::to_string_pretty(&snarkjs_proof).unwrap(),
            )?;
            std::fs::write(
                "tests/res/arkworks_mul3_public.json",
                serde_json::to_string_pretty(&snarkjs_public_signals).unwrap(),
            )?;

            let output = snarkjs_verify_groth16(
                "tests/res/mul3_groth16_vkey.json",
                "tests/res/arkworks_mul3_public.json",
                "tests/res/arkworks_mul3_proof.json",
            )?;
            assert!(output.status.success());

            Ok(())
        })
}

#[tokio::test]
async fn test_arkworks_multiplier_3_witness() -> eyre::Result<()> {
    let wtns_path = "tests/res/mul3.wtns";
    let r1cs_path = "tests/res/mul3.r1cs";
    let pkey_path = "tests/res/mul3_groth16.zkey";

    let snarkjs_out = prove_with_witness_with_setup(r1cs_path, wtns_path, pkey_path);

    std::fs::write(
        "tests/res/arkworks_mul3_proof.json",
        serde_json::to_string_pretty(&snarkjs_out.proof).unwrap(),
    )?;
    std::fs::write(
        "tests/res/arkworks_mul3_public.json",
        serde_json::to_string_pretty(&snarkjs_out.public_signals).unwrap(),
    )?;

    let output = snarkjs_verify_groth16(
        "tests/res/mul3_groth16_vkey.json",
        "tests/res/arkworks_mul3_public.json",
        "tests/res/arkworks_mul3_proof.json",
    )?;
    assert!(output.status.success());

    Ok(())
}

#[test]
fn test_arkworks_multiplier_3_witness_reader() -> eyre::Result<()> {
    use ark_bn254::Fr;
    let wtns_path = "tests/res/mul3.wtns";
    let wtns = load_witness::<Fr>(wtns_path).unwrap();
    assert_eq!(wtns.len(), 6);
    assert_eq!(wtns[0], Fr::from(1)); // constant
    assert_eq!(wtns[1], Fr::from(80)); // public
    assert_eq!(wtns[2], Fr::from(2));
    assert_eq!(wtns[3], Fr::from(4));
    assert_eq!(wtns[4], Fr::from(10));
    assert_eq!(wtns[5], Fr::from(8));

    Ok(())
}
