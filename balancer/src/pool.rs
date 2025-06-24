use std::collections::{BinaryHeap, HashMap};
use crate::metric::NodeMetrics;
use crate::priority_queue::PriorityQueue;

#[derive(Debug)]
pub struct Pool {
    priority_queue: PriorityQueue,
}

impl Pool {
    pub fn new() -> Pool {
        Self {
            priority_queue: PriorityQueue::new(),
        }
    }
    
    pub fn add(&mut self, metric: NodeMetrics) {
        self.priority_queue.insert(metric);
    }
}
