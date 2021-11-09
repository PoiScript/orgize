# orgize

![npm](https://img.shields.io/npm/v/orgize)

## Install

```sh
npm install orgize
yarn add orgize
```

## Browser

```js
import init, { Org } from "orgize";

init().then(() => {
  const org = new Org("* Hello, /world/!");
  const html = org.html();
  console.log(html);
  org.free();
});
```

## Node.js

```js
import { Org, initSync } from "orgize";
import { readFile } from "node:fs/promises";

// you can also use import.meta.resolve, but it's currently behind
// an experimental flag --experimental-import-meta-resolve
import { createRequire } from "node:module";
const require = createRequire(import.meta.url);

readFile(require.resolve("orgize/wasm")).then((bytes) => {
  initSync(bytes);

  const org = new Org("* Hello, /world/!");
  const html = org.html();
  console.log(html);
  org.free();
});
```

## Notes

1. You must **initialize** the WebAssembly module (using either `init` or
   `initSync` function) before using the `Org` class;

2. Don't forgot to call `org.free()` to **release the memory** that
   allocated by Rust;

3. This npm package is primarily aim to demonstrate and power the online
   demo, so it doesn't provide any customization or settings.

   If you need to, please build your own npm package by `wasm-pack`.
   (or `napi` if you're only targeting node.js users)

## License

MIT
