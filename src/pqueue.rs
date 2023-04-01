
use std::rc::Rc;
use std::fmt;
use std::cmp;

use crate::item::{Item};


pub enum Priority {Asc, Desc}

pub struct PQueue<K: cmp::Ord + Copy + fmt::Display, V: Clone> {
    vec: Vec<Item<K, V>>,
    order: cmp::Ordering
}

impl<K: cmp::Ord + Copy + fmt::Display, V: Clone> PQueue<K, V> {
    pub fn new(priority: Priority) -> PQueue<K, V> {
        let order = match priority {
            Priority::Asc => cmp::Ordering::Less,
            Priority::Desc => cmp::Ordering::Greater
        };
        let queue = PQueue {
            vec: Vec::new(),
            order
        };
        queue
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn insert_k(&mut self, key: K) {
        self.insert(key, None)
    }

    pub fn bulk_insert_k(&mut self, keys: Vec<K>) {
        for key in keys {
            self.insert(key, None)
        }
    }

    pub fn bulk_insert_kv(&mut self, keys_values: Vec<(K, V)>) {
        for (key, value) in keys_values {
            self.insert(key, Some(value))
        }
    }

    pub fn insert_kv(&mut self, key: K, value: V) {
        self.insert(key, Some(value))
    }

    fn insert(&mut self, key: K, value: Option<V>) {
        let rc_value: Option<Rc<V>> = value.map(|rc| Rc::new(rc));
        self.vec.push(Item {key, value: rc_value});
        let mut node_i: usize = self.vec.len();
        if node_i > 1 {
            let mut father_i = (node_i / 2) as usize;
            while (node_i > 1) && (self.vec[node_i - 1].cmp(&self.vec[father_i - 1]) == self.order) {
                self.vec.swap(father_i - 1, node_i - 1);
                node_i = father_i;
                father_i = (node_i / 2) as usize;
            }
        }
    }

    pub fn pop_k(&mut self) -> Option<K> {
        self.pop_kv().map(|kv| kv.0)
    }

    pub fn bulk_pop_k(&mut self, quantity: usize) -> Vec<K> {
        let mut elements = Vec::new();
        for _ in 0..quantity {
            if let Some((key, _)) = self.pop_kv() {
                elements.push(key);
            } else {
                break;
            }
        }
        elements
    }

    pub fn bulk_pop_kv(&mut self, quantity: usize) -> Vec<(K, Option<V>)> {
        let mut elements = Vec::new();
        for _ in 0..quantity {
            if let Some(pair) = self.pop_kv() {
                elements.push(pair);
            } else {
                break;
            }
        }
        elements
    }

    pub fn pop_kv(&mut self) -> Option<(K, Option<V>)> {
        if let Some(last) = self.vec.pop() {
            let key;
            let rc_value;
            if self.vec.len() > 0 {
                let first = self.vec[0].clone();
                self.vec[0] = last;
                self.heapify(1);
                key = first.key;
                rc_value = first.value;
            } else {
                key = last.key;
                rc_value = last.value;
            }
            Some((
                key,
                match rc_value {
                    Some(rc) => Rc::try_unwrap(rc).ok(),
                    None => None,
                }
            ))
        } else {
            None
        }
    }

    pub fn top_kv(&self) -> Option<(K, Option<Rc<V>>)> {
        if self.vec.len() == 0 {
            None
        } else {
            let item = self.vec[0].clone();
            Some((item.key, item.value))
        }
    }

    pub fn top_k(&self) -> Option<K> {
        if self.vec.len() == 0 {
            None
        } else {
            Some(self.vec[0].key)
        }
    }

    fn heapify(&mut self, node_i: usize) {
        let size: usize = self.vec.len();
        let left: usize = node_i * 2;
        let right: usize = left + 1;
        if node_i < size {
            if left <= size {
                let node = self.vec[node_i - 1].clone();
                if right <= size {
                    // both left and right exists
                    if (self.vec[left - 1].cmp(&self.vec[node_i - 1]) == self.order) ||
                            (self.vec[right - 1].cmp(&self.vec[node_i - 1]) == self.order) {
                        if self.vec[left - 1].cmp(&self.vec[right - 1]) == self.order {
                            let higher_priority = self.vec[left - 1].clone();
                            self.vec[node_i - 1] = higher_priority;
                            self.vec[left - 1] = node;
                            self.heapify(left);
                        } else {
                            let higher_priority = self.vec[right - 1].clone();
                            self.vec[node_i - 1] = higher_priority;
                            self.vec[right - 1] = node;
                            self.heapify(right);
                        }
                    }
                } else {
                    if self.vec[left - 1].cmp(&self.vec[node_i - 1]) == self.order {
                        let higher_priority = self.vec[left - 1].clone();
                        self.vec[node_i - 1] = higher_priority;
                        self.vec[left - 1] = node;
                    }
                }
            }
        }
    }

    fn to_string(&self, node_i: usize, level: usize) -> String {
        let identation = if level > 0 {
            "    ".repeat(level)
        } else {
            String::new()
        };
        let header = format!("{} - {}, pos: {}",
            identation,
            self.vec[node_i - 1].key,
            node_i
        );
        let left: usize = node_i * 2;
        let right: usize = left + 1;
        if left <= self.vec.len() {
            let left_string = self.to_string(left, level + 1);
            if right <= self.vec.len() {
                let right_string = self.to_string(right, level + 1);
                format!("{}\n{}\n{}", header, left_string, right_string)
            } else {
                format!("{}\n{}", header, left_string)
            }
        } else {
            header
        }
    }
}

impl<K: cmp::Ord + Copy + fmt::Display, V: Clone> fmt::Debug for PQueue<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.vec.len() > 0 {
            let repr = self.to_string(1, 0);
            write!(f, "{}", repr.as_str())
        } else {
            write!(f, "empty queue")
        }
    }

}

