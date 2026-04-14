# jaq-wasm

[![npm version](https://img.shields.io/npm/v/jaq-wasm.svg)](https://www.npmjs.com/package/jaq-wasm)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

`jaq-wasm` is the WebAssembly build of `jaq`, a jq-like JSON processor written in Rust. It allows you to process JSON data using jq filters directly in the browser or Node.js.

## Installation

```bash
npm install jaq-wasm
```

## Usage

`jaq-wasm` provides two main functions:

- `run_jaq(filter, input)`: Returns raw CLI-style output as bytes
- `run_jaq_values(filter, input)`: Returns structured JSON results

### Browser (ESM)

```html
<!DOCTYPE html>
<html>
<head>
  <title>jaq-wasm Example</title>
</head>
<body>
  <script type="module">
    import init, { run_jaq, run_jaq_values } from 'https://unpkg.com/jaq-wasm/jaq_wasm.js';

    async function main() {
      // Initialize the WebAssembly module
      await init();

      // Example 1: Simple object access
      const data = { name: "Alice", age: 30, city: "New York" };
      const filter = '.name';
      const result = JSON.parse(run_jaq_values(filter, JSON.stringify(data)));
      console.log(result.ok); // "Alice"

      // Example 2: Array filtering
      const users = [
        { name: "Alice", active: true },
        { name: "Bob", active: false },
        { name: "Charlie", active: true }
      ];
      const activeUsers = JSON.parse(run_jaq_values('.[] | select(.active) | .name', JSON.stringify(users)));
      console.log(activeUsers.ok); // ["Alice", "Charlie"]

      // Example 3: Complex transformation
      const products = [
        { name: "Laptop", price: 1200, category: "electronics" },
        { name: "Book", price: 20, category: "books" },
        { name: "Phone", price: 800, category: "electronics" }
      ];
      const electronics = JSON.parse(run_jaq_values('.[] | select(.category == "electronics") | {name, price}', JSON.stringify(products)));
      console.log(electronics.ok);
      // [{name: "Laptop", price: 1200}, {name: "Phone", price: 800}]

      // Example 4: Using raw output
      const rawOutput = run_jaq('.[] | .name', JSON.stringify(users));
      console.log(new TextDecoder().decode(rawOutput).trim());
      // "Alice"
      // "Bob"
      // "Charlie"
    }

    main();
  </script>
</body>
</html>
```

### Node.js

```javascript
import init, { run_jaq, run_jaq_values } from 'jaq-wasm';

async function main() {
  await init();

  // Example: Process JSON file
  const fs = await import('fs');
  const data = JSON.parse(fs.readFileSync('data.json', 'utf8'));

  // Extract specific fields
  const result = JSON.parse(run_jaq_values('.users[] | {name, email}', JSON.stringify(data)));
  if (result.error) {
    console.error('Error:', result.error);
  } else {
    console.log('Processed data:', result.ok);
  }

  // Example: Transform data
  const input = { items: [1, 2, 3, 4, 5] };
  const doubled = JSON.parse(run_jaq_values('.items | map(. * 2)', JSON.stringify(input)));
  console.log('Doubled:', doubled.ok); // [2, 4, 6, 8, 10]
}

main();
```

### CDN Usage

You can also use jaq-wasm directly from a CDN without npm:

```html
<script type="module">
  import init, { run_jaq_values } from 'https://unpkg.com/jaq-wasm@latest/jaq_wasm.js';

  (async () => {
    await init();
    const result = JSON.parse(run_jaq_values('. + 1', '5'));
    console.log(result.ok); // 6
  })();
</script>
```

## API Reference

### `run_jaq(filter: string, input: string): Uint8Array`

Runs a jq filter and returns the raw output as bytes, matching the CLI behavior.

- `filter`: jq filter expression
- `input`: JSON string to process
- Returns: `Uint8Array` containing the output bytes

### `run_jaq_values(filter: string, input: string): string`

Runs a jq filter and returns a structured JSON response.

- `filter`: jq filter expression
- `input`: JSON string to process
- Returns: JSON string with `{ok: result}` on success or `{error: message}` on failure

## Supported jq Features

jaq-wasm supports most jq features including:

- Object and array access (`.key`, `.[index]`)
- Filtering (`select()`, `map()`)
- Arithmetic operations
- String manipulation
- Array/object construction
- Conditionals
- And much more!

For the full list of supported features, see the [jaq documentation](https://github.com/01mf02/jaq).

## Contributing

Contributions are welcome! Please see the [main repository](https://github.com/01mf02/jaq) for jaq development.

## License

MIT

## Contributing

Contributions are welcome! Please see the [main repository](https://github.com/01mf02/jaq) for jaq development.

## License

MIT