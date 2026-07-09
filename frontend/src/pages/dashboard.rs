use leptos::prelude::*;
use leptos::task::spawn_local;
use shared_types::HealthResponse;

use crate::auth::AuthContext;

#[component]
pub fn Dashboard() -> impl IntoView {
    let (status, set_status) = signal(String::from("Memuat..."));
    let auth = use_context::<AuthContext>().expect("AuthContext not found");

    spawn_local(async move {
        let client = reqwest::Client::new();
        match client
            .get("http://localhost:3000/api/health")
            .send()
            .await
        {
            Ok(resp) => match resp.json::<HealthResponse>().await {
                Ok(data) => set_status.set(data.status),
                Err(_) => set_status.set("Gagal parse".to_string()),
            },
            Err(_) => set_status.set("Gagal fetch".to_string()),
        }
    });

    view! {
        <div>
            <h2>"Dashboard"</h2>
            <p>"Selamat datang, " {move || auth.user.get().map(|u| u.username).unwrap_or_default()}</p>
            <p>"Role: " {move || auth.user.get().map(|u| u.role).unwrap_or_default()}</p>
            <p>"Status backend: " {status}</p>
        </div>
    }
}
