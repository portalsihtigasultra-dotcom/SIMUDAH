use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

use crate::api;
use crate::auth::AuthContext;

#[component]
pub fn InputDataHujan() -> impl IntoView {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let (pos_list, _set_pos_list) = signal(Vec::new());
    let (pos_id, set_pos_id) = signal(String::new());
    let (tanggal, set_tanggal) = signal(String::new());
    let (nilai_mm, set_nilai_mm) = signal(String::new());
    let (jam_pengamatan, set_jam_pengamatan) = signal(String::new());
    let (error, set_error) = signal(String::new());
    let (success, set_success) = signal(String::new());
    let (submitting, set_submitting) = signal(false);

    {
        let token = auth.token.get();
        spawn_local(async move {
            if let Some(t) = token {
                if let Ok(list) = api::list_pos(&t).await {
                    _set_pos_list.set(list);
                }
            }
        });
    }

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_submitting.set(true);
        set_error.set(String::new());
        set_success.set(String::new());

        let token = auth.token.get();
        let p_id = pos_id.get().parse::<i64>();
        let tgl = tanggal.get();
        let nilai = nilai_mm.get().parse::<f64>();
        let jam = {
            let v = jam_pengamatan.get();
            if v.is_empty() { None } else { Some(v) }
        };

        if p_id.is_err() || nilai.is_err() {
            set_error.set("Pos dan nilai harus diisi dengan benar.".to_string());
            set_submitting.set(false);
            return;
        }

        let req = shared_types::CreateDataCurahHujanRequest {
            pos_id: p_id.unwrap(),
            tanggal: tgl,
            nilai_mm: nilai.unwrap(),
            jam_pengamatan: jam,
        };

        spawn_local(async move {
            if let Some(t) = token {
                match api::create_data_hujan(&t, req).await {
                    Ok(_) => {
                        set_success.set("Data berhasil disimpan.".to_string());
                        set_tanggal.set(String::new());
                        set_nilai_mm.set(String::new());
                        set_jam_pengamatan.set(String::new());
                        set_submitting.set(false);
                    }
                    Err(e) => {
                        set_error.set(e);
                        set_submitting.set(false);
                    }
                }
            }
        });
    };

    let pos_options = move || {
        let list = pos_list.get();
        let mut opts = vec![view! { <option value="">"-- Pilih Pos --"</option> }.into_any()];
        for pos in list {
            let id = pos.id.to_string();
            let nama = pos.nama.clone();
            opts.push(
                view! { <option value=id>{nama}</option> }.into_any(),
            );
        }
        opts
    };

    view! {
        <div style="max-width: 600px">
            <h2>"Input Data Curah Hujan"</h2>

            {move || {
                let msg = success.get();
                if !msg.is_empty() {
                    view! { <p style="color: green; background: #d4edda; padding: 8px; border-radius: 4px">{msg}</p> }.into_any()
                } else {
                    view! {}.into_any()
                }
            }}

            <form on:submit=on_submit style="display: flex; flex-direction: column; gap: 12px; margin-top: 16px">
                <div>
                    <label>"Pos *"</label>
                    <select
                        prop:value=pos_id
                        on:change=move |ev| set_pos_id.set(event_target_value(&ev))
                        required
                        style="width: 100%; padding: 8px"
                    >
                        {pos_options}
                    </select>
                </div>
                <div>
                    <label>"Tanggal *"</label>
                    <input
                        type="date"
                        prop:value=tanggal
                        on:input=move |ev| set_tanggal.set(event_target_value(&ev))
                        required
                        style="width: 100%; padding: 8px"
                    />
                </div>
                <div>
                    <label>"Nilai (mm) *"</label>
                    <input
                        type="number"
                        step="0.1"
                        prop:value=nilai_mm
                        on:input=move |ev| set_nilai_mm.set(event_target_value(&ev))
                        required
                        style="width: 100%; padding: 8px"
                    />
                </div>
                <div>
                    <label>"Jam Pengamatan"</label>
                    <input
                        type="time"
                        prop:value=jam_pengamatan
                        on:input=move |ev| set_jam_pengamatan.set(event_target_value(&ev))
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
                <button type="submit" disabled=submitting style="padding: 8px 16px; width: 200px">
                    {move || if submitting.get() { "Menyimpan..." } else { "Simpan" }}
                </button>
            </form>
        </div>
    }
}