impl<K: cmp::Ord + Copy + fmt::Display, V: Clone> Iterator for PQueue<K, V> {
    type Item = (K, Option<V>);

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_kv()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_of_the_queue() {
        let mut queue: PQueue<u8, String> = PQueue::new(Priority::Asc);
        assert_eq!(queue.len(), 0);
        queue.insert_k(0);
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn insert_key() {
        let mut queue: PQueue<u8, String> = PQueue::new(Priority::Asc);
        queue.insert_k(0);
    }

    #[test]
    fn insert_min_in_correct_order() {
        let mut queue: PQueue<i32, String> = PQueue::new(Priority::Asc);
        queue.insert_k(0);
        queue.insert_k(1);
        queue.insert_k(-2);
        queue.insert_k(3);
        queue.insert_k(-10);
        queue.insert_k(10);
        assert_eq!(queue.vec[0].key, -10);
        assert!(vec![queue.vec[1].key, queue.vec[2].key].contains(&-2));
        assert!(vec![queue.vec[1].key, queue.vec[2].key].contains(&0));
    }

    #[test]
    fn pop_key() {
        let mut queue: PQueue<u8, String> = PQueue::new(Priority::Asc);
        queue.insert_k(0);
        assert_eq!(queue.pop_k(), Some(0));
    }

    #[test]
    fn insert_key_value() {
        let mut queue: PQueue<u8, String> = PQueue::new(Priority::Asc);
        queue.insert_kv(0, String::from("Value on key 0"));
    }

    #[test]
    fn pop_key_value() {
        let mut queue: PQueue<u8, String> = PQueue::new(Priority::Asc);
        queue.insert_kv(0, String::from("Value on key 0"));
        assert_eq!(queue.pop_kv(), Some((0, Some(String::from("Value on key 0")))));
    }

    #[test]
    fn pop_empty_queue() {
        let mut queue: PQueue<u8, String> = PQueue::new(Priority::Asc);
        assert_eq!(queue.pop_kv(), None);
    }

    #[test]
    fn top_key() {
        let mut queue: PQueue<u8, String> = PQueue::new(Priority::Asc);
        assert_eq!(queue.top_k(), None);
        queue.insert_kv(0, String::from("Value on key 0"));
        assert_eq!(queue.top_k(), Some(0));
        assert_eq!(queue.top_k(), Some(0));
    }

    #[test]
    fn top_and_pop_key_value() {
        let mut queue: PQueue<u8, String> = PQueue::new(Priority::Asc);
        queue.insert_kv(0, String::from("Value on key 0"));
        match queue.top_kv() {
            Some((0, Some(rc))) => {
                assert_eq!(Rc::strong_count(&rc), 2);
                assert_eq!((*rc).clone(), String::from("Value on key 0"));
            },
            _ => assert!(false),
        };
        queue.pop_kv();
        assert_eq!(queue.top_kv(), None);
    }

    #[test]
    fn bulk_insert_key() {
        let mut queue: PQueue<u8, String> = PQueue::new(Priority::Desc);
        let keys: Vec<u8> = Vec::from((0..10).collect::<Vec<u8>>());
        queue.bulk_insert_k(keys);
        assert_eq!(queue.len(), 10);
        assert_eq!(queue.top_k(), Some(9));
        assert_eq!(queue.pop_k(), Some(9));
        assert_eq!(queue.pop_k(), Some(8));
        assert_eq!(queue.pop_k(), Some(7));
    }

    #[test]
    fn bulk_insert_key_and_value() {
        let mut queue: PQueue<u8, i8> = PQueue::new(Priority::Desc);
        let keys_and_values = Vec::from(
            (0..10)
            .map(|n| (n, - (n as i8)))
            .collect::<Vec<(u8, i8)>>()
        );
        queue.bulk_insert_kv(keys_and_values);
        assert_eq!(queue.len(), 10);
        assert_eq!(queue.pop_kv(), Some((9, Some(-9))) );
        assert_eq!(queue.pop_kv(), Some((8, Some(-8))) );
        assert_eq!(queue.pop_kv(), Some((7, Some(-7))) );
        assert_eq!(queue.len(), 7);
    }

    #[test]
    fn bulk_pop_key_and_values() {
        let to_pop = 3;
        let mut queue: PQueue<u8, i8> = PQueue::new(Priority::Desc);
        let keys_and_values = Vec::from(
            (0..10)
            .map(|n| (n, - (n as i8)))
            .collect::<Vec<(u8, i8)>>()
        );
        queue.bulk_insert_kv(keys_and_values);
        let elements = queue.bulk_pop_kv(to_pop);
        assert_eq!(queue.len(), 10 - to_pop);
        assert_eq!(elements.len(), to_pop);
        assert_eq!(elements, vec![(9, Some(-9)), (8, Some(-8)), (7, Some(-7))]);
    }

    #[test]
    fn bulk_pop_key() {
        let to_pop = 3;
        let mut queue: PQueue<u8, String> = PQueue::new(Priority::Desc);
        let keys: Vec<u8> = Vec::from((0..10).collect::<Vec<u8>>());
        queue.bulk_insert_k(keys);
        let mut elements = queue.bulk_pop_k(to_pop);
        assert_eq!(queue.len(), 10 - to_pop);
        assert_eq!(elements.len(), to_pop);
        assert_eq!(elements, vec![9, 8, 7]);
        elements = queue.bulk_pop_k(10);
        assert_eq!(queue.len(), 0);
        assert_eq!(elements.len(), 10 - to_pop);
        assert_eq!(elements, vec![6, 5, 4, 3, 2, 1, 0]);
    }
}
