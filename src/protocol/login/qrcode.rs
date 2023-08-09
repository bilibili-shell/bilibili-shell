use crate::request::Request;
use crate::protocol::login::bean::qrcode_apply;

use super::bean::qrcode_apply::QCARRS;

use qrcode::QrCode as PureQrCode;

pub type QrCode = qrcode_apply::QCARDS;
impl QrCode {
    pub async fn apply() -> Result<QrCode, Box<dyn std::error::Error>> {
        let url = String::from("https://passport.bilibili.com/x/passport-login/web/qrcode/generate");
        let text = Request::new(&url).get_text().await?;
        let obj: QCARRS = serde_json::from_str(&text)?;
        Ok(obj.data)
    }
    pub async fn println(&self) -> Result<(), Box<dyn std::error::Error>> {
        let code = PureQrCode::new(&self.url)?;
        let string = code.render::<char>().quiet_zone(false).module_dimensions(2, 1).build();
        println!("{}", string);
        Ok(())
    }
}