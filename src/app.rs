use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use prism_client::SigningKey;
use prism_prover::Prover;

use crate::db::Database;
// Application state
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub prover: Arc<Prover>,
    pub service_id: String,
    pub service_sk: SigningKey,
}

impl AppState {
    pub fn new(prover: Arc<Prover>, service_id: String, service_sk: SigningKey) -> Self {
        let db = Arc::new(Database::new());
        Self { prover, service_id, service_sk, db }
    }
}

// Custom error type
#[derive(Debug)]
pub struct AppError(pub anyhow::Error);

// Convert errors to responses
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", self.0)).into_response()
    }
}

// Convert anyhow::Error to our AppError
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError(err)
    }
}

// Result type alias for convenience
pub type HandlerResult<T> = Result<T, AppError>;
