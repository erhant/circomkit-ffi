use circomkit_ffi::{lambdaworks::*, SnarkjsProof, SnarkjsPublicSignals};
use std::fs;

#[test]
fn test_lambda_multiplier_3() {
    let r1cs_path = "tests/res/mul3.r1cs.json";
    let wtns_path = "tests/res/mul3.wtns.json";

    let (qap, wtns) = circom_to_lambda(
        &fs::read_to_string(r1cs_path).unwrap(),
        &fs::read_to_string(wtns_path).unwrap(),
    );

    let (proving_key, verifying_key) = setup(&qap);

    let proof = Prover::prove(&wtns, &qap, &proving_key);
    println!("POINT: {:#?}", proof.pi2.to_affine().x());
    let public_inputs = &wtns[..qap.num_of_public_inputs];

    let snarkjs_proof = SnarkjsProof::from(&proof);
    println!("{:#?}", snarkjs_proof);

    let snarkjs_public_signals = SnarkjsPublicSignals::from_lambdaworks_slice(public_inputs);
    println!("{:#?}", snarkjs_public_signals);

    let accept = verify(&verifying_key, &proof, public_inputs);
    assert!(accept, "proof is not accepted");
}
