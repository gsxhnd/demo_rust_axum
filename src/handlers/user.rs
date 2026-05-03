use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use tracing::info;

use crate::error::AppError;
use crate::models::{ActiveModel, CreateUserRequest, User};
use crate::state::AppState;

pub async fn get_users(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    info!("Fetching all users from database");
    let users = User::find().all(&state.db).await?;
    Ok((StatusCode::OK, Json(users)))
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    info!("Creating user: {} ({})", payload.name, payload.email);

    let user = ActiveModel {
        name: Set(payload.name.clone()),
        email: Set(payload.email.clone()),
        ..Default::default()
    };

    let user = user.insert(&state.db).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get_user(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    info!("Fetching user with id: {}", id);

    match User::find_by_id(id).one(&state.db).await? {
        Some(user) => Ok((StatusCode::OK, Json(user)).into_response()),
        None => Err(AppError::NotFound(format!("User with id {} not found", id))),
    }
}

pub async fn delete_user(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    info!("Deleting user with id: {}", id);

    let res = User::delete_by_id(id).exec(&state.db).await?;
    if res.rows_affected > 0 {
        Ok(StatusCode::NO_CONTENT.into_response())
    } else {
        Err(AppError::NotFound(format!("User with id {} not found", id)))
    }
}
