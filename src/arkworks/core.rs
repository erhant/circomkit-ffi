use ark_bn254::{Bn254, Fr};
use ark_circom::{
    circom::R1CSFile, read_zkey, CircomBuilder, CircomCircuit, CircomConfig, CircomReduction,
};
use ark_ff::PrimeField;
use ark_groth16::{Groth16, Proof, ProvingKey};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystem, SynthesisError};
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

/// Loads Circom files from an existing R1CS and WASM file to compute the witness dynamically.
#[inline(always)]
pub fn load_circom_config<F: PrimeField>(
    r1cs_path: impl AsRef<Path>,
    wasm_path: impl AsRef<Path>,
) -> Result<CircomConfig<F>> {
    CircomConfig::new(wasm_path, r1cs_path)
}

/// Loads Circom files from an existing R1CS and computed witness file in JSON.
#[inline(always)]
pub fn load_circom_with_witness_json<F: PrimeField>(
    r1cs_path: impl AsRef<Path>,
    wtns_path: impl AsRef<Path>,
) -> Result<CircomCircuit<F>>
where
    <F as FromStr>::Err: Debug,
{
    let witness = load_witness_json::<F>(wtns_path)?;

    let reader = BufReader::new(File::open(r1cs_path)?);
    let r1cs = R1CSFile::new(reader)?.into();

    let circom = CircomCircuit {
        r1cs,
        witness: Some(witness),
    };

    debug_assert_eq!(
        verify_constraints(circom.clone()),
        Ok(true),
        "constraints not satisfied"
    );

    Ok(circom)
}

/// Loads proving key (which can generate verification key too) from an existing `zKey` file.
#[inline(always)]
pub fn load_prover_key(pkey_path: impl AsRef<Path>) -> Result<ProvingKey<Bn254>> {
    let f = File::open(pkey_path)?;
    let mut reader = BufReader::new(f);
    let (params, _) = read_zkey(&mut reader)?;

    Ok(params)
}

/// Creates a circuit with the given witness.
pub fn with_witness<F: PrimeField>(cfg: CircomConfig<F>, witness: Vec<F>) -> CircomCircuit<F> {
    let circom = CircomCircuit {
        r1cs: cfg.r1cs,
        witness: Some(witness),
    };

    debug_assert_eq!(
        verify_constraints(circom.clone()),
        Ok(true),
        "constraints not satisfied"
    );

    circom
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
    circuit: CircomCircuit<Fr>,
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
