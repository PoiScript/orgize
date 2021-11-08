const { readFile } = require("fs/promises");
const { resolve } = require("path");
const { deepStrictEqual } = require("assert");

const { init, keywords } = require("../lib/orgize.umd");

const assert = (org, kw) => deepStrictEqual(keywords(org), kw);

readFile(resolve(__dirname, "../lib/orgize_bg.wasm"))
  .then((bytes) => new WebAssembly.Module(bytes))
  .then((module) => init(module))
  .then(() => {
    assert("#+TITLE: orgize test cases\n#+FOO: bar", {
      TITLE: ["orgize test cases"],
      FOO: ["bar"],
    });
  });
