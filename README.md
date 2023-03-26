
# Efficient Priority Queue compiled to WebAssembly

A priority queue library compiled to WebAssembly to be imported in any JavaScript/TypeScript environment with support to Wasm.

The purpose of the library is to provide a efficient implementation and take advantage of the native speed of WebAssembly language. The algorithm was implemented using a [Heap data-structure](https://www.geeksforgeeks.org/binary-heap/).

To install and import the library, is simple as any other node.js library:

```sh
# installation
npm install epqueue

# Importation
const epqueue = require('epqueue');
```

## Usage example

```js
const {PQueue} = require('epqueue');

// Creating a ascending priority queue
let queue = new PQueue("asc");

// Inserting keys without associated data
queue.insert_k(5)
queue.insert_k(1)

console.log(`Popping the key with higher priority: ${queue.pop_k()}`);
// output: Popping the key with higher priority: 1

// Inserting keys with associated data
queue.insert_kv(2, "The data associated to key 2");

console.log(`Popping the key/data with higher priority: ${queue.pop_kv()}`);
// output: Popping the key/data with higher priority: 2,The data associated to key 2
```

## License

Released under the MIT License. See [LICENSE.md](/LICENSE) for details.