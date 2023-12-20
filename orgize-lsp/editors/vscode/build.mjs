import * as esbuild from "esbuild";

await esbuild.build({
  bundle: true,
  entryPoints: ["src/main.ts"],
  external: ["vscode"],
  outfile: "dist/main.js",
  format: "cjs",
  platform: "node",
  target: "node16",
  minify: true,
  treeShaking: true,
});
