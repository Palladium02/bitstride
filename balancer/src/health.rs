use tonic::{Request, Response, Status};
use crate::health::health_rpc::health_server::Health;
use crate::health::health_rpc::{Empty, HealthData};

pub(crate) mod health_rpc {
    tonic::include_proto!("health");
}

#[derive(Debug, Default)]
pub struct HealthService;

#[tonic::async_trait]
impl Health for HealthService {
    async fn report_health(&self, request: Request<HealthData>) -> Result<Response<Empty>, Status> {
        println!("Got request {:?}", request.into_inner());
        
        Ok(Response::new(Empty {}))
    }
}
