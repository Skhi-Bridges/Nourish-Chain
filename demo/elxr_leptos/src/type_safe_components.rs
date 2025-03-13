use leptos::*;
use crate::nextjs_bridge::{NextJsRuntime, NextRoute};

// Type-safe component styles
pub struct TailwindClass {
    classes: Vec<String>,
}

impl TailwindClass {
    pub fn new() -> Self {
        Self { classes: vec![] }
    }
    
    pub fn add(mut self, class: &str) -> Self {
        self.classes.push(class.to_string());
        self
    }
    
    pub fn add_when(mut self, class: &str, condition: bool) -> Self {
        if condition {
            self.classes.push(class.to_string());
        }
        self
    }
    
    pub fn to_string(&self) -> String {
        self.classes.join(" ")
    }
}

// Type-safe navigation component
#[component]
pub fn TypeSafeNavigation(cx: Scope) -> impl IntoView {
    let runtime = NextJsRuntime::new();
    
    let navigate_to = move |route: NextRoute| {
        let rt = runtime.clone();
        move |_| {
            let _ = rt.router.navigate(route.clone());
        }
    };
    
    view! { cx,
        <nav class="bg-midnight-800 border-b border-gray-700">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex items-center justify-between h-16">
                    <div class="flex items-center">
                        <div class="flex-shrink-0">
                            <span class="text-2xl font-bold text-spirulina-500">"NRSH"</span>
                        </div>
                        <div class="hidden md:block">
                            <div class="ml-10 flex items-baseline space-x-4">
                                <button 
                                    class="text-gray-300 hover:text-white px-3 py-2 rounded-md text-sm font-medium"
                                    on:click=navigate_to(NextRoute::Dashboard)
                                >
                                    "Dashboard"
                                </button>
                                <button 
                                    class="text-gray-300 hover:text-white px-3 py-2 rounded-md text-sm font-medium"
                                    on:click=navigate_to(NextRoute::Staking)
                                >
                                    "Staking"
                                </button>
                                <button 
                                    class="text-gray-300 hover:text-white px-3 py-2 rounded-md text-sm font-medium"
                                    on:click=navigate_to(NextRoute::Dex)
                                >
                                    "DEX"
                                </button>
                                <button 
                                    class="bg-spirulina-600 text-white px-3 py-2 rounded-md text-sm font-medium"
                                >
                                    "Community"
                                </button>
                            </div>
                        </div>
                    </div>
                    <div>
                        <button class="bg-spirulina-500 hover:bg-spirulina-600 text-white px-4 py-2 rounded-md text-sm font-medium transition">
                            "Connect Wallet"
                        </button>
                    </div>
                </div>
            </div>
        </nav>
    }
}

// Type-safe footer component
#[component]
pub fn TypeSafeFooter(cx: Scope) -> impl IntoView {
    let current_year = 2025; // In a real app, this would be computed

    view! { cx,
        <footer class="bg-midnight-800 border-t border-gray-700 py-8">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="grid md:grid-cols-4 gap-8">
                    <div>
                        <h3 class="text-xl font-bold text-spirulina-500 mb-4">"Nourish Chain"</h3>
                        <p class="text-gray-400 text-sm">
                            "Revolutionizing food security through blockchain technology and sustainable spirulina cultivation."
                        </p>
                    </div>
                    <div>
                        <h4 class="font-bold mb-4">"Resources"</h4>
                        <ul class="space-y-2">
                            <li><a href="#" class="text-gray-400 hover:text-white transition-colors">"Documentation"</a></li>
                            <li><a href="#" class="text-gray-400 hover:text-white transition-colors">"Whitepaper"</a></li>
                            <li><a href="#" class="text-gray-400 hover:text-white transition-colors">"GitHub"</a></li>
                        </ul>
                    </div>
                    <div>
                        <h4 class="font-bold mb-4">"Community"</h4>
                        <ul class="space-y-2">
                            <li><a href="#" class="text-gray-400 hover:text-white transition-colors">"Discord"</a></li>
                            <li><a href="#" class="text-gray-400 hover:text-white transition-colors">"Twitter"</a></li>
                            <li><a href="#" class="text-gray-400 hover:text-white transition-colors">"Telegram"</a></li>
                        </ul>
                    </div>
                    <div>
                        <h4 class="font-bold mb-4">"Legal"</h4>
                        <ul class="space-y-2">
                            <li><a href="#" class="text-gray-400 hover:text-white transition-colors">"Privacy Policy"</a></li>
                            <li><a href="#" class="text-gray-400 hover:text-white transition-colors">"Terms of Service"</a></li>
                        </ul>
                    </div>
                </div>
                <div class="mt-8 pt-8 border-t border-gray-700 text-center text-gray-400 text-sm">
                    {format!("© {} Nourish Chain DAO. All rights reserved.", current_year)}
                </div>
            </div>
        </footer>
    }
}

// Type-safe CSS utility for responsive design
pub enum Breakpoint {
    SM,
    MD,
    LG,
    XL,
}

pub struct ResponsiveClass {
    base: String,
    sm: Option<String>,
    md: Option<String>,
    lg: Option<String>,
    xl: Option<String>,
}

impl ResponsiveClass {
    pub fn new(base: &str) -> Self {
        Self {
            base: base.to_string(),
            sm: None,
            md: None,
            lg: None,
            xl: None,
        }
    }
    
    pub fn with(mut self, breakpoint: Breakpoint, value: &str) -> Self {
        let class = match breakpoint {
            Breakpoint::SM => format!("sm:{}", value),
            Breakpoint::MD => format!("md:{}", value),
            Breakpoint::LG => format!("lg:{}", value),
            Breakpoint::XL => format!("xl:{}", value),
        };
        
        match breakpoint {
            Breakpoint::SM => self.sm = Some(class),
            Breakpoint::MD => self.md = Some(class),
            Breakpoint::LG => self.lg = Some(class),
            Breakpoint::XL => self.xl = Some(class),
        }
        
        self
    }
    
    pub fn to_string(&self) -> String {
        let mut classes = vec![self.base.clone()];
        
        if let Some(sm) = &self.sm {
            classes.push(sm.clone());
        }
        
        if let Some(md) = &self.md {
            classes.push(md.clone());
        }
        
        if let Some(lg) = &self.lg {
            classes.push(lg.clone());
        }
        
        if let Some(xl) = &self.xl {
            classes.push(xl.clone());
        }
        
        classes.join(" ")
    }
}

