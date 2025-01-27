use circomkit_ffi::{lambdaworks::*, SnarkjsProof, SnarkjsPublicSignals};

#[test]
fn test_lambda_multiplier_3() {
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
    println!("{:#?}", snarkjs_proof);

    let snarkjs_public_signals = SnarkjsPublicSignals::from_lambdaworks(pubs);
    println!("{:#?}", snarkjs_public_signals);
}
