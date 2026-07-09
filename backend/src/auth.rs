use std::sync::Arc;

use argon2::password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use axum::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use thiserror::Error;

use shared_types::{AuthResponse, LoginRequest, RegisterRequest, UserResponse};

const JWT_SECRET: &str = "simudah_dev_secret_key_2026";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub role: String,
    pub exp: usize,
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Username already taken")]
    UsernameTaken,
    #[error("Database error")]
    Db(#[from] sqlx::Error),
    #[error("Token error")]
    Token,
    #[error("Internal error")]
    Internal,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, body) = match &self {
            AuthError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Invalid username or password")
            }
            AuthError::UsernameTaken => (StatusCode::CONFLICT, "Username already taken"),
            AuthError::Token => (StatusCode::UNAUTHORIZED, "Invalid token"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };
        (status, Json(serde_json::json!({"error": body}))).into_response()
    }
}

pub struct AppState {
    pub pool: PgPool,
}

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: i64,
    pub role: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or(AuthError::Token)?;

        let token_data = decode::<Claims>(
            auth_header,
            &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AuthError::Token)?;

        Ok(AuthUser {
            user_id: token_data.claims.sub,
            role: token_data.claims.role,
        })
    }
}

fn generate_token(user_id: i64, role: &str) -> Result<String, AuthError> {
    let exp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + 86400;

    let claims = Claims {
        sub: user_id,
        role: role.to_string(),
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .map_err(|_| AuthError::Internal)
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AuthError> {
    if req.username.is_empty() || req.password.is_empty() {
        return Err(AuthError::InvalidCredentials);
    }

    let existing: Option<(i64,)> =
        sqlx::query_as("SELECT id FROM users WHERE username = $1")
            .bind(&req.username)
            .fetch_optional(&state.pool)
            .await?;

    if existing.is_some() {
        return Err(AuthError::UsernameTaken);
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(req.password.as_bytes(), &salt)
        .map_err(|_| AuthError::Internal)?
        .to_string();

    let user_id: (i64,) = sqlx::query_as(
        "INSERT INTO users (username, password_hash, role) VALUES ($1, $2, 'petugas'::user_role) RETURNING id",
    )
    .bind(&req.username)
    .bind(&password_hash)
    .fetch_one(&state.pool)
    .await?;

    let token = generate_token(user_id.0, "petugas")?;

    Ok(Json(AuthResponse {
        token,
        user: UserResponse {
            id: user_id.0,
            username: req.username,
            role: "petugas".to_string(),
        },
    }))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AuthError> {
    let row: Option<(i64, String, String)> = sqlx::query_as(
        "SELECT id, username, password_hash FROM users WHERE username = $1",
    )
    .bind(&req.username)
    .fetch_optional(&state.pool)
    .await?;

    let (user_id, username, password_hash) = row.ok_or(AuthError::InvalidCredentials)?;

    let parsed_hash = PasswordHash::new(&password_hash).map_err(|_| AuthError::Internal)?;
    let argon2 = Argon2::default();
    let valid = argon2
        .verify_password(req.password.as_bytes(), &parsed_hash)
        .is_ok();

    if !valid {
        return Err(AuthError::InvalidCredentials);
    }

    let role: (String,) =
        sqlx::query_as("SELECT role::text FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_one(&state.pool)
            .await?;

    let token = generate_token(user_id, &role.0)?;

    Ok(Json(AuthResponse {
        token,
        user: UserResponse {
            id: user_id,
            username,
            role: role.0,
        },
    }))
}
