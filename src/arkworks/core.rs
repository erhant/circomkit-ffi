use ark_bn254::{Bn254, Fr};
use ark_circom::{
    circom::{R1CSFile, R1CS},
    read_zkey, CircomBuilder, CircomCircuit, CircomConfig, CircomReduction,
};
use ark_ff::PrimeField;
use ark_groth16::{Groth16, Proof, ProvingKey};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystem, SynthesisError};
use ark_serialize::SerializationError;
use ark_std::rand::thread_rng;
use eyre::Result;
use std::{
    fmt::Debug,
    fs::File,
    io::{self, BufReader},
    path::Path,
    str::FromStr,
};

use crate::witness::parse_witness_to_elems;

#[inline(always)]
pub fn load_r1cs<F: PrimeField>(
    r1cs_path: impl AsRef<Path>,
) -> Result<R1CS<F>, SerializationError> {
    let reader = BufReader::new(File::open(r1cs_path)?);
    Ok(R1CSFile::new(reader)?.into())
}

/// Loads proving key (which can generate verification key too) from an existing `zKey` file.
#[inline(always)]
pub fn load_proving_key(pkey_path: impl AsRef<Path>) -> Result<ProvingKey<Bn254>> {
    let mut reader = BufReader::new(File::open(pkey_path)?);
    let (params, _) = read_zkey(&mut reader)?;

    Ok(params)
}

/// Loads a witness from witness JSON file.
pub fn load_witness_json<F: PrimeField>(
    wtns_json_path: impl AsRef<Path>,
) -> Result<Vec<F>, io::Error>
where
    <F as FromStr>::Err: Debug,
{
    let f = File::open(wtns_json_path)?;
    let wtns: Vec<String> = serde_json::from_reader(BufReader::new(f))?;

    Ok(wtns.iter().map(|s| F::from_str(s).unwrap()).collect())
}

/// Loads a witness from raw witness file.
pub fn load_witness<F: PrimeField>(wtns_path: impl AsRef<Path>) -> Result<Vec<F>, io::Error> {
    let wtns_data = std::fs::read(wtns_path)?;
    parse_witness_to_elems(&wtns_data, F::from_le_bytes_mod_order)
}

/// Creates a circuit by computing the witness from the given inputs.
///
/// This makes use of WASM as well, so it may not necessarily provide advantages for witness computation.
pub fn compute_witness<F: PrimeField>(
    cfg: CircomConfig<F>,
    inputs: Vec<(impl ToString, impl Into<num_bigint::BigInt>)>,
) -> Result<CircomCircuit<F>> {
    let mut builder = CircomBuilder::new(cfg);
    for (label, value) in inputs {
        builder.push_input(label, value);
    }

    // compute witness i.e. building circuit with inputs
    let circom = builder.build()?;
    debug_assert!(
        verify_constraints(circom.clone())?,
        "constraints not satisfied"
    );

    Ok(circom)
}

/// Asserts all constraints to pass.
///
/// Returns `true` if all constraints are satisfied for the built circuit.
#[inline]
pub fn verify_constraints<F: PrimeField>(
    circuit: CircomCircuit<F>,
) -> Result<bool, SynthesisError> {
    let cs = ConstraintSystem::<F>::new_ref();
    circuit.generate_constraints(cs.clone())?;
    cs.is_satisfied()
}

/// Creates an empty instance from the given builder & runs a trusted setup to generate keys.
/// Using `load_prover_key` may have a problem with proof verification, so this is just an alternative
/// that is tested to be working correctly.
///
/// https://github.com/arkworks-rs/circom-compat/issues/35 see this for a related issue
#[inline(always)]
#[allow(unused)]
pub fn setup_circom_bn254_circuit(
    builder: CircomBuilder<Fr>,
) -> Result<ProvingKey<Bn254>, SynthesisError> {
    Groth16::<Bn254, CircomReduction>::generate_random_parameters_with_reduction(
        builder.setup(),
        &mut thread_rng(),
    )
}

/// Creates a proof from a circuit with public inputs fed into.
#[inline(always)]
pub fn prove_circuit(
    circuit: CircomCircuit<ark_bn254::Fr>,
    pkey: &ProvingKey<Bn254>,
) -> Result<Proof<Bn254>, SynthesisError> {
    Groth16::<Bn254, CircomReduction>::create_random_proof_with_reduction(
        circuit,
        pkey,
        &mut thread_rng(),
    )
}

/// Verifies a proof with public inputs.
#[inline(always)]
pub fn verify(
    proof: &Proof<Bn254>,
    public_inputs: &[Fr],
    proving_key: &ProvingKey<Bn254>,
) -> Result<bool, SynthesisError> {
    Groth16::<Bn254, CircomReduction>::verify_proof(
        &ark_groth16::prepare_verifying_key(&proving_key.vk),
        proof,
        public_inputs,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr;

    const CIRCUIT: &str = "multiplier_3";
    const INPUT: &str = "default";

    #[test]
    fn test_arkworks_multiplier_3_witness_reader() -> eyre::Result<()> {
        let dir = Path::new("example/build").join(CIRCUIT);
        let wtns_path = dir
            .join(INPUT) // input name
            .join("witness")
            .with_extension("wtns");

        let wtns = load_witness::<Fr>(wtns_path).unwrap();
        assert!(wtns.len() != 0);

        Ok(())
    }
}
