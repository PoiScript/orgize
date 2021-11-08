import dts from "rollup-plugin-dts";
import copy from "rollup-plugin-copy";

export default [
  {
    input: "./out-tsc/index.d.ts",
    output: {
      file: "./lib/orgize.d.ts",
    },
    plugins: [dts()],
  },
  {
    input: "./out-tsc/index.js",
    output: [
      {
        file: "./lib/orgize.es.js",
        format: "es",
      },
      {
        name: "orgize",
        file: "./lib/orgize.umd.js",
        format: "umd",
      },
    ],
    plugins: [
      copy({
        targets: [{ src: "index.html", dest: "lib" }],
      }),
    ],
  },
];
