/// [Arkworks](https://github.com/arkworks-rs/circom-compat)
pub mod arkworks;

/// [Lambdaworks](https://github.com/lambdaclass/lambdaworks)
pub mod lambdaworks;

// [Rust Witness](https://github.com/chancehudson/rust-witness)
// pub mod rust_witness;

mod traits;

mod witness;

pub mod snarkjs;
use std::ffi::{c_char, CStr, CString};

/// Given a string input, returns the same.
/// Should be used for testing purposes of the FFI logic.
#[no_mangle]
#[allow(improper_ctypes_definitions)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn echo(input: *const c_char) -> CString {
    let input_cstr = unsafe {
        assert!(!input.is_null());
        CStr::from_ptr(input)
    };
    let input = input_cstr.to_str().unwrap().to_string();
    CString::new(input).unwrap()
}

/// Generate an Arkworks proof from a given witness, R1CS and prover key path.
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn arkworks_prove(
    wtns_path_ptr: *const c_char,
    r1cs_path_ptr: *const c_char,
    pkey_path_ptr: *const c_char,
) -> CString {
    let [wtns_path, r1cs_path, pkey_path] =
        [wtns_path_ptr, r1cs_path_ptr, pkey_path_ptr].map(|ptr| {
            unsafe {
                assert!(!ptr.is_null());
                CStr::from_ptr(ptr)
            }
            .to_str()
            .unwrap()
        });

    // due to internals of Arkworks we need `tokio` runtime even if nothing is async within the thread
    let snarkjs_out = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap()
        .block_on(async { arkworks::prove_with_existing_witness(r1cs_path, wtns_path, pkey_path) });

    let output = serde_json::to_string_pretty(&snarkjs_out).unwrap();
    CString::new(output).unwrap()
}

/// Generate an Arkworks proof from a given witness, R1CS and prover key path.
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn lambdaworks_prove(
    wtns_path_ptr: *const c_char,
    r1cs_path_ptr: *const c_char,
    pkey_path_ptr: *const c_char,
) -> CString {
    let [wtns_path, r1cs_path, pkey_path] =
        [wtns_path_ptr, r1cs_path_ptr, pkey_path_ptr].map(|ptr| {
            unsafe {
                assert!(!ptr.is_null());
                CStr::from_ptr(ptr)
            }
            .to_str()
            .unwrap()
        });

    // due to internals of Arkworks we need `tokio` runtime even if nothing is async within the thread
    let snarkjs_out = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap()
        .block_on(async { todo!("todo") });

    let output = serde_json::to_string_pretty(&snarkjs_out).unwrap();
    CString::new(output).unwrap()
}
