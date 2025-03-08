use std::path::Path;

use lambdaworks_circom_adapter::*;
use lambdaworks_groth16::common::FrElement;
use lambdaworks_math::traits::ByteConversion;

use crate::snarkjs::*;
use crate::witness::parse_witness_to_elems;

mod snarkjs;
// mod zkey; // TODO: !!!

pub fn prove_with_witness(
    r1cs_path: impl AsRef<Path>,
    wtns_path: impl AsRef<Path>,
) -> SnarkjsOutput {
    if !r1cs_path.as_ref().to_string_lossy().ends_with(".json") {
        panic!("R1CS file must be in JSON format");
    }
    let r1cs = read_circom_r1cs(r1cs_path).unwrap();

    // if wtns path ends with JSON, use `load_witness_json`, otherwise, use `load_witness`
    let wtns = if wtns_path.as_ref().ends_with(".json") {
        read_circom_witness(wtns_path).expect("could not load witness JSON")
    } else {
        read_raw_circom_witness(wtns_path).expect("could not load witness")
    };

    let (qap, wtns, pubs) = circom_to_lambda(r1cs, wtns);

    let (proving_key, verifying_key) = lambdaworks_groth16::setup(&qap);
    let proof = lambdaworks_groth16::Prover::prove(&wtns, &qap, &proving_key);

    // println!(
    //     "{:#?}",
    //     wtns.iter()
    //         .map(|s| s.representative().to_string())
    //         .collect::<Vec<_>>()
    // );

    debug_assert!(
        lambdaworks_groth16::verify(&verifying_key, &proof, &pubs),
        "proof is not accepted"
    );

    let snarkjs_proof = SnarkjsGroth16Proof::from(&proof);
    let snarkjs_public_inputs = SnarkjsPublicInputs::from_lambdaworks(pubs);
    // TODO: export verifying key from Lambda as well for this to work with snarkjs

    SnarkjsOutput {
        proof: snarkjs_proof,
        public_signals: snarkjs_public_inputs,
    }
}

/// Like `read_raw_circom_witness`, but actually reads raw witness file instead of JSON.
#[inline]
fn read_raw_circom_witness(wtns_path: impl AsRef<Path>) -> Result<Vec<FrElement>, std::io::Error> {
    let wtns_data = std::fs::read(wtns_path)?;
    parse_witness_to_elems(&wtns_data, |bytes| FrElement::from_bytes_le(bytes).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const CIRCUIT: &str = "multiplier_30";

    #[test]
    fn test_lambdaworks_witness_reader() -> eyre::Result<()> {
        let dir = Path::new("example/build").join(CIRCUIT);
        let wtns_path = dir
            .join("default") // input name
            .join("witness")
            .with_extension("wtns");
        let wtns = read_raw_circom_witness(wtns_path).unwrap();
        assert_eq!(wtns[0], FrElement::from(1)); // constant

        Ok(())
    }

    #[tokio::test]
    async fn test_lambdaworks_with_witness() -> eyre::Result<()> {
        let dir = Path::new("example/build").join(CIRCUIT);
        let r1cs_path = dir.join(CIRCUIT).with_extension("r1cs.json");
        let wtns_path = dir
            .join("default") // input name
            .join("witness")
            .with_extension("wtns");

        let _ = prove_with_witness(r1cs_path, wtns_path);
        Ok(())
    }
}
