use leptos::*;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

mod additional_components;
mod nextjs_bridge;
mod type_safe_components;
mod js_types;
mod next_integration;

use additional_components::*;
use nextjs_bridge::*;
use type_safe_components::*;
use js_types::*;
use next_integration::*;

#[derive(Default, Clone, Serialize, Deserialize)]
struct CommunityStats {
    members: u32,
    validators: u32,
    staked_nrsh: f64,
    nodes: u32,
}

async fn fetch_community_stats() -> CommunityStats {
    // For a real implementation, we would use our type-safe NextApiClient:
    // let api_client = NextApiClient::new();
    // let stats: CommunityStats = api_client.get("/api/community-stats").await.unwrap_or_default();
    
    // For now, return mock data
    CommunityStats {
        members: 2450,
        validators: 142,
        staked_nrsh: 12_450_000.0,
        nodes: 89,
    }
}

// Main component with type-safe props
#[component]
pub fn SocialPage(
    cx: Scope,
    #[prop(optional, default = false)] initial_dark_mode: bool,
    #[prop(optional, default = "en".to_string())] language: String,
) -> impl IntoView {
    // Set up Next.js integration in a type-safe way
    let next_runtime = NextJsRuntime::new();
    let _ = next_runtime.set_head(
        "Nourish Chain - Community Hub",
        "Join the Nourish Chain community and stay connected with updates, events, and fellow members"
    );
    
    // State management with initial value from props
    let (dark_mode, set_dark_mode) = create_signal(initial_dark_mode);
    let community_stats = create_rw_signal(CommunityStats::default());
    
    // Mock data - replace with real API calls
    let _ = create_resource(
        || (),
        |_| async move {
            let stats = fetch_community_stats().await;
            community_stats.set(stats);
        },
    );

    view! { cx,
        // Type-safe navigation
        <TypeSafeNavigation/>
        
        // Main container with type-safe classes
        <main 
            class={TailwindClass::new()
                .add("min-h-screen p-8 transition-colors duration-300")
                .add_when("bg-gray-900 text-white", dark_mode())
                .add_when("bg-white text-gray-900", !dark_mode())
                .to_string()
            }
            data-language={language}
        >
            // Dark mode toggle
            <button
                class={TailwindClass::new()
                    .add("absolute top-4 right-4 p-2 rounded-full hover:bg-opacity-10")
                    .add_when("bg-gray-800 text-white", dark_mode())
                    .add_when("bg-gray-200 text-black", !dark_mode())
                    .to_string()
                }
                on:click=move |_| set_dark_mode.update(|d| *d = !*d)
            >
                {move || if dark_mode() { "🌞" } else { "🌙" }}
            </button>
            
            <div class="max-w-6xl mx-auto">
                // Community Dashboard Section
                <CommunityDashboard stats=community_stats/>

                // Social Integration Grid
                <div class={ResponsiveClass::new("grid gap-6 mt-12")
                        .with(Breakpoint::MD, "grid-cols-2")
                        .with(Breakpoint::LG, "grid-cols-4")
                        .to_string()
                    }
                >
                    <TwitterFeed/>
                    <DiscordWidget/>
                    <TelegramWidget/>
                    <YouTubeFeed/>
                </div>

                // Content Showcase
                <ContentShowcase/>

                // Interactive Elements
                <InteractiveSection/>
            </div>
        </main>
        
        // Type-safe footer
        <TypeSafeFooter/>
    }
}

#[component]
fn CommunityDashboard(cx: Scope, stats: RwSignal<CommunityStats>) -> impl IntoView {
    // Using our type-safe responsive class helper
    let grid_class = ResponsiveClass::new("grid grid-cols-2 gap-4")
        .with(Breakpoint::MD, "grid-cols-4")
        .to_string();
    
    view! { cx,
        <div class="bg-primary-500 p-6 rounded-xl shadow-lg">
            <h2 class="text-2xl font-bold mb-4">Community Dashboard</h2>
            <div class={grid_class}>
                <StatCard title="Members" value=move || stats().members.to_string()/>
                <StatCard title="Active Validators" value=move || stats().validators.to_string()/>
                <StatCard title="Staked NRSH" value=move || format!("{:.2}M", stats().staked_nrsh / 1_000_000.0)/>
                <StatCard title="Total Nodes" value=move || stats().nodes.to_string()/>
            </div>
        </div>
    }
}

#[component]
fn StatCard(cx: Scope, title: &'static str, value: impl Fn() -> String + 'static) -> impl IntoView {
    // Using our type-safe class helper
    let card_class = TailwindClass::new()
        .add("bg-white")
        .add("bg-opacity-10")
        .add("p-4")
        .add("rounded-lg")
        .to_string();
    
    view! { cx,
        <div class={card_class}>
            <h3 class="text-sm font-semibold text-opacity-80">{title}</h3>
            <p class="text-2xl font-bold mt-2">{value}</p>
        </div>
    }
}

// Export for JavaScript - While we're using a type-safe Rust approach,
// this allows seamless integration with Next.js
#[wasm_bindgen]
pub fn render_social_page(props_js: Option<JsValue>) -> String {
    // Parse props in a type-safe way
    let props: ReactProps = match props_js {
        Some(js_props) => serde_wasm_bindgen::from_value(js_props).unwrap_or_default(),
        None => ReactProps {
            initial_dark_mode: false,
            language: "en".to_string(),
        },
    };
    
    let mut output = String::new();
    
    leptos::ssr::render_to_string_fragment(|cx| {
        view! { cx, 
            <SocialPage 
                initial_dark_mode={props.initial_dark_mode}
                language={props.language.clone()}
            /> 
        }
    })
    .map(|html| {
        output = html;
    })
    .expect("Failed to render social page");
    
    output
}

// Type-safe mount function for direct Rust-to-DOM rendering
#[wasm_bindgen]
pub fn mount_social_page(selector: &str, props_js: Option<JsValue>) -> Result<(), JsValue> {
    // Register panic hook for better error messages
    console_error_panic_hook::set_once();
    
    // Parse props in a type-safe way
    let props: ReactProps = match props_js {
        Some(js_props) => serde_wasm_bindgen::from_value(js_props).unwrap_or_default(),
        None => ReactProps {
            initial_dark_mode: false,
            language: "en".to_string(),
        },
    };
    
    // Mount component with proper props
    mount_to(selector, move |cx| {
        view! { cx,
            <SocialPage 
                initial_dark_mode={props.initial_dark_mode}
                language={props.language.clone()}
            />
        }
    })?;
    
    Ok(())
}

// Utility function to mount component to specified selector
fn mount_to<F, IV>(selector: &str, view: F) -> Result<(), JsValue>
where
    F: FnOnce(Scope) -> IV + 'static,
    IV: IntoView,
{
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window found"))?;
    let document = window.document().ok_or_else(|| JsValue::from_str("No document found"))?;
    
    let element = document
        .query_selector(selector)?
        .ok_or_else(|| JsValue::from_str(&format!("Element with selector '{}' not found", selector)))?;
    
    leptos::mount_to(element, view);
    Ok(())
}
