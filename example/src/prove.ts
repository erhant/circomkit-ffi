import { Circomkit } from "circomkit";
import { CircomkitFFIBun, CircomkitFFINode, downloadRelease, getLibPath } from "circomkit-ffi";
import { open, load, close } from "ffi-rs";
import { existsSync } from "fs";
import * as snarkjs from "snarkjs";

const circomkit = new Circomkit({
  inspect: false,
});
const N = 30;
const IN = Array.from({ length: N }, (_, i) => i + 1);

const circuitName = `multiplier_${N}`;
const inputName = "default";

console.info("Building circuit...");
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

// console.info("Generating a proof with Arkworks");
const libPath = getLibPath(import.meta.dir);
if (!existsSync(libPath)) {
  console.info("Downloading FFI library.");
  await downloadRelease(import.meta.dir);
}

const circomkitFFI = new CircomkitFFIBun(libPath);
// const circomkitFFI = new CircomkitFFINode(libPath, open, close, load);

// const verifierKey: object = JSON.parse(readFileSync(circomkit.path.ofCircuit(circuitName, "vkey"), "utf-8"));

{
  console.info("Generating a proof with Arkworks");
  const arkworkOutput = circomkitFFI.arkworks_prove(
    circomkit.path.ofCircuitWithInput(circuitName, inputName, "wtns"),
    circomkit.path.ofCircuit(circuitName, "r1cs"),
    circomkit.path.ofCircuit(circuitName, "pkey")
  );

  const { proof, publicSignals } = arkworkOutput;
  console.info("Proof generated:");
  console.log(proof);
}

{
  console.info("Generating a proof with SnarkJS");
  const { proof, publicSignals } = await snarkjs.groth16.prove(
    circomkit.path.ofCircuit(circuitName, "pkey"),
    circomkit.path.ofCircuitWithInput(circuitName, inputName, "wtns"),
    undefined,
    { singleThread: true }
  );
  console.info("Proof generated:");
  console.log(proof);
}
