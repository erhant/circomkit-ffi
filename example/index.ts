import { dlopen, FFIType, suffix } from "bun:ffi";

// const path =
//   import.meta.dirname + `/../target/debug/libcircomkit_ffi.${suffix}`; // TODO: download these from github releases
const path = import.meta.dirname + `/lib/libcircomkit_ffi-macOS-arm64.dylib`; // FIXME: remove after testing
const lib = dlopen(path, {
  /** A diagnostic function, returns the given input back.
   *
   * @param input string to be echoed back
   * @returns the input string
   * @example
   * const result = lib.symbols.echo(new Uint8Array(Buffer.from("hi theree\0", "utf8")));
   * console.log(result.toString())
   */
  echo: {
    args: [FFIType.cstring],
    returns: FFIType.cstring,
  },
  /** Prove with Arkworks.
   *
   * @param wtns witness file path
   * @param r1cs r1cs file path
   * @param pk proving key file path
   * @returns SnarkJS groth16 proof as a string
   * @example
   * const result = lib.symbols.arkworks_prove(
   *   new Uint8Array(Buffer.from("../path/to/wtns.json\0", "utf8")),
   *   new Uint8Array(Buffer.from("../path/to/circuit.r1cs\0", "utf8")),
   *   new Uint8Array(Buffer.from("../path/to/circuit_groth16.zkey\0", "utf8"))
   * );
   * console.log(JSON.parse(result.toString()));
   */
  arkworks_prove: {
    args: [FFIType.cstring, FFIType.cstring, FFIType.cstring],
    returns: FFIType.cstring,
  },
});

// const result = lib.symbols.arkworks_prove(
//   new Uint8Array(Buffer.from("../tests/res/mul3.wtns.json\0", "utf8")),
//   new Uint8Array(Buffer.from("../tests/res/mul3.r1cs\0", "utf8")),
//   new Uint8Array(Buffer.from("../tests/res/mul3_groth16.zkey\0", "utf8"))
// );
// console.log(JSON.parse(result.toString()));

const ret = lib.symbols.echo(
  new Uint8Array(Buffer.from("hi theree\0", "utf8"))
);
console.log(ret.toString());
