import assert from 'node:assert/strict';

import init, { run, runJson, runJsonBytes, runJsonValues, runValues } from '../pkg/index.js';

await init();

const queryInput = {
  users: [
    { name: 'Ada', active: true },
    { name: 'Grace', active: false },
    { name: 'Linus', active: true },
  ],
};

assert.deepEqual(
  runValues('.users[] | select(.active) | .name', queryInput),
  ['Ada', 'Linus'],
);
assert.equal(
  run('.users | length', queryInput).trim(),
  '3',
);
assert.deepEqual(
  runJsonValues('.items | map(. * 2)', '{"items":[1,2,3]}'),
  [[2, 4, 6]],
);
assert.equal(
  runJson('.flag', '{"flag":true}').trim(),
  'true',
);
assert.equal(
  new TextDecoder().decode(runJsonBytes('.message', '{"message":"ok"}')).trim(),
  '"ok"',
);

assert.throws(
  () => runJson('.flag', { flag: true }),
  /inputJson must be a string/,
);
assert.throws(
  () => runValues('.items[]', undefined),
  /JSON-serializable/,
);

console.log('Smoke test passed.');