#[component]
pub fn ListDataHujan() -> impl IntoView {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let (data_list, set_data_list) = signal(Vec::new());
    let (pos_map, set_pos_map) = signal(std::collections::HashMap::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(String::new());

    let (filter_pos, set_filter_pos) = signal(String::new());
    let (filter_status, set_filter_status) = signal(String::new());

    {
        let token = auth.token.get();
        set_loading.set(true);
        spawn_local(async move {
            if let Some(t) = token {
                let data_res = api::list_data_hujan(&t).await;
                match data_res {
                    Ok(list) => set_data_list.set(list),
                    Err(e) => set_error.set(e),
                }
                if let Ok(pos_list) = api::list_pos(&t).await {
                    let map: std::collections::HashMap<i64, String> =
                        pos_list.into_iter().map(|p| (p.id, p.nama)).collect();
                    set_pos_map.set(map);
                }
                set_loading.set(false);
            }
        });
    }

    let delete = move |id: i64| {
        let token = auth.token.get();
        spawn_local(async move {
            if let Some(t) = token {
                let _ = api::delete_data_hujan(&t, id).await;
                if let Ok(list) = api::list_data_hujan(&t).await {
                    set_data_list.set(list);
                }
            }
        });
    };

    let filtered = move || {
        let all = data_list.get();
        let fp = filter_pos.get();
        let fs = filter_status.get();
        all.into_iter()
            .filter(|d| fp.is_empty() || d.pos_id.to_string() == fp)
            .filter(|d| fs.is_empty() || d.status_mutu == fs)
            .collect::<Vec<_>>()
    };

    view! {
        <div>
            <h2>"Data Curah Hujan"</h2>

            <div style="display: flex; gap: 12px; margin: 16px 0; padding: 12px; background: #f5f5f5; border-radius: 4px">
                <div>
                    <label style="display: block; font-size: 0.9em">"Filter Pos"</label>
                    <select prop:value=filter_pos on:change=move |ev| set_filter_pos.set(event_target_value(&ev))>
                        <option value="">"Semua"</option>
                        {move || pos_map.get().into_iter().map(|(id, nama)| {
                            let v = id.to_string();
                            view! { <option value=v>{nama.clone()}</option> }
                        }).collect::<Vec<_>>()}
                    </select>
                </div>
                <div>
                    <label style="display: block; font-size: 0.9em">"Filter Status"</label>
                    <select prop:value=filter_status on:change=move |ev| set_filter_status.set(event_target_value(&ev))>
                        <option value="">"Semua"</option>
                        <option value="mentah">"Mentah"</option>
                        <option value="terverifikasi">"Terverifikasi"</option>
                        <option value="tervalidasi">"Tervalidasi"</option>
                        <option value="terkoreksi">"Terkoreksi"</option>
                        <option value="ditolak">"Ditolak"</option>
                    </select>
                </div>
                <div style="display: flex; align-items: flex-end">
                    <A href="/data-hujan/input" attr:style="background: #1a1a2e; color: white; padding: 8px 16px; text-decoration: none; border-radius: 4px">
                        "+ Input Data"
                    </A>
                </div>
            </div>

            {move || {
                if loading.get() {
                    view! { <p>"Memuat..."</p> }.into_any()
                } else if !error.get().is_empty() {
                    view! { <p style="color: red">{error.get()}</p> }.into_any()
                } else {
                    let items = filtered();
                    if items.is_empty() {
                        view! { <p>"Belum ada data."</p> }.into_any()
                    } else {
                        let pm = pos_map.get();
                        view! {
                            <table style="width: 100%; border-collapse: collapse">
                                <thead>
                                    <tr style="background: #f5f5f5; text-align: left">
                                        <th style="padding: 8px; border: 1px solid #ddd">"Tanggal"</th>
                                        <th style="padding: 8px; border: 1px solid #ddd">"Pos"</th>
                                        <th style="padding: 8px; border: 1px solid #ddd">"Nilai (mm)"</th>
                                        <th style="padding: 8px; border: 1px solid #ddd">"Jam"</th>
                                        <th style="padding: 8px; border: 1px solid #ddd">"Status"</th>
                                        <th style="padding: 8px; border: 1px solid #ddd">"Aksi"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {items.into_iter().map(|d| {
                                        let d_id = d.id;
                                        let del = delete.clone();
                                        let nama_pos = pm.get(&d.pos_id).cloned().unwrap_or(d.pos_id.to_string());
                                        let status_mutu = d.status_mutu.clone();
                                        let status_class = match status_mutu.as_str() {
                                            "mentah" => "color: #856404; background: #fff3cd",
                                            "terverifikasi" => "color: #004085; background: #cce5ff",
                                            "tervalidasi" => "color: #155724; background: #d4edda",
                                            "terkoreksi" => "color: #721c24; background: #f8d7da",
                                            "ditolak" => "color: #383d41; background: #e2e3e5",
                                            _ => "",
                                        };
                                        let is_mentah = status_mutu == "mentah";
                                        view! {
                                            <tr>
                                                <td style="padding: 8px; border: 1px solid #ddd">{d.tanggal.clone()}</td>
                                                <td style="padding: 8px; border: 1px solid #ddd">{nama_pos}</td>
                                                <td style="padding: 8px; border: 1px solid #ddd">{d.nilai_mm.to_string()}</td>
                                                <td style="padding: 8px; border: 1px solid #ddd">{d.jam_pengamatan.clone().unwrap_or("-".to_string())}</td>
                                                <td style="padding: 8px; border: 1px solid #ddd">
                                                    <span style=status_class>{status_mutu}</span>
                                                </td>
                                                <td style="padding: 8px; border: 1px solid #ddd">
                                                    {move || {
                                                        if is_mentah {
                                                            view! {
                                                                <A href={format!("/data-hujan/{}/edit", d_id)}
                                                                    attr:style="margin-right: 8px; color: #1a1a2e"
                                                                >
                                                                    "Edit"
                                                                </A>
                                                                <button on:click=move |_| del(d_id) style="color: red">"Hapus"</button>
                                                            }.into_any()
                                                        } else {
                                                            view! { <span style="color: #999">"-"</span> }.into_any()
                                                        }
                                                    }}
                                                </td>
                                            </tr>
                                        }
                                    }).collect::<Vec<_>>()}
                                </tbody>
                            </table>
                        }.into_any()
                    }
                }
            }}
        </div>
    }
}

#[component]
pub fn EditDataHujan() -> impl IntoView {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let params = use_params_map();
    let id = move || {
        params
            .get()
            .get("id")
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(0)
    };

    let (pos_list, set_pos_list) = signal(Vec::new());
    let (pos_id, set_pos_id) = signal(String::new());
    let (tanggal, set_tanggal) = signal(String::new());
    let (nilai_mm, set_nilai_mm) = signal(String::new());
    let (jam_pengamatan, set_jam_pengamatan) = signal(String::new());
    let (error, set_error) = signal(String::new());
    let (success, set_success) = signal(String::new());
    let (submitting, set_submitting) = signal(false);
    let (loading, set_loading) = signal(true);

    {
        let token = auth.token.get();
        let data_id = id();
        set_loading.set(true);
        spawn_local(async move {
            if let Some(t) = token {
                let _ = api::list_pos(&t).await.map(|list| set_pos_list.set(list));
                if let Ok(data) = api::get_data_hujan(&t, data_id).await {
                    set_pos_id.set(data.pos_id.to_string());
                    set_tanggal.set(data.tanggal.clone());
                    set_nilai_mm.set(data.nilai_mm.to_string());
                    set_jam_pengamatan.set(data.jam_pengamatan.clone().unwrap_or_default());
                }
                set_loading.set(false);
            }
        });
    }

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_submitting.set(true);
        set_error.set(String::new());
        set_success.set(String::new());

        let token = auth.token.get();
        let data_id = id();
        let p_id = pos_id.get().parse::<i64>();
        let tgl = tanggal.get();
        let nilai = nilai_mm.get().parse::<f64>();
        let jam = {
            let v = jam_pengamatan.get();
            if v.is_empty() { None } else { Some(v) }
        };

        if p_id.is_err() || nilai.is_err() {
            set_error.set("Pos dan nilai harus diisi dengan benar.".to_string());
            set_submitting.set(false);
            return;
        }

        let req = shared_types::CreateDataCurahHujanRequest {
            pos_id: p_id.unwrap(),
            tanggal: tgl,
            nilai_mm: nilai.unwrap(),
            jam_pengamatan: jam,
        };

        spawn_local(async move {
            if let Some(t) = token {
                match api::update_data_hujan(&t, data_id, req).await {
                    Ok(_) => {
                        set_success.set("Data berhasil diperbarui.".to_string());
                        set_submitting.set(false);
                    }
                    Err(e) => {
                        set_error.set(e);
                        set_submitting.set(false);
                    }
                }
            }
        });
    };

    let pos_options = move || {
        let list = pos_list.get();
        let mut opts = vec![view! { <option value="">"-- Pilih Pos --"</option> }.into_any()];
        for pos in list {
            let id = pos.id.to_string();
            let nama = pos.nama.clone();
            opts.push(
                view! { <option value=id>{nama}</option> }.into_any(),
            );
        }
        opts
    };

    view! {
        <div style="max-width: 600px">
            <h2>"Edit Data Curah Hujan"</h2>

            {move || {
                if loading.get() {
                    return view! { <p>"Memuat data..."</p> }.into_any();
                }
                let msg = success.get();
                if !msg.is_empty() {
                    return view! {
                        <div>
                            <p style="color: green; background: #d4edda; padding: 8px; border-radius: 4px">{msg}</p>
                            <A href="/data-hujan" attr:style="display: inline-block; margin-top: 12px; color: #1a1a2e">
                                "Kembali ke daftar"
                            </A>
                        </div>
                    }.into_any();
                }
                view! {}.into_any()
            }}

            <form on:submit=on_submit style="display: flex; flex-direction: column; gap: 12px; margin-top: 16px">
                <div>
                    <label>"Pos *"</label>
                    <select
                        prop:value=pos_id
                        on:change=move |ev| set_pos_id.set(event_target_value(&ev))
                        required
                        style="width: 100%; padding: 8px"
                    >
                        {pos_options}
                    </select>
                </div>
                <div>
                    <label>"Tanggal *"</label>
                    <input
                        type="date"
                        prop:value=tanggal
                        on:input=move |ev| set_tanggal.set(event_target_value(&ev))
                        required
                        style="width: 100%; padding: 8px"
                    />
                </div>
                <div>
                    <label>"Nilai (mm) *"</label>
                    <input
                        type="number"
                        step="0.1"
                        prop:value=nilai_mm
                        on:input=move |ev| set_nilai_mm.set(event_target_value(&ev))
                        required
                        style="width: 100%; padding: 8px"
                    />
                </div>
                <div>
                    <label>"Jam Pengamatan"</label>
                    <input
                        type="time"
                        prop:value=jam_pengamatan
                        on:input=move |ev| set_jam_pengamatan.set(event_target_value(&ev))
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
                <button type="submit" disabled=submitting style="padding: 8px 16px; width: 200px">
                    {move || if submitting.get() { "Menyimpan..." } else { "Simpan Perubahan" }}
                </button>
            </form>
        </div>
    }
}
