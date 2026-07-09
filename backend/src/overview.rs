use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde_json::json;

use crate::auth::AppState;
use shared_types::CurahHujanOverview;

pub async fn curah_hujan_overview(
    State(state): State<Arc<AppState>>,
) -> Result<Json<CurahHujanOverview>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    let total_pos: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM pos_curah_hujan")
        .fetch_one(&state.pool)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    let total_data: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM data_curah_hujan")
        .fetch_one(&state.pool)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    let statuses = vec!["mentah", "terverifikasi", "tervalidasi", "terkoreksi", "ditolak"];

    let mut per_status = Vec::new();
    for s in statuses {
        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM data_curah_hujan WHERE status_mutu = $1::data_status")
                .bind(s)
                .fetch_one(&state.pool)
                .await
                .map_err(|e| {
                    (
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": e.to_string()})),
                    )
                })?;
        per_status.push(shared_types::StatusCount {
            status: s.to_string(),
            count,
        });
    }

    Ok(Json(CurahHujanOverview {
        total_pos,
        total_data,
        per_status,
    }))
}
