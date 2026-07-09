use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use sqlx::Row;
use thiserror::Error;

use crate::auth::AppState;
use shared_types::LogMutu;

#[derive(Error, Debug)]
pub enum LogMutuError {
    #[error("Database error")]
    Db(#[from] sqlx::Error),
}

impl IntoResponse for LogMutuError {
    fn into_response(self) -> Response {
        let (status, body) = match &self {
            LogMutuError::Db(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
        };
        (status, Json(json!({"error": body}))).into_response()
    }
}

fn row_to_log(r: &sqlx::postgres::PgRow) -> LogMutu {
    LogMutu {
        id: r.get("id"),
        data_id: r.get("data_id"),
        status_sebelum: r.get("status_sebelum"),
        status_sesudah: r.get("status_sesudah"),
        diubah_oleh: r.get("diubah_oleh"),
        catatan: r.get("catatan"),
        created_at: r.get("created_at"),
    }
}

pub async fn create_log_mutu(
    pool: &sqlx::PgPool,
    data_id: i64,
    status_sebelum: &str,
    status_sesudah: &str,
    diubah_oleh: i64,
    catatan: Option<&str>,
) -> Result<LogMutu, sqlx::Error> {
    let row = sqlx::query(
        r#"INSERT INTO log_mutu (data_id, status_sebelum, status_sesudah, diubah_oleh, catatan)
        VALUES ($1, $2::data_status, $3::data_status, $4, $5)
        RETURNING id, data_id, status_sebelum::text, status_sesudah::text,
           diubah_oleh, catatan, created_at::text"#,
    )
    .bind(data_id)
    .bind(status_sebelum)
    .bind(status_sesudah)
    .bind(diubah_oleh)
    .bind(catatan)
    .fetch_one(pool)
    .await?;

    Ok(row_to_log(&row))
}

pub async fn list_log_mutu(
    Path(data_id): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<LogMutu>>, LogMutuError> {
    let rows = sqlx::query(
        r#"SELECT id, data_id, status_sebelum::text, status_sesudah::text,
           diubah_oleh, catatan, created_at::text
           FROM log_mutu
           WHERE data_id = $1
           ORDER BY created_at ASC"#,
    )
    .bind(data_id)
    .fetch_all(&state.pool)
    .await?
    .iter()
    .map(row_to_log)
    .collect::<Vec<_>>();

    Ok(Json(rows))
}
