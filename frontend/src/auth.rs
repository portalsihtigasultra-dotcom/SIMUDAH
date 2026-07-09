use leptos::prelude::*;
use shared_types::{AuthResponse, LoginRequest};
use web_sys::window;

#[derive(Clone)]
pub struct AuthContext {
    pub token: RwSignal<Option<String>>,
    pub user: RwSignal<Option<shared_types::UserResponse>>,
    pub loading: RwSignal<bool>,
}

impl AuthContext {
    pub fn new() -> Self {
        let token = RwSignal::new(None);
        let user = RwSignal::new(None);
        let loading = RwSignal::new(true);

        if let Some(storage) = window()
            .and_then(|w| w.local_storage().ok())
            .flatten()
        {
            if let Ok(Some(saved)) = storage.get_item("token") {
                token.set(Some(saved));
            }
        }

        loading.set(false);
        let ctx = AuthContext {
            token,
            user,
            loading,
        };
        provide_context(ctx.clone());
        ctx
    }
}

pub async fn login_request(username: String, password: String) -> Result<AuthResponse, String> {
    let client = reqwest::Client::new();
    let resp = client
        .post("http://localhost:3000/api/auth/login")
        .json(&LoginRequest { username, password })
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
            .unwrap_or("Gagal login".to_string()));
    }

    resp.json::<AuthResponse>()
        .await
        .map_err(|e| e.to_string())
}

pub fn logout(auth: &AuthContext) {
    if let Some(storage) = window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
    {
        let _ = storage.remove_item("token");
    }
    auth.token.set(None);
    auth.user.set(None);
}
