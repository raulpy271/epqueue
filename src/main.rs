
mod pqueue;

use crate::pqueue::PQueue;

fn main() {
    let mut queue: PQueue<i32> = PQueue::new();
    queue.insert(2);
    queue.insert(5);
    queue.insert(3);
    queue.insert(4);
    queue.insert(3);
    queue.insert(1);
    queue.insert(7);
    queue.print();
    match queue.get() {
        Some(max) => println!("max element: {}", max),
        None => println!("Empty priority queue!")
    };
}
