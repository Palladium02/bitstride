use sysinfo::System;
use crate::health::health_rpc::health_client::HealthClient;
use crate::health::health_rpc::HealthData;

pub(crate) mod health_rpc {
    tonic::include_proto!("health");
}

pub(crate) struct HealthService;

impl HealthService {
    pub fn new() -> Self {
        Self {}
    }
    
    pub async fn setup_report_loop(&self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = HealthClient::connect(url.to_string()).await?;
        let mut system = System::new_all();

        let total_memory = system.total_memory();

        loop {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            system.refresh_all();
            let currently_used_memory = system.used_memory();

            println!("Used memory: {}/{} MB", currently_used_memory / 1024, total_memory / 1024);

            let avg_cpu_usage: f32 = system.cpus().iter().map(|c| c.cpu_usage()).sum::<f32>() / system.cpus().len() as f32;
            println!("Average CPU usage: {avg_cpu_usage:.2}%");

            let request = tonic::Request::new(HealthData {
                ram: currently_used_memory as f32 / total_memory as f32,
                cpu: avg_cpu_usage,
            });

            client.report_health(request).await?;
        }
    }
}
