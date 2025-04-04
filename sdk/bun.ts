import { dlopen, FFIType } from "bun:ffi";
import type {
  IcicleDevice,
  ProofWithPublicSignals,
  ProverBackend,
} from "./interface";
import { isBun } from "./common";
import { existsSync } from "fs";

/**
 * A prover backend for Bun environments.
 *
 * This class is used to interact with the Circomkit FFI shared library, using `bun-ffi`.
 *
 * @implements ProverBackend
 */
export class CircomkitFFIBun implements ProverBackend {
  ffiName: string = "bun:ffi";

  constructor(
    /** Path to the library, */
    readonly path: string
  ) {
    // ensure that Bun is the current environment
    if (!isBun()) {
      throw new Error("This is not a Bun environment!");
    }

    // ensure path exists
    if (!existsSync(path)) {
      throw new Error(`No library exists at ${path}.`);
    }
  }

  echo(input: string): string {
    const {
      symbols: { echo },
    } = dlopen(this.path, {
      echo: {
        args: [FFIType.cstring],
        returns: FFIType.cstring,
      },
    });

    const result = echo(new Uint8Array(Buffer.from(input + "\0", "utf8")));
    return result.toString();
  }

  arkworks_prove(
    wtnsPath: string,
    r1csPath: string,
    pkeyPath: string
  ): ProofWithPublicSignals {
    const {
      symbols: { arkworks_prove },
    } = dlopen(this.path, {
      arkworks_prove: {
        args: [FFIType.cstring, FFIType.cstring, FFIType.cstring],
        returns: FFIType.cstring,
      },
    });

    const result = arkworks_prove(
      new Uint8Array(Buffer.from(wtnsPath + "\0", "utf8")),
      new Uint8Array(Buffer.from(r1csPath + "\0", "utf8")),
      new Uint8Array(Buffer.from(pkeyPath + "\0", "utf8"))
    );

    return JSON.parse(result.toString());
  }

  icicle_prove(
    wtnsPath: string,
    pkeyPath: string,
    device: IcicleDevice
  ): ProofWithPublicSignals {
    const {
      symbols: { icicle_prove },
    } = dlopen(this.path, {
      icicle_prove: {
        args: [FFIType.cstring, FFIType.cstring, FFIType.cstring],
        returns: FFIType.cstring,
      },
    });

    const result = icicle_prove(
      new Uint8Array(Buffer.from(wtnsPath + "\0", "utf8")),
      new Uint8Array(Buffer.from(pkeyPath + "\0", "utf8")),
      new Uint8Array(Buffer.from(device + "\0", "utf8"))
    );
    return JSON.parse(result.toString());
  }

  lambdaworks_prove(
    wtnsPath: string,
    r1csPath: string
  ): ProofWithPublicSignals {
    // make sure r1cs path is JSON
    if (!r1csPath.endsWith(".json")) {
      throw new Error("r1csPath must be a JSON file");
    }

    const {
      symbols: { lambdaworks_prove },
    } = dlopen(this.path, {
      lambdaworks_prove: {
        args: [FFIType.cstring, FFIType.cstring],
        returns: FFIType.cstring,
      },
    });

    const result = lambdaworks_prove(
      new Uint8Array(Buffer.from(wtnsPath + "\0", "utf8")),
      new Uint8Array(Buffer.from(r1csPath + "\0", "utf8"))
    );
    return JSON.parse(result.toString());
  }
}
