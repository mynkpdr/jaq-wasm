import init, { run, runValues } from '../pkg/index.js';

async function main(): Promise<void> {
  await init();

  const queryInput = [{ a: 1 }, { a: 2 }];
  const stdout = run('.[] | .a', queryInput);
  console.log(stdout.trim());

  const values = runValues<number>('.[] | .a', queryInput);
  console.log(values);
}

void main();
