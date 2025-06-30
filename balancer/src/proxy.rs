use std::sync::Arc;
use tokio::{net::TcpListener, spawn, sync::Mutex};
use tokio::io::copy_bidirectional;
use tokio::net::TcpStream;
use crate::pool::Pool;

pub(crate) struct Proxy {
    pool: Arc<Mutex<Pool>>
}

impl Proxy {
    pub fn new(pool: Arc<Mutex<Pool>>) -> Self {
        Self { pool }
    }
    
    pub async fn run(&self, port: u16) {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await.expect("Failed to bind to address");
        loop {
            let pool = Arc::clone(&self.pool);
            let mut client = match listener.accept().await {
                Ok((client, _)) => client,
                Err(_) => continue,
            };
            
            spawn(async move {
                let node = match pool.lock().await.get_best() {
                    Some(node) => node,
                    None => return,
                };
                
                if let Ok(mut stream) = TcpStream::connect(node.ip).await {
                    if let Err(error) = copy_bidirectional(&mut client, &mut stream).await {
                        eprintln!("{}", error);
                    }
                }
            });
        }
    }
}
