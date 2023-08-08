use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct QrCodeApplyResponseRootStruct {
    pub code: i8,
    pub message: String,
    pub ttl: i8, // 固定为1
    pub data: QrCodeApplyResponseDataStruct
}

#[derive(Serialize, Deserialize)]
pub struct QrCodeApplyResponseDataStruct {
    pub url: String,
    pub qrcode_key: String
}

pub type QCARRS = QrCodeApplyResponseRootStruct;
pub type QCARDS = QrCodeApplyResponseDataStruct;
