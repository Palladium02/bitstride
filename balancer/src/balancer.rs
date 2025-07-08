use crate::config::Config;
use crate::grpc::GrpcServer;
use crate::persistence::PersistenceService;
use crate::pool::Pool;
use crate::proxy::Proxy;
use std::sync::Arc;
use tokio::spawn;
use tokio::sync::Mutex;

pub(crate) struct Balancer {
    pool: Arc<Mutex<Pool>>,
    persistence_service: Arc<Mutex<PersistenceService>>,
    grpc_server: GrpcServer,
    config: Config,
}

impl Balancer {
    pub async fn new(config: Config) -> Self {
        let pool = Arc::new(Mutex::new(Pool::new()));
        Self {
            pool: Arc::clone(&pool),
            persistence_service: Arc::new(Mutex::new(
                PersistenceService::new(&config.db_url).await.expect(""),
            )),
            grpc_server: GrpcServer::new(Arc::clone(&pool)),
            config,
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        spawn(async move {
            Proxy::new(Arc::clone(&self.pool))
                .run(self.config.port)
                .await
        });

        self.grpc_server.serve(&self.config.rpc_address).await?;

        Ok(())
    }
}
