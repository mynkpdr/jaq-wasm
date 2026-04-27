# jaq-wasm

[![npm version](https://img.shields.io/npm/v/jaq-wasm.svg)](https://www.npmjs.com/package/jaq-wasm)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

`jaq-wasm` is the WebAssembly build of [`jaq`](https://github.com/01mf02/jaq), a jq-like JSON processor written in Rust. This package exposes a small JavaScript-first API for running jaq filters in browsers and in Node.js.

## Installation

```bash
npm install jaq-wasm
```

## API

The package exposes two ergonomic helpers for most callers:

- `run(filter, input)` returns CLI-style output as a string.
- `runValues(filter, input)` returns the produced values as a JavaScript array.

If you already have raw JSON text, use the `runJson(...)` and `runJsonValues(...)` variants instead of pre-parsing it yourself. For byte-accurate CLI output, `runJsonBytes(...)` returns a `Uint8Array`.

## Browser

```html
<script type="module">
  import init, { run, runValues } from 'https://unpkg.com/jaq-wasm/index.js';

  await init();

  const names = runValues('.users[] | .name', {
    users: [
      { name: 'Ada' },
      { name: 'Grace' }
    ]
  });

  console.log(names); // ["Ada", "Grace"]
  console.log(run('.users | length', { users: names }));
</script>
```

## Node.js

```js
import init, { run, runValues } from 'jaq-wasm';

await init();

const activeEmails = runValues(
  '.users[] | select(.active) | .email',
  {
    users: [
      { email: 'ada@example.com', active: true },
      { email: 'grace@example.com', active: false }
    ]
  },
);

console.log(activeEmails);
console.log(run('.users | map(.email)', {
  users: [
    { email: 'ada@example.com' },
    { email: 'grace@example.com' },
  ],
}));
```

## JSON String Inputs

```js
import init, { runJson, runJsonValues, runJsonBytes } from 'jaq-wasm';

await init();

const inputJson = '{"items":[1,2,3,4]}';

console.log(runJsonValues('.items | map(. * 2)', inputJson)); // [2, 4, 6, 8]
console.log(runJson('.items | length', inputJson)); // 4\n
console.log(new TextDecoder().decode(runJsonBytes('.items[]', inputJson)));
```

## Function Reference

### `run(filter: string, input: unknown): string`

Serializes `input` as JSON, runs the filter, and returns newline-delimited CLI-style output.

### `runValues<T = unknown>(filter: string, input: unknown): T[]`

Serializes `input` as JSON, runs the filter, and returns every produced value as a JavaScript array.

### `runJson(filter: string, inputJson: string): string`

Runs the filter against an existing JSON string and returns newline-delimited CLI-style output.

### `runJsonValues<T = unknown>(filter: string, inputJson: string): T[]`

Runs the filter against an existing JSON string and returns every produced value as a JavaScript array.

### `runJsonBytes(filter: string, inputJson: string): Uint8Array`

Runs the filter against an existing JSON string and returns raw output bytes.

## Runtime Notes

- Invalid filters throw JavaScript errors.
- Invalid JSON input throws JavaScript errors.
- The WebAssembly build does not allow filesystem-backed module imports.
- Output values are converted to JSON-compatible JavaScript values before they are returned by `runValues(...)` and `runJsonValues(...)`.

## Local Development

```bash
npm run build
npm run smoke
npm run test
npm run check
```

The generated publishable package is written to [`pkg`](./pkg). The local test bench in [`site/index.html`](./site/index.html) loads that built artifact directly, so it stays aligned with what would actually be published to npm.

## Publishing

The GitHub release workflow publishes the contents of `pkg/` to npm. To verify the tarball locally before a release:

```bash
npm run pack
```

## License

MIT
