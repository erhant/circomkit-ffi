#![allow(unused)]
#![cfg(feature = "witness-gen")]

use num_bigint::BigInt;
use rust_witness::{transpile::transpile_wasm, witness};
use std::collections::HashMap;

/// Execute with:
///
/// ```sh
/// cargo test --package circomkit-ffi --test rust_witness_test -- test_rust_witness_multiplier_3 --exact --show-output
/// ```
///
/// Requires `nasm` and `cmake`!
#[test]
#[ignore = "does not build on arm64"]
fn test_rust_witness_multiplier_3() -> eyre::Result<()> {
    // directory where the wasm file resides
    let wasm_path = "tests/res/";

    std::env::set_var("OUT_DIR", "./tests/rw");
    // This function will recursively search the target directory
    // for any files with the `wasm` extension and compile
    // them to C and link them
    // transpile_wasm(wasm_path.to_string());

    witness!(mul3);

    let mut inputs: HashMap<String, Vec<BigInt>> = HashMap::default();
    inputs.insert("in".into(), vec![2.into(), 4.into(), 10.into()]);

    // the generated function will be the circuit name followed by _witness
    let witness = mul3_witness(inputs);

    Ok(())
}
