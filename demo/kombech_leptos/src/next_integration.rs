use wasm_bindgen::prelude::*;
use crate::js_types::*;

// Type-safe Next.js integration
#[wasm_bindgen]
pub struct NextIntegration {
    props: ReactProps,
}

#[wasm_bindgen]
impl NextIntegration {
    #[wasm_bindgen(constructor)]
    pub fn new(js_props: NextJsProps) -> Self {
        let props_js_value = props_to_rust(js_props);
        let props: ReactProps = serde_wasm_bindgen::from_value(props_js_value).unwrap_or(ReactProps {
            initial_dark_mode: false,
            language: "en".to_string(),
        });
        
        Self { props }
    }
    
    // Get type-safe props for Rust component
    pub fn get_props(&self) -> ReactProps {
        self.props.clone()
    }
    
    // Render the social page with type-safe props
    pub fn render(&self) -> String {
        // In a real implementation, this would use the props to render the component
        // For now, we'll return a placeholder
        format!(
            "<div data-hydrate=\"true\" data-dark-mode=\"{}\" data-language=\"{}\"></div>",
            self.props.initial_dark_mode,
            self.props.language
        )
    }
}

// Generate TypeScript definition files
#[wasm_bindgen(typescript_custom_section)]
const TS_INTERFACE: &'static str = r#"
// Generated TypeScript interfaces
export interface NextIntegrationOptions {
    initialDarkMode?: boolean;
    language?: string;
}

export class NextIntegration {
    constructor(options: NextIntegrationOptions);
    render(): string;
}
"#;

// External JavaScript bindings in a type-safe way
#[wasm_bindgen]
extern "C" {
    // NextRouter type with TypeScript type definition
    #[wasm_bindgen(typescript_type = "import('next/router').NextRouter")]
    pub type NextRouter;
    
    // Methods with proper TypeScript types
    #[wasm_bindgen(method)]
    pub fn push(this: &NextRouter, url: &str) -> js_sys::Promise;
    
    // Global Next.js objects
    #[wasm_bindgen(js_namespace = window)]
    pub fn get_next_router() -> NextRouter;
}

// Type-safe navigation using Next.js Router
#[wasm_bindgen]
pub fn navigate_to(path: &str) -> Result<(), JsValue> {
    let router = get_next_router();
    
    // Create a future from the Promise
    let promise = router.push(path);
    
    // Set up a future to handle the result
    wasm_bindgen_futures::spawn_local(async move {
        match wasm_bindgen_futures::JsFuture::from(promise).await {
            Ok(_) => (),
            Err(e) => web_sys::console::error_1(&e),
        }
    });
    
    Ok(())
}
