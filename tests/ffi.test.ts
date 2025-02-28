import { beforeAll, describe, expect, it } from "bun:test";
import { getLibPath } from "../sdk";
import { CircomkitFFIBun } from "../sdk/bun";
import { CircomkitFFINode } from "../sdk/node";
import { existsSync } from "fs";

describe("ffi", () => {
  const PATH = getLibPath(".");

  beforeAll(() => {
    expect(existsSync(PATH)).toBeTrue();
  });

  it("should work with bun:ffi (Bun)", () => {
    const lib = new CircomkitFFIBun(PATH);
    const input = "hi theree";
    const output = lib.echo(input);
    expect(output.toString()).toEqual(input);
  });

  it("should work with ffi-rs (Node)", () => {
    const lib = new CircomkitFFINode(PATH);
    const input = "hi theree";
    const output = lib.echo(input);
    expect(output.toString()).toEqual(input);
  });
});
