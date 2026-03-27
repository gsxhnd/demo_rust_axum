use axum::{Router, routing::get};

use crate::handlers::{create_user, delete_user, get_user, get_users, health_check};
use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/users", get(get_users).post(create_user))
        .route("/users/{id}", get(get_user).delete(delete_user))
        .with_state(state)
}
