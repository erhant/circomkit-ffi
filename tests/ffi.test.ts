import { beforeAll, describe, expect, it } from "bun:test";
import { existsSync } from "fs";
import { open, close, load } from "ffi-rs";
import { Circomkit } from "circomkit";

import { downloadRelease, getLibPath } from "../sdk";
import { CircomkitFFIBun } from "../sdk/bun";
import { CircomkitFFINode } from "../sdk/node";

describe("ffi", () => {
  let libpath: string = getLibPath(".");

  // we use these specifically for testing
  const circuitName = "multiplier_30";
  const inputName = "default";

  let circomkit: Circomkit;

  beforeAll(async () => {
    // download the library if required
    if (!existsSync(libpath)) {
      libpath = await downloadRelease(".");
    }
    expect(existsSync(libpath)).toBeTrue();

    circomkit = new Circomkit({
      inspect: false,
    });
  });

  describe("echo", () => {
    it("should work with bun:ffi (Bun)", () => {
      const lib = new CircomkitFFIBun(libpath);
      const input = "hi theree";
      const output = lib.echo(input);
      expect(output).toEqual(input);
    });

    it("should work with ffi-rs (Node)", () => {
      const lib = new CircomkitFFINode(libpath, open, close, load);
      const input = "hi theree";
      const output = lib.echo(input);
      expect(output).toEqual(input);
    });
  });

  describe("Arkworks", () => {
    it("should generate a valid Arkworks proof with CircomkitFFIBun", () => {
      const lib = new CircomkitFFIBun(libpath);

      const [witnessPath, r1csPath, pkeyPath] = [
        circomkit.path.ofCircuitWithInput(circuitName, inputName, "wtns"),
        circomkit.path.ofCircuit(circuitName, "r1cs"),
        circomkit.path.ofCircuit(circuitName, "pkey"),
      ].map((path) => import.meta.dir + "/../example/" + path);

      // Check if required files exist before attempting to prove
      expect(existsSync(witnessPath)).toBeTrue();
      expect(existsSync(r1csPath)).toBeTrue();
      expect(existsSync(pkeyPath)).toBeTrue();

      const result = lib.arkworks_prove(witnessPath, r1csPath, pkeyPath);

      // Verify the proof structure
      expect(result).toBeDefined();
      expect(result.proof).toBeDefined();
      expect(result.publicSignals).toBeDefined();
      expect(Array.isArray(result.publicSignals)).toBeTrue();
    });

    it("should generate a valid Arkworks proof with CircomkitFFINode", () => {
      const lib = new CircomkitFFINode(libpath, open, close, load);

      const [witnessPath, r1csPath, pkeyPath] = [
        circomkit.path.ofCircuitWithInput(circuitName, inputName, "wtns"),
        circomkit.path.ofCircuit(circuitName, "r1cs"),
        circomkit.path.ofCircuit(circuitName, "pkey"),
      ].map((path) => import.meta.dir + "/../example/" + path);

      // Check if required files exist before attempting to prove
      expect(existsSync(witnessPath)).toBeTrue();
      expect(existsSync(r1csPath)).toBeTrue();
      expect(existsSync(pkeyPath)).toBeTrue();

      const result = lib.arkworks_prove(witnessPath, r1csPath, pkeyPath);

      // Verify the proof structure
      expect(result).toBeDefined();
      expect(result.proof).toBeDefined();
      expect(result.publicSignals).toBeDefined();
      expect(Array.isArray(result.publicSignals)).toBeTrue();
    });
  });

  describe("Lambdaworks", () => {
    it("should generate a valid Lambdaworks proof with CircomkitFFIBun", () => {
      const lib = new CircomkitFFIBun(libpath);

      const [witnessPath, r1csPath, pkeyPath] = [
        circomkit.path.ofCircuitWithInput(circuitName, inputName, "wtns"),
        circomkit.path.ofCircuit(circuitName, "r1cs") + ".json",
        circomkit.path.ofCircuit(circuitName, "pkey"),
      ].map((path) => import.meta.dir + "/../example/" + path);

      // Check if required files exist before attempting to prove
      expect(existsSync(witnessPath)).toBeTrue();
      expect(existsSync(r1csPath)).toBeTrue();
      expect(existsSync(pkeyPath)).toBeTrue();

      const result = lib.lambdaworks_prove(witnessPath, r1csPath);

      // Verify the proof structure
      expect(result).toBeDefined();
      expect(result.proof).toBeDefined();
      expect(result.publicSignals).toBeDefined();
      expect(Array.isArray(result.publicSignals)).toBeTrue();
    });

    it("should generate a valid Lambdaworks proof with CircomkitFFINode", () => {
      const lib = new CircomkitFFINode(libpath, open, close, load);

      const [witnessPath, r1csPath, pkeyPath] = [
        circomkit.path.ofCircuitWithInput(circuitName, inputName, "wtns"),
        circomkit.path.ofCircuit(circuitName, "r1cs"),
        circomkit.path.ofCircuit(circuitName, "pkey"),
      ].map((path) => import.meta.dir + "/../example/" + path);

      // Check if required files exist before attempting to prove
      expect(existsSync(witnessPath)).toBeTrue();
      expect(existsSync(r1csPath)).toBeTrue();
      expect(existsSync(pkeyPath)).toBeTrue();

      const result = lib.arkworks_prove(witnessPath, r1csPath, pkeyPath);

      // Verify the proof structure
      expect(result).toBeDefined();
      expect(result.proof).toBeDefined();
      expect(result.publicSignals).toBeDefined();
      expect(Array.isArray(result.publicSignals)).toBeTrue();
    });
  });
});
