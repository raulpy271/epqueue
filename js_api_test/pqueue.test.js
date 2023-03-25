
const epqueue = require('./pkg');

test("Should instantiate queue", () => {
    let queue = new epqueue.PQueue("asc");
    queue = new epqueue.PQueue("desc");
})

test("Should fail on instantiate with wrong parameter", () => {
    expect(() => { new epqueue.PQueue("xxx")}).toThrow();
    expect(() => { new epqueue.PQueue()}).toThrow();
    expect(() => { new epqueue.PQueue(12)}).toThrow();
})

test("Should insert key and insert key and value", () => {
    let data1 = {"a": 1, "b": 2, "c": 3};
    let data2 = {"a": 1, "b": 2, "c": 3};
    let queue = new epqueue.PQueue("asc");
    queue.insert_k(1);
    queue.insert_k(2);
    queue.insert_kv(3, data1);
    queue.insert_kv(4, data2);
})

test("Should insert and pop key", () => {
    let queue = new epqueue.PQueue("asc");
    queue.insert_k(1);
    expect(queue.pop()).toEqual([1]);
})

test("Should insert and pop key and value", () => {
    let data = {"a": 1, "b": 2, "c": 3};
    let queue = new epqueue.PQueue("asc");
    queue.insert_kv(1, data);
    expect(queue.pop()).toEqual([1, data]);
})

test("Should fail on popping from empty queue", () => {
    expect(() => {
        let queue = new epqueue.PQueue("asc");
        queue.pop();
    }).toThrow();
})