pub mod arkworks;
pub mod lambdaworks;

mod traits;

mod snarkjs;
use std::ffi::CString;

pub use snarkjs::*;

// TODO: !!!
#[no_mangle]
pub extern "C" fn test_me(echo_me: *const std::ffi::c_char) {
    let mysr = CString::from(echo_me);
    // println!("I am echoing: {:?}", String::from_utf8(echo_me).unwrap());
    println!("I am echoing: {:?}", echo_me);
}

#[no_mangle]
pub extern "C" fn prove_with_witness(backend: CString, wtns_path: CString, r1cs_path: CString) {
    let wtns_path = wtns_path.to_str().expect("invalid wtns string");
    let r1cs_path = r1cs_path.to_str().expect("invalid r1cs string");
    // match backend.to_str().expect("invalid backend string") {
    //     "lambdaworks" => {
    //         lambdaworks::prove_with_witness(wtns_path, r1cs_path);
    //     }
    //     "arkworks" => {
    //         arkworks::prove_with_witness_with_setup(wtns_path, r1cs_path, "TODO: !!!");
    //     }
    //     _ => panic!("Unsupported backend"),
    // }
}
