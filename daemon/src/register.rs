use crate::register::register_rpc::NodeInformation;
use crate::register::register_rpc::register_client::RegisterClient;

pub(crate) mod register_rpc {
    tonic::include_proto!("register");
}

pub(crate) struct RegisterService;

impl RegisterService {
    pub fn new() -> Self {
        Self {}
    }
    
    pub async fn register_self(&self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = RegisterClient::connect(url.to_string()).await?;

        client.register_node(NodeInformation {
            id: "".to_string(),
            address: "".to_string(),
            max_connections: 0,
        }).await?;
        
        Ok(())
    }
}

