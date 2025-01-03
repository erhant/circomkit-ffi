use ark_bn254::{Bn254, Fr};
use ark_circom::{read_zkey, CircomBuilder, CircomCircuit, CircomConfig, CircomReduction};
use ark_groth16::{Groth16, Proof, ProvingKey};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystem, SynthesisError};
use ark_std::rand::thread_rng;
use eyre::{Context, Result};
use serde::Serialize;
use std::{fs::File, io::BufReader};

/// Loads Circom files from an existing WASM and R1CS.
pub fn load_circom_config(wasm_path: &str, r1cs_path: &str) -> Result<CircomConfig<Fr>> {
    CircomConfig::<Fr>::new(wasm_path, r1cs_path)
}

/// Loads proving key (which can generate verification key too) from an existing `zKey` file.
pub fn load_prover_key(pkey_path: &str) -> Result<ProvingKey<Bn254>> {
    let f = File::open(pkey_path)?;
    let mut reader = BufReader::new(f);
    let (params, _) = read_zkey(&mut reader)?;

    Ok(params)
}

/// Asserts all constraints to pass.
pub fn check_constraints(circuit: CircomCircuit<Fr>) -> Result<(), SynthesisError> {
    let cs = ConstraintSystem::<Fr>::new_ref();

    circuit.generate_constraints(cs.clone())?;
    assert!(cs.is_satisfied()?);

    Ok(())
}

/// Creates an empty instance from the given builder & runs a trusted setup to generate keys.
/// Using `load_prover_key` may have a problem with proof verification, so this is just an alternative
/// that is tested to be working correctly.
///
/// https://github.com/arkworks-rs/circom-compat/issues/35 see this for a related issue
pub fn setup_circuit(builder: CircomBuilder<Fr>) -> Result<ProvingKey<Bn254>, SynthesisError> {
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

#[derive(Serialize)]
struct SnarkjsProof {
    pi_a: [String; 2],
    pi_b: [[String; 2]; 2],
    pi_c: [String; 2],
    protocol: String,
}

/// Exports proof as a JSON object.
pub fn export_proof(proof: &Proof<Bn254>) -> eyre::Result<String> {
    let obj = SnarkjsProof {
        pi_a: [proof.a.x.to_string(), proof.a.y.to_string()],
        pi_b: [
            [proof.b.x.c0.to_string(), proof.b.x.c1.to_string()],
            [proof.b.y.c0.to_string(), proof.b.y.c1.to_string()],
        ],
        pi_c: [proof.c.x.to_string(), proof.c.y.to_string()],
        protocol: "groth16".to_string(),
    };

    serde_json::to_string(&obj).wrap_err("could not serialize proof")
}

/// Exports public signals as a JSON array of string bigints.
pub fn export_public_signals(pubs: &Vec<Fr>) -> Result<String, serde_json::Error> {
    let signal_strings = pubs.iter().map(|s| s.to_string()).collect::<Vec<String>>();

    serde_json::to_string(&signal_strings)
}
