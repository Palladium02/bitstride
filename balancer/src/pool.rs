use std::collections::{BinaryHeap, HashMap};
use crate::metric::NodeMetrics;

pub struct Pool {
    heap: BinaryHeap<NodeMetrics>,
    index: HashMap<String, NodeMetrics>,
}

impl Pool {
    pub fn new() -> Pool {
        Self {
            heap: BinaryHeap::new(),
            index: HashMap::new(),
        }
    }
}
