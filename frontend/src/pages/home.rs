use leptos::prelude::*;
use leptos::task::spawn_local;
use shared_types::HealthResponse;

use crate::auth::{logout, AuthContext};

#[component]
pub fn HomePage() -> impl IntoView {
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

    let auth_for_logout = auth.clone();
    let on_logout = move |_| {
        logout(&auth_for_logout);
    };

    view! {
        <div>
            <div style="display: flex; justify-content: space-between">
                <h1>"SIMUDAH"</h1>
                <div>
                    <span>{move || auth.user.get().map(|u| u.username).unwrap_or("".to_string())}</span>
                    <button on:click=on_logout>"Logout"</button>
                </div>
            </div>
            <p>"Status backend: " {status}</p>
        </div>
    }
}
