import {
  type DataType,
  type open as ffiOpen,
  type close as ffiClose,
  type load as ffiLoad,
} from "ffi-rs";

import type { ProofWithPublicSignals, ProverBackend } from "./interface";

const DataTypeString = 0 satisfies DataType.String;

/**
 * A prover backend for Node environments.
 *
 * This class is used to interact with the Circomkit FFI shared library, using `ffi-rs`.
 *
 * Due to how `ffi-rs` works internally, the `open`, `close`, and `load` functions must be passed in as arguments.
 * These can be imported from `ffi-rs` directly.
 *
 * @example
 * ```ts
 * import { open, close, load } from "ffi-rs";
 * import { CircomkitFFINode } from "circomkit-ffi";
 *
 * const circomkitFFI = new CircomkitFFINode("path/to/libcircomkit_ffi.so", open, close, load);
 * ```
 *
 * @implements ProverBackend
 */
export class CircomkitFFINode implements ProverBackend {
  readonly LIBRARY_NAME = "libcircomkit_ffi";

  /** Whether the `lib` is open. */
  isOpen = false;

  constructor(
    /** Path to the library. */
    readonly path: string,
    /** The `open` function from `ffi-rs`. */
    readonly open: typeof ffiOpen,
    /** The `close` function from `ffi-rs`. */
    readonly close: typeof ffiClose,
    /** The `load` function from `ffi-rs`. */
    readonly load: typeof ffiLoad
  ) {}

  echo(input: string): string {
    this.openIfClosed();
    const result = this.load({
      library: this.LIBRARY_NAME,
      funcName: "echo",
      paramsType: [DataTypeString],
      retType: DataTypeString,
      paramsValue: [input],
    });
    this.closeIfOpen();

    return result;
  }

  arkworks_prove(
    wtnsPath: string,
    r1csPath: string,
    pkeyPath: string
  ): ProofWithPublicSignals {
    this.openIfClosed();
    const result = this.load({
      library: this.LIBRARY_NAME,
      funcName: "arkworks_prove",
      paramsType: [DataTypeString, DataTypeString, DataTypeString],
      retType: DataTypeString,
      paramsValue: [wtnsPath, r1csPath, pkeyPath],
    });
    this.closeIfOpen();

    return JSON.parse(result);
  }

  lambdaworks_prove(
    wtnsPath: string,
    r1csPath: string
  ): ProofWithPublicSignals {
    this.openIfClosed();
    const result = this.load({
      library: this.LIBRARY_NAME,
      funcName: "lambdaworks_prove",
      paramsType: [DataTypeString, DataTypeString],
      retType: DataTypeString,
      paramsValue: [wtnsPath, r1csPath],
    });
    this.closeIfOpen();

    return JSON.parse(result);
  }

  /** Opens the library for usage, must be done prior to calling functions here. */
  private openIfClosed() {
    if (!this.isOpen) {
      this.open({
        library: this.LIBRARY_NAME,
        path: this.path,
      });
      this.isOpen = true;
    }
  }

  /** Closes the library & frees its memory. */
  private closeIfOpen() {
    if (this.isOpen) {
      this.close(this.LIBRARY_NAME);
      this.isOpen = false;
    }
  }

  // additional safety measure
  [Symbol.dispose]() {
    this.closeIfOpen();
  }
}
