//! Entry points to the library.
//!
//! These are to be called via FFI.

pub fn prove_with_witness(backend: String) {
    match backend.as_str() {
        // "lambdaworks" => lambdaworks::prove(),
        // "arkworks" => arkworks::prove(),
        _ => panic!("Unsupported backend: {}", backend),
    }
}

pub fn prove() {}

pub fn witness() {}
