use std::env::args;
use sysinfo::{System};
use crate::health::health_client::HealthClient;
use crate::health::HealthData;

pub(crate) mod health {
    tonic::include_proto!("health");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    match args().skip(1).nth(0) {
        Some(url) => {
            let mut client = HealthClient::connect(url).await?;
            let mut system = System::new_all();
            let total_memory = system.total_memory();
            
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                system.refresh_all();
                let currently_used_memory = system.used_memory();

                println!("Used memory: {}/{} MB", currently_used_memory / 1024, total_memory / 1024);

                let avg_cpu_usage: f32 = system.cpus().iter().map(|c| c.cpu_usage()).sum::<f32>() / system.cpus().len() as f32;
                println!("Average CPU usage: {:.2}%", avg_cpu_usage);
                
                let request = tonic::Request::new(HealthData {
                    ram: currently_used_memory as f32 / total_memory as f32,
                    cpu: avg_cpu_usage,
                });

                client.report_health(request).await?;
            }
        }
        None => {
            eprint!(
                "Missing argument for url\nUsage: bitstride_daemon http://url_for_balancer or cargo run -p daemon -- http://url_for_balancer (when running from source)\n"
            )
        }
    }
    
    Ok(())
}
