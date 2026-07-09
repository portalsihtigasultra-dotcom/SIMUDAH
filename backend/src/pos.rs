use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use sqlx::Row;
use thiserror::Error;

use crate::auth::{AppState, AuthUser};
use shared_types::{CreatePosRequest, PosCurahHujan};

#[derive(Error, Debug)]
pub enum PosError {
    #[error("Pos not found")]
    NotFound,
    #[error("Database error")]
    Db(#[from] sqlx::Error),
}

impl IntoResponse for PosError {
    fn into_response(self) -> Response {
        let (status, body) = match &self {
            PosError::NotFound => (StatusCode::NOT_FOUND, "Pos tidak ditemukan"),
            PosError::Db(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
        };
        (status, Json(json!({"error": body}))).into_response()
    }
}

fn row_to_pos(r: &sqlx::postgres::PgRow) -> PosCurahHujan {
    PosCurahHujan {
        id: r.get("id"),
        nama: r.get("nama"),
        latitude: r.get("latitude"),
        longitude: r.get("longitude"),
        elevasi: r.get("elevasi"),
        tipe_alat: r.get("tipe_alat"),
    }
}

pub async fn list_pos(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<PosCurahHujan>>, PosError> {
    let rows = sqlx::query(
        "SELECT id, nama, latitude, longitude, elevasi, tipe_alat FROM pos_curah_hujan ORDER BY nama",
    )
    .fetch_all(&state.pool)
    .await?
    .iter()
    .map(row_to_pos)
    .collect::<Vec<_>>();

    Ok(Json(rows))
}

pub async fn get_pos(
    Path(id): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<PosCurahHujan>, PosError> {
    let row = sqlx::query(
        "SELECT id, nama, latitude, longitude, elevasi, tipe_alat FROM pos_curah_hujan WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .map(|r| row_to_pos(&r))
    .ok_or(PosError::NotFound)?;

    Ok(Json(row))
}

pub async fn create_pos(
    State(state): State<Arc<AppState>>,
    _user: AuthUser,
    Json(req): Json<CreatePosRequest>,
) -> Result<(StatusCode, Json<PosCurahHujan>), PosError> {
    let row = sqlx::query(
        r#"INSERT INTO pos_curah_hujan (nama, latitude, longitude, elevasi, tipe_alat)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, nama, latitude, longitude, elevasi, tipe_alat"#,
    )
    .bind(&req.nama)
    .bind(req.latitude)
    .bind(req.longitude)
    .bind(req.elevasi)
    .bind(&req.tipe_alat)
    .fetch_one(&state.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(row_to_pos(&row))))
}

pub async fn update_pos(
    Path(id): Path<i64>,
    State(state): State<Arc<AppState>>,
    _user: AuthUser,
    Json(req): Json<CreatePosRequest>,
) -> Result<Json<PosCurahHujan>, PosError> {
    let row = sqlx::query(
        r#"UPDATE pos_curah_hujan
        SET nama = $1, latitude = $2, longitude = $3, elevasi = $4, tipe_alat = $5, updated_at = NOW()
        WHERE id = $6
        RETURNING id, nama, latitude, longitude, elevasi, tipe_alat"#,
    )
    .bind(&req.nama)
    .bind(req.latitude)
    .bind(req.longitude)
    .bind(req.elevasi)
    .bind(&req.tipe_alat)
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .map(|r| row_to_pos(&r))
    .ok_or(PosError::NotFound)?;

    Ok(Json(row))
}

pub async fn delete_pos(
    Path(id): Path<i64>,
    State(state): State<Arc<AppState>>,
    _user: AuthUser,
) -> Result<StatusCode, PosError> {
    let result = sqlx::query("DELETE FROM pos_curah_hujan WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(PosError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
