/// A proof object, similar to how SnarkJS exports it.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SnarkjsProof {
    pub pi_a: [String; 2],
    pub pi_b: [[String; 2]; 2],
    pub pi_c: [String; 2],
    pub protocol: String, // usually groth16
}
