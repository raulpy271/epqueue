
use std::fmt;

pub struct PQueue<T> {
    vec: Vec<T>
}

impl<T: std::cmp::PartialOrd + Copy + fmt::Display> PQueue<T> {
    pub fn new() -> PQueue<T> {
        let queue: PQueue<T> = PQueue {
            vec: Vec::new()
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
                if self.vec[father_i - 1] < self.vec[node_i - 1] {
                    temp = self.vec[father_i - 1];
                    self.vec[father_i - 1] = self.vec[node_i - 1];
                    self.vec[node_i - 1] = temp;
                }
                node_i = father_i;
                father_i = (node_i / 2) as usize;
            }
        }
    }

    pub fn popmax(&mut self) -> Option<T> {
        if let Some(last) = self.vec.pop() {
            if let Some(&max) = self.vec.get(0) {
                self.vec[0] = last;
                self.heapify(1);
                Some(max)
            } else {
                Some(last)
            }
        } else {
            None
        }
    }

    pub fn getmax(&self) -> Option<&T> {
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
                    if self.vec[node_i - 1] >= self.vec[left - 1] &&
                            self.vec[node_i - 1] >= self.vec[right - 1] {
                        return;
                    } else if self.vec[left - 1] > self.vec[right - 1] {
                        let max = self.vec[left - 1];
                        self.vec[node_i - 1] = max;
                        self.vec[left - 1] = node;
                        self.heapify(left);
                    } else {
                        let max = self.vec[right - 1];
                        self.vec[node_i - 1] = max;
                        self.vec[right - 1] = node;
                        self.heapify(right);
                    }
                } else {
                    if self.vec[left - 1] > self.vec[node_i - 1] {
                        let max = self.vec[left - 1];
                        self.vec[node_i - 1] = max;
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

impl<T: std::cmp::PartialOrd + Copy + fmt::Display> fmt::Debug for PQueue<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.vec.len() > 0 {
            let repr = self.to_string(1, 0);
            write!(f, "{}", repr.as_str())
        } else {
            write!(f, "empty queue")
        }
    }

}