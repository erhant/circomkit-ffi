use serde::{Deserialize, Serialize};

/// A proof object, similar to how SnarkJS exports it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnarkjsProof {
    pub pi_a: [String; 2],
    pub pi_b: [[String; 2]; 2],
    pub pi_c: [String; 2],
    pub protocol: String,
    pub curve: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnarkjsPublicSignals(Vec<String>);

impl SnarkjsProof {
    pub fn from_lambdaworks(proof: &lambdaworks_groth16::Proof) -> Self {
        Self {
            pi_a: [proof.pi1.x().to_string(), proof.pi1.y().to_string()],
            pi_b: todo!(),
            // [
            //     [
            //         proof.pi2.x().value().representative().to_string(),
            //         proof.pi2.x().c1.to_string(),
            //     ],
            //     [proof.pi2.y().c0.to_string(), proof.pi2.y().c1.to_string()],
            // ]
            pi_c: [proof.pi3.x().to_string(), proof.pi3.y().to_string()],
            protocol: "groth16".to_string(),
            curve: "bls12381".to_string(),
        }
    }

    pub fn from_arkworks(proof: &ark_groth16::Proof<ark_bn254::Bn254>) -> Self {
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
