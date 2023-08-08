mod request;
mod protocol;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = protocol::login::qrcode::QrCode::apply().await?;
    println!("{}", code.url);
    Ok(())
}