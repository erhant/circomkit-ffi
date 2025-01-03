use ark_bn254::{Bn254, Fr};
use ark_circom::CircomConfig;
use ark_groth16::ProvingKey;

#[derive(Debug)]
pub struct Prover {
    config: CircomConfig<Fr>,
    prover_key: ProvingKey<Bn254>,
}

// TODO: !!!
