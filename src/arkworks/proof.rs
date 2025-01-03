/// A Groth16 proof over Bn254.
type ArkProof = ark_groth16::Proof<ark_bn254::Bn254>;

impl From<&ArkProof> for crate::SnarkjsProof {
    fn from(proof: &ArkProof) -> Self {
        Self {
            pi_a: [proof.a.x.to_string(), proof.a.y.to_string()],
            pi_b: [
                [proof.b.x.c0.to_string(), proof.b.x.c1.to_string()],
                [proof.b.y.c0.to_string(), proof.b.y.c1.to_string()],
            ],
            pi_c: [proof.c.x.to_string(), proof.c.y.to_string()],
            protocol: "groth16".to_string(),
        }
    }
}
