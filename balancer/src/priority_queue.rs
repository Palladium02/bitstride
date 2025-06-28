use std::collections::HashMap;
use crate::metric::NodeMetrics;

/// A custom priority queue that allows for dynamic updates of priorities.
#[derive(Debug, Clone)]
pub(crate) struct PriorityQueue {
    heap: Vec<NodeMetrics>,
    index: HashMap<String, usize>,
}

impl PriorityQueue
{
    pub fn new() -> Self {
        Self {
            heap: Vec::new(),
            index: HashMap::new(),
        }
    }
    
    /// Inserts a given NodeMetrics instance into the queue.
    /// Inserting preserves/automatically restores the heap properties.
    pub fn insert(&mut self, item: NodeMetrics) {
        self.heap.push(item.clone());
        
        let index = self.sift_up(self.heap.len() - 1);
        
        self.index.insert(item.id.clone(), index);
    }
    
    /// Removes entry from the queue while also returning said entry if present.
    /// After returning the heap properties are guaranteed to be restored.
    #[allow(unused)] // TODO: remove later, when implementing proxy logic
    pub fn pop(&mut self) -> Option<NodeMetrics> {
        if self.heap.is_empty() {
            return None;
        }
        
        self.swap(0, self.heap.len() - 1);
        let item = self.heap.pop().unwrap();
        self.index.remove(&item.id);
        
        self.sift_down(0);
        
        Some(item)
    }
    
    /// Takes a NodeMetrics instance and updates the corresponding instance (same id) inside the queue.
    /// Updates only happen to existing instances, so no inserts will happen.
    /// Potentially violated heap properties are restored by first trying to sift up and then if no change in position has happened by sifting down.
    pub fn update(&mut self, item: NodeMetrics) {
        if let Some(&index) = self.index.get(&item.id) {
            self.heap[index] = item;
            if self.sift_up(index) == index {
                self.sift_down(index);
            }
        }
    }
    
    /// Returns ref to NodeMetrics instance by id if present.
    /// Panics if instance id is not present (subject to change).
    pub fn get_by_id(&self, id: &str) -> Option<&NodeMetrics> {
        // TODO: this could panic
        self.heap.get(*self.index.get(id).unwrap())
    }
    
    fn sift_up(&mut self, index: usize) -> usize {
        let mut index = index;
        while index > 0 {
            let parent = (index - 1) / 2;
            if self.heap[index] > self.heap[parent] {
                self.heap.swap(index, parent);
                index = parent;
            } else {
                break;
            }
        }
        
        index
    }
    
    fn sift_down(&mut self, index: usize) -> usize {
        let len = self.heap.len();
        let mut index = index;
        
        loop {
            let left = 2 * index + 1;
            let right = 2 * index + 2;
            let mut largest = index;
            
            if left < len && self.heap[left] > self.heap[largest] {
                largest = left;
            }
            
            if right < len && self.heap[right] < self.heap[largest] {
                largest = right;
            }
            
            if largest != index {
                self.swap(index, largest);
                index = largest;
            } else {
                break;
            }
        }
        
        index
    }
    
    fn swap(&mut self, a: usize, b: usize) {
        self.heap.swap(a, b);
        self.index.insert(self.heap[a].id.clone(), a);
        self.index.insert(self.heap[b].id.clone(), b);
    }
}
