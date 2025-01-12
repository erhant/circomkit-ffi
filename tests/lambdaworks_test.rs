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
    let public_inputs = &wtns[..qap.num_of_public_inputs];
    // TODO: something wrong with public inputs? i think it should be something else
    // let public_inputs = &wtns[1 + qap.num_of_private_inputs()
    //     ..1 + qap.num_of_private_inputs() + qap.num_of_public_inputs];
    println!(
        "{:#?}",
        wtns.iter()
            .map(|s| s.representative().to_string())
            .collect::<Vec<_>>()
    );

    let snarkjs_proof = SnarkjsProof::from(&proof);
    println!("{:#?}", snarkjs_proof);

    let snarkjs_public_signals = SnarkjsPublicSignals::from_lambdaworks(public_inputs);
    println!("{:#?}", snarkjs_public_signals);

    let accept = verify(&verifying_key, &proof, public_inputs);
    assert!(accept, "proof is not accepted");
}
