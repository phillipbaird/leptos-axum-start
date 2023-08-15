use cfg_if::cfg_if;
pub mod app;
pub mod error_template;
pub mod fileserv;
pub mod server;

cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;
        use app::*;
        use leptos::*;

        #[wasm_bindgen]
        pub fn hydrate() {
            console_error_panic_hook::set_once();
            _ = console_log::init_with_level(log::Level::Debug);

            leptos::mount_to_body(|| {
                view! {   <App/> }
            });
        }
    }
}
