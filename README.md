
# :spider_web: Efficient Priority Queue compiled to WebAssembly

A priority queue library compiled to WebAssembly to be imported in any JavaScript/TypeScript environment with support to Wasm.

The purpose of the library is to provide a efficient implementation and take advantage of the native speed of WebAssembly language. The algorithm was implemented using a [Heap data-structure](https://www.geeksforgeeks.org/binary-heap/).

To install and import the library, is simple as any other node.js library:

```sh
# installation
npm install epqueue

# Importation
const epqueue = require('epqueue');
```

## :children_crossing: Usage example

```js
const {PQueue} = require('epqueue');

// Creating a ascending priority queue
let queue = new PQueue("asc");

// Inserting keys without associated data
queue.insertK(5)
queue.insertK(1)

console.log(`Popping the key with higher priority: ${queue.popK()}`);
// output: Popping the key with higher priority: 1

// Inserting keys with associated data
queue.insertKV(2, "The data associated to key 2");

console.log(`Popping the key/data with higher priority: ${queue.popKV()}`);
// output: Popping the key/data with higher priority: 2,The data associated to key 2
```

## :scroll: API Documentation

The exported methods/objects can be seen below. It follows the typescript `d.ts` syntax and is available in the file `epqueue.d.ts` of the builded package.

```ts
export class PQueue {
/**
* Construct a new priority queue.
* 
* If the `order` is "asc" then it's a ascending priority queue.
* If the `order` is "desc" then it's a descending priority queue.
* Otherwise, it throws a exception.
* @param {string} order
*/
  constructor(order: string);


/**
* Insert a key in the priority queue.
*
* The method assumes that there is no data associated with the inserted key.
* @param {number} key
*/
  insertK(key: number): void;


/**
* Insert a key in the priority queue. `insertKV` stands for `insert key and value`.
*
* The `value` should be any data associated with the inserted key.
* @param {number} key
* @param {any} value
*/
  insertKV(key: number, value: any): void;


/**
* Pop from the queue the pair key/value with higher priority.
*
* Returns a array which the first element is the key.
* The returned array have the length 2 if the key has inserted with associated data.
* If the key has inserted without any associated data, it returns a array with a single element.
* @returns {Array<any>}
*/
  popKV(): Array<any>;


/**
* Pop from the queue the key with higher priority.
* @returns {number}
*/
  popK(): number;


/**
* Return the key with higher priority without remove it
* @returns {number}
*/
  topK(): number;


/**
* Return the pair key/value with higher priority without remove it
*
* Returns a array which the first element is the key.
* The returned array have the length 2 if the key has inserted with associated data.
* If the key has inserted without any associated data, it returns a array with a single element.
* @returns {Array<any>}
*/
  topKV(): Array<any>;


/**
* The number of keys in the queue.
*/
  readonly length: number;
}
```

## :envelope: License

Released under the MIT License. See [LICENSE.md](/LICENSE) for details.