
mod pqueue;

use crate::pqueue::PQueue;

fn main() {
    let mut queue: PQueue<i32> = PQueue::new();
    queue.insert(9);
    queue.insert(1);
    queue.insert(7);
    queue.insert(4);
    queue.insert(1);
    queue.insert(20);
    queue.insert(9);
    queue.insert(10);
    println!("{:?}", queue);
    while let Some(max) = queue.popmax() {
        println!("max element: {}", max);
    }
}
