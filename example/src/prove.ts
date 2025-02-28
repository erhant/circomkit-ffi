import { Circomkit } from "circomkit";
import { CircomkitFFIBun, CircomkitFFINode } from "circomkit-ffi";
import { existsSync } from "fs";

const circomkit = new Circomkit({
  inspect: false,
});
const N = 3;
const IN = Array.from({ length: N }, (_, i) => i + 1);

const OUT = IN.reduce((acc, x) => acc * x);

const circuitName = `multiplier_${N}`;
const inputName = "default";

console.info("Building circuit...");
const buildPath = await circomkit.compile(circuitName, {
  file: "multiplier",
  template: "Multiplier",
  params: [N],
});
console.info(`Compiled circuit to ${buildPath}`);

console.info("Creating a witness...");
const witnessPath = await circomkit.witness(circuitName, inputName, {
  in: IN,
});
console.info(`Witness created at ${witnessPath}`);

// console.info("Generating a proof with Arkworks");
const circomkitFFI = new CircomkitFFIBun("./");
const proof = circomkitFFI.arkworks_prove(
  circomkit.path.ofCircuitWithInput(circuitName, inputName, "wtns"),
  circomkit.path.ofCircuit(circuitName, "r1cs"),
  circomkit.path.ofPtau("powersOfTau28_hez_final_08.ptau")
);

console.log(proof);
