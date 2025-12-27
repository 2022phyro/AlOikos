#[derive(serde::Serialize)]
pub struct ApiError {
    success: bool,
    error: String,
}

impl ApiError {
    pub fn new<E: ToString>(error: E) -> Self {
        Self {
            success: false,
            error: error.to_string(),
        }
    }
}
