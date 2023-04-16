
# :spider_web: Efficient Priority Queue compiled to WebAssembly

[![Downloads](https://img.shields.io/npm/dt/epqueue.svg)](https://npmjs.com/package/epqueue)
[![last commit](https://img.shields.io/github/last-commit/raulpy271/epqueue.svg)](https://github.com/raulpy271/epqueue)
[![Github stars](https://img.shields.io/github/stars/raulpy271/epqueue.svg)](https://github.com/raulpy271/epqueue)
[![Known Vulnerabilities](https://snyk.io/test/npm/epqueue/badge.svg)](https://snyk.io/test/npm/epqueue)
[![npm version](https://img.shields.io/npm/v/epqueue.svg)](https://npmjs.com/package/epqueue)
[![license MIT](https://img.shields.io/npm/l/epqueue.svg)](https://github.com/raulpy271/epqueue/blob/main/LICENSE)

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
* Insert a array of keys in the priority queue. This method is roughly equivalent for calling `insert_k` for each element of the array `keys`, but it's more faster. 
*
* This method is more faster than calling `insert_k` for each key, because using this method just only one call to webassembly happens.
* @param {Array<number>} keys
*/
  bulkInsertK(keys: Array<number>): void;

/**
* Insert an array of keys in the priority queue with associated values. The value `values[0]` is associated with the key `keys[0]`, and so on.
* This method is roughly equivalent for calling `insertKV` for each element of the two array `keys` and `values`, but it's more faster. 
*
* Both array must have the same length, otherwise, an error is thrown.
* @param {Array<number>} keys
* @param {Array<any>} values
*/
  bulkInsertKV(keys: Array<number>, values: Array<any>): void;

/**
* Return a array with the keys with highest priority. The returned keys are removed from the Queue.
* 
* The `quantity` is the number of elements to be returned.
* This methods is more faster than calling `pop_k` repeatedly for popping a sequence of keys.
* @param {number} quantity
* @returns {Float64Array}
*/
  bulkPopK(quantity: number): Float64Array;

/**
* Return a array with the the pairs key/value with highest priority. The returned pairs are removed from the Queue.
* 
* Each element of the returned array is like the returned element of `popKV`, so it's a array with two or one element.
* This methods is more faster than calling `popKV` repeatedly for popping a sequence of key/value.
* @param {number} quantity
* @returns {(Array<any>)[]}
*/
  bulkPopKV(quantity: number): (Array<any>)[];

/**
* The number of keys in the queue.
*/
  readonly length: number;
}
```

## :envelope: License

Released under the MIT License. See [LICENSE.md](/LICENSE) for details.
