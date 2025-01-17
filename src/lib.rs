pub mod arkworks;
pub mod lambdaworks;

mod traits;

mod snarkjs;
use std::ffi::{c_char, CStr, CString};

pub use snarkjs::*;

// TODO: !!!
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn echo(input: *const c_char) -> CString {
    let input_cstr = unsafe {
        assert!(!input.is_null());
        CStr::from_ptr(input)
    };
    let input = input_cstr.to_str().unwrap().to_string();
    CString::new(input).unwrap()
}

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

    // arkworks code is async, so we need to run it in a tokio runtime
    let snarkjs_out = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap()
        .block_on(async {
            arkworks::prove_with_witness_with_setup(r1cs_path, wtns_path, pkey_path)
        });

    CString::new(serde_json::to_string_pretty(&snarkjs_out).unwrap()).unwrap()
}
