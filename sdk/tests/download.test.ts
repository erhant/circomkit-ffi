import { describe, it } from "bun:test";
import { downloadRelease } from "../src";

describe("library", () => {
  it("should download", async () => {
    const path = await downloadRelease(".");
    console.log(path);
  });
});
