use super::{circom, utils::hash_to_group};
use ark_bn254::Fr;
use ark_circom::CircomConfig;
use ark_groth16::ProvingKey;
use eyre::Result;
use num_bigint::BigUint;
use serde::Serialize;

#[derive(Clone, Debug)]
pub struct Prover {
    config: CircomConfig<Bn254>,
    prover_key: ProvingKey<Bn254>,
}

// TODO: !!!
