import { Circomkit } from "circomkit";
import { downloadRelease, getLibPath, isBun } from "circomkit-ffi";
import { CircomkitFFIBun } from "circomkit-ffi/bun";
// import { CircomkitFFINode } from "circomkit-ffi/node";
// import { open, load, close } from "ffi-rs"; // used by CircomkitFFINode
import { existsSync } from "fs";
import path from "path";
import * as snarkjs from "snarkjs";
import { fileURLToPath } from "url";

const BUILD_CIRCUIT = false;

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

for (const circomkitFFI of [new CircomkitFFIBun(libPath) /* new CircomkitFFINode(libPath, open, close, load) */]) {
  // echo test to fail early
  const echoMe = "Hello, world!";
  const echoResult = circomkitFFI.echo(echoMe);
  // FIXME: the lengths here differ due to encodings between Bun and Node
  // console.log({
  //   len1: echoMe.length,
  //   len2: echoResult.length,
  // });
  if (echoResult !== echoMe) {
    throw new Error(`Echo test failed: ${echoMe} != ${echoResult}`);
  }

  // warm-ups
  const warmupN = 3;
  console.info("Warm-up iteration...");
  {
    const circuitName = `multiplier_${warmupN}`;
    circomkitFFI.arkworks_prove(
      circomkit.path.ofCircuitWithInput(circuitName, "default", "wtns"),
      circomkit.path.ofCircuit(circuitName, "r1cs"),
      circomkit.path.ofCircuit(circuitName, "pkey")
    );
    await snarkjs.groth16.prove(
      circomkit.path.ofCircuit(circuitName, "pkey"),
      circomkit.path.ofCircuitWithInput(circuitName, "default", "wtns"),
      undefined,
      { singleThread: true }
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

    // Arkworks benchmarking
    {
      const iterations = 5;
      const times: number[] = [];

      for (let i = 0; i < iterations; i++) {
        const start = performance.now();
        circomkitFFI.arkworks_prove(
          circomkit.path.ofCircuitWithInput(circuitName, inputName, "wtns"),
          circomkit.path.ofCircuit(circuitName, "r1cs"),
          circomkit.path.ofCircuit(circuitName, "pkey")
        );
        const end = performance.now();
        times.push(end - start);
      }

      const average = times.reduce((a, b) => a + b, 0) / iterations;
      console.info(`Arkworks average time (${iterations} runs): ${average.toFixed(2)}ms`);
    }

    // SnarkJS benchmarking
    {
      const iterations = 5;
      const times: number[] = [];

      for (let i = 0; i < iterations; i++) {
        const start = performance.now();
        await snarkjs.groth16.prove(
          circomkit.path.ofCircuit(circuitName, "pkey"),
          circomkit.path.ofCircuitWithInput(circuitName, inputName, "wtns"),
          undefined,
          { singleThread: true }
        );
        const end = performance.now();
        times.push(end - start);
      }

      const average = times.reduce((a, b) => a + b, 0) / iterations;
      console.info(`SnarkJS average time (${iterations} runs): ${average.toFixed(2)}ms`);
    }
  }
}
