use std::sync::Arc;

use keystore_rs::{KeyChain, KeyStore};
use prism_be::app::AppState;
use prism_be::config::parse_config;
use prism_be::ops;
use prism_be::server::run_server;
use prism_client::SigningKey;
use prism_da::DataAvailabilityLayer;
use prism_da::memory::InMemoryDataAvailabilityLayer;
use prism_prover::webserver::WebServerConfig;
use prism_prover::{Config, Prover};
use prism_storage::inmemory::InMemoryDatabase;
use tokio::spawn;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app_config = parse_config("config.toml").unwrap();

    tracing::info!("App config: {:?}", app_config);

    let db = InMemoryDatabase::new();
    let (da_layer, _, _) = InMemoryDataAvailabilityLayer::new(3);

    let keystore_sk = KeyChain
        .get_or_create_signing_key(&app_config.service_id)
        .map_err(|e| anyhow::anyhow!("Error getting key from store: {}", e))
        .unwrap();

    let service_sk = SigningKey::Ed25519(Box::new(keystore_sk.clone()));

    let cfg = Config {
        prover: true,
        batcher: true,
        webserver: WebServerConfig { enabled: false, host: "0.0.0.0".to_string(), port: 50524 },
        signing_key: service_sk.clone(),
        verifying_key: service_sk.verifying_key(),
        start_height: 1,
    };

    let prover = Arc::new(
        Prover::new(
            Arc::new(Box::new(db)),
            Arc::new(da_layer) as Arc<dyn DataAvailabilityLayer>,
            &cfg,
        )
        .unwrap(),
    );

    let state = Arc::new(AppState {
        prover: prover.clone(),
        service_id: app_config.service_id.clone(),
        service_sk,
    });

    let state_clone = state.clone();

    let server_handle = spawn(async move {
        tracing::info!("Starting server...");
        run_server(state_clone, app_config).await;
    });

    let runner = prover.clone();
    let runner_handle = spawn(async move {
        tracing::debug!("starting prover");
        if let Err(e) = runner.run().await {
            tracing::error!("Error occurred while running prover: {:?}", e);
        }
    });

    ops::register_service(state).await.unwrap();

    tokio::select! {
        _ = runner_handle => {
            println!("Prover runner task completed");
        }
        _ = server_handle => {
            println!("Server task completed");
        }
    }
}
