use ark_bn254::{Bn254, Fr};
use ark_circom::{read_zkey, CircomBuilder, CircomCircuit, CircomConfig, CircomReduction};
use ark_ff::PrimeField;
use ark_groth16::{Groth16, Proof, ProvingKey};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystem, SynthesisError};
use ark_std::rand::thread_rng;
use eyre::Result;
use std::{fs::File, io::BufReader, path::Path};

mod proof;

/// Loads Circom files from an existing WASM and R1CS.
pub fn load_circom_config<F: PrimeField>(
    wasm_path: impl AsRef<Path>,
    r1cs_path: impl AsRef<Path>,
) -> Result<CircomConfig<F>> {
    CircomConfig::new(wasm_path, r1cs_path)
}

/// Loads proving key (which can generate verification key too) from an existing `zKey` file.
pub fn load_prover_key(pkey_path: impl AsRef<Path>) -> Result<ProvingKey<Bn254>> {
    let f = File::open(pkey_path)?;
    let mut reader = BufReader::new(f);
    let (params, _) = read_zkey(&mut reader)?;

    Ok(params)
}

pub fn compute_witness<F: PrimeField>(
    cfg: CircomConfig<F>,
    witness_values: Vec<(impl ToString, impl Into<num_bigint::BigInt>)>,
) -> Result<CircomCircuit<F>> {
    let mut builder = CircomBuilder::new(cfg);
    for (label, value) in witness_values {
        builder.push_input(label, value);
    }

    // compute witness i.e. building circuit with inputs
    let circom = builder.build()?;
    assert!(
        verify_constraints(circom.clone())?,
        "constraints not satisfied"
    );

    Ok(circom)
}

/// Asserts all constraints to pass.
///
/// Returns `true` if all constraints are satisfied for the built circuit.
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
pub fn setup_circom_bn254_circuit(
    builder: CircomBuilder<Fr>,
) -> Result<ProvingKey<Bn254>, SynthesisError> {
    let mut rng = thread_rng();

    Groth16::<Bn254, CircomReduction>::generate_random_parameters_with_reduction(
        builder.setup(),
        &mut rng,
    )
}

/// Creates a proof from a circuit with public inputs fed into.
pub fn prove_circuit(
    circuit: CircomCircuit<Fr>,
    pkey: &ProvingKey<Bn254>,
) -> Result<Proof<Bn254>, SynthesisError> {
    let mut rng = thread_rng();
    Groth16::<Bn254, CircomReduction>::create_random_proof_with_reduction(circuit, pkey, &mut rng)
}

pub fn verify(
    proof: &Proof<Bn254>,
    public_inputs: &[Fr],
    proving_key: &ProvingKey<Bn254>,
) -> Result<bool, SynthesisError> {
    Groth16::<Bn254, CircomReduction>::verify_proof(
        &ark_groth16::prepare_verifying_key(&proving_key.vk),
        &proof,
        &public_inputs,
    )
}

pub fn prove(
    wasm_path: impl AsRef<Path>,
    r1cs_path: impl AsRef<Path>,
    pkey_path: impl AsRef<Path>,
    witness_values: Vec<(impl ToString, impl Into<num_bigint::BigInt>)>,
) -> eyre::Result<(Proof<Bn254>, Vec<Fr>)> {
    let config = load_circom_config(wasm_path, r1cs_path)?;
    let prover_key = load_prover_key(pkey_path)?;

    let circom = compute_witness::<Fr>(config, witness_values)?;

    let pubs = circom.get_public_inputs().unwrap_or_default();

    let proof = prove_circuit(circom, &prover_key)?;

    Ok((proof, pubs))
}

// TODO: test multiplier_3 circuit

/// Exports public signals as a JSON array of string bigints.
pub fn export_public_signals<F: PrimeField>(pubs: &Vec<F>) -> Result<String, serde_json::Error> {
    let signal_strings = pubs.iter().map(|s| s.to_string()).collect::<Vec<String>>();

    serde_json::to_string(&signal_strings)
}
