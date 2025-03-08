<p align="center">
  <h1 align="center">
    Circomkit FFI
  </h1>
  <p align="center"><i>Rust-based static libraries for alternative provers.</i></p>
</p>

<p align="center">
  <a href="https://www.npmjs.com/package/circomkit-ffi" target="_blank">
        <img alt="NPM" src="https://img.shields.io/npm/v/circomkit-ffi?logo=npm&color=CB3837">
    </a>
    <a href="./.github/workflows/test-lib.yml" target="_blank">
        <img alt="Workflow: Tests" src="https://github.com/erhant/circomkit-ffi/actions/workflows/test-lib.yaml/badge.svg?branch=main">
    </a>
    <a href="https://opensource.org/licenses/MIT" target="_blank">
        <img src="https://img.shields.io/badge/license-MIT-blue.svg">
    </a>
</p>

This repository contains an all-in-one adapter for several backends, mainly to be used by existing Javascript code via FFI. It features prover backends via [Lambdaworks](https://github.com/lambdaclass/lambdaworks/tree/main/provers/groth16/circom-adapter) Circom adapter for **Groth16** over **BLS12-381** and [Arkworks](https://github.com/arkworks-rs/circom-compat) Circom adapter for **Groth16** over **BN254**. It also provides [SnarkJS](https://github.com/iden3/snarkjs) exports for both prover backends, to export proof objects and public signals.

## Installation

The FFI libraries are exposed through an SDK, which is published on NPM.

```sh
npm install circomkit-ffi
```

The libraries are kept under this repository's releases. It supports the following platforms & architectures:

- Linux amd64
- Linux arm64
- MacOS Intel (amd64)
- MacoS Apple Silicon (arm64)

The library will be automatically downloaded when required by the SDK.

## Usage

The SDK exposes two types of classes:

- `CircomkitFFIBun` uses [`bun:ffi`](https://bun.sh/docs/api/ffi) and must be used within a Bun runtime.
- `CircomkitFFINode` uses [`ffi-rs`](https://github.com/zhangyuang/node-ffi-rs/) and can be used by both Bun and Node.

> [!NOTE]
>
> `CircomkitFFINode` has a peer dependency for `ffi-rs` as well:
>
> ```sh
> npm install ffi-rs
> ```

## Development

Clone the repository:

```sh
git clone https://github.com/erhant/circomkit-ffi
```

Note that Arkworks requires a Tokio runtime, even if nothing has to be `await`ed.

### Library

The library called by FFI is written in Rust. You can build the library (for your own machine) using:

```sh
cargo build
```

You can run the tests with:

```sh
cargo test
```

Before running tests:

- you need to have installed [SnarkJS](https://github.com/iden3/snarkjs) globally, to verify that the FFI-generated proof is valid.
- you need generated circuit files, which can be done with:

```sh
# preparations (do only once)
cd example
bun install

# create circuit (e.g. multiplier_3) artifacts for tests
CIRCUIT=multiplier_3
bunx circomkit witness $CIRCUIT default
bunx circomkit prove $CIRCUIT default
bunx circomkit json wtns $CIRCUIT default
bunx circomkit json r1cs $CIRCUIT
```

### SDK

The SDK that provides an interface to the FFI-library is written in TypeScript, and we use Bun for this. Install packages with:

```sh
bun install
```

Run tests with:

```sh
bun test
```

You can build the package using:

```sh
bun run build.ts
```

Note that we only export ESM modules, we do not have CommonJS support.

## Acknowledgements

This project is kindly supported by [Soulforge zkBankai](https://soulforge.zkbankai.com/) grant, with the application [here](https://github.com/zk-bankai/soulforge/blob/main/applications/circomkit-bunffi.md).

Some helpful resources for this project on FFI usage were:

- <https://jakegoulding.com/rust-ffi-omnibus/string_return/>
- <https://jakegoulding.com/rust-ffi-omnibus/string_arguments/>
- <https://bun.sh/docs/api/ffi>
- <https://github.com/node-ffi/node-ffi/wiki/Node-FFI-Tutorial>
- <https://tokio.rs/tokio/topics/bridging>
