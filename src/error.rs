use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),
    
    #[error("Rate limit exceeded: {0}")]
    RateLimitError(String),
    
    #[error("Resource not found: {0}")]
    NotFoundError(String),
    
    #[error("Invalid request: {0}")]
    ValidationError(String),
    
    #[error("Server error: {0}")]
    ServerError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
}
