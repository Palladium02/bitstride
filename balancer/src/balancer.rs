use std::sync::Arc;
use tokio::spawn;
use tokio::sync::Mutex;
use crate::config::Config;
use crate::grpc::GrpcServer;
use crate::pool::Pool;
use crate::proxy::Proxy;

pub(crate) struct Balancer {
    pool: Arc<Mutex<Pool>>,
    grpc_server: GrpcServer,
    config: Config,
}

impl Balancer {
    pub fn new(config: Config) -> Self {
        let pool = Arc::new(Mutex::new(Pool::new()));
        Self {
            pool: Arc::clone(&pool),
            grpc_server: GrpcServer::new(Arc::clone(&pool)),
            config,
        }
    }
    
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        spawn(async move {
            Proxy::new(Arc::clone(&self.pool)).run().await
        });
        
        self.grpc_server.serve(&self.config.rpc_address).await?;
        
        Ok(())
    }
}
