mod register;
mod health;

use std::env::args;
use std::process::exit;
use uuid::Uuid;
use crate::health::HealthService;
use crate::register::RegisterService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    match args().nth(1) {
        Some(url) => {
            let id = Uuid::new_v4().to_string();
            
            RegisterService::new(id.clone())
                .register_self(&url).await?;
            
            HealthService::new(id.clone())
               .setup_report_loop(&url).await?;
            
        }
        None => {
            eprint!(
                "Missing argument for url\nUsage: bitstride_daemon http://url_for_balancer or cargo run -p daemon -- http://url_for_balancer (when running from source)\n"
            );
            exit(1);
        }
    }
    
    Ok(())
}
