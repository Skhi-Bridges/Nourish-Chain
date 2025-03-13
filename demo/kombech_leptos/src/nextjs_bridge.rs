use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

// Type-safe Next.js router integration
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn next_router_push(url: &str);
    
    #[wasm_bindgen(js_namespace = window)]
    fn next_api_fetch(endpoint: &str, method: &str, body: &str) -> js_sys::Promise;
}

// Type-safe representation of Next.js router
#[derive(Clone)]
pub struct NextRouter;

impl NextRouter {
    pub fn new() -> Self {
        Self
    }
    
    // Type-safe navigation
    pub fn navigate(&self, route: NextRoute) -> Result<(), JsValue> {
        next_router_push(&route.to_string());
        Ok(())
    }
}

// Type-safe route enum
#[derive(Clone, Debug)]
pub enum NextRoute {
    Home,
    Dashboard,
    Staking,
    Dex,
    Social,
    Custom(String),
}

impl ToString for NextRoute {
    fn to_string(&self) -> String {
        match self {
            NextRoute::Home => "/".to_string(),
            NextRoute::Dashboard => "/dashboard".to_string(),
            NextRoute::Staking => "/staking".to_string(),
            NextRoute::Dex => "/dex".to_string(),
            NextRoute::Social => "/social".to_string(),
            NextRoute::Custom(path) => path.clone(),
        }
    }
}

// Type-safe Next.js API client
#[derive(Clone)]
pub struct NextApiClient;

impl NextApiClient {
    pub fn new() -> Self {
        Self
    }
    
    // Generic API call with type-safe request and response
    pub async fn call<T, R>(&self, endpoint: &str, method: &str, payload: &T) -> Result<R, JsValue>
    where
        T: Serialize,
        R: for<'de> Deserialize<'de>,
    {
        let body = serde_json::to_string(payload).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let promise = next_api_fetch(endpoint, method, &body);
        let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
        let json = js_sys::JSON::stringify(&result)?;
        let json_str = json.as_string().unwrap();
        let response: R = serde_json::from_str(&json_str).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(response)
    }
    
    // Type-safe GET request
    pub async fn get<R>(&self, endpoint: &str) -> Result<R, JsValue>
    where
        R: for<'de> Deserialize<'de>,
    {
        self.call::<(), R>(endpoint, "GET", &()).await
    }
    
    // Type-safe POST request
    pub async fn post<T, R>(&self, endpoint: &str, payload: &T) -> Result<R, JsValue>
    where
        T: Serialize,
        R: for<'de> Deserialize<'de>,
    {
        self.call::<T, R>(endpoint, "POST", payload).await
    }
}

// Type-safe Head component wrapper
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn set_page_title(title: &str);
    
    #[wasm_bindgen(js_namespace = window)]
    fn set_page_description(description: &str);
}

#[derive(Clone)]
pub struct NextHead {
    title: String,
    description: String,
}

impl NextHead {
    pub fn new(title: &str, description: &str) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
        }
    }
    
    pub fn apply(&self) -> Result<(), JsValue> {
        set_page_title(&self.title);
        set_page_description(&self.description);
        Ok(())
    }
}

// Type-safe Next.js runtime context
#[derive(Clone)]
pub struct NextJsRuntime {
    pub router: NextRouter,
    pub api: NextApiClient,
}

impl NextJsRuntime {
    pub fn new() -> Self {
        Self {
            router: NextRouter::new(),
            api: NextApiClient::new(),
        }
    }
    
    pub fn set_head(&self, title: &str, description: &str) -> Result<(), JsValue> {
        let head = NextHead::new(title, description);
        head.apply()
    }
}
