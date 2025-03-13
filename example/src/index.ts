import { Circomkit } from "circomkit";
import { downloadRelease, getLibPath, isBun } from "circomkit-ffi";
// import { CircomkitFFIBun as CircomkitFFI } from "circomkit-ffi/bun";
import { CircomkitFFINode as CircomkitFFI } from "circomkit-ffi/node";
import { open, load, close } from "ffi-rs";
import { existsSync, readFileSync } from "fs";
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
// const circomkitFFI = new CircomkitFFI(libPath);
const circomkitFFI = new CircomkitFFI(libPath, open, close, load);

// echo test
console.log(circomkitFFI.echo("Hello, world!"));
// const verifierKey: object = JSON.parse(readFileSync(circomkit.path.ofCircuit(circuitName, "vkey"), "utf-8"));

for (const N of [3, 30 /* 300, 3000, 30000, 300000 */]) {
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

  console.info(`Proving for multiplier_${N}`);

  {
    console.time("Generating a proof with Arkworks");
    const arkworkOutput = circomkitFFI.arkworks_prove(
      circomkit.path.ofCircuitWithInput(circuitName, inputName, "wtns"),
      circomkit.path.ofCircuit(circuitName, "r1cs"),
      circomkit.path.ofCircuit(circuitName, "pkey")
    );
    console.timeEnd("Generating a proof with Arkworks");

    // console.info("Proof generated:");
    // const { proof, publicSignals } = arkworkOutput;
    // console.log(proof);
  }

  {
    console.time("Generating a proof with SnarkJS");
    const snarkjsOutput = await snarkjs.groth16.prove(
      circomkit.path.ofCircuit(circuitName, "pkey"),
      circomkit.path.ofCircuitWithInput(circuitName, inputName, "wtns"),
      undefined,
      { singleThread: true }
    );
    console.timeEnd("Generating a proof with SnarkJS");

    // console.info("Proof generated:");
    // const { proof, publicSignals } = snarkjsOutput;
    // console.log(proof);
  }
}
