// await Bun.$`rm -rf dist`;
//
// taken from: https://github.com/oven-sh/bun/issues/5141#issuecomment-2595032410

import type { BunPlugin } from "bun";
import { isolatedDeclaration } from "oxc-transform";

function getDtsBunPlugin(): BunPlugin {
  const wroteTrack = new Set<string>();
  return {
    name: "oxc-transform-dts",
    setup(builder) {
      if (builder.config.root && builder.config.outdir) {
        const rootPath = Bun.pathToFileURL(builder.config.root).pathname;
        const outPath = Bun.pathToFileURL(builder.config.outdir).pathname;
        builder.onStart(() => wroteTrack.clear());
        builder.onLoad({ filter: /\.ts$/ }, async (args) => {
          if (args.path.startsWith(rootPath) && !wroteTrack.has(args.path)) {
            wroteTrack.add(args.path);
            const { code } = isolatedDeclaration(
              args.path,
              await Bun.file(args.path).text()
            );
            await Bun.write(
              args.path
                .replace(new RegExp(`^${rootPath}`), outPath)
                .replace(/\.ts$/, ".d.ts"),
              code
            );
          }
          return undefined;
        });
      }
    },
  };
}

const result = await Bun.build({
  entrypoints: ["src/index.ts"],
  root: "src",
  outdir: "dist",
  // minify: true,
  // splitting: true,
  target: "node",
  plugins: [getDtsBunPlugin()],
});
if (!result.success) {
  for (const log of result.logs) {
    console.error(log);
  }
}
