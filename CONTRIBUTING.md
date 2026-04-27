# Contributing

Thanks for contributing to `jaq-wasm`.

## Workflow

1. Create a branch for your change.
2. Make the code change.
3. Regenerate the package output and run the checks:

```bash
npm run build
npm run smoke
npm run test
npm run lint
npm run typecheck
npm run pack
```

4. Commit both the source changes and the refreshed `pkg/` output when the publishable package changes.
5. Open a pull request.

## Standards

- Keep the JavaScript package entrypoint in `package-template/` aligned with the wasm exports in `src/lib.rs`.
- Keep the npm package surface small and intentional.
- Avoid changing the public JavaScript API without a clear reason.
- Prefer small, reviewable patches.
- Do not edit files under `pkg/` by hand; regenerate them through `npm run build`.

## Release Notes

If a change affects the published npm package, mention it clearly in the PR
description so the release notes can be updated.
