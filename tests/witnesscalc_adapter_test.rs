#![cfg(feature = "witness-gen")]

/// Execute with:
///
/// ```sh
/// cargo test --package circomkit-ffi --test witnesscalc_adapter_test -- test_witnesscalc_multiplier_3 --exact --show-output
/// ```
///
/// Requires `nasm` and `cmake`!
#[test]
#[ignore = "doesnt work due to build-time environment variables"]
fn test_witnesscalc_multiplier_3() -> eyre::Result<()> {
    std::env::set_var("TARGET", "arm64-apple-darwin"); // for MacOS
    std::env::set_var("CARGO_CFG_TARGET_OS", "macos"); // for MacOS
    std::env::set_var("OUT_DIR", "./tests/wc");
    witnesscalc_adapter::build_and_link("./tests/wc/multiplier_3_cpp");

    // TODO: doesnt work as a callable due to `OUT_DIR` compile-time environment variable
    // witnesscalc_adapter::witness!(multiplier_3);

    Ok(())
}
