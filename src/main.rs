
use epqueue::{PQueue, Priority};

fn main() {
    let mut asc_priority_queue = PQueue::new(Priority::Asc);

    asc_priority_queue.insert(9);
    asc_priority_queue.insert(1);
    asc_priority_queue.insert(7);
    asc_priority_queue.insert(4);

    println!("queue representation as heap:\n{:?}", asc_priority_queue);

    match asc_priority_queue.pop() {
        Some(top) => println!("pop operation: {}", top),
        None => println!("cannot pop from empty queue")
    };
}
