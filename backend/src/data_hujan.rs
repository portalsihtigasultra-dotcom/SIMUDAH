use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use sqlx::Row;
use thiserror::Error;

use crate::auth::{AppState, AuthUser};
use crate::log_mutu;
use shared_types::{CreateDataCurahHujanRequest, DataCurahHujan, VerifyDataRequest};

#[derive(Error, Debug)]
pub enum DataHujanError {
    #[error("Not found")]
    NotFound,
    #[error("Forbidden")]
    Forbidden,
    #[error("Invalid status")]
    InvalidStatus,
    #[error("Wrong role")]
    WrongRole,
    #[error("Invalid transition")]
    InvalidTransition,
    #[error("Database error")]
    Db(#[from] sqlx::Error),
}

impl IntoResponse for DataHujanError {
    fn into_response(self) -> Response {
        let (status, body) = match &self {
            DataHujanError::NotFound => (StatusCode::NOT_FOUND, "Data tidak ditemukan"),
            DataHujanError::Forbidden => (StatusCode::FORBIDDEN, "Akses ditolak"),
            DataHujanError::InvalidStatus => {
                (StatusCode::BAD_REQUEST, "Hanya data mentah yang bisa diubah")
            }
            DataHujanError::WrongRole => {
                (StatusCode::FORBIDDEN, "Anda tidak memiliki wewenang")
            }
            DataHujanError::InvalidTransition => {
                (StatusCode::BAD_REQUEST, "Transisi status tidak valid")
            }
            DataHujanError::Db(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
        };
        (status, Json(json!({"error": body}))).into_response()
    }
}

fn row_to_data(r: &sqlx::postgres::PgRow) -> DataCurahHujan {
    DataCurahHujan {
        id: r.get("id"),
        pos_id: r.get("pos_id"),
        tanggal: r.get("tanggal"),
        nilai_mm: r.get("nilai_mm"),
        jam_pengamatan: r.get("jam_pengamatan"),
        petugas_id: r.get("petugas_id"),
        status_mutu: r.get("status_mutu"),
    }
}

pub async fn list_data_hujan(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<DataCurahHujan>>, DataHujanError> {
    let rows = sqlx::query(
        r#"SELECT id, pos_id, tanggal::text, nilai_mm,
           jam_pengamatan::text, petugas_id, status_mutu::text
           FROM data_curah_hujan
           ORDER BY tanggal DESC, id DESC"#,
    )
    .fetch_all(&state.pool)
    .await?
    .iter()
    .map(row_to_data)
    .collect::<Vec<_>>();

    Ok(Json(rows))
}

pub async fn get_data_hujan(
    Path(id): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<DataCurahHujan>, DataHujanError> {
    let row = sqlx::query(
        r#"SELECT id, pos_id, tanggal::text, nilai_mm,
           jam_pengamatan::text, petugas_id, status_mutu::text
           FROM data_curah_hujan WHERE id = $1"#,
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .map(|r| row_to_data(&r))
    .ok_or(DataHujanError::NotFound)?;

    Ok(Json(row))
}

pub async fn create_data_hujan(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
    Json(req): Json<CreateDataCurahHujanRequest>,
) -> Result<(StatusCode, Json<DataCurahHujan>), DataHujanError> {
    let row = sqlx::query(
        r#"INSERT INTO data_curah_hujan (pos_id, tanggal, nilai_mm, jam_pengamatan, petugas_id)
        VALUES ($1, $2::date, $3, $4::time, $5)
        RETURNING id, pos_id, tanggal::text, nilai_mm,
           jam_pengamatan::text, petugas_id, status_mutu::text"#,
    )
    .bind(req.pos_id)
    .bind(&req.tanggal)
    .bind(req.nilai_mm)
    .bind(req.jam_pengamatan.as_deref())
    .bind(user.user_id)
    .fetch_one(&state.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(row_to_data(&row))))
}

pub async fn update_data_hujan(
    Path(id): Path<i64>,
    State(state): State<Arc<AppState>>,
    user: AuthUser,
    Json(req): Json<CreateDataCurahHujanRequest>,
) -> Result<Json<DataCurahHujan>, DataHujanError> {
    let existing = sqlx::query(
        "SELECT petugas_id, status_mutu::text FROM data_curah_hujan WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(DataHujanError::NotFound)?;

    let owner_id: i64 = existing.get("petugas_id");
    let status: String = existing.get("status_mutu");

    if owner_id != user.user_id {
        return Err(DataHujanError::Forbidden);
    }
    if status != "mentah" {
        return Err(DataHujanError::InvalidStatus);
    }

    let row = sqlx::query(
        r#"UPDATE data_curah_hujan
        SET pos_id = $1, tanggal = $2::date, nilai_mm = $3,
            jam_pengamatan = $4::time, updated_at = NOW()
        WHERE id = $5
        RETURNING id, pos_id, tanggal::text, nilai_mm,
           jam_pengamatan::text, petugas_id, status_mutu::text"#,
    )
    .bind(req.pos_id)
    .bind(&req.tanggal)
    .bind(req.nilai_mm)
    .bind(req.jam_pengamatan.as_deref())
    .bind(id)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(row_to_data(&row)))
}

pub async fn delete_data_hujan(
    Path(id): Path<i64>,
    State(state): State<Arc<AppState>>,
    user: AuthUser,
) -> Result<StatusCode, DataHujanError> {
    let existing = sqlx::query(
        "SELECT petugas_id, status_mutu::text FROM data_curah_hujan WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(DataHujanError::NotFound)?;

    let owner_id: i64 = existing.get("petugas_id");
    let status: String = existing.get("status_mutu");

    if owner_id != user.user_id {
        return Err(DataHujanError::Forbidden);
    }
    if status != "mentah" {
        return Err(DataHujanError::InvalidStatus);
    }

    sqlx::query("DELETE FROM data_curah_hujan WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn verify_data_hujan(
    Path(id): Path<i64>,
    State(state): State<Arc<AppState>>,
    user: AuthUser,
    Json(req): Json<VerifyDataRequest>,
) -> Result<Json<DataCurahHujan>, DataHujanError> {
    if user.role != "verifikator" {
        return Err(DataHujanError::WrongRole);
    }

    let valid_statuses = ["terverifikasi", "ditolak"];
    if !valid_statuses.contains(&req.status.as_str()) {
        return Err(DataHujanError::InvalidTransition);
    }

    let existing = sqlx::query(
        "SELECT petugas_id, status_mutu::text FROM data_curah_hujan WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(DataHujanError::NotFound)?;

    let old_status: String = existing.get("status_mutu");
    if old_status != "mentah" {
        return Err(DataHujanError::InvalidStatus);
    }

    let row = sqlx::query(
        r#"UPDATE data_curah_hujan
        SET status_mutu = $1::data_status, updated_at = NOW()
        WHERE id = $2
        RETURNING id, pos_id, tanggal::text, nilai_mm,
           jam_pengamatan::text, petugas_id, status_mutu::text"#,
    )
    .bind(&req.status)
    .bind(id)
    .fetch_one(&state.pool)
    .await?;

    log_mutu::create_log_mutu(
        &state.pool,
        id,
        &old_status,
        &req.status,
        user.user_id,
        req.catatan.as_deref(),
    )
    .await?;

    Ok(Json(row_to_data(&row)))
}
