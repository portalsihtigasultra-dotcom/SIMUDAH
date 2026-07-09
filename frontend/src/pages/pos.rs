use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use leptos_router::NavigateOptions;

use crate::api;
use crate::auth::AuthContext;

#[component]
pub fn ListPos() -> impl IntoView {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let (pos_list, set_pos_list) = signal(Vec::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(String::new());

    let fetch = move || {
        let token = auth.token.get();
        set_loading.set(true);
        spawn_local(async move {
            if let Some(t) = token {
                match api::list_pos(&t).await {
                    Ok(list) => {
                        set_pos_list.set(list);
                        set_loading.set(false);
                    }
                    Err(e) => {
                        set_error.set(e);
                        set_loading.set(false);
                    }
                }
            }
        });
    };

    fetch();

    let delete_pos = move |id: i64| {
        let token = auth.token.get();
        spawn_local(async move {
            if let Some(t) = token {
                let _ = api::delete_pos(&t, id).await;
                fetch();
            }
        });
    };

    view! {
        <div>
            <div style="display: flex; justify-content: space-between; align-items: center">
                <h2>"Daftar Pos Curah Hujan"</h2>
                <A href="/pos/baru" attr:style="background: #1a1a2e; color: white; padding: 8px 16px; text-decoration: none; border-radius: 4px">
                    "+ Tambah Pos"
                </A>
            </div>

            {move || {
                if loading.get() {
                    view! { <p>"Memuat..."</p> }.into_any()
                } else if !error.get().is_empty() {
                    view! { <p style="color: red">{error.get()}</p> }.into_any()
                } else if pos_list.get().is_empty() {
                    view! { <p>"Belum ada pos."</p> }.into_any()
                } else {
                    view! {
                        <table style="width: 100%; border-collapse: collapse; margin-top: 16px">
                            <thead>
                                <tr style="background: #f5f5f5; text-align: left">
                                    <th style="padding: 8px; border: 1px solid #ddd">"Nama"</th>
                                    <th style="padding: 8px; border: 1px solid #ddd">"Latitude"</th>
                                    <th style="padding: 8px; border: 1px solid #ddd">"Longitude"</th>
                                    <th style="padding: 8px; border: 1px solid #ddd">"Elevasi"</th>
                                    <th style="padding: 8px; border: 1px solid #ddd">"Tipe Alat"</th>
                                    <th style="padding: 8px; border: 1px solid #ddd">"Aksi"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {move || pos_list.get().into_iter().map(|pos| {
                                    let pos_id = pos.id;
                                    let delete = delete_pos.clone();
                                    view! {
                                        <tr>
                                            <td style="padding: 8px; border: 1px solid #ddd">{pos.nama.clone()}</td>
                                            <td style="padding: 8px; border: 1px solid #ddd">{pos.latitude.map(|v| v.to_string()).unwrap_or("-".to_string())}</td>
                                            <td style="padding: 8px; border: 1px solid #ddd">{pos.longitude.map(|v| v.to_string()).unwrap_or("-".to_string())}</td>
                                            <td style="padding: 8px; border: 1px solid #ddd">{pos.elevasi.map(|v| v.to_string()).unwrap_or("-".to_string())}</td>
                                            <td style="padding: 8px; border: 1px solid #ddd">{pos.tipe_alat.clone().unwrap_or("-".to_string())}</td>
                                            <td style="padding: 8px; border: 1px solid #ddd">
                                                <A href={format!("/pos/{}", pos_id)} attr:style="margin-right: 8px">"Edit"</A>
                                                <button on:click=move |_| delete(pos_id) style="color: red">"Hapus"</button>
                                            </td>
                                        </tr>
                                    }
                                }).collect::<Vec<_>>()}
                            </tbody>
                        </table>
                    }.into_any()
                }
            }}
        </div>
    }
}

#[component]
pub fn PosForm() -> impl IntoView {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let params = use_params_map();
    let is_edit = move || params.get().get("id").map(|s| s != "baru").unwrap_or(false);
    let pos_id = move || {
        params
            .get()
            .get("id")
            .and_then(|s| s.parse::<i64>().ok())
    };

    let (nama, set_nama) = signal(String::new());
    let (latitude, set_latitude) = signal(String::new());
    let (longitude, set_longitude) = signal(String::new());
    let (elevasi, set_elevasi) = signal(String::new());
    let (tipe_alat, set_tipe_alat) = signal(String::new());
    let (error, set_error) = signal(String::new());
    let (submitting, set_submitting) = signal(false);

    if is_edit() {
        let token = auth.token.get();
        let id = pos_id();
        spawn_local(async move {
            if let (Some(t), Some(i)) = (token, id) {
                if let Ok(pos) = api::get_pos(&t, i).await {
                    set_nama.set(pos.nama);
                    set_latitude.set(pos.latitude.map(|v| v.to_string()).unwrap_or_default());
                    set_longitude.set(pos.longitude.map(|v| v.to_string()).unwrap_or_default());
                    set_elevasi.set(pos.elevasi.map(|v| v.to_string()).unwrap_or_default());
                    set_tipe_alat.set(pos.tipe_alat.unwrap_or_default());
                }
            }
        });
    }

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_submitting.set(true);
        set_error.set(String::new());

        let token = auth.token.get();
        let req = shared_types::CreatePosRequest {
            nama: nama.get(),
            latitude: latitude.get().parse::<f64>().ok(),
            longitude: longitude.get().parse::<f64>().ok(),
            elevasi: elevasi.get().parse::<i32>().ok(),
            tipe_alat: {
                let v = tipe_alat.get();
                if v.is_empty() {
                    None
                } else {
                    Some(v)
                }
            },
        };

        let is_edit_val = is_edit();
        let pos_id_val = pos_id();

        spawn_local(async move {
            let result = match (is_edit_val, pos_id_val) {
                (true, Some(id)) => {
                    if let Some(t) = token {
                        api::update_pos(&t, id, req).await
                    } else {
                        Err("Token tidak ditemukan".to_string())
                    }
                }
                _ => {
                    if let Some(t) = token {
                        api::create_pos(&t, req).await
                    } else {
                        Err("Token tidak ditemukan".to_string())
                    }
                }
            };

            match result {
                Ok(_) => {
                    set_submitting.set(false);
                    let nav = leptos_router::hooks::use_navigate();
                    let _ = nav("/pos", NavigateOptions::default());
                }
                Err(e) => {
                    set_error.set(e);
                    set_submitting.set(false);
                }
            }
        });
    };

    let title = move || {
        if is_edit() {
            "Edit Pos"
        } else {
            "Tambah Pos"
        }
    };

    view! {
        <div style="max-width: 600px">
            <h2>{title}</h2>
            <form on:submit=on_submit style="display: flex; flex-direction: column; gap: 12px; margin-top: 16px">
                <div>
                    <label>"Nama Pos *"</label>
                    <input
                        type="text"
                        prop:value=nama
                        on:input=move |ev| set_nama.set(event_target_value(&ev))
                        required
                        style="width: 100%; padding: 8px"
                    />
                </div>
                <div>
                    <label>"Latitude"</label>
                    <input
                        type="text"
                        prop:value=latitude
                        on:input=move |ev| set_latitude.set(event_target_value(&ev))
                        placeholder="-7.25"
                        style="width: 100%; padding: 8px"
                    />
                </div>
                <div>
                    <label>"Longitude"</label>
                    <input
                        type="text"
                        prop:value=longitude
                        on:input=move |ev| set_longitude.set(event_target_value(&ev))
                        placeholder="112.75"
                        style="width: 100%; padding: 8px"
                    />
                </div>
                <div>
                    <label>"Elevasi (m)"</label>
                    <input
                        type="number"
                        prop:value=elevasi
                        on:input=move |ev| set_elevasi.set(event_target_value(&ev))
                        style="width: 100%; padding: 8px"
                    />
                </div>
                <div>
                    <label>"Tipe Alat"</label>
                    <input
                        type="text"
                        prop:value=tipe_alat
                        on:input=move |ev| set_tipe_alat.set(event_target_value(&ev))
                        placeholder="Ombrometer"
                        style="width: 100%; padding: 8px"
                    />
                </div>
                {move || {
                    let msg = error.get();
                    if !msg.is_empty() {
                        view! { <p style="color: red">{msg}</p> }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }}
                <div style="display: flex; gap: 8px">
                    <button type="submit" disabled=submitting style="padding: 8px 16px">
                        {move || if submitting.get() { "Menyimpan..." } else { "Simpan" }}
                    </button>
                    <A
                        href="/pos"
                        attr:style="padding: 8px 16px; text-decoration: none; border: 1px solid #ddd; border-radius: 4px; color: inherit"
                    >
                        "Batal"
                    </A>
                </div>
            </form>
        </div>
    }
}
