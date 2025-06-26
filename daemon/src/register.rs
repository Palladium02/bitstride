use crate::register::register_rpc::NodeInformation;
use crate::register::register_rpc::register_client::RegisterClient;

pub(crate) mod register_rpc {
    tonic::include_proto!("register");
}

pub(crate) struct RegisterService {
    id: String,
}

impl RegisterService {
    pub fn new(id: String) -> Self {
        Self {
            id
        }
    }
    
    pub async fn register_self(&self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = RegisterClient::connect(url.to_string()).await?;

        client.register_node(NodeInformation {
            id: self.id.clone(),
            max_connections: 10, // TODO: remove static default
            service_port: 80,
        }).await?;
        
        Ok(())
    }
}

