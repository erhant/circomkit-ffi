import type { Groth16Proof, PublicSignals } from "snarkjs";

export type ProofWithPublicSignals = {
  proof: Groth16Proof;
  publicSignals: PublicSignals;
};

export interface ProverBackend {
  /** The path to the shared library. */
  path: string;

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
   * @returns SnarkJS Groth16 proof & public signals TODO: return object here
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
   * @returns SnarkJS Groth16 proof & public signals TODO: return object here
   */
  lambdaworks_prove(wtnsPath: string, r1csPath: string): ProofWithPublicSignals;
}
