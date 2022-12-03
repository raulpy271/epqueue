
pub struct PQueue<T> {
    vec: Vec<T>
}

impl<T: std::cmp::PartialOrd + Copy + std::fmt::Display> PQueue<T> {
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

    pub fn get(&self) -> Option<&T> {
        if self.vec.len() == 0 { None } else { Some(&(self.vec[0])) }
    }

    pub fn print(&self) {
        self.print_recursive(1, 0);
    }

    fn print_recursive(&self, node_i: usize, level: usize) {
        if level > 0 {
            let identation = "    ".repeat(level);
            print!("{}", identation);
        }
        println!(" - {}", self.vec[node_i - 1]);
        let left: usize = node_i * 2;
        let right: usize = left + 1;
        if left <= self.vec.len() {
            self.print_recursive(left, level + 1);
            if right <= self.vec.len() {
                self.print_recursive(right, level + 1);
            }
        }
    }
}