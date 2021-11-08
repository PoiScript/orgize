# orgize

![npm](https://img.shields.io/npm/v/orgize)

## Quick start

Install the package:

```sh
npm install orgize
yarn add orgize
```

Load the wasm module and init:

### Browser

```js
import { init, renderHtml } from "orgize";

init().then(() => {
  console.log(renderHtml("* Hello, /world/!"));
});
```

### Node.js

```js
const { init, renderHtml } = require("orgize");
const { readFile } = require("fs/promises");

readFile(require.resolve("orgize/lib/orgize_bg.wasm"))
  .then((bytes) => init(new WebAssembly.Module(bytes)))
  .then(() => {
    console.log(renderHtml("* Hello, /world/!"));
  });
```

## License

MIT
