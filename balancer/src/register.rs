use tonic::{Request, Response, Status};
use crate::register::register_rpc::register_server::Register;
use crate::register::register_rpc::{Empty, NodeInformation};

pub(crate) mod register_rpc {
    tonic::include_proto!("register");
}

#[derive(Debug, Default)]
pub struct RegisterService;

#[tonic::async_trait]
impl Register for RegisterService {
    async fn register_node(&self, request: Request<NodeInformation>) -> Result<Response<Empty>, Status> {
        println!("Got request {:?}", request.into_inner());
        
        Ok(Response::new(Empty {}))
    }
}
