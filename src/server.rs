use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use prism_client::Account;
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

use crate::app::{AppError, AppState, HandlerResult};
use crate::config::AppConfig;
use crate::ops::{add_key, get_account, request_create_account, send_create_account};
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
    verifying_key: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct AddDataRequest {
    id: String,
    data: String,
}

#[derive(Serialize)]
struct AccountResult {
    id: String,
}

#[derive(Serialize)]
struct AccountInfo {
    id: String,
    nonce: u64,
    data: Vec<String>,
    keys: Vec<String>,
}

#[derive(Serialize)]
struct ListAccountsResponse {
    accounts: Vec<AccountInfo>,
}

#[derive(Deserialize, Serialize, Debug)]
struct RequestCreateAccountResponse {
    payload: Vec<u8>,
}

#[derive(Deserialize, Serialize, Debug)]
struct GetDataResponse {
    data: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct ListKeysRequest {
    id: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct AddAccountRequest {
    id: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct GetKeyResponse {
    key: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct GetAccountQuery {
    id: String,
}

// Run the server with the given app state and config
// Panics if the server fails to start
pub async fn run_server(app_state: Arc<AppState>, config: AppConfig) {
    // Wrap app_state in Arc
    let app_state = app_state.clone();

    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any).allow_headers(Any);

    // Build the router
    let app = Router::new()
        .route("/v1/health", get(health_check_handler))
        .route("/v1/account/get", get(get_account_handler))
        .route("/v1/account/add-manual", post(add_account_handler))
        .route("/v1/account/get-key", get(get_key_handler))
        .route("/v1/account/get-data", get(get_data_handler))
        .route("/v1/account/send-create", post(send_create_account_handler))
        .route("/v1/account/request-create", post(request_create_account_handler))
        .route("/v1/account/add-key", post(add_key_handler))
        .route("/v1/account/add-data", post(add_data_handler))
        .route("/v1/account/list-accounts", get(list_accounts_handler))
        .route("/v1/account/list-keys", get(list_keys_handler))
        .with_state(app_state)
        .layer(cors);

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
) -> HandlerResult<impl IntoResponse> {
    let state = state.clone();
    let verifying_key = parse_cosmos_adr36_verifying_key(req.verifying_key)
        .map_err(|e| AppError(anyhow::anyhow!("Invalid verifying key: {}", e)))?;
    let bytes_to_be_signed = request_create_account(state, req.id, verifying_key)
        .await
        .map_err(|e| AppError(anyhow::anyhow!("Failed to request account creation: {}", e)))?;

    Ok((StatusCode::OK, Json(RequestCreateAccountResponse { payload: bytes_to_be_signed })))
}

async fn send_create_account_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SendCreateAccountRequest>,
) -> HandlerResult<impl IntoResponse> {
    let state = state.clone();
    let signature_bundle = parse_signature_bundle(req.verifying_key, req.signature)
        .map_err(|e| AppError(anyhow::anyhow!("Invalid signature bundle: {}", e)))?;
    let account = send_create_account(state, req.id, signature_bundle)
        .await
        .map_err(|e| AppError(anyhow::anyhow!("Failed to send account creation: {}", e)))?;

    Ok((StatusCode::OK, Json(AccountResult { id: account.id().to_string() })))
}

async fn add_key_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AddKeyRequest>,
) -> HandlerResult<impl IntoResponse> {
    let state = state.clone();
    let new_key = parse_cosmos_adr36_verifying_key(req.verifying_key)
        .map_err(|e| AppError(anyhow::anyhow!("Invalid verifying key: {}", e)))?;
    let account = add_key(state, req.id, new_key)
        .await
        .map_err(|e| AppError(anyhow::anyhow!("Failed to add key: {}", e)))?;

    Ok((StatusCode::OK, Json(AccountResult { id: account.id().to_string() })))
}

#[derive(Deserialize, Serialize, Debug)]
struct GetDataQuery {
    id: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct GetKeyQuery {
    id: String,
}
async fn get_data_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<GetDataQuery>,
) -> HandlerResult<impl IntoResponse> {
    let state = state.clone();
    let data = state.db.clone().get_data(query.id);
    Ok((StatusCode::OK, Json(GetDataResponse { data })))
}

async fn get_key_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<GetKeyQuery>,
) -> HandlerResult<impl IntoResponse> {
    let state = state.clone();
    let key = state.db.clone().get_key(query.id);
    Ok((StatusCode::OK, Json(GetKeyResponse { key })))
}

async fn add_data_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AddDataRequest>,
) -> HandlerResult<impl IntoResponse> {
    let state = state.clone();
    state.db.clone().insert_data(req.id.clone(), req.data.clone());
    Ok((StatusCode::OK, Json(AccountResult { id: req.id })))
}

async fn get_account_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<GetAccountQuery>,
) -> HandlerResult<impl IntoResponse> {
    tracing::info!("Getting account for {}", query.id);
    let state = state.clone();
    let account = get_account(state, query.id)
        .await
        .map_err(|e| AppError(anyhow::anyhow!("Failed to get account: {}", e)))?;

    Ok((StatusCode::OK, Json(account)))
}

async fn add_account_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AddAccountRequest>,
) -> HandlerResult<impl IntoResponse> {
    let state = state.clone();
    state.db.clone().insert_account(req.id.clone(), Account::default());
    Ok((StatusCode::OK, Json(AccountResult { id: req.id })))
}

async fn list_accounts_handler(
    State(state): State<Arc<AppState>>,
) -> HandlerResult<impl IntoResponse> {
    let state = state.clone();
    let accounts = state.db.clone().get_accounts();

    let mut accounts_info = Vec::new();
    for id in accounts {
        match get_account(state.clone(), id.clone()).await {
            Ok(account) => {
                let keys = state.db.clone().get_keys(id.clone());
                let data = state.db.clone().get_data(id.clone());
                accounts_info.push(AccountInfo {
                    id: id.clone(),
                    keys,
                    data,
                    nonce: account.account.unwrap_or_default().nonce(),
                });
            }
            Err(e) => {
                tracing::warn!("Failed to get account for {}: {}", id, e);
                continue; // Skip this account and continue with the next
            }
        }
    }

    Ok((StatusCode::OK, Json(ListAccountsResponse { accounts: accounts_info })))
}

async fn list_keys_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> HandlerResult<impl IntoResponse> {
    let state = state.clone();
    let keys = state.db.clone().get_keys(id);

    Ok((StatusCode::OK, Json(keys)))
}
