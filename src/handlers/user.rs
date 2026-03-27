use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use tracing::{info, warn};

use crate::models::{ActiveModel, CreateUserRequest, User};
use crate::state::AppState;

pub async fn get_users(State(state): State<AppState>) -> impl IntoResponse {
    info!("Fetching all users from database");
    match User::find().all(&state.db).await {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(e) => {
            warn!("Failed to fetch users: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let user = ActiveModel {
        name: Set(payload.name.clone()),
        email: Set(payload.email.clone()),
        ..Default::default()
    };

    info!("Creating user: {} ({})", payload.name, payload.email);

    match user.insert(&state.db).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => {
            warn!("Failed to create user: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}

pub async fn get_user(Path(id): Path<i32>, State(state): State<AppState>) -> impl IntoResponse {
    info!("Fetching user with id: {}", id);

    match User::find_by_id(id).one(&state.db).await {
        Ok(Some(user)) => (StatusCode::OK, Json(user)).into_response(),
        Ok(None) => {
            warn!("User not found: {}", id);
            (StatusCode::NOT_FOUND, "User not found").into_response()
        }
        Err(e) => {
            warn!("Database error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}

pub async fn delete_user(Path(id): Path<i32>, State(state): State<AppState>) -> impl IntoResponse {
    info!("Deleting user with id: {}", id);

    match User::delete_by_id(id).exec(&state.db).await {
        Ok(res) if res.rows_affected > 0 => {
            (StatusCode::NO_CONTENT, "User deleted successfully").into_response()
        }
        Ok(_) => {
            warn!("User not found for deletion: {}", id);
            (StatusCode::NOT_FOUND, "User not found").into_response()
        }
        Err(e) => {
            warn!("Database error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}
