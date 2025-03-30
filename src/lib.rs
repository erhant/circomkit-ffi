use std::ffi::{c_char, CStr, CString};

/// SnarkJS compatibility layer.
pub mod snarkjs;

/// [Arkworks](https://github.com/arkworks-rs/circom-compat)
pub mod arkworks;
/// [ICICLE](https://github.com/ingonyama-zk/icicle-snark/)
pub mod icicle;
/// [Lambdaworks](https://github.com/lambdaclass/lambdaworks)
pub mod lambdaworks;

mod traits;
mod witness;

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
    match tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap()
        .block_on(async { arkworks::prove_with_existing_witness(r1cs_path, wtns_path, pkey_path) })
    {
        Ok(snarkjs_out) => {
            let output = serde_json::to_string_pretty(&snarkjs_out).unwrap();
            CString::new(output).unwrap()
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
}

/// Generate a Lambdaworks proof from a given witness, R1CS.
///
/// It creates its own prover key within.
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn lambdaworks_prove(
    wtns_path_ptr: *const c_char,
    r1cs_path_ptr: *const c_char,
) -> CString {
    let [wtns_path, r1cs_path] = [wtns_path_ptr, r1cs_path_ptr].map(|ptr| {
        unsafe {
            assert!(!ptr.is_null());
            CStr::from_ptr(ptr)
        }
        .to_str()
        .unwrap()
    });

    match lambdaworks::prove_with_witness(r1cs_path, wtns_path) {
        Ok(snarkjs_out) => {
            let output = serde_json::to_string_pretty(&snarkjs_out).unwrap();
            CString::new(output).unwrap()
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
}

/// Generate an ICICLE proof from a given witness, R1CS, and a device type.
///
/// The device type can be one of: `CPU`, `CUDA`, `METAL`.
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn icicle_prove(
    wtns_path_ptr: *const c_char,
    pkey_path_ptr: *const c_char,
    device_ptr: *const c_char,
) -> CString {
    let [wtns_path, pkey_path, device] = [wtns_path_ptr, pkey_path_ptr, device_ptr].map(|ptr| {
        unsafe {
            assert!(!ptr.is_null());
            CStr::from_ptr(ptr)
        }
        .to_str()
        .unwrap()
    });

    match icicle::prove_with_existing_witness(wtns_path, pkey_path, device) {
        Ok(snarkjs_out) => {
            let output = serde_json::to_string_pretty(&snarkjs_out).unwrap();
            CString::new(output).unwrap()
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
}
