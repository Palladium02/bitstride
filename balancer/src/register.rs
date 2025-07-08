use crate::metric::NodeMetrics;
use crate::pool::Pool;
use crate::register::register_rpc::register_server::Register;
use crate::register::register_rpc::{Empty, NodeInformation};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

pub(crate) mod register_rpc {
    tonic::include_proto!("register");
}

#[derive(Debug, Clone)]
pub struct RegisterService {
    pool: Arc<Mutex<Pool>>,
}

impl RegisterService {
    pub fn new(pool: Arc<Mutex<Pool>>) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl Register for RegisterService {
    async fn register_node(
        &self,
        request: Request<NodeInformation>,
    ) -> Result<Response<Empty>, Status> {
        self.pool
            .lock()
            .await
            .add(NodeMetrics::try_from(request).expect(""));

        Ok(Response::new(Empty {}))
    }
}
