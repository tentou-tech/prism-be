use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use prism_client::SignatureBundle;
use prism_keys::VerifyingKey;
use serde::{Deserialize, Serialize};

use crate::app::AppState;
use crate::config::AppConfig;
use crate::ops::{add_data, add_key, get_account, request_create_account, send_create_account};
use crate::utils::{parse_cosmos_adr36_verifying_key, parse_signature_bundle};

#[derive(Deserialize, Serialize, Debug)]
struct SendCreateAccountRequest {
    id: String,
    // The verifying key is in base64 format
    verifying_key: String,
    // The signature is in base64 format
    signature: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct RequestCreateAccountRequest {
    id: String,
    verifying_key: String,
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
struct RequestCreateAccountResponse {
    payload: Vec<u8>,
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
        .route("/v1/account/send-create", post(send_create_account_handler))
        .route("/v1/account/request-create", post(request_create_account_handler))
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

async fn request_create_account_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RequestCreateAccountRequest>,
) -> impl IntoResponse {
    let state = state.clone();
    let verifying_key = parse_cosmos_adr36_verifying_key(req.verifying_key).unwrap();
    let bytes_to_be_signed = request_create_account(state, req.id, verifying_key).await.unwrap();

    (StatusCode::OK, Json(RequestCreateAccountResponse { payload: bytes_to_be_signed }))
}

async fn send_create_account_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SendCreateAccountRequest>,
) -> impl IntoResponse {
    let state = state.clone();
    let signature_bundle = parse_signature_bundle(req.verifying_key, req.signature).unwrap();
    let account = send_create_account(state, req.id, signature_bundle).await.unwrap();

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
