<p align="center">
  <h1 align="center">
    Circomkit FFI
  </h1>
  <p align="center"><i>Rust-based static libraries for alternative provers.</i></p>
</p>

<p align="center">
  <a href="https://github.com/erhant/circomkit-ffi/releases/latest" target="_blank">
    <img alt="Release" src="https://img.shields.io/github/v/release/erhant/circomkit-ffi?logo=github&label=lib">
  </a>
  <a href="https://www.npmjs.com/package/circomkit-ffi" target="_blank">
      <img alt="NPM" src="https://img.shields.io/npm/v/circomkit-ffi?logo=npm&color=CB3837&label=sdk">
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

The libraries are kept under this repository's releases, and supports the following platforms & architectures:

- Linux amd64
- Linux arm64
- MacOS Intel (amd64)
- MacoS Apple Silicon (arm64)

## Usage

The SDK exposes two types of classes:

- `CircomkitFFIBun` is exported at `/circomkit-ffi/bun` and uses [`bun:ffi`](https://bun.sh/docs/api/ffi) and must be used within a Bun runtime.
- `CircomkitFFINode` uses [`ffi-rs`](https://github.com/zhangyuang/node-ffi-rs/) and can be used by both Bun and Node.

> [!NOTE]
>
> `CircomkitFFINode` has a peer dependency for `ffi-rs` as well:
>
> ```sh
> npm install ffi-rs
> ```

### Downloading the library

First you need to download the library suited to your machine. We provide some utilities for this:

```ts
import { downloadRelease, getLibPath } from "circomkit-ffi";
import { existsSync } from "fs";

// you can point to any directory for the library to be downloaded at:
const libDir = "~/path/to/somewhere";

// you can get the path to library with `getLibPath`, it will attach the machine information to the file,
// e.g.: on MacOS & Apple Silicon it will be:
//   ~/path/to/somewhere/libcircomkit_ffi-macOS-arm64.dylib
const libPath = getLibPath(libDir);

// now we can download if needed
if (!existsSync(libPath)) {
  console.info("Downloading FFI library.");
  await downloadRelease(libDir);
}
```

### Preparing Node SDK

The Node SDK is exported from `/node` path, and requires `ffi-rs` as a peer dependency.

```ts
import { CircomkitFFINode } from "circomkit-ffi/node";
import { open, load, close } from "ffi-rs";

// assume library exists at path `libPath`
const lib = new CircomkitFFINode(libPath, open, close, load);
```

### Preparing Bun SDK

The Bun SDK is exported from `/bun` path, and works only for Bun runtime; trying to import this within Node runtime will cause an error.

```ts
import { CircomkitFFIBun } from "circomkit-ffi/bun";

// assume library exists at path `libPath`
const lib = new CircomkitFFIBun(libPath);
```

### Using the SDK

We are all set! We now have access to all the functions within our static library. For example, we can call `arkworks_prove` to generate a proof using Arkworks. We just have to provide the necessary paths to R1CS, witness file and the prover key.

We can use a Circomkit instance to get these paths, but they can be manually written as well:

```ts
const { proof, publicSignals } = lib.arkworks_prove(
  circomkit.path.ofCircuitWithInput(circuitName, inputName, "wtns"),
  circomkit.path.ofCircuit(circuitName, "r1cs"),
  circomkit.path.ofCircuit(circuitName, "pkey")
);
```

> [!TIP]
>
> If for any reason you have to know whether you are in Bun or Node, you can use the `isBun` function exported by our SDK.

## Development

Clone the repository:

```sh
git clone https://github.com/erhant/circomkit-ffi
```

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
- you need generated circuit files, which can be done using the commands below.

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

> [!TIP]
>
> You can take the library directly from within `/target/debug/libcircomkit_ffi.<your-extension>` and use with the SDK, for easier debugging with the SDK tests.

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

## Example

See the [`example`](./example/) folder.

## Benchmarks

See the [`bench`](./bench/) folder.

## Acknowledgements

This project is kindly supported by [Soulforge zkBankai](https://soulforge.zkbankai.com/) grant, with the application [here](https://github.com/zk-bankai/soulforge/blob/main/applications/circomkit-bunffi.md).

Some helpful resources for this project on FFI usage were:

- <https://jakegoulding.com/rust-ffi-omnibus/string_return/>
- <https://jakegoulding.com/rust-ffi-omnibus/string_arguments/>
- <https://bun.sh/docs/api/ffi>
- <https://github.com/node-ffi/node-ffi/wiki/Node-FFI-Tutorial>
- <https://tokio.rs/tokio/topics/bridging>

## License

This project is [MIT](./LICENSE) licensed.
