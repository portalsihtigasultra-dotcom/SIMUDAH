use shared_types::{CreatePosRequest, PosCurahHujan};

const API_BASE: &str = "http://localhost:3000";

pub async fn list_pos(token: &str) -> Result<Vec<PosCurahHujan>, String> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/api/pos", API_BASE))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("Gagal memuat pos (HTTP {})", resp.status()));
    }

    resp.json::<Vec<PosCurahHujan>>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_pos(token: &str, id: i64) -> Result<PosCurahHujan, String> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/api/pos/{}", API_BASE, id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("Pos tidak ditemukan (HTTP {})", resp.status()));
    }

    resp.json::<PosCurahHujan>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn create_pos(
    token: &str,
    req: CreatePosRequest,
) -> Result<PosCurahHujan, String> {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/api/pos", API_BASE))
        .header("Authorization", format!("Bearer {}", token))
        .json(&req)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        let err = resp.json::<serde_json::Value>().await.ok();
        return Err(err
            .and_then(|v| {
                v.get("error")
                    .and_then(|e| e.as_str().map(|s| s.to_string()))
            })
            .unwrap_or("Gagal membuat pos".to_string()));
    }

    resp.json::<PosCurahHujan>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn update_pos(
    token: &str,
    id: i64,
    req: CreatePosRequest,
) -> Result<PosCurahHujan, String> {
    let client = reqwest::Client::new();
    let resp = client
        .put(format!("{}/api/pos/{}", API_BASE, id))
        .header("Authorization", format!("Bearer {}", token))
        .json(&req)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        let err = resp.json::<serde_json::Value>().await.ok();
        return Err(err
            .and_then(|v| {
                v.get("error")
                    .and_then(|e| e.as_str().map(|s| s.to_string()))
            })
            .unwrap_or("Gagal mengupdate pos".to_string()));
    }

    resp.json::<PosCurahHujan>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn list_data_hujan(
    token: &str,
) -> Result<Vec<shared_types::DataCurahHujan>, String> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/api/data-hujan", API_BASE))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("Gagal memuat data (HTTP {})", resp.status()));
    }

    resp.json::<Vec<shared_types::DataCurahHujan>>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn create_data_hujan(
    token: &str,
    req: shared_types::CreateDataCurahHujanRequest,
) -> Result<shared_types::DataCurahHujan, String> {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/api/data-hujan", API_BASE))
        .header("Authorization", format!("Bearer {}", token))
        .json(&req)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        let err = resp.json::<serde_json::Value>().await.ok();
        return Err(err
            .and_then(|v| {
                v.get("error")
                    .and_then(|e| e.as_str().map(|s| s.to_string()))
            })
            .unwrap_or("Gagal menyimpan data".to_string()));
    }

    resp.json::<shared_types::DataCurahHujan>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_data_hujan(
    token: &str,
    id: i64,
) -> Result<shared_types::DataCurahHujan, String> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/api/data-hujan/{}", API_BASE, id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("Data tidak ditemukan (HTTP {})", resp.status()));
    }

    resp.json::<shared_types::DataCurahHujan>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn update_data_hujan(
    token: &str,
    id: i64,
    req: shared_types::CreateDataCurahHujanRequest,
) -> Result<shared_types::DataCurahHujan, String> {
    let client = reqwest::Client::new();
    let resp = client
        .put(format!("{}/api/data-hujan/{}", API_BASE, id))
        .header("Authorization", format!("Bearer {}", token))
        .json(&req)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        let err = resp.json::<serde_json::Value>().await.ok();
        return Err(err
            .and_then(|v| {
                v.get("error")
                    .and_then(|e| e.as_str().map(|s| s.to_string()))
            })
            .unwrap_or("Gagal mengupdate data".to_string()));
    }

    resp.json::<shared_types::DataCurahHujan>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn verify_data_hujan(
    token: &str,
    id: i64,
    req: shared_types::VerifyDataRequest,
) -> Result<shared_types::DataCurahHujan, String> {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/api/data-hujan/{}/verify", API_BASE, id))
        .header("Authorization", format!("Bearer {}", token))
        .json(&req)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        let err = resp.json::<serde_json::Value>().await.ok();
        return Err(err
            .and_then(|v| {
                v.get("error")
                    .and_then(|e| e.as_str().map(|s| s.to_string()))
            })
            .unwrap_or("Gagal memverifikasi data".to_string()));
    }

    resp.json::<shared_types::DataCurahHujan>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn delete_data_hujan(token: &str, id: i64) -> Result<(), String> {
    let client = reqwest::Client::new();
    let resp = client
        .delete(format!("{}/api/data-hujan/{}", API_BASE, id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("Gagal menghapus data (HTTP {})", resp.status()));
    }

    Ok(())
}

pub async fn validate_data_hujan(
    token: &str,
    id: i64,
    req: shared_types::ValidasiDataRequest,
) -> Result<shared_types::DataCurahHujan, String> {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/api/data-hujan/{}/validate", API_BASE, id))
        .header("Authorization", format!("Bearer {}", token))
        .json(&req)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        let err = resp.json::<serde_json::Value>().await.ok();
        return Err(err
            .and_then(|v| {
                v.get("error")
                    .and_then(|e| e.as_str().map(|s| s.to_string()))
            })
            .unwrap_or("Gagal memvalidasi data".to_string()));
    }

    resp.json::<shared_types::DataCurahHujan>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn list_log_mutu(token: &str, data_id: i64) -> Result<Vec<shared_types::LogMutu>, String> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/api/data-hujan/{}/log", API_BASE, data_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("Gagal memuat log (HTTP {})", resp.status()));
    }

    resp.json::<Vec<shared_types::LogMutu>>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn delete_pos(token: &str, id: i64) -> Result<(), String> {
    let client = reqwest::Client::new();
    let resp = client
        .delete(format!("{}/api/pos/{}", API_BASE, id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("Gagal menghapus pos (HTTP {})", resp.status()));
    }

    Ok(())
}
