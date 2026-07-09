pub mod api;
pub mod auth;
pub mod components;
pub mod pages;

use auth::AuthContext;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use wasm_bindgen::prelude::*;

use components::sidebar::Sidebar;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn ProtectedLayout(children: Children) -> impl IntoView {
    view! {
        <div style="display: flex">
            <Sidebar/>
            <main style="flex: 1; padding: 24px">
                {children()}
            </main>
        </div>
    }
}

#[component]
fn RedirectLogin() -> impl IntoView {
    view! {
        <p>"Silakan login terlebih dahulu."</p>
        <a href="/login">"Login"</a>
    }
}

fn guarded_view(auth: &AuthContext, content: impl IntoView + 'static) -> impl IntoView {
    if auth.token.get().is_some() {
        view! { <ProtectedLayout>{content}</ProtectedLayout> }.into_any()
    } else {
        view! { <RedirectLogin/> }.into_any()
    }
}

#[component]
pub fn App() -> impl IntoView {
    let auth = AuthContext::new();

    view! {
        <Router>
            <Routes fallback=|| "Halaman tidak ditemukan">
                <Route path=path!("/login") view=move || {
                    view! { <pages::login::LoginPage/> }.into_any()
                }/>
                <Route path=path!("/") view={
                    let auth = auth.clone();
                    move || guarded_view(&auth, view! { <pages::dashboard::Dashboard/> })
                }/>
                <Route path=path!("/curah-hujan") view={
                    let auth = auth.clone();
                    move || guarded_view(&auth, view! { <pages::curah_hujan::CurahHujanOverview/> })
                }/>
                <Route path=path!("/pos") view={
                    let auth = auth.clone();
                    move || guarded_view(&auth, view! { <pages::pos::ListPos/> })
                }/>
                <Route path=path!("/pos/baru") view={
                    let auth = auth.clone();
                    move || guarded_view(&auth, view! { <pages::pos::PosForm/> })
                }/>
                <Route path=path!("/pos/:id") view={
                    let auth = auth.clone();
                    move || guarded_view(&auth, view! { <pages::pos::PosForm/> })
                }/>
                <Route path=path!("/curah-hujan/input") view={
                    let auth = auth.clone();
                    move || guarded_view(&auth, view! { <pages::curah_hujan::CurahHujanInput/> })
                }/>

                <Route path=path!("/data-hujan") view={
                    let auth = auth.clone();
                    move || guarded_view(&auth, view! { <pages::data_hujan::ListDataHujan/> })
                }/>
                <Route path=path!("/data-hujan/input") view={
                    let auth = auth.clone();
                    move || guarded_view(&auth, view! { <pages::data_hujan::InputDataHujan/> })
                }/>
                <Route path=path!("/data-hujan/verifikasi") view={
                    let auth = auth.clone();
                    move || guarded_view(&auth, view! { <pages::data_hujan::VerifikasiData/> })
                }/>
                <Route path=path!("/data-hujan/validasi") view={
                    let auth = auth.clone();
                    move || guarded_view(&auth, view! { <pages::data_hujan::ValidasiData/> })
                }/>
                <Route path=path!("/data-hujan/:id/edit") view={
                    let auth = auth.clone();
                    move || guarded_view(&auth, view! { <pages::data_hujan::EditDataHujan/> })
                }/>
                <Route path=path!("/data-hujan/:id/log") view={
                    let auth = auth.clone();
                    move || guarded_view(&auth, view! { <pages::data_hujan::LogMutuData/> })
                }/>
            </Routes>
        </Router>
    }
}
