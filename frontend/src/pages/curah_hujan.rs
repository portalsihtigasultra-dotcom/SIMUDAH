use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;

use crate::api;
use crate::auth::AuthContext;

#[component]
pub fn CurahHujanOverview() -> impl IntoView {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let (overview, set_overview) = signal::<Option<shared_types::CurahHujanOverview>>(None);
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(String::new());

    {
        let token = auth.token.get();
        set_loading.set(true);
        spawn_local(async move {
            if let Some(t) = token {
                match api::get_curah_hujan_overview(&t).await {
                    Ok(data) => set_overview.set(Some(data)),
                    Err(e) => set_error.set(e),
                }
                set_loading.set(false);
            }
        });
    }

    fn status_label(s: &str) -> &'static str {
        match s {
            "mentah" => "Mentah",
            "terverifikasi" => "Terverifikasi",
            "tervalidasi" => "Tervalidasi",
            "terkoreksi" => "Terkoreksi",
            "ditolak" => "Ditolak",
            _ => "Tidak Diketahui",
        }
    }

    fn status_style(s: &str) -> &'static str {
        match s {
            "mentah" => "color: #856404; background: #fff3cd",
            "terverifikasi" => "color: #004085; background: #cce5ff",
            "tervalidasi" => "color: #155724; background: #d4edda",
            "terkoreksi" => "color: #721c24; background: #f8d7da",
            "ditolak" => "color: #383d41; background: #e2e3e5",
            _ => "color: #333; background: #f5f5f5",
        }
    }

    view! {
        <div>
            <h2>"Curah Hujan"</h2>

            {move || {
                if loading.get() {
                    return view! { <p>"Memuat..."</p> }.into_any();
                }
                let msg = error.get();
                if !msg.is_empty() {
                    return view! { <p style="color: red">{msg}</p> }.into_any();
                }
                match overview.get() {
                    None => view! { <p>"Tidak ada data."</p> }.into_any(),
                    Some(ov) => {
                        view! {
                            <div style="display: flex; gap: 16px; margin: 16px 0; flex-wrap: wrap">
                                <div style="flex: 1; min-width: 180px; padding: 20px; background: #f5f5f5; border-radius: 8px; text-align: center">
                                    <div style="font-size: 2em; font-weight: bold">{ov.total_pos}</div>
                                    <div style="margin-top: 4px; color: #666">"Pos"</div>
                                </div>
                                <div style="flex: 1; min-width: 180px; padding: 20px; background: #f5f5f5; border-radius: 8px; text-align: center">
                                    <div style="font-size: 2em; font-weight: bold">{ov.total_data}</div>
                                    <div style="margin-top: 4px; color: #666">"Total Data"</div>
                                </div>
                            </div>

                            <div style="display: flex; gap: 16px; margin: 16px 0; flex-wrap: wrap">
                                {ov.per_status.iter().map(|s| {
                                    let label = status_label(&s.status);
                                    let style = format!("flex: 1; min-width: 150px; padding: 16px; border-radius: 8px; text-align: center; {}", status_style(&s.status));
                                    view! {
                                        <div style=style>
                                            <div style="font-size: 1.5em; font-weight: bold">{s.count}</div>
                                            <div style="margin-top: 4px">{label}</div>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>

                            <div style="margin-top: 24px; display: flex; gap: 12px; flex-wrap: wrap">
                                <A href="/data-hujan/input"
                                    attr:style="background: #1a1a2e; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px"
                                >
                                    "Input Data Baru"
                                </A>
                                <A href="/data-hujan"
                                    attr:style="background: #1a1a2e; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px"
                                >
                                    "Lihat Semua Data"
                                </A>
                                <A href="/pos"
                                    attr:style="background: #1a1a2e; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px"
                                >
                                    "Kelola Pos"
                                </A>
                            </div>
                        }.into_any()
                    }
                }
            }}
        </div>
    }
}

#[component]
pub fn CurahHujanInput() -> impl IntoView {
    view! {
        <div>
            <h2>"Input Curah Hujan"</h2>
            <p>"Gunakan menu Input Data Baru di halaman Curah Hujan."</p>
        </div>
    }
}
