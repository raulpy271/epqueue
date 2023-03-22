
use epqueue::{PQueue, Priority};

fn main() {
    let mut asc_priority_queue = PQueue::new(Priority::Asc);
    let s1 = String::from("abc");
    let s2 = String::from("xyz");
    let s3 = String::from("mnp");
    let s4 = String::from("sss");

    asc_priority_queue.insert(9, &s4);
    asc_priority_queue.insert(1, &s1);
    asc_priority_queue.insert(7, &s3);
    asc_priority_queue.insert(4, &s2);

    println!("queue representation as heap:\n{:?}", asc_priority_queue);

    for s in asc_priority_queue {
        println!("s: {}", s);
    }

}
