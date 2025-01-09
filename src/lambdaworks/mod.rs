pub use lambdaworks_circom_adapter::circom_to_lambda;
pub use lambdaworks_groth16::*;

mod snarkjs;

// IMPORTANT:
//
// - Circom witness ordering: `["1", ..outputs, ...inputs, ...other_signals]`
// - Lambda witness ordering: `["1", ...inputs, ..outputs, ...other_signals]`
