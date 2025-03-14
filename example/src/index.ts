import { Circomkit } from "circomkit";
import { downloadRelease, getLibPath, isBun } from "circomkit-ffi";
// import { CircomkitFFIBun } from "circomkit-ffi/bun";
import { CircomkitFFINode } from "circomkit-ffi/node";
import { open, load, close } from "ffi-rs"; // used by CircomkitFFINode
import { existsSync, readFileSync } from "fs";
import path from "path";
import * as snarkjs from "snarkjs";
import { fileURLToPath } from "url";

// set this to `true` if you would like to build the circuits as well
const BUILD_CIRCUIT = false;

// this is used by snarkjs prover function,
// must be explicity set to use single thread when the runtime is Bun
const USE_SINGLE_THREAD = isBun() ? { singleThread: true } : undefined;

const circomkit = new Circomkit({
  inspect: false,
});

// download the FFI library if it doesn't exist
const thisPath = path.dirname(fileURLToPath(import.meta.url));
const libPath = getLibPath(thisPath);
if (!existsSync(libPath)) {
  console.info("Downloading FFI library.");
  await downloadRelease(thisPath);
}

console.log("Using FFI library at", libPath, "for", isBun() ? "Bun" : "Node");

for (const circomkitFFI of [
  // with bun:
  // new CircomkitFFIBun(libPath),
  // with node:
  new CircomkitFFINode(libPath, open, close, load),
]) {
  // echo test to fail early
  {
    const echoMe = "Hello, world!";
    const echoResult = circomkitFFI.echo(echoMe);
    if (echoResult !== echoMe) {
      throw new Error(`Echo test failed: ${echoMe} != ${echoResult}`);
    }
  }

  // warm-ups
  {
    console.info("Doing warm-up iterations...");
    const warmUpCircuit = "multiplier_3";
    const warmupInput = "default";

    // load verification key as well
    const vk = JSON.parse(readFileSync(circomkit.path.ofCircuit(warmUpCircuit, "vkey"), "utf-8"));

    // generate with arkworks
    const { proof, publicSignals } = circomkitFFI.arkworks_prove(
      circomkit.path.ofCircuitWithInput(warmUpCircuit, warmupInput, "wtns"),
      circomkit.path.ofCircuit(warmUpCircuit, "r1cs"),
      circomkit.path.ofCircuit(warmUpCircuit, "pkey")
    );
    // verify Arkworks proof (only if not Bun, because it fails due to some worker error)
    if (!isBun()) {
      const ok = await snarkjs.groth16.verify(vk, publicSignals, proof, USE_SINGLE_THREAD);
      if (!ok) {
        throw new Error("Verification failed");
      }
    }

    // generate with snarkjs
    await snarkjs.groth16.prove(
      circomkit.path.ofCircuit(warmUpCircuit, "pkey"),
      circomkit.path.ofCircuitWithInput(warmUpCircuit, warmupInput, "wtns"),
      undefined,
      USE_SINGLE_THREAD
    );
  }

  for (const N of [3, 30, 300, 3000, 30000, 300000]) {
    const IN = Array.from({ length: N }, (_, i) => i + 1);
    const circuitName = `multiplier_${N}`;
    const inputName = "default";

    if (BUILD_CIRCUIT) {
      console.info(`Building circuit for multiplier_${N}`);
      const buildPath = await circomkit.compile(circuitName, {
        file: "multiplier",
        template: "Multiplier",
        params: [N],
      });
      console.info(`Compiled circuit to ${buildPath}`);

      console.info("Exporting input");
      const path = circomkit.input(circuitName, inputName, { in: IN });
      console.info(`Input exported to ${path}`);

      console.info("Creating a witness...");
      const witnessPath = await circomkit.witness(circuitName, inputName);
      console.info(`Witness created at ${witnessPath}`);
    }

    console.info(`\nProving for multiplier_${N}`);
    const NUM_ITERS = 5; // this many iters each

    // Arkworks benchmarking
    {
      const times: number[] = [];

      for (let i = 0; i < NUM_ITERS; i++) {
        const start = performance.now();
        circomkitFFI.arkworks_prove(
          circomkit.path.ofCircuitWithInput(circuitName, inputName, "wtns"),
          circomkit.path.ofCircuit(circuitName, "r1cs"),
          circomkit.path.ofCircuit(circuitName, "pkey")
        );
        const end = performance.now();
        times.push(end - start);
      }

      const average = times.reduce((a, b) => a + b, 0) / NUM_ITERS;
      console.info(`Arkworks average time (${NUM_ITERS} runs): ${average.toFixed(2)}ms`);
    }

    // SnarkJS benchmarking
    {
      const times: number[] = [];

      for (let i = 0; i < NUM_ITERS; i++) {
        const start = performance.now();
        await snarkjs.groth16.prove(
          circomkit.path.ofCircuit(circuitName, "pkey"),
          circomkit.path.ofCircuitWithInput(circuitName, inputName, "wtns"),
          undefined,
          USE_SINGLE_THREAD
        );
        const end = performance.now();
        times.push(end - start);
      }

      const average = times.reduce((a, b) => a + b, 0) / NUM_ITERS;
      console.info(`SnarkJS average time (${NUM_ITERS} runs): ${average.toFixed(2)}ms`);
    }
  }
}
