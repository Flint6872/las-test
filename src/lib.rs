pub mod app;
pub mod fallback;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod server_fns;
pub mod user;

pub mod server;
pub mod routes;
pub mod components;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}


