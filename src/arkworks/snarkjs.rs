use crate::{SnarkjsProof, SnarkjsPublicSignals};

type ArkworksProof = ark_groth16::Proof<ark_bn254::Bn254>;

impl From<&ArkworksProof> for SnarkjsProof {
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

impl From<ArkworksProof> for SnarkjsProof {
    fn from(proof: ArkworksProof) -> Self {
        Self::from(&proof)
    }
}

impl<F: ark_ff::PrimeField> From<&[F]> for SnarkjsPublicSignals {
    fn from(public_signals: &[F]) -> Self {
        Self(public_signals.iter().map(|s| s.to_string()).collect())
    }
}

impl<F: ark_ff::PrimeField> From<&Vec<F>> for SnarkjsPublicSignals {
    fn from(public_signals: &Vec<F>) -> Self {
        Self::from(public_signals.as_slice())
    }
}

impl<F: ark_ff::PrimeField> From<Vec<F>> for SnarkjsPublicSignals {
    fn from(public_signals: Vec<F>) -> Self {
        Self::from(&public_signals)
    }
}
