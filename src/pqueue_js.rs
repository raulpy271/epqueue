
use std::rc::Rc;
use std::cmp;
use std::fmt;

use wasm_bindgen::prelude::*;

use crate::pqueue::{PQueue, Priority};


#[derive(Copy, Clone, Debug)]
struct NumberJs (f64);

impl NumberJs {
    pub fn new(f: f64) -> NumberJs {
        NumberJs {
            0: f
        }
    }
}

impl cmp::Ord for NumberJs {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match self.0.partial_cmp(&other.0) {
            Some(order) => order,
            None => panic!("Cannot compare NaN"),
        }
    }
}

impl cmp::PartialOrd for NumberJs {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl PartialEq for NumberJs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for NumberJs {}

impl fmt::Display for NumberJs {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0)
    }
}

#[wasm_bindgen( js_name = PQueue )]
pub struct PQueueJs {
    queue: PQueue<NumberJs, JsValue>,
}

#[wasm_bindgen( js_class = PQueue )]
impl PQueueJs {
    #[wasm_bindgen(constructor)]
    pub fn new(order: String) -> Result<PQueueJs, String> {
        let queue;
        if order == String::from("asc") {
            queue = PQueue::new(Priority::Asc);
        } else if order == String::from("desc") {
            queue = PQueue::new(Priority::Desc);
        } else {
            return Err(String::from("The order parameter should be asc or desc"));
        }
        let queue_js = PQueueJs{ queue };
        Ok(queue_js)
    }

    pub fn insert(&mut self, key: f64, value: JsValue) {
        self.queue.insert(NumberJs::new(key), value)
    }

    pub fn pop(&mut self) -> Result<JsValue, String> {
        let value = self.queue.pop();
        value.ok_or(String::from("Cannot pop from empty queue"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_js_order() {
        let zero = NumberJs::new(0.0);
        let one = NumberJs::new(1.0);
        assert_eq!(one.cmp(&zero), cmp::Ordering::Greater);
    }
}