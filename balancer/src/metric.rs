use std::cmp::Ordering;
use crate::register::register_rpc::NodeInformation;
use crate::success::SuccessTracker;

const ALPHA: f32 = 1.0;
const BETA: f32 = 1.0;
const GAMMA: f32 = 1.0;
const DELTA: f32 = 1.0;

#[derive(Debug, Clone)]
pub struct NodeMetrics {
    pub id: String,
    max_connections: usize,
    current_connections: usize,
    cpu_usage: f32,
    ram_usage: f32,
    success_tracker: SuccessTracker,
}

impl NodeMetrics {
    pub fn evaluate(&self) -> f32 {
        let connection_ratio = self.current_connections as f32 / self.max_connections as f32;

        let success_score = self.success_tracker.ratio();
        let cpu_score = 1.0 - self.cpu_usage;
        let ram_score = 1.0 - self.ram_usage;
        let connection_score = 1.0 - connection_ratio;

        let sum_weights = ALPHA + BETA + GAMMA + DELTA;

        (
            ALPHA * success_score +
                BETA * cpu_score +
                GAMMA * ram_score +
                DELTA * connection_score
        ) / sum_weights
    }
}

impl PartialEq<Self> for NodeMetrics {
    fn eq(&self, other: &Self) -> bool {
        self.evaluate() == other.evaluate()
    }
}

impl Eq for NodeMetrics {}

impl PartialOrd<Self> for NodeMetrics {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for NodeMetrics {
    fn cmp(&self, other: &Self) -> Ordering {
        self.evaluate().partial_cmp(&other.evaluate()).expect("")
    }
}

impl From<NodeInformation> for NodeMetrics {
    fn from(info: NodeInformation) -> NodeMetrics {
        NodeMetrics {
            id: info.id.to_string(),
            max_connections: 0,
            current_connections: 0,
            cpu_usage: 0.0,
            ram_usage: 0.0,
            success_tracker: SuccessTracker::new(0.5),
        }
    }
}