use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Resource not found")]
    NotFoundError,
    
    #[error("Bad request: {0}")]
    BadRequestError(String),
    
    #[error("Internal server error")]
    InternalServerError,
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let error = self.to_string();
        let response = ErrorResponse { error };
        
        match self {
            ApiError::NotFoundError => HttpResponse::NotFound().json(response),
            ApiError::BadRequestError(_) => HttpResponse::BadRequest().json(response),
            ApiError::RateLimitExceeded => HttpResponse::TooManyRequests().json(response),
            _ => HttpResponse::InternalServerError().json(response),
        }
    }
}
