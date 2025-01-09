use crate::SnarkjsProof;

type LambdaworksProof = lambdaworks_groth16::Proof;

impl From<&LambdaworksProof> for SnarkjsProof {
    fn from(proof: &lambdaworks_groth16::Proof) -> Self {
        // TODO: implement this
        Self {
            pi_a: [proof.pi1.x().to_string(), proof.pi1.y().to_string()],
            pi_b: [
                [
                    "TODO".to_string(), // proof.pi2.x().value().representative().to_string(),
                    "TODO".to_string(), // proof.pi2.x().c1.to_string(),
                ],
                [
                    "TODO".to_string(), // proof.pi2.y().c0.to_string(),
                    "TODO".to_string(), // proof.pi2.y().c1.to_string()
                ],
            ],
            pi_c: [proof.pi3.x().to_string(), proof.pi3.y().to_string()],
            protocol: "groth16".to_string(),
            curve: "bls12381".to_string(),
        }
    }
}

impl From<LambdaworksProof> for SnarkjsProof {
    fn from(proof: LambdaworksProof) -> Self {
        Self::from(&proof)
    }
}
