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