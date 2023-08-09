use log::info;
use log4rs;
mod request;
mod protocol;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    info!("Please scan the qrcode on your phone to login bilibili."); 
    let code = protocol::login::qrcode::QrCode::apply().await?;
    code.println().await?;
    Ok(())
}