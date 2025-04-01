import { existsSync } from "fs";
import path from "path";
import { fileURLToPath } from "url";
import { Circomkit } from "circomkit";
import { downloadRelease, getLibPath } from "circomkit-ffi";
import { CircomkitFFIBun } from "circomkit-ffi/bun";

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

const circomkitFFI = new CircomkitFFIBun(libPath);

// generate with Arkworks
const [CIRCUIT, INPUT] = ["multiplier_3", "default"];
const { proof, publicSignals } = circomkitFFI.arkworks_prove(
  circomkit.path.ofCircuitWithInput(CIRCUIT, INPUT, "wtns"),
  circomkit.path.ofCircuit(CIRCUIT, "r1cs"),
  circomkit.path.ofCircuit(CIRCUIT, "pkey")
);
console.log(proof);
console.log(publicSignals);
