use std::path::Path;

pub use lambdaworks_circom_adapter::*;
use lambdaworks_groth16::common::FrElement;
pub use lambdaworks_groth16::*;
use lambdaworks_math::traits::ByteConversion;

use crate::{
    snarkjs::{SnarkjsProof, SnarkjsPublicSignals},
    witness::parse_witness_to_elems,
};

mod snarkjs;
// mod zkey;

pub fn prove_with_witness(wtns_json_path: impl AsRef<Path>, r1cs_path: impl AsRef<Path>) {
    let circom_r1cs = read_circom_r1cs(r1cs_path).unwrap();
    let circom_witness = read_circom_witness(wtns_json_path).unwrap();

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

pub fn load_witness(wtns_path: impl AsRef<Path>) -> Result<Vec<FrElement>, std::io::Error> {
    let wtns_data = std::fs::read(wtns_path)?;
    parse_witness_to_elems(&wtns_data, |bytes| FrElement::from_bytes_le(bytes).unwrap())
}
