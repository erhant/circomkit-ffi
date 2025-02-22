import { beforeAll, describe, expect, it } from "bun:test";
import { circomkitFFIBun, circomkitFFINode, getLibPath } from "../src";
import { existsSync } from "fs";

describe("ffi", () => {
  const PATH = getLibPath(".");

  beforeAll(() => {
    expect(existsSync(PATH)).toBeTrue();
  });

  it("should work with bun:ffi (Bun)", () => {
    const lib = circomkitFFIBun(PATH);
    const input = "hi theree";
    const output = lib.symbols.echo(
      new Uint8Array(Buffer.from(input + "\0", "utf8"))
    );
    expect(output.toString()).toEqual(input);
  });

  it("should work with ffi-rs (Node)", () => {
    const { open, lib, close } = circomkitFFINode(PATH);
    const input = "hi theree";
    open();
    const output = lib.echo([input]);
    expect(output.toString()).toEqual(input);
    close();
  });
});
