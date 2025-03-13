use leptos::*;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub token_a: String,
    pub token_b: String,
    pub reserve_a: f64,
    pub reserve_b: f64,
    pub price_ratio: f64,
    pub fee: f64,
}

#[component]
pub fn DexSwapForm(cx: Scope, pair: TokenPair) -> impl IntoView {
    let (input_amount, set_input_amount) = create_signal(cx, 0.0);
    let (output_amount, set_output_amount) = create_signal(cx, 0.0);
    
    let calculate_output = move |input: f64| {
        if input <= 0.0 {
            return 0.0;
        }
        
        // Calculate fee (0.369%)
        let fee_amount = input * pair.fee;
        let input_after_fee = input - fee_amount;
        
        // Constant product formula: x * y = k
        // (reserve_a + amount_in) * (reserve_b - amount_out) = reserve_a * reserve_b
        let numerator = pair.reserve_a * pair.reserve_b;
        let denominator = pair.reserve_a + input_after_fee;
        let new_reserve_b = numerator / denominator;
        pair.reserve_b - new_reserve_b
    };
    
    let handle_input_change = move |ev| {
        let value = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
        set_input_amount(value);
        let output = calculate_output(value);
        set_output_amount(output);
    };
    
    view! { cx,
        <div class="bg-midnight-800 rounded-xl p-6 border border-gold-700/30 shadow-xl">
            <h2 class="text-xl font-semibold text-spirulina-500 mb-4">"Swap Tokens"</h2>
            
            <div class="space-y-4">
                <div class="bg-midnight-900/50 rounded-lg p-4 border border-midnight-700">
                    <div class="flex justify-between items-center mb-2">
                        <span class="text-gray-400 text-sm">"From"</span>
                        <span class="text-white font-medium">{pair.token_a}</span>
                    </div>
                    <div class="relative">
                        <input 
                            type="number"
                            min="0"
                            step="0.01"
                            placeholder="0.0"
                            class="w-full bg-midnight-700 text-white text-right text-xl py-2 px-3 rounded border border-midnight-600 focus:border-gold-500 focus:outline-none"
                            prop:value={input_amount}
                            on:input=handle_input_change
                        />
                    </div>
                </div>
                
                <div class="flex justify-center">
                    <div class="bg-midnight-700 rounded-full p-2 -my-2 z-10">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-gray-400" viewBox="0 0 20 20" fill="currentColor">
                            <path fill-rule="evenodd" d="M16.707 10.293a1 1 0 010 1.414l-6 6a1 1 0 01-1.414 0l-6-6a1 1 0 111.414-1.414L9 14.586V3a1 1 0 012 0v11.586l4.293-4.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                        </svg>
                    </div>
                </div>
                
                <div class="bg-midnight-900/50 rounded-lg p-4 border border-midnight-700">
                    <div class="flex justify-between items-center mb-2">
                        <span class="text-gray-400 text-sm">"To"</span>
                        <span class="text-white font-medium">{pair.token_b}</span>
                    </div>
                    <div class="relative">
                        <input 
                            type="number"
                            readonly
                            class="w-full bg-midnight-700 text-white text-right text-xl py-2 px-3 rounded border border-midnight-600 focus:border-gold-500 focus:outline-none"
                            prop:value={output_amount}
                        />
                    </div>
                </div>
                
                <div class="text-gray-400 text-sm">
                    <div class="flex justify-between">
                        <span>"Exchange Rate:"</span>
                        <span>
                            {"1 "}{pair.token_a}{" = "}{pair.price_ratio}{" "}{pair.token_b}
                        </span>
                    </div>
                    <div class="flex justify-between">
                        <span>"Fee (0.369%):"</span>
                        <span>{(input_amount() * pair.fee).toFixed(4)}{" "}{pair.token_a}</span>
                    </div>
                </div>
                
                <button class="w-full py-3 bg-spirulina-600 hover:bg-spirulina-700 text-white font-medium rounded-lg transition-colors">
                    "Swap Tokens"
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn LiquidityPoolCard(cx: Scope, pair: TokenPair) -> impl IntoView {
    view! { cx,
        <div class="bg-midnight-800 rounded-xl p-6 border border-gold-700/30 shadow-xl">
            <h2 class="text-xl font-semibold text-spirulina-500 mb-4">
                {pair.token_a}{" / "}{pair.token_b}{" Pool"}
            </h2>
            
            <div class="space-y-3">
                <div class="flex justify-between">
                    <span class="text-gray-400">{pair.token_a}{" Reserve"}</span>
                    <span class="text-white font-medium">{pair.reserve_a.toFixed(2)}</span>
                </div>
                <div class="flex justify-between">
                    <span class="text-gray-400">{pair.token_b}{" Reserve"}</span>
                    <span class="text-white font-medium">{pair.reserve_b.toFixed(2)}</span>
                </div>
                <div class="flex justify-between">
                    <span class="text-gray-400">"Trading Fee"</span>
                    <span class="text-spirulina-500 font-medium">"0.369%"</span>
                </div>
                <div class="flex justify-between">
                    <span class="text-gray-400">"Your Liquidity"</span>
                    <span class="text-white font-medium">"0.00"</span>
                </div>
                
                <div class="mt-4 pt-4 border-t border-midnight-700">
                    <button class="w-full py-2 bg-gold-600 hover:bg-gold-700 text-white font-medium rounded-lg transition-colors">
                        "Add Liquidity"
                    </button>
                </div>
            </div>
        </div>
    }
}

#[wasm_bindgen]
pub fn render_dex_swap_form(pair_json: &str) -> String {
    let pair: TokenPair = serde_json::from_str(pair_json).unwrap_or(TokenPair {
        token_a: "NRSH".to_string(),
        token_b: "USDT".to_string(),
        reserve_a: 10000.0,
        reserve_b: 12800.0,
        price_ratio: 1.28,
        fee: 0.00369, // 0.369%
    });
    
    let dex = DexSwapForm(Scope::new(), pair);
    dex.into_view().to_string()
}

#[wasm_bindgen]
pub fn render_liquidity_pool_card(pair_json: &str) -> String {
    let pair: TokenPair = serde_json::from_str(pair_json).unwrap_or(TokenPair {
        token_a: "NRSH".to_string(),
        token_b: "USDT".to_string(),
        reserve_a: 10000.0,
        reserve_b: 12800.0,
        price_ratio: 1.28,
        fee: 0.00369, // 0.369%
    });
    
    let pool_card = LiquidityPoolCard(Scope::new(), pair);
    pool_card.into_view().to_string()
}
