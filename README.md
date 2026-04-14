# jaq-wasm

[![CI](https://github.com/mynkpdr/jaq-wasm/actions/workflows/ci.yml/badge.svg)](https://github.com/mynkpdr/jaq-wasm/actions/workflows/ci.yml)
[![npm version](https://img.shields.io/npm/v/jaq-wasm.svg)](https://www.npmjs.com/package/jaq-wasm)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

`jaq-wasm` is the WebAssembly build of `jaq`, the jq-like JSON processor.
It is set up as a standalone repository with a Rust workspace, a publishable
npm package under `pkg/`, and CI scaffolding for professional release flows.

## What It Exposes

The wasm crate provides two public JavaScript-facing entry points:

- `run_jaq(filter, input)` returns CLI-style stdout bytes for the rendered results.
- `run_jaq_values(filter, input)` returns a JSON envelope for JS-friendly inspection.

The browser demo and test harness use the structured helper, while the raw export
tracks the native CLI output path more closely.

## Usage

You can use `jaq-wasm` in a browser or in Node.js. Since it is a WebAssembly module, it must be initialized before use.

### Browser (ESM)

```html
<script type="module">
  // Import the initialization function and the API
  import init, { run_jaq, run_jaq_values } from "https://unpkg.com/jaq-wasm/jaq_wasm.js";

  async function main() {
    // Load and initialize the Wasm module
    await init();

    // 1. Raw CLI-style output
    const filter = '.[] | select(.status == "active") | .name';
    const input = JSON.stringify([
      { name: "Alice", status: "active" },
      { name: "Bob", status: "inactive" },
      { name: "Charlie", status: "active" }
    ]);
    
    // Returns a Uint8Array containing standard CLI stdout bytes
    const rawOutputBytes = run_jaq(filter, input);
    console.log(new TextDecoder().decode(rawOutputBytes));
    // Output:
    // "Alice"
    // "Charlie"

    // 2. Structured JSON values
    const structuredOutputStr = run_jaq_values(filter, input);
    const result = JSON.parse(structuredOutputStr);
    
    if (result.error) {
      console.error("jaq error:", result.error);
    } else {
      console.log("Success:", result.ok);
      // Output: ["Alice", "Charlie"]
    }
  }

  main();
</script>
```

### Node.js

Using modern ES Modules in Node.js:

```javascript
import init, { run_jaq, run_jaq_values } from 'jaq-wasm';

async function main() {
  await init();

  const input = JSON.stringify({ a: 10, b: 20 });
  
  // Calculate sum of values
  const structured = JSON.parse(run_jaq_values('add', input));
  
  if (structured.error) {
    throw new Error(structured.error);
  }
  
  console.log("Sum:", structured.ok[0]); // Sum: 30
}

main();
```

## Repository Layout

- `src/` - Rust crate that compiles to WebAssembly
- `jaq-core/`, `jaq-json/`, `jaq-std/`, `jaq-fmts/` - vendored Rust dependencies
- `pkg/` - wasm-pack publish artifact for npm
- `examples/` - TypeScript usage examples

## Build

Prerequisites:

- Rust toolchain
- `wasm-pack`
- Node.js 18.18 or newer

Build both wasm targets:

```bash
wasm-pack build --target web --release
wasm-pack build --target nodejs --release
```

## Publish to npm

The package name is `jaq-wasm`. The published artifact is the generated `pkg/`
directory.

Dry run:

```bash
npm run publish:dry-run
```

Publish:

```bash
npm run publish:npm
```

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for the workflow.

## License

MIT. See [LICENSE](LICENSE).