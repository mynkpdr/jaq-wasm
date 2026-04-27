export type JsonPrimitive = null | boolean | number | string;
export type JsonValue = JsonPrimitive | JsonValue[] | { [key: string]: JsonValue };

export type {
  InitInput,
  InitOutput,
  SyncInitInput,
} from './jaq_wasm.js';

export function init(
  moduleOrPath?: import('./jaq_wasm.js').InitInput | Promise<import('./jaq_wasm.js').InitInput>,
): Promise<import('./jaq_wasm.js').InitOutput>;
export function initSync(
  module: import('./jaq_wasm.js').SyncInitInput,
): import('./jaq_wasm.js').InitOutput;
export { runJsonBytes } from './jaq_wasm.js';
export function runJson(filter: string, inputJson: string): string;
export function runJsonValues<T = JsonValue>(filter: string, inputJson: string): T[];
export function run<TInput = JsonValue>(filter: string, input: TInput): string;
export function runValues<TOutput = JsonValue, TInput = JsonValue>(
  filter: string,
  input: TInput,
): TOutput[];

export default init;
