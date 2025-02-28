import { describe, it } from "bun:test";
import { downloadRelease } from "../sdk";

describe.skip("download release", () => {
  it("should download", async () => {
    const path = await downloadRelease(".");
    console.log(path);
  });
});
