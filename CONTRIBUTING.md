# Contributing

Thanks for contributing to `jaq-wasm`.

## Workflow

1. Create a branch for your change.
2. Make the code change.
3. Run the checks:

```bash
cargo test
npm run lint
npm run typecheck
wasm-pack build --target web --release
wasm-pack build --target nodejs --release
```

4. Update `pkg/` if the wasm artifact changes.
5. Open a pull request.

## Standards

- Keep the CLI semantics as the source of truth.
- Avoid changing the public JavaScript API without a clear reason.
- Prefer small, reviewable patches.
- Keep generated files out of `target/` and `node_modules/`.

## Release Notes

If a change affects the published npm package, mention it clearly in the PR
description so the release notes can be updated.