use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::Server;
use crate::health::health_rpc::health_server::HealthServer;
use crate::health::HealthService;
use crate::pool::Pool;
use crate::register::register_rpc::register_server::RegisterServer;
use crate::register::RegisterService;

pub(crate) struct GrpcServer {
    register_service: RegisterService,
    health_service: HealthService,
}

impl GrpcServer {
    pub fn new(pool: Arc<Mutex<Pool>>) -> Self {
        Self {
            register_service: RegisterService::new(Arc::clone(&pool)),
            health_service: HealthService::new(Arc::clone(&pool)),
        }
    }
    
    pub async fn serve(self, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        Server::builder()
            .add_service(HealthServer::new(self.health_service))
            .add_service(RegisterServer::new(self.register_service))
            .serve(addr.parse()?).await?;
        
        Ok(())
            // .serve(config.rpc_address.parse()?).await?;
    }
}
