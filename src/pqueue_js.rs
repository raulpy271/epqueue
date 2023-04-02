
use std::rc::Rc;
use std::cmp;
use std::fmt;

use js_sys::{Array, Error, TypeError};
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
    /// Construct a new priority queue.
    /// 
    /// If the `order` is "asc" then it's a ascending priority queue.
    /// If the `order` is "desc" then it's a descending priority queue.
    /// Otherwise, it throws a exception.
    pub fn new(order: String) -> Result<PQueueJs, TypeError> {
        let queue;
        if order == String::from("asc") {
            queue = PQueue::new(Priority::Asc);
        } else if order == String::from("desc") {
            queue = PQueue::new(Priority::Desc);
        } else {
            return Err(TypeError::new("The order parameter should be asc or desc"));
        }
        let queue_js = PQueueJs{ queue };
        Ok(queue_js)
    }

    #[wasm_bindgen(getter)]
    /// The number of keys in the queue.
    pub fn length(&self) -> usize {
        self.queue.len()
    }

    #[wasm_bindgen( js_name = insertK )]
    /// Insert a key in the priority queue.
    ///
    /// The method assumes that there is no data associated with the inserted key.
    pub fn insert_k(&mut self, key_op: Option<f64>) -> Result<(), TypeError> {
        match key_op {
            Some(key) => {
                self.queue.insert_k(NumberJs::new(key));
                Ok(())
            }
            None => Err(TypeError::new("insertK expect a number")),
        }
    }

    #[wasm_bindgen( js_name = insertKV )]
    /// Insert a key in the priority queue. `insertKV` stands for `insert key and value`.
    ///
    /// The `value` should be any data associated with the inserted key.
    pub fn insert_kv(&mut self, key_op: Option<f64>, value: JsValue) -> Result<(), TypeError> {
        match key_op {
            Some(key) => {
                self.queue.insert_kv(NumberJs::new(key), value);
                Ok(())
            }
            None => Err(TypeError::new("insertK expect a number")),
        }
    }

    #[wasm_bindgen( js_name = bulkInsertK )]
    pub fn bulk_insert_k(&mut self, keys: Vec<JsValue>) -> Result<(), TypeError> {
        let mut vec = Vec::new();
        for key_js in keys {
            if let Some(key) = key_js.as_f64() {
                vec.push(NumberJs::new(key));
            } else {
                return Err(TypeError::new("The elements of the array should be numbers"));
            }
        }
        self.queue.bulk_insert_k(vec);
        Ok(())
    }

    #[wasm_bindgen( js_name = popKV )]
    /// Pop from the queue the pair key/value with higher priority.
    ///
    /// Returns a array which the first element is the key.

    /// The returned array have the length 2 if the key has inserted with associated data.
    /// If the key has inserted without any associated data, it returns a array with a single element.
    pub fn pop_kv(&mut self) -> Result<Array, Error> {
        let value = self.queue.pop_kv();
        value
            .map(|pair| match pair.1 {
                Some(value) => Array::of2(&JsValue::from_f64(pair.0.0), &value),
                None => Array::of1(&JsValue::from_f64(pair.0.0)),
            })
            .ok_or(Error::new("Cannot pop from empty queue"))
    }

    #[wasm_bindgen( js_name = popK )]
    /// Pop from the queue the key with higher priority.
    pub fn pop_k(&mut self) -> Result<f64, Error> {
        let value = self.queue.pop_k();
        value
            .map(|k| k.0)
            .ok_or(Error::new("Cannot pop from empty queue"))
    }

    #[wasm_bindgen( js_name = bulkPopK )]
    pub fn bulk_pop_k(&mut self, quantity_op: Option<i32>) -> Result<Vec<f64>, TypeError> {
        let err = Err(TypeError::new("The bulk pop parameter should be a positive number"));
        if let Some(quantity) = quantity_op {
            if quantity > 0 {
                let vec = self.queue.bulk_pop_k(quantity as usize);
                let vec_f64 = vec
                    .iter()
                    .map(|k| k.0)
                    .collect::<Vec<f64>>();
                Ok(vec_f64)
            } else {
                err
            }
        }  else {
            err
        }
    }

    #[wasm_bindgen( js_name = topK )]
    /// Return the key with higher priority without remove it
    pub fn top_k(&mut self) -> Result<f64, Error> {
        let value = self.queue.top_k();
        value
            .map(|k| k.0)
            .ok_or(Error::new("Cannot call top from empty queue"))
    }

    #[wasm_bindgen( js_name = topKV )]
    /// Return the pair key/value with higher priority without remove it
    ///
    /// Returns a array which the first element is the key.
    /// The returned array have the length 2 if the key has inserted with associated data.
    /// If the key has inserted without any associated data, it returns a array with a single element.
    pub fn top_kv(&mut self) -> Result<Array, Error> {
        let value = self.queue.top_kv();
        value
            .map(|pair| match pair.1 {
                Some(rc) => Array::of2(&JsValue::from_f64(pair.0.0), &rc),
                None => Array::of1(&JsValue::from_f64(pair.0.0)),
            })
            .ok_or(Error::new("Cannot call top from empty queue"))
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