import type { Groth16Proof, PublicSignals } from "snarkjs";

export type ProofWithPublicSignals = {
  proof: Groth16Proof;
  publicSignals: PublicSignals;
};

export interface ProverBackend {
  /** The path to the shared library. */
  path: string;
  /** Name of the FFI library used. */
  ffiName: string;

  /**
   * A diagnostic function, returns the given input back.
   *
   * @param input string to be echoed back
   * @returns the input string
   */
  echo(input: string): string;

  /**
   * Prove with Arkworks.
   *
   * @param wtnsPath witness file path (`.json` or `.wtns.json`)
   * @param r1csPath r1cs file path (`.r1cs`)
   * @param zkeyPath proving key file path (`.zkey`)
   * @returns SnarkJS Groth16 proof & public signals
   */
  arkworks_prove(
    wtnsPath: string,
    r1csPath: string,
    pkeyPath: string
  ): ProofWithPublicSignals;

  /**
   * Prove with Lambdaworks.
   *
   * @param wtnsPath witness file path (`.json` or `.wtns.json`)
   * @param r1csPath r1cs file path (`.r1cs`)
   * @returns SnarkJS Groth16 proof & public signals
   */
  lambdaworks_prove(wtnsPath: string, r1csPath: string): ProofWithPublicSignals;

  /**
   * Prove with Ingonyama ICICLE.
   *
   * @param wtnsPath witness file path (`.json` or `.wtns.json`)
   * @param zkeyPath proving key file path (`.zkey`)
   * @param device device to be used for proving
   * @returns SnarkJS Groth16 proof & public signals
   * @deprecated **DO NOT USE UNTIL ICICLE IS FIXED**
   */
  icicle_prove(
    wtnsPath: string,
    r1csPath: string,
    device: IcicleDevice
  ): ProofWithPublicSignals;
}

/** Devices supported by ICICLE prover. */
export type IcicleDevice = "CPU" | "CUDA" | "METAL";
