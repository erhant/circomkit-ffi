use lambdaworks_circom_adapter::circom_to_lambda;
use lambdaworks_groth16::*;

// TODO: add proof `From` for snarkjs proof

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_multiplier_3() {
        let circuit_name = "multiplier_3";
        let input_name = "default";
        let circuit_build_dir = format!("./circuits/build/{}", circuit_name);

        let (qap, wtns) = circom_to_lambda(
            &fs::read_to_string(format!("{}/{}.r1cs.json", circuit_build_dir, circuit_name))
                .expect("could not read file"),
            &fs::read_to_string(format!(
                "{}/{}/witness.wtns.json",
                circuit_build_dir, input_name
            ))
            .expect("could not read file"),
        );

        let (proving_key, verifying_key) = setup(&qap);

        let proof = Prover::prove(&wtns, &qap, &proving_key);
        let public_inputs = &wtns[..qap.num_of_public_inputs];

        let accept = verify(&verifying_key, &proof, public_inputs);
        assert!(accept, "proof is not accepted");
    }
}
