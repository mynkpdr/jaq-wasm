import initWasm, {
  initSync as initSyncWasm,
  runJsonBytes,
  runJsonValuesJson,
} from './jaq_wasm.js';

const textDecoder = new TextDecoder();
const isNodeRuntime =
  typeof process !== 'undefined' &&
  typeof process.versions?.node === 'string';

function assertStringArgument(value, name) {
  if (typeof value !== 'string') {
    throw new TypeError(`${name} must be a string.`);
  }
}

function normalizeInput(input) {
  const serializedInput = JSON.stringify(input);

  if (serializedInput === undefined) {
    throw new TypeError('Input value must be JSON-serializable.');
  }

  return serializedInput;
}

function parseJsonValues(filter, inputJson) {
  try {
    return JSON.parse(runJsonValuesJson(filter, inputJson));
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    throw new Error(`Failed to parse jaq output as JSON: ${message}`);
  }
}

export { runJsonBytes };

export async function init(moduleOrPath) {
  if (moduleOrPath !== undefined) {
    return initWasm({ module_or_path: moduleOrPath });
  }

  if (!isNodeRuntime) {
    return initWasm();
  }

  const { readFile } = await import('node:fs/promises');
  const wasmBytes = await readFile(new URL('./jaq_wasm_bg.wasm', import.meta.url));
  return initWasm({ module_or_path: wasmBytes });
}

export function initSync(module) {
  if (module === undefined) {
    throw new TypeError('initSync requires a WebAssembly module or raw bytes.');
  }

  return initSyncWasm({ module });
}

export function runJson(filter, inputJson) {
  assertStringArgument(filter, 'filter');
  assertStringArgument(inputJson, 'inputJson');
  return textDecoder.decode(runJsonBytes(filter, inputJson));
}

export function runJsonValues(filter, inputJson) {
  assertStringArgument(filter, 'filter');
  assertStringArgument(inputJson, 'inputJson');
  return parseJsonValues(filter, inputJson);
}

export function run(filter, input) {
  return runJson(filter, normalizeInput(input));
}

export function runValues(filter, input) {
  return runJsonValues(filter, normalizeInput(input));
}

export default init;
