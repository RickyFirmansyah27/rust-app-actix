use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
    pub timestamp: String,
}

impl<T> BaseResponse<T> {
    pub fn success(message: String, data: Option<T>) -> Self {
        Self {
            success: true,
            message,
            data,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn error(message: String, data: Option<T>) -> Self {
        Self {
            success: false,
            message,
            data,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}