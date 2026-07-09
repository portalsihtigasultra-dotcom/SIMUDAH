use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;
use web_sys::window;

use crate::auth::{login_request, AuthContext};

#[component]
pub fn LoginPage() -> impl IntoView {
    let (username, set_username) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (error, set_error) = signal(String::new());
    let (loading, set_loading) = signal(false);
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let navigate = use_navigate();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_loading.set(true);
        set_error.set(String::new());

        let u = username.get();
        let p = password.get();
        let nav = navigate.clone();

        spawn_local(async move {
            match login_request(u, p).await {
                Ok(resp) => {
                    if let Some(storage) = window()
                        .and_then(|w| w.local_storage().ok())
                        .flatten()
                    {
                        let _ = storage.set_item("token", &resp.token);
                    }
                    auth.token.set(Some(resp.token));
                    auth.user.set(Some(resp.user));
                    set_loading.set(false);
                    let _ = nav("/", NavigateOptions::default());
                }
                Err(e) => {
                    set_error.set(e);
                    set_loading.set(false);
                }
            }
        });
    };

    view! {
        <div style="max-width: 400px; margin: 100px auto; text-align: center">
            <h2>"SIMUDAH - Login"</h2>
            <form on:submit=on_submit style="display: flex; flex-direction: column; gap: 12px; margin-top: 24px">
                <input
                    type="text"
                    prop:value=username
                    on:input=move |ev| set_username.set(event_target_value(&ev))
                    placeholder="Username"
                    required
                />
                <input
                    type="password"
                    prop:value=password
                    on:input=move |ev| set_password.set(event_target_value(&ev))
                    placeholder="Password"
                    required
                />
                <button type="submit" disabled=loading>
                    {move || if loading.get() { "Memuat..." } else { "Login" }}
                </button>
            </form>
            {move || {
                let msg = error.get();
                if !msg.is_empty() {
                    view! { <p style="color: red">{msg}</p> }.into_any()
                } else {
                    view! {}.into_any()
                }
            }}
        </div>
    }
}
