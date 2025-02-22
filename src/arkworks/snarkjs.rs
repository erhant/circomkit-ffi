use crate::snarkjs::{SnarkjsGroth16Proof, SnarkjsPublicSignals};

type ArkworksProof = ark_groth16::Proof<ark_bn254::Bn254>;

impl From<&ArkworksProof> for SnarkjsGroth16Proof {
    fn from(proof: &ArkworksProof) -> Self {
        Self {
            pi_a: [proof.a.x.to_string(), proof.a.y.to_string()],
            pi_b: [
                [proof.b.x.c0.to_string(), proof.b.x.c1.to_string()],
                [proof.b.y.c0.to_string(), proof.b.y.c1.to_string()],
            ],
            pi_c: [proof.c.x.to_string(), proof.c.y.to_string()],
            protocol: "groth16".to_string(),
            curve: "bn254".to_string(),
        }
    }
}

impl From<ArkworksProof> for SnarkjsGroth16Proof {
    fn from(proof: ArkworksProof) -> Self {
        Self::from(&proof)
    }
}

impl SnarkjsPublicSignals {
    #[inline]
    pub fn from_arkworks<F: ark_ff::PrimeField>(public_signals: impl AsRef<[F]>) -> Self {
        Self(
            public_signals
                .as_ref()
                .iter()
                .map(|s| s.to_string())
                .collect(),
        )
    }
}
