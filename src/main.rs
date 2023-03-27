use std::error::Error;

use log::info;

pub mod crawler;
pub mod database;
pub mod models;
pub mod parser;
pub mod networking;
pub mod schema;

#[tokio::main(worker_threads = 16)]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    env_logger::init();
    
    crawler::download_all_parliament(9).await?;
    

    info!("Done.");
    Ok(())
}
