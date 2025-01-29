//! Lambdaworks does not have a zkey reader by default, but we can add a converter from Arkworks perhaps.
//!
//! Added an issue: <https://github.com/lambdaclass/lambdaworks/issues/965>

use ark_ec::pairing::Pairing;
use ark_groth16::VerifyingKey;
use lambdaworks_groth16::common::{G1Point, G2Point};

/// The prover key for for the Groth16 zkSNARK.
pub struct ArkworksProvingKey<E: Pairing> {
    /// The underlying verification key.
    pub vk: VerifyingKey<E>,
    /// The element `beta * G` in `E::G1`.
    pub beta_g1: E::G1Affine,
    /// The element `delta * G` in `E::G1`.
    pub delta_g1: E::G1Affine,
    /// The elements `a_i * G` in `E::G1`.
    pub a_query: Vec<E::G1Affine>,
    /// The elements `b_i * G` in `E::G1`.
    pub b_g1_query: Vec<E::G1Affine>,
    /// The elements `b_i * H` in `E::G2`.
    pub b_g2_query: Vec<E::G2Affine>,
    /// The elements `h_i * G` in `E::G1`.
    pub h_query: Vec<E::G1Affine>,
    /// The elements `l_i * G` in `E::G1`.
    pub l_query: Vec<E::G1Affine>,
}

/// Converts an Arkworks `ProvingKey` to a Lambdaworks `ProvingKey`.
///
/// Here is how the mapping is done Arkworks to Lambdaworks:
/// - `pk.vb.alpha_g1` -> `alpha_g1`
///
/// - `pk.b_g1_query` -> `beta_g1`
/// - `pk.vk.beta_g2` -> `beta_g2`
///
/// - `pk.delta_g1` -> `delta_g1`
/// - `pk.vk.delta_g2` -> `delta_g2`
///
/// - `pk.l_query` -> `l_tau_g1`
/// - `pk.b_g1_query` -> `r_tau_g1`
/// - `pk.b_g2_query` -> `r_tau_g2`
///
/// - `pk.vk.gamma_abc_g1` -> `prover_k_tau_g1`
///
pub fn convert_zkey<E: Pairing>(
    ark_pk: ark_groth16::ProvingKey<E>,
) -> lambdaworks_groth16::ProvingKey {
    todo!()
}
