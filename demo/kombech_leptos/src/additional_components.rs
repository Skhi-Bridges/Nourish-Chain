use leptos::*;
use crate::type_safe_components::*;

#[component]
pub fn LoadingSpinner(cx: Scope) -> impl IntoView {
    let spinner_class = TailwindClass::new()
        .add("animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-white")
        .to_string();
    
    view! { cx,
        <div class="flex items-center justify-center p-4">
            <div class={spinner_class}></div>
        </div>
    }
}

#[component]
pub fn TwitterFeed(cx: Scope) -> impl IntoView {
    let card_class = TailwindClass::new()
        .add("bg-[#1DA1F2] p-4 rounded-xl h-96 overflow-y-auto")
        .to_string();
    
    view! { cx,
        <div class={card_class}>
            <h3 class="text-lg font-bold mb-4">Latest Tweets</h3>
            <Suspense fallback=move || view! { cx, <LoadingSpinner/> }>
                <TwitterFeedContent/>
            </Suspense>
        </div>
    }
}

#[component]
fn TwitterFeedContent(cx: Scope) -> impl IntoView {
    // Mock data - would be replaced with API call
    let tweets = vec![
        (
            "@NRSH_ELXR",
            "2h ago",
            "Excited to announce our new partnership with EigenLayer for enhanced validator security! #blockchain #sustainability"
        ),
        (
            "@NRSH_ELXR",
            "6h ago",
            "Join our next community call to discuss the latest spirulina cultivation techniques. Link in bio!"
        ),
        (
            "@NRSH_Community",
            "1d ago",
            "Our validator network has reached 150 nodes spanning 32 countries! Global decentralization at work."
        ),
    ];
    
    view! { cx,
        <div class="space-y-4">
            {tweets.into_iter().map(|(username, time, content)| {
                view! { cx,
                    <div class="border-b border-white border-opacity-10 pb-3 mb-3">
                        <div class="flex items-center">
                            <div class="w-10 h-10 rounded-full bg-white bg-opacity-20 flex items-center justify-center">
                                <span class="text-lg">"🌱"</span>
                            </div>
                            <div class="ml-2">
                                <p class="font-bold text-sm">{username}</p>
                                <p class="text-xs text-gray-200">{time}</p>
                            </div>
                        </div>
                        <p class="mt-2 text-sm">{content}</p>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

#[component]
pub fn DiscordWidget(cx: Scope) -> impl IntoView {
    let card_class = TailwindClass::new()
        .add("bg-[#5865F2] p-4 rounded-xl h-96 overflow-y-auto")
        .to_string();
    
    view! { cx,
        <div class={card_class}>
            <h3 class="text-lg font-bold mb-4">Discord Community</h3>
            <div class="bg-white bg-opacity-10 rounded-lg p-4 mb-4">
                <div class="flex justify-between items-center">
                    <div>
                        <h4 class="font-bold">Nourish Chain DAO</h4>
                        <p class="text-sm">3,245 members online</p>
                    </div>
                    <div class="w-12 h-12 rounded-full bg-white bg-opacity-20 flex items-center justify-center">
                        <span class="text-xl">"🌿"</span>
                    </div>
                </div>
                <a href="#" 
                   class="mt-4 block w-full bg-white bg-opacity-20 hover:bg-opacity-30 text-center py-2 rounded-md font-medium transition"
                >
                    Join Server
                </a>
            </div>
        </div>
    }
}

#[component]
pub fn TelegramWidget(cx: Scope) -> impl IntoView {
    let card_class = TailwindClass::new()
        .add("bg-[#0088cc] p-4 rounded-xl h-96 overflow-y-auto")
        .to_string();
    
    view! { cx,
        <div class={card_class}>
            <h3 class="text-lg font-bold mb-4">Telegram Group</h3>
            <div class="bg-white bg-opacity-10 rounded-lg p-4">
                <div class="flex justify-between items-center">
                    <div>
                        <h4 class="font-bold">NRSH Community</h4>
                        <p class="text-sm">1,876 members</p>
                    </div>
                    <div class="w-12 h-12 rounded-full bg-white bg-opacity-20 flex items-center justify-center">
                        <span class="text-xl">"📱"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn YouTubeFeed(cx: Scope) -> impl IntoView {
    let card_class = TailwindClass::new()
        .add("bg-[#FF0000] p-4 rounded-xl h-96 overflow-y-auto")
        .to_string();
    
    view! { cx,
        <div class={card_class}>
            <h3 class="text-lg font-bold mb-4">YouTube Channel</h3>
            <div class="space-y-4">
                <div class="bg-white bg-opacity-10 rounded-lg overflow-hidden">
                    <div class="h-32 bg-black flex items-center justify-center">
                        <div class="w-12 h-12 rounded-full bg-white bg-opacity-20 flex items-center justify-center">
                            <span class="text-xl">"▶️"</span>
                        </div>
                    </div>
                    <div class="p-3">
                        <h4 class="font-medium text-sm">Nourish Chain: Revolutionizing Food Security</h4>
                        <p class="text-xs text-gray-300 mt-1">32K views • 2 weeks ago</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ContentShowcase(cx: Scope) -> impl IntoView {
    let grid_class = ResponsiveClass::new("grid gap-6 mt-12")
        .with(Breakpoint::MD, "grid-cols-3")
        .to_string();
    
    view! { cx,
        <div class="mt-12">
            <h2 class="text-2xl font-bold mb-6">Latest Updates</h2>
            <div class={grid_class}>
                <div class="bg-midnight-800 rounded-xl overflow-hidden shadow-lg">
                    <div class="h-48 bg-spirulina-500 flex items-center justify-center">
                        <span class="text-4xl">"📈"</span>
                    </div>
                    <div class="p-6">
                        <h3 class="font-bold text-lg mb-2">Q1 Growth Report</h3>
                        <p class="text-gray-400 text-sm mb-4">
                            The NRSH ecosystem grew by 38% this quarter with significant expansion in Southeast Asia.
                        </p>
                        <a href="#" class="text-spirulina-500 text-sm font-medium hover:underline">Read Full Report →</a>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn InteractiveSection(cx: Scope) -> impl IntoView {
    let grid_class = ResponsiveClass::new("grid gap-8 mt-16 mb-8")
        .with(Breakpoint::MD, "grid-cols-2")
        .to_string();
    
    view! { cx,
        <div class="mt-16 mb-8">
            <h2 class="text-2xl font-bold mb-6">Join the Movement</h2>
            <div class={grid_class}>
                <div class="bg-midnight-800 p-6 rounded-xl">
                    <h3 class="text-xl font-bold mb-4">Ask the Community</h3>
                    <form>
                        <div class="mb-4">
                            <label for="email" class="block text-sm font-medium mb-1">Your Email</label>
                            <input
                                type="email"
                                id="email"
                                class="w-full p-2 rounded-md bg-midnight-900 border border-gray-700 focus:border-spirulina-500 focus:outline-none transition"
                                required
                            />
                        </div>
                        <div class="mb-4">
                            <label for="question" class="block text-sm font-medium mb-1">Your Question</label>
                            <textarea
                                id="question"
                                class="w-full p-2 rounded-md bg-midnight-900 border border-gray-700 focus:border-spirulina-500 focus:outline-none transition h-24"
                                required
                            ></textarea>
                        </div>
                        <button
                            type="submit"
                            class="w-full bg-spirulina-500 hover:bg-spirulina-600 text-white py-2 px-4 rounded-md font-medium transition"
                        >
                            Submit Question
                        </button>
                    </form>
                </div>
            </div>
        </div>
    }
}

