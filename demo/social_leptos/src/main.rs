use leptos::*;
use social_leptos::{SocialPage, mount_social_page, render_social_page};
use wasm_bindgen::prelude::*;

fn main() {
    // For local development without Next.js
    mount_to_body(|cx| view! { cx, <SocialPage /> });
}

// Export all public functions to JavaScript
#[wasm_bindgen(start)]
pub fn start() {
    // Set panic hook for better error messages
    console_error_panic_hook::set_once();
    
    // Log that the WASM module has loaded
    web_sys::console::log_1(&"Social page WASM module loaded!".into());
    
    // Here you could automatically hydrate if desired
    // social_leptos::js_types::register_hydration_listener().expect("Failed to register hydration listener");
}
