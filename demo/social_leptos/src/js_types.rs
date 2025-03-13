use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

// Type-safe representations of JavaScript objects for Next.js integration

// Type-safe React props
#[derive(Serialize, Deserialize)]
pub struct ReactProps {
    pub initial_dark_mode: bool,
    pub language: String,
}

#[wasm_bindgen]
extern "C" {
    // TypeScript interfaces for Next.js
    #[wasm_bindgen(typescript_type = "{ initialDarkMode: boolean, language: string }")]
    pub type NextJsProps;
    
    // Get properties in a type-safe way
    #[wasm_bindgen(method, getter)]
    pub fn initial_dark_mode(this: &NextJsProps) -> bool;
    
    #[wasm_bindgen(method, getter)]
    pub fn language(this: &NextJsProps) -> String;
}

// Convert between Rust and JavaScript types
#[wasm_bindgen]
pub fn props_to_rust(props: NextJsProps) -> JsValue {
    let rust_props = ReactProps {
        initial_dark_mode: props.initial_dark_mode(),
        language: props.language(),
    };
    
    serde_wasm_bindgen::to_value(&rust_props).unwrap()
}

// Type-safe event handling for React events
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "React.MouseEvent")]
    pub type ReactMouseEvent;
    
    #[wasm_bindgen(method, getter)]
    pub fn target(this: &ReactMouseEvent) -> JsValue;
    
    #[wasm_bindgen(method)]
    pub fn preventDefault(this: &ReactMouseEvent);
}

// Safe callback type
#[wasm_bindgen]
pub fn create_event_handler(f: &Closure<dyn FnMut(ReactMouseEvent)>) -> JsValue {
    f.as_ref().clone()
}

// TypeScript declaration generators for Next.js integration
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
// Type definitions for Rust WASM components
export interface SocialPageProps {
    initialDarkMode?: boolean;
    language?: string;
}

export function renderSocialPage(): string;
export function mountSocialPage(selector: string): void;
"#;

// Type-safe event listener for Next.js hydration
#[wasm_bindgen]
pub fn register_hydration_listener() -> Result<(), JsValue> {
    let window = web_sys::window().expect("should have a window");
    let document = window.document().expect("should have a document");
    
    let closure = Closure::wrap(Box::new(move || {
        // Call mount function when document is ready
        let _ = mount_social_page("#social-root");
    }) as Box<dyn FnMut()>);
    
    let event_name = if document.ready_state() == "loading" {
        "DOMContentLoaded"
    } else {
        // Already loaded, call immediately
        closure.call_once(());
        return Ok(());
    };
    
    window.add_event_listener_with_callback(
        event_name,
        closure.as_ref().unchecked_ref(),
    )?;
    
    closure.forget();
    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Mount function exposed to JavaScript
#[wasm_bindgen]
pub fn mount_social_page(selector: &str) -> Result<(), JsValue> {
    log(&format!("Mounting social page to {}", selector));
    
    // In a real implementation, this would render the Leptos component
    // into the specified selector using hydration
    
    Ok(())
}
