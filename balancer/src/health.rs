use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};
use crate::health::health_rpc::health_server::Health;
use crate::health::health_rpc::{Empty, HealthData};
use crate::pool::Pool;

pub(crate) mod health_rpc {
    tonic::include_proto!("health");
}

#[derive(Debug, Clone)]
pub struct HealthService {
    pool: Arc<Mutex<Pool>>,
}

impl HealthService {
    pub fn new(pool: Arc<Mutex<Pool>>) -> Self {
        HealthService { pool }
    }
}

#[tonic::async_trait]
impl Health for HealthService {
    async fn report_health(&self, request: Request<HealthData>) -> Result<Response<Empty>, Status> {
        self.pool.lock().await.update(request.into_inner());
        
        Ok(Response::new(Empty {}))
    }
}
