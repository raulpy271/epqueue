
use std::fmt;
use std::cmp;

pub enum Priority {Asc, Desc}

pub struct PQueue<T> {
    vec: Vec<T>,
    order: cmp::Ordering
}

impl<T: cmp::Ord + Copy + fmt::Display> PQueue<T> {
    pub fn new(priority: Priority) -> PQueue<T> {
        let order = match priority {
            Priority::Asc => cmp::Ordering::Less,
            Priority::Desc => cmp::Ordering::Greater
        };
        let queue: PQueue<T> = PQueue {
            vec: Vec::new(),
            order
        };
        queue
    }

    pub fn insert(&mut self, value: T) {
        self.vec.push(value);
        let mut node_i: usize = self.vec.len();
        if node_i > 1 {
            let mut temp: T;
            let mut father_i = (node_i / 2) as usize;
            while node_i > 1 {
                if self.vec[node_i - 1].cmp(&self.vec[father_i - 1]) == self.order {
                    temp = self.vec[father_i - 1];
                    self.vec[father_i - 1] = self.vec[node_i - 1];
                    self.vec[node_i - 1] = temp;
                }
                node_i = father_i;
                father_i = (node_i / 2) as usize;
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(last) = self.vec.pop() {
            if let Some(&top) = self.vec.get(0) {
                self.vec[0] = last;
                self.heapify(1);
                Some(top)
            } else {
                Some(last)
            }
        } else {
            None
        }
    }

    pub fn top(&self) -> Option<&T> {
        if self.vec.len() == 0 { None } else { Some(&(self.vec[0])) }
    }

    fn heapify(&mut self, node_i: usize) {
        let size: usize = self.vec.len();
        let left: usize = node_i * 2;
        let right: usize = left + 1;
        if node_i < size {
            if left <= size {
                let node = self.vec[node_i - 1];
                if right <= size {
                    // both left and right exists
                    if (self.vec[left - 1].cmp(&self.vec[node_i - 1]) == self.order) ||
                            (self.vec[right - 1].cmp(&self.vec[node_i - 1]) == self.order) {
                        if self.vec[left - 1].cmp(&self.vec[right - 1]) == self.order {
                            let higher_priority = self.vec[left - 1];
                            self.vec[node_i - 1] = higher_priority;
                            self.vec[left - 1] = node;
                            self.heapify(left);
                        } else {
                            let higher_priority = self.vec[right - 1];
                            self.vec[node_i - 1] = higher_priority;
                            self.vec[right - 1] = node;
                            self.heapify(right);
                        }
                    }
                } else {
                    if self.vec[left - 1].cmp(&self.vec[node_i - 1]) == self.order {
                        let higher_priority = self.vec[left - 1];
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
            self.vec[node_i - 1],
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

impl<T: cmp::Ord + Copy + fmt::Display> fmt::Debug for PQueue<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.vec.len() > 0 {
            let repr = self.to_string(1, 0);
            write!(f, "{}", repr.as_str())
        } else {
            write!(f, "empty queue")
        }
    }

}

impl<T: cmp::Ord + Copy + fmt::Display> Iterator for PQueue<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}
