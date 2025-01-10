# Circomkit FFI

This repository contains an all-in-one adapter for several backends, mainly to be used by existing Javascript code via FFI. Its features:

- [x] Provers:
  - [x] [Lambdaworks Circom Groth16 (BLS12-381) Adapter](https://github.com/lambdaclass/lambdaworks/tree/main/provers/groth16/circom-adapter)
  - [x] [Arkworks Circom Groth16 (BN254) Adapter](https://github.com/arkworks-rs/circom-compat)
- [ ] Witness Calculators:
  - [ ] [Iden3 Circom Witnesscalc](https://github.com/iden3/circom-witnesscalc)
- [x] SnarkJS Compatiblity
  - [x] JSON proof export
  - [x] JSON public signal export

## Installation

TODO:

maybe we do feature-gating and have a separate library for each backend?
or just a single backend for all

## Usage

TODO:

## Acknowledgements

This project is kindly supported by [Soulforge zkBankai](https://soulforge.zkbankai.com/) grant, with the application [here](https://github.com/zk-bankai/soulforge/blob/main/applications/circomkit-bunffi.md). The grant is given for 2 months with 1 milestone at the end of per month, the project is expected to be finished within that duration.
