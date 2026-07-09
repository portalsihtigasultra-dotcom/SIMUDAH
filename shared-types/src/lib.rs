use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct HealthResponse {
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub role: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PosCurahHujan {
    pub id: i64,
    pub nama: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub elevasi: Option<i32>,
    pub tipe_alat: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreatePosRequest {
    pub nama: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub elevasi: Option<i32>,
    pub tipe_alat: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DataCurahHujan {
    pub id: i64,
    pub pos_id: i64,
    pub tanggal: String,
    pub nilai_mm: f64,
    pub jam_pengamatan: Option<String>,
    pub petugas_id: i64,
    pub status_mutu: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateDataCurahHujanRequest {
    pub pos_id: i64,
    pub tanggal: String,
    pub nilai_mm: f64,
    pub jam_pengamatan: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LogMutu {
    pub id: i64,
    pub data_id: i64,
    pub status_sebelum: String,
    pub status_sesudah: String,
    pub diubah_oleh: i64,
    pub catatan: Option<String>,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VerifyDataRequest {
    pub status: String,
    pub catatan: Option<String>,
}
