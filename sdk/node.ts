// import { DataType, open, close, load } from "ffi-rs";
// import type { ProverBackend } from "./interface";

// /**
//  * A prover backend for Node environments.
//  *
//  * This class is used to interact with the Circomkit FFI shared library, using `ffi-rs`.
//  *
//  * @implements ProverBackend
//  */
// export class CircomkitFFINode implements ProverBackend {
//   readonly LIBRARY_NAME = "libcircomkit_ffi";

//   /** Whether the `lib` is open. */
//   isOpen = false;

//   constructor(readonly path: string) {}

//   echo(input: string): string {
//     this.openIfClosed();
//     const result = load({
//       library: this.LIBRARY_NAME,
//       funcName: "echo",
//       paramsType: [DataType.String],
//       retType: DataType.String,
//       paramsValue: [input],
//     });
//     this.closeIfOpen();

//     return result;
//   }

//   arkworks_prove(wtnsPath: string, r1csPath: string, pkeyPath: string): string {
//     this.openIfClosed();
//     const result = load({
//       library: this.LIBRARY_NAME,
//       funcName: "arkworks_prove",
//       paramsType: [DataType.String, DataType.String, DataType.String],
//       retType: DataType.String,
//       paramsValue: [wtnsPath, r1csPath, pkeyPath],
//     });
//     this.closeIfOpen();

//     return result;
//   }

//   lambdaworks_prove(wtnsPath: string, r1csPath: string): string {
//     this.openIfClosed();
//     const result = load({
//       library: this.LIBRARY_NAME,
//       funcName: "lambdaworks_prove",
//       paramsType: [DataType.String, DataType.String],
//       retType: DataType.String,
//       paramsValue: [wtnsPath, r1csPath],
//     });
//     this.closeIfOpen();

//     return result;
//   }

//   /** Opens the library for usage, must be done prior to calling functions here. */
//   private openIfClosed() {
//     if (!this.isOpen) {
//       open({
//         library: this.LIBRARY_NAME,
//         path: this.path,
//       });
//       this.isOpen = true;
//     }
//   }

//   /** Closes the library & frees its memory. */
//   private closeIfOpen() {
//     if (this.isOpen) {
//       close(this.LIBRARY_NAME);
//       this.isOpen = false;
//     }
//   }

//   // additional safety measure
//   [Symbol.dispose]() {
//     this.closeIfOpen();
//   }
// }
