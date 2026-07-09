use leptos::prelude::*;
use leptos_router::components::A;

use crate::auth::{logout, AuthContext};

#[component]
pub fn Sidebar() -> impl IntoView {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let role = move || auth.user.get().map(|u| u.role).unwrap_or_default();

    let on_logout = {
        let auth = auth.clone();
        move |_| logout(&auth)
    };

    view! {
        <nav style="width: 220px; min-height: 100vh; background: #1a1a2e; color: white; padding: 16px; display: flex; flex-direction: column; gap: 8px">
            <h2 style="font-size: 1.2em; margin-bottom: 16px">"SIMUDAH"</h2>

            <A href="/" attr:style="color: white; text-decoration: none; padding: 8px; border-radius: 4px">
                "Dashboard"
            </A>

            <A href="/pos" attr:style="color: white; text-decoration: none; padding: 8px; border-radius: 4px">
                "Kelola Pos"
            </A>

            <A href="/data-hujan" attr:style="color: white; text-decoration: none; padding: 8px; border-radius: 4px">
                "Lihat Data Hujan"
            </A>

            {move || {
                let r = role();
                match r.as_str() {
                    "petugas" => view! {
                        <A href="/data-hujan/input" attr:style="color: white; text-decoration: none; padding: 8px; border-radius: 4px">
                            "Input Curah Hujan"
                        </A>
                    }.into_any(),
                    "verifikator" => view! {
                        <A href="/data-hujan/verifikasi" attr:style="color: white; text-decoration: none; padding: 8px; border-radius: 4px">
                            "Verifikasi Data"
                        </A>
                    }.into_any(),
                    "validator" => view! {
                        <A href="/data-hujan/validasi" attr:style="color: white; text-decoration: none; padding: 8px; border-radius: 4px">
                            "Validasi & Koreksi Data"
                        </A>
                    }.into_any(),
                    _ => view! {}.into_any(),
                }
            }}

            <div style="margin-top: auto; padding-top: 16px; border-top: 1px solid #333">
                <span>{move || auth.user.get().map(|u| u.username).unwrap_or_default()}</span>
                <span style="display: block; font-size: 0.8em; opacity: 0.7">
                    {move || auth.user.get().map(|u| u.role).unwrap_or_default()}
                </span>
                <button on:click=on_logout style="margin-top: 8px; width: 100%">"Logout"</button>
            </div>
        </nav>
    }
}
