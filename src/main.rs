
use epqueue::{PQueue, Priority};

fn main() {
    let mut asc_priority_queue = PQueue::new(Priority::Asc);
    let s1 = String::from("abc");
    let s2 = String::from("xyz");
    let s3 = String::from("mnp");
    let s4 = String::from("sss");

    asc_priority_queue.insert_kv(9, s4);
    asc_priority_queue.insert_kv(1, s1);
    asc_priority_queue.insert_kv(7, s3);
    asc_priority_queue.insert_kv(4, s2);

    println!("queue representation as heap:\n{:?}", asc_priority_queue);

    for (k, v) in asc_priority_queue {
        println!("k: {}, v: {}", k, v.unwrap());
    }

}
