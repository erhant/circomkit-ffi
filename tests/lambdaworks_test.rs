use circomkit_ffi::lambdaworks::*;
use circomkit_ffi::snarkjs::*;

#[test]
fn test_lambda_multiplier_3() -> eyre::Result<()> {
    let circom_r1cs = read_circom_r1cs("tests/res/mul3.r1cs.json").unwrap();
    let circom_witness = read_circom_witness("tests/res/mul3.wtns.json").unwrap();

    let (qap, wtns, pubs) = circom_to_lambda(circom_r1cs, circom_witness);

    let (proving_key, verifying_key) = setup(&qap);
    let proof = Prover::prove(&wtns, &qap, &proving_key);

    println!(
        "{:#?}",
        wtns.iter()
            .map(|s| s.representative().to_string())
            .collect::<Vec<_>>()
    );

    let accept = verify(&verifying_key, &proof, &pubs);
    assert!(accept, "proof is not accepted");

    let snarkjs_proof = SnarkjsProof::from(&proof);
    std::fs::write(
        "tests/res/lambdaworks_mul3_proof.json",
        serde_json::to_string_pretty(&snarkjs_proof).unwrap(),
    )?;

    let snarkjs_public_signals = SnarkjsPublicSignals::from_lambdaworks(pubs);
    std::fs::write(
        "tests/res/lambdaworks_mul3_public.json",
        serde_json::to_string_pretty(&snarkjs_public_signals).unwrap(),
    )?;

    // FIXME: lambdaworks snarkjs does not verify correctly because it does its own setup
    // see: https://github.com/lambdaclass/lambdaworks/issues/958
    // see: https://github.com/lambdaclass/lambdaworks/issues/965

    // let output = snarkjs_verify_groth16(
    //     "tests/res/mul3_groth16_vkey.json",
    //     "tests/res/lambdaworks_mul3_public.json",
    //     "tests/res/lambdaworks_mul3_proof.json",
    // )?;
    // assert!(output.status.success());

    Ok(())
}
