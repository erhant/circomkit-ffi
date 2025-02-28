# Circomkit FFI

This repository contains an all-in-one adapter for several backends, mainly to be used by existing Javascript code via FFI. It features prover backends via [Lambdaworks](https://github.com/lambdaclass/lambdaworks/tree/main/provers/groth16/circom-adapter) Circom adapter for Groth16 over BLS12-381 and [Arkworks](https://github.com/arkworks-rs/circom-compat) Circom adapter for Groth16 over BN254. It also provides [SnarkJS](https://github.com/iden3/snarkjs) exports for both prover backends, to export proof objects and public signals.

## Installation

The FFI libraries are exposed through an SDK, which is published on NPM.

```sh
npm install circomkit-ffi
```

The libraries are kept under this repository's releases. It supports the following platforms & architectures:

- Linux `amd64`
- Linux `arm64`
- MacOS Intel (`amd64`)
- MacoS Apple Silicon (`arm64`)

The library will be automatically downloaded when required by the SDK.

## Usage

The SDK exposes two types of classes:

- `CircomkitFFIBun` uses `bun:ffi` and must be used within a Bun runtime.
- `CircomkitFFINode` uses `ffi-rs` and can be used by both Bun and Node.

These classes expose a "prove" function for both Arkworks and Lambdaworks backends.

## Development

Clone the repository:

```sh
git clone https://github.com/erhant/circomkit-ffi
```

For Rust library, we use `cargo`:

```sh
# build the libraries
cargo build

# run tests
cargo test
```

For SDK, we use `Bun`:

```bash
# install packages
bun install

# run tests
bun test

# build NPM package
bun run build.ts
```

## Acknowledgements

This project is kindly supported by [Soulforge zkBankai](https://soulforge.zkbankai.com/) grant, with the application [here](https://github.com/zk-bankai/soulforge/blob/main/applications/circomkit-bunffi.md).

Some helpful resources for this project on FFI usage were:

- <https://jakegoulding.com/rust-ffi-omnibus/string_return/>
- <https://jakegoulding.com/rust-ffi-omnibus/string_arguments/>
- <https://bun.sh/docs/api/ffi>
- <https://github.com/node-ffi/node-ffi/wiki/Node-FFI-Tutorial>
- <https://tokio.rs/tokio/topics/bridging>
