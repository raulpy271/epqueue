
mod pqueue;

use crate::pqueue::PQueue;
use crate::pqueue::Priority;

fn main() {
    let mut queue: PQueue<i32> = PQueue::new(Priority::Asc);
    queue.insert(9);
    queue.insert(1);
    queue.insert(7);
    queue.insert(4);
    queue.insert(1);
    queue.insert(20);
    queue.insert(9);
    queue.insert(10);
    println!("{:?}", queue);
    while let Some(max) = queue.pop() {
        println!("higher priority element: {}", max);
    }
}
