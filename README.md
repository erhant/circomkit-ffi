# Circomkit FFI

This repository contains an all-in-one adapter for several backends, mainly to be used by existing Javascript code via FFI. It features:

- [x] **Provers**
  - [x] [Lambdaworks](https://github.com/lambdaclass/lambdaworks/tree/main/provers/groth16/circom-adapter) Circom adapter for Groth16 over BLS12-381
  - [x] [Arkworks](https://github.com/arkworks-rs/circom-compat) Circom adapter for Groth16 over BN254
- [ ] **Witness Calculators**
  - [ ] [Witnesscalc Adapter](https://github.com/zkmopro/witnesscalc_adapter) for witness calculation over C++ artifact
  - [ ] [Rust-Witness](https://github.com/chancehudson/rust-witness) for witness calculation over WASM artifact
- [x] **SnarkJS Exports**
  - [x] JSON proof export
  - [x] JSON public signal export
  - [ ] JSON witness export

## Installation

TODO:

maybe we do feature-gating and have a separate library for each backend?
or just a single backend for all

## Usage

TODO:

## Acknowledgements

This project is kindly supported by [Soulforge zkBankai](https://soulforge.zkbankai.com/) grant, with the application [here](https://github.com/zk-bankai/soulforge/blob/main/applications/circomkit-bunffi.md).

Some helpful resources for this project on FFI usage were:

- <https://jakegoulding.com/rust-ffi-omnibus/string_return/>
- <https://jakegoulding.com/rust-ffi-omnibus/string_arguments/>
- <https://bun.sh/docs/api/ffi>
- <https://github.com/node-ffi/node-ffi/wiki/Node-FFI-Tutorial>
- <https://tokio.rs/tokio/topics/bridging>
