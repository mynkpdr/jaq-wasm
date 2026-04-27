import { copyFile, mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const currentFilePath = fileURLToPath(import.meta.url);
const scriptsDirectory = path.dirname(currentFilePath);
const workspaceDirectory = path.resolve(scriptsDirectory, '..');
const packageDirectory = path.join(workspaceDirectory, 'pkg');
const templateDirectory = path.join(workspaceDirectory, 'package-template');
const rootPackagePath = path.join(workspaceDirectory, 'package.json');
const readmePath = path.join(workspaceDirectory, 'README.md');
const licensePath = path.join(workspaceDirectory, 'LICENSE');
const pkgGitignorePath = path.join(packageDirectory, '.gitignore');
const pkgGitignoreContents = `*
!.gitignore
!LICENSE
!README.md
!index.d.ts
!index.js
!jaq_wasm.d.ts
!jaq_wasm.js
!jaq_wasm_bg.wasm
!jaq_wasm_bg.wasm.d.ts
!package.json
`;

const requiredGeneratedFiles = [
  'jaq_wasm.js',
  'jaq_wasm.d.ts',
  'jaq_wasm_bg.wasm',
];

for (const fileName of requiredGeneratedFiles) {
  const filePath = path.join(packageDirectory, fileName);

  try {
    await readFile(filePath);
  } catch (error) {
    throw new Error(
      `Missing generated package artifact: ${fileName}. Run \`npm run build\` after wasm-pack succeeds.`,
      { cause: error },
    );
  }
}

await mkdir(packageDirectory, { recursive: true });

await copyFile(
  path.join(templateDirectory, 'index.js'),
  path.join(packageDirectory, 'index.js'),
);
await copyFile(
  path.join(templateDirectory, 'index.d.ts'),
  path.join(packageDirectory, 'index.d.ts'),
);
await copyFile(readmePath, path.join(packageDirectory, 'README.md'));
await copyFile(licensePath, path.join(packageDirectory, 'LICENSE'));
await writeFile(pkgGitignorePath, pkgGitignoreContents);

const rootPackage = JSON.parse(await readFile(rootPackagePath, 'utf8'));

const publishPackage = {
  name: rootPackage.name,
  version: rootPackage.version,
  description: 'WebAssembly bindings for jaq with a JavaScript-friendly API',
  type: 'module',
  license: rootPackage.license,
  repository: {
    type: rootPackage.repository.type,
    url: rootPackage.repository.url,
  },
  bugs: rootPackage.bugs,
  homepage: rootPackage.homepage,
  engines: rootPackage.engines,
  keywords: ['jaq', 'jq', 'wasm', 'webassembly', 'json'],
  publishConfig: {
    access: 'public',
  },
  files: [
    'index.js',
    'index.d.ts',
    'jaq_wasm.js',
    'jaq_wasm.d.ts',
    'jaq_wasm_bg.wasm',
    'README.md',
    'LICENSE',
  ],
  exports: {
    '.': {
      types: './index.d.ts',
      import: './index.js',
      default: './index.js',
    },
    './jaq_wasm.js': './jaq_wasm.js',
    './package.json': './package.json',
  },
  main: './index.js',
  module: './index.js',
  types: './index.d.ts',
  sideEffects: ['./snippets/*'],
};

await writeFile(
  path.join(packageDirectory, 'package.json'),
  `${JSON.stringify(publishPackage, null, 2)}\n`,
);
