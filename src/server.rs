use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use prism_client::{SignatureBundle, VerifyingKey};
use serde::{Deserialize, Serialize};

use crate::app::AppState;
use crate::config::AppConfig;
use crate::ops::{add_data, add_key, create_account, get_account};

#[derive(Deserialize, Serialize, Debug)]
struct CreateAccountRequest {
    id: String,
    pub_key: String,
    signature: SignatureBundle,
}

#[derive(Deserialize, Serialize, Debug)]
struct AddKeyRequest {
    id: String,
    pub_key: String,
    signature: SignatureBundle,
}

#[derive(Deserialize, Serialize, Debug)]
struct AddDataRequest {
    id: String,
    data: Vec<u8>,
    data_signature: SignatureBundle,
    signature: SignatureBundle,
}

#[derive(Serialize)]
struct AccountResponse {
    id: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct GetAccountRequest {
    id: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct ListKeysRequest {
    id: String,
}

pub async fn run_server(app_state: Arc<AppState>, config: AppConfig) {
    // Wrap app_state in Arc
    let app_state = app_state.clone();

    // Build the router
    let app = Router::new()
        .route("/v1/health", get(health_check_handler))
        .route("/v1/account/get", get(get_account_handler))
        .route("/v1/account/create", post(create_account_handler))
        .route("/v1/account/add-key", post(add_key_handler))
        .route("/v1/account/add-data", post(add_data_handler))
        .route("/v1/account/list-accounts", get(list_accounts_handler))
        .route("/v1/account/list-keys", get(list_keys_handler))
        .with_state(app_state);

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    tracing::info!("Server running on {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app).await.unwrap();
}

// Handlers

// Health check
async fn health_check_handler() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

async fn create_account_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateAccountRequest>,
) -> impl IntoResponse {
    let state = state.clone();
    let account = create_account(state, req.id, req.signature).await.unwrap();

    (StatusCode::OK, Json(AccountResponse { id: account.id().to_string() }))
}

async fn add_key_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AddKeyRequest>,
) -> impl IntoResponse {
    let state = state.clone();
    let new_key = VerifyingKey::try_from(req.pub_key).unwrap();
    let account = add_key(state, req.id, new_key, req.signature).await.unwrap();

    (StatusCode::OK, Json(AccountResponse { id: account.id().to_string() }))
}

async fn add_data_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AddDataRequest>,
) -> impl IntoResponse {
    let state = state.clone();
    let account =
        add_data(state, req.id, req.data, req.data_signature, req.signature).await.unwrap();

    (StatusCode::OK, Json(AccountResponse { id: account.id().to_string() }))
}

async fn get_account_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<GetAccountRequest>,
) -> impl IntoResponse {
    tracing::info!("Getting account for {}", req.id);
    let state = state.clone();
    let account = get_account(state, req.id).await.unwrap();

    (StatusCode::OK, Json(account))
}

async fn list_accounts_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let state = state.clone();
    let accounts = state.db.clone().get_accounts();

    (StatusCode::OK, Json(accounts))
}

async fn list_keys_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ListKeysRequest>,
) -> impl IntoResponse {
    let state = state.clone();
    let keys = state.db.clone().get_keys(req.id);

    (StatusCode::OK, Json(keys))
}
