import { CString, ptr, dlopen, FFIType, suffix } from "bun:ffi";

const path = `${
  import.meta.dirname
}/../target/debug/libcircomkit_ffi.${suffix}`;
const lib = dlopen(path, {
  test_me: {
    args: [FFIType.cstring],
  },
});

const arg_str = new Uint8Array(Buffer.from("hi theree\0", "utf8"));
console.log(arg_str);
lib.symbols.test_me(arg_str);
