import init, { run_jaq, run_jaq_values } from '../pkg/jaq_wasm.js';

async function main(): Promise<void> {
  await init();

  const stdout = new TextDecoder().decode(run_jaq('.[] | .a', '[{"a":1},{"a":2}]'));
  console.log(stdout.trim());

  const structured = JSON.parse(run_jaq_values('.[] | .a', '[{"a":1},{"a":2}]')) as {
    ok?: unknown;
    error?: string;
  };

  if (structured.error) {
    throw new Error(structured.error);
  }

  console.log(structured.ok);
}

void main();