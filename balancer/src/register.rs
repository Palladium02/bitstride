use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};
use crate::metric::NodeMetrics;
use crate::pool::Pool;
use crate::register::register_rpc::register_server::Register;
use crate::register::register_rpc::{Empty, NodeInformation};

pub(crate) mod register_rpc {
    tonic::include_proto!("register");
}

#[derive(Debug)]
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
    async fn register_node(&self, request: Request<NodeInformation>) -> Result<Response<Empty>, Status> {
        self.pool.lock().expect("Poisoned lock").add(NodeMetrics::from(request.into_inner()));
        
        println!("Node registered");
        
        Ok(Response::new(Empty {}))
    }
}
