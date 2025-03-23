use eyre::Result;
use icicle_snark::{groth16_prove, CacheManager};
use serde::Serialize;
use std::path::Path;

use crate::snarkjs::SnarkjsOutput;

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

    let proof_path = "./proof.json".to_string();
    let public_path = "./public.json".to_string();
    let proof = groth16_prove(
        wtns_path.as_ref().as_os_str().to_str().unwrap(),
        pkey_path.as_ref().as_os_str().to_str().unwrap(),
        &proof_path,
        &public_path,
        &device,
        &mut CacheManager::default(),
    );

    todo!("todo")
}

#[cfg(test)]
mod tests {
    use icicle_snark::{groth16_prove, CacheManager};
    use std::path::Path;

    const CIRCUIT: &str = "multiplier_30";

    /// Run with:
    ///
    /// ```sh
    /// cargo test --package circomkit-ffi --lib -- icicle::tests::test_main --exact --show-output
    /// ```
    #[test]
    fn test_main() {
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
        let proof = "./proof.json".to_string();
        let public = "./public.json".to_string();
        let device = "CPU"; //CPU

        groth16_prove(
            &witness.into_os_string().into_string().unwrap(),
            &zkey.into_os_string().into_string().unwrap(),
            &proof,
            &public,
            device,
            &mut CacheManager::default(),
        )
        .expect("could not prove");
    }
}
