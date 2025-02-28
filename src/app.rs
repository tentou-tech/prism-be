use std::sync::Arc;

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
