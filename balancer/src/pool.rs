use crate::health::health_rpc::HealthData;
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
    
    pub fn update(&mut self, health_data: HealthData) {
        if let Some(metric) = self.priority_queue.get_by_id(health_data.id.as_str()) {
            let updated_metric = NodeMetrics {
                id: health_data.id,
                max_connections: metric.max_connections,
                current_connections: metric.current_connections,
                cpu_usage: health_data.cpu,
                ram_usage: health_data.ram,
                success_tracker: metric.success_tracker.clone(),
            };
            
            self.priority_queue.update(updated_metric)
        }
        
    }
}
