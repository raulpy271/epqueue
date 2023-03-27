
const {PQueue} = require('./pkg');

test("Should instantiate queue", () => {
    let queue = new PQueue("asc");
    queue = new PQueue("desc");
})

test("Should fail on instantiate with wrong parameter", () => {
    expect(() => { new PQueue("xxx")}).toThrow();
    expect(() => { new PQueue()}).toThrow();
    expect(() => { new PQueue(12)}).toThrow();
})

test("Should insert key and insert value", () => {
    let data1 = {"a": 1, "b": 2, "c": 3};
    let data2 = {"a": 1, "b": 2, "c": 3};
    let queue = new PQueue("asc");
    queue.insertK(1);
    queue.insertK(2);
    queue.insertKV(3, data1);
    queue.insertKV(4, data2);
})

test("Should return the size of the queue", () => {
    let queue = new PQueue("asc");
    expect(queue.length).toBe(0);
    queue.insertK(1);
    queue.insertK(2);
    expect(queue.length).toBe(2);
})

test("Should insert and pop key", () => {
    let queue = new PQueue("asc");
    queue.insertK(1);
    expect(queue.popK()).toBe(1);
})

test("Should insert and pop key and value", () => {
    let data = {"a": 1, "b": 2, "c": 3};
    let queue = new PQueue("asc");
    queue.insertKV(1, data);
    expect(queue.popKV()).toEqual([1, data]);
})

test("Should return the top of the queue without remove it", () => {
    let data = {"a": 1, "b": 2, "c": 3};
    let queue = new PQueue("asc");
    queue.insertKV(1, data);
    expect(queue.topK()).toBe(1);
    expect(queue.topKV()).toEqual([1, data]);
    queue.insertKV(0);
    expect(queue.topK()).toBe(0);
    expect(queue.topKV()).toEqual([0]);
    expect(queue.length).toBe(2);
})

test("Should raise exception on calling top in empty queue", () => {
    expect(() => {
        let queue = new PQueue("asc");
        queue.topK();
    }).toThrow();
    expect(() => {
        let queue = new PQueue("asc");
        queue.topKV();
    }).toThrow();
})

test("Should raise exception on popping from empty queue", () => {
    expect(() => {
        let queue = new PQueue("asc");
        queue.popK();
    }).toThrow();
    expect(() => {
        let queue = new PQueue("asc");
        queue.popKV();
    }).toThrow();
})