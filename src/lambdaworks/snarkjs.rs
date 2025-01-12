use lambdaworks_math::field::{element::FieldElement, traits::IsPrimeField};

use crate::{SnarkjsProof, SnarkjsPublicSignals};

type LambdaworksProof = lambdaworks_groth16::Proof;

impl From<&LambdaworksProof> for SnarkjsProof {
    fn from(proof: &lambdaworks_groth16::Proof) -> Self {
        Self {
            pi_a: [
                proof.pi1.x().representative().to_string(),
                proof.pi1.y().representative().to_string(),
            ],
            pi_b: [
                [
                    proof.pi2.x().value()[0].representative().to_string(),
                    proof.pi2.x().value()[1].representative().to_string(),
                ],
                [
                    proof.pi2.y().value()[0].representative().to_string(),
                    proof.pi2.y().value()[1].representative().to_string(),
                ],
            ],
            pi_c: [
                proof.pi3.x().representative().to_string(),
                proof.pi3.y().representative().to_string(),
            ],
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

impl SnarkjsPublicSignals {
    pub fn from_lambdaworks<F: IsPrimeField>(
        public_signals: impl AsRef<[FieldElement<F>]>,
    ) -> Self {
        Self(
            public_signals
                .as_ref()
                .iter()
                .map(|s| s.representative().to_string())
                .collect(),
        )
    }
}
