use crate::request::Request;
use crate::protocol::login::bean::qrcode_apply;

use super::bean::qrcode_apply::QCARRS;

pub type QrCode = qrcode_apply::QCARDS;
impl QrCode {
    pub async fn apply() -> Result<QrCode, Box<dyn std::error::Error>> {
        let url = String::from("https://passport.bilibili.com/x/passport-login/web/qrcode/generate");
        let text = Request::new(&url).get_text().await?;
        let obj: QCARRS = serde_json::from_str(&text)?;
        Ok(obj.data)
    }
}