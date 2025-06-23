use std::collections::{BinaryHeap, HashMap};
use crate::metric::NodeMetrics;

#[derive(Debug)]
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
    
    pub fn add(&mut self, metric: NodeMetrics) {
        self.heap.push(metric.clone());
        self.index.insert(metric.id.clone(), metric);
    }
}
