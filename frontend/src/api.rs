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
