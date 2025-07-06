use crate::register::register_rpc::NodeInformation;
use crate::success::SuccessTracker;
use crate::traits::Identifiable;
use std::cmp::Ordering;
use std::net::SocketAddr;
use tonic::Request;

const ALPHA: f32 = 1.0;
const BETA: f32 = 1.0;
const GAMMA: f32 = 1.0;
const DELTA: f32 = 1.0;

#[derive(Debug, Clone)]
pub struct NodeMetrics {
    pub id: String,
    pub ip: SocketAddr,
    pub max_connections: usize,
    pub current_connections: usize,
    pub cpu_usage: f32,
    pub ram_usage: f32,
    pub success_tracker: SuccessTracker,
}

impl NodeMetrics {
    pub fn evaluate(&self) -> f32 {
        let connection_ratio = self.current_connections as f32 / self.max_connections as f32;

        let success_score = self.success_tracker.ratio();
        let cpu_score = 1.0 - self.cpu_usage;
        let ram_score = 1.0 - self.ram_usage;
        let connection_score = 1.0 - connection_ratio;

        let sum_weights = ALPHA + BETA + GAMMA + DELTA;

        (ALPHA * success_score + BETA * cpu_score + GAMMA * ram_score + DELTA * connection_score)
            / sum_weights
    }
}

impl Identifiable for NodeMetrics {
    fn get_id(&self) -> &str {
        &self.id
    }
}

impl PartialEq<Self> for NodeMetrics {
    fn eq(&self, other: &Self) -> bool {
        self.evaluate() == other.evaluate()
    }
}

impl Eq for NodeMetrics {}

impl PartialOrd<Self> for NodeMetrics {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NodeMetrics {
    fn cmp(&self, other: &Self) -> Ordering {
        self.evaluate().partial_cmp(&other.evaluate()).expect("")
    }
}

impl From<Request<NodeInformation>> for NodeMetrics {
    fn from(value: Request<NodeInformation>) -> Self {
        let ip = value
            .remote_addr()
            .expect("Failed to get remote address")
            .ip();
        let node_information = value.into_inner();
        let service_address = SocketAddr::from((ip, node_information.service_port as u16));

        Self {
            id: node_information.id,
            ip: service_address,
            max_connections: 0,
            current_connections: 0,
            cpu_usage: 0.0,
            ram_usage: 0.0,
            success_tracker: SuccessTracker::new(0.5),
        }
    }
}
