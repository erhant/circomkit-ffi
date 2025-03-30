use eyre::{eyre, Result};
use icicle_snark::{groth16_prove, CacheManager};
use std::path::Path;

use crate::snarkjs::{SnarkjsGroth16Proof, SnarkjsOutput, SnarkjsPublicInputs};

const ALLOWED_DEVICES: [&str; 3] = ["CPU", "CUDA", "METAL"];

pub fn prove_with_existing_witness(
    wtns_path: impl AsRef<Path>,
    pkey_path: impl AsRef<Path>,
    device: &str,
) -> Result<SnarkjsOutput> {
    let device = device.to_uppercase();
    if !ALLOWED_DEVICES.contains(&device.as_str()) {
        return Err(eyre::eyre!("device must be one of {:?}", ALLOWED_DEVICES));
    }

    let (proof_value, public_signals_value) = groth16_prove(
        wtns_path.as_ref().as_os_str().to_str().unwrap(),
        pkey_path.as_ref().as_os_str().to_str().unwrap(),
        // &proof_path,
        // &public_path,
        &device,
        &mut CacheManager::default(),
    )
    .map_err(|e| eyre!("could not generate proof: {}", e))?;

    let proof =
        serde_json::from_value::<SnarkjsGroth16Proof>(proof_value).expect("could not parse proof");
    let public_signals = serde_json::from_value::<SnarkjsPublicInputs>(public_signals_value)
        .expect("could not parse public signals");

    Ok(SnarkjsOutput {
        proof,
        public_signals,
    })
}

#[cfg(test)]
mod tests {
    use crate::snarkjs::check_snarkjs_output;

    use super::*;
    use std::path::Path;

    const CIRCUIT: &str = "multiplier_30";

    /// Run with:
    ///
    /// ```sh
    /// cargo test --package circomkit-ffi --lib -- icicle::tests::test_icicle_with_witness --exact --show-output
    /// ```
    #[test]
    fn test_icicle_with_witness() -> eyre::Result<()> {
        std::env::set_var(
            "ICICLE_BACKEND_INSTALL_DIR",
            "/Users/erhant/.ingonyama/icicle",
        );
        let dir = Path::new("example/build").join(CIRCUIT);
        let zkey = dir.join("groth16_pkey").with_extension("zkey");
        let witness = dir
            .join("default") // input name
            .join("witness")
            .with_extension("wtns");
        let device = "CPU"; //CPU

        let snarkjs_out =
            prove_with_existing_witness(&witness, &zkey, device).expect("could not prove");
        check_snarkjs_output(&snarkjs_out, &dir, CIRCUIT, "icicle")
    }
}
