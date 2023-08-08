use log::info;
use log4rs;
mod request;
mod protocol;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    info!("Hello, world!");
    Ok(())
}