
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

test("Should insert and pop key", () => {
    let queue = new PQueue("asc");
    queue.insertK(1);
    expect(queue.popK()).toEqual(1);
})

test("Should insert and pop key and value", () => {
    let data = {"a": 1, "b": 2, "c": 3};
    let queue = new PQueue("asc");
    queue.insertKV(1, data);
    expect(queue.popKV()).toEqual([1, data]);
})

test("Should fail on popping from empty queue", () => {
    expect(() => {
        let queue = new PQueue("asc");
        queue.popK();
    }).toThrow();
    expect(() => {
        let queue = new PQueue("asc");
        queue.popK();
    }).toThrow();
})