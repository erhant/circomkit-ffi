import { DataType, open, close, define } from "ffi-rs";

export function circomkitFFINode(path: string) {
  // unique name for the library
  const LIBRARY_NAME = "libcircomkit_ffi";

  // Use define function to define a function signature
  const lib = define({
    echo: {
      library: LIBRARY_NAME,
      paramsType: [DataType.String],
      retType: DataType.String,
    },
    // atoi: {
    //   library: path,
    //   paramsType: [DataType.String],
    //   retType: DataType.I32,
    // },
  });
  // equal(res.sum([1, 2]), 3);
  // equal(res.atoi(["1000"]), 1000);

  return {
    lib,
    /** Opens the library for usage, must be done prior to calling functions here. */
    open: () => {
      open({
        library: LIBRARY_NAME,
        path,
      });
    },
    /** Closes the library & frees its memory. */
    close: () => close(LIBRARY_NAME),
  };
}
