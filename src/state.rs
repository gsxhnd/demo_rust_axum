use crate::models::User;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<RwLock<Vec<User>>>,
    pub db: DatabaseConnection,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            users: Arc::new(RwLock::new(vec![])),
            db,
        }
    }
}
