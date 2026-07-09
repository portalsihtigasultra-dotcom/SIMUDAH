pub mod api;
pub mod auth;
pub mod components;
pub mod pages;

use auth::AuthContext;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use components::sidebar::Sidebar;

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
                <Route path=path!("/curah-hujan/verifikasi") view={
                    let auth = auth.clone();
                    move || guarded_view(&auth, view! { <pages::curah_hujan::CurahHujanVerifikasi/> })
                }/>
                <Route path=path!("/curah-hujan/validasi") view={
                    let auth = auth.clone();
                    move || guarded_view(&auth, view! { <pages::curah_hujan::CurahHujanValidasi/> })
                }/>
                <Route path=path!("/curah-hujan/koreksi") view={
                    let auth = auth.clone();
                    move || guarded_view(&auth, view! { <pages::curah_hujan::CurahHujanKoreksi/> })
                }/>
                <Route path=path!("/data") view={
                    let auth = auth.clone();
                    move || guarded_view(&auth, view! { <pages::curah_hujan::SemuaData/> })
                }/>
            </Routes>
        </Router>
    }
}
