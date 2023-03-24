
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

test("Should insert data PQueue", () => {
    let data1 = {"a": 1, "b": 2, "c": 3};
    let data2 = {"a": 1, "b": 2, "c": 3};
    let queue = new epqueue.PQueue("asc");
    queue.insert(1, data1);
    queue.insert(2, data2);
})

test("Should insert and pop data PQueue", () => {
    let data = {"a": 1, "b": 2, "c": 3};
    let queue = new epqueue.PQueue("asc");
    queue.insert(1, data);
    expect(queue.pop()).toEqual(data);
})

test("Should fail on popping from empty queue", () => {
    expect(() => {
        let queue = new epqueue.PQueue("asc");
        queue.pop();
    }).toThrow();
})