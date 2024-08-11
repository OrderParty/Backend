/// error type
#[derive(Debug, serde::Serialize)]
pub struct ErrorContent {
    // HTTP Status Code returned
    code: u16,
    // Reason for an error
    reason: String,
    // Description for an error if any
    description: Option<String>,
}

/// Error messages returned to user
#[derive(Debug, serde::Serialize)]
pub struct MyError {
    pub error: ErrorContent,
}