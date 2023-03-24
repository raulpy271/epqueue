
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

    pub fn insert(&mut self, key: K, value: V) {
        let rc_value = Rc::new(value);
        self.vec.push(Item {key, value: rc_value});
        let mut node_i: usize = self.vec.len();
        if node_i > 1 {
            let mut father_i = (node_i / 2) as usize;
            while node_i > 1 {
                if self.vec[node_i - 1].cmp(&self.vec[father_i - 1]) == self.order {
                    let temp = self.vec[father_i - 1].clone();
                    self.vec[father_i - 1] = self.vec[node_i - 1].clone();
                    self.vec[node_i - 1] = temp;
                }
                node_i = father_i;
                father_i = (node_i / 2) as usize;
            }
        }
    }

    pub fn pop(&mut self) -> Option<V> {
        if let Some(last) = self.vec.pop() {
            let rc_value;
            if self.vec.len() > 0 {
                let first = self.vec[0].clone();
                self.vec[0] = last;
                self.heapify(1);
                rc_value = first.value;
            } else {
                rc_value = last.value;
            }
            Rc::try_unwrap(rc_value).ok()
        } else {
            None
        }
    }

    pub fn top(&self) -> Option<Rc<V>> {
        if self.vec.len() == 0 {
            None
        } else {
            Some(self.vec[0].value.clone())
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
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_value() {
        let mut queue: PQueue<u8, String> = PQueue::new(Priority::Asc);
        queue.insert(0, String::from("Value on key 0"));
    }

    #[test]
    fn pop_value() {
        let mut queue: PQueue<u8, String> = PQueue::new(Priority::Asc);
        queue.insert(0, String::from("Value on key 0"));
        assert_eq!(queue.pop(), Some(String::from("Value on key 0")));
    }

    #[test]
    fn pop_empty_queue() {
        let mut queue: PQueue<u8, String> = PQueue::new(Priority::Asc);
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn top_and_pop_value() {
        let mut queue: PQueue<u8, String> = PQueue::new(Priority::Asc);
        queue.insert(0, String::from("Value on key 0"));
        assert_eq!(queue.top().map(|rc| Rc::strong_count(&rc)), Some(2));
        assert_eq!(queue.top().map(|rc| (*rc).clone()), Some(String::from("Value on key 0")));
        queue.pop();
        assert_eq!(queue.top(), None);
    }
}
