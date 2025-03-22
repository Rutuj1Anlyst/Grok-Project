// slynqix/src/pages/console.rs
use leptos::*;
use leptos_router::*;
use leptos_icons::{Icon, LuArrowRight, LuClock, LuPictureInPicture, LuSearch};
use crate::components::{Button, AnalysisTable, PictureInPicture};

#[derive(Clone)]
struct AnalysisData {
    label: String,
    value: String,
    status: String, // "neutral" | "positive" | "negative"
}

#[component]
pub fn Console() -> impl IntoView {
    let symbols = vec!["Nifty 50", "Banknifty", "Finnifty", "Sensex", "Midcap", "Bitcoin", "Gold"];
    let statistical_analysis_data = vec![
        AnalysisData { label: "Mean".to_string(), value: "22,456.78".to_string(), status: "neutral".to_string() },
        AnalysisData { label: "Standard Deviation".to_string(), value: "345.67".to_string(), status: "neutral".to_string() },
        AnalysisData { label: "52-Week High".to_string(), value: "23,567.89".to_string(), status: "positive".to_string() },
        AnalysisData { label: "52-Week Low".to_string(), value: "19,876.54".to_string(), status: "negative".to_string() },
        AnalysisData { label: "Daily Range".to_string(), value: "22,345.67 - 22,567.89".to_string(), status: "neutral".to_string() },
    ];
    let visual_analysis_data = vec![
        AnalysisData { label: "Trend".to_string(), value: "Uptrend".to_string(), status: "positive".to_string() },
        AnalysisData { label: "Support Level".to_string(), value: "22,100.00".to_string(), status: "neutral".to_string() },
        AnalysisData { label: "Resistance Level".to_string(), value: "22,600.00".to_string(), status: "neutral".to_string() },
        AnalysisData { label: "Pattern".to_string(), value: "Bull Flag".to_string(), status: "positive".to_string() },
        AnalysisData { label: "Volume Profile".to_string(), value: "Above Average".to_string(), status: "positive".to_string() },
    ];
    let indicator_analysis_data = vec![
        AnalysisData { label: "RSI (14)".to_string(), value: "62.5".to_string(), status: "neutral".to_string() },
        AnalysisData { label: "MACD".to_string(), value: "Bullish".to_string(), status: "positive".to_string() },
        AnalysisData { label: "Stochastic".to_string(), value: "78.3".to_string(), status: "positive".to_string() },
        AnalysisData { label: "Bollinger Bands".to_string(), value: "Upper Range".to_string(), status: "neutral".to_string() },
        AnalysisData { label: "Moving Averages".to_string(), value: "Above 50 & 200 MA".to_string(), status: "positive".to_string() },
    ];
    let suggestion_data = vec![
        AnalysisData { label: "Entry Point".to_string(), value: "22,450".to_string(), status: "neutral".to_string() },
        AnalysisData { label: "Target 1".to_string(), value: "22,550".to_string(), status: "positive".to_string() },
        AnalysisData { label: "Target 2".to_string(), value: "22,650".to_string(), status: "positive".to_string() },
        AnalysisData { label: "Stop Loss".to_string(), value: "22,380".to_string(), status: "negative".to_string() },
        AnalysisData { label: "Risk:Reward".to_string(), value: "1:2.5".to_string(), status: "positive".to_string() },
    ];

    let (selected_symbol, set_selected_symbol) = create_signal(symbols[0].to_string());
    let (search_query, set_search_query) = create_signal(String::new());
    let (current_time, set_current_time) = create_signal(chrono::Utc::now());
    let (is_pip_mode, set_is_pip_mode) = create_signal(false);

    // Simulate useEffect with a timer
    let timer = set_interval_with_handle(
        move || set_current_time.set(chrono::Utc::now()),
        std::time::Duration::from_secs(1),
    ).expect("Failed to set interval");

    on_cleanup(move || timer.clear());

    let filtered_symbols = move || {
        symbols.iter()
            .filter(|symbol| symbol.to_lowercase().contains(&search_query.get().to_lowercase()))
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
    };

    let format_time = |date: chrono::DateTime<chrono::Utc>| {
        date.format("%H:%M:%S").to_string()
    };

    let handle_analyze = move |_| {
        logging::log!("Analyzing {}", selected_symbol.get());
        // Toast simulation (log for now; implement a Toast component if needed)
        logging::log!("Analysis Complete: Analysis for {} has been completed.", selected_symbol.get());
    };

    let toggle_pip_mode = move |_| {
        set_is_pip_mode.update(|val| *val = !*val);
        logging::log!("{}", if is_pip_mode.get() { "PiP Mode Disabled" } else { "PiP Mode Enabled" });
    };

    #[component]
    fn PipContent(symbol: String) -> impl IntoView {
        view! {
            <div class="text-sm">
                <h4 class="font-medium mb-2">{symbol} " Analysis"</h4>
                <div class="space-y-2">
                    <div class="bg-gray-800 rounded p-2">
                        <div class="text-xs font-medium text-gray-400 mb-1">"Current Status"</div>
                        <div class="font-medium">"Trading at 22,456.78"</div>
                    </div>
                    <div class="grid grid-cols-2 gap-2">
                        <div class="bg-gray-800 rounded p-2">
                            <div class="text-xs font-medium text-gray-400 mb-1">"Support"</div>
                            <div class="font-medium">"22,100.00"</div>
                        </div>
                        <div class="bg-gray-800 rounded p-2">
                            <div class="text-xs font-medium text-gray-400 mb-1">"Resistance"</div>
                            <div class="font-medium">"22,600.00"</div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }

    view! {
        <div class="page-transition pt-20 pb-6 px-4">
            <div class="max-w-7xl mx-auto">
                <header class="mb-8 flex flex-col md:flex-row md:items-center md:justify-between">
                    <div>
                        <h1 class="text-3xl font-semibold tracking-tight">"Console"</h1>
                        <p class="text-gray-600 dark:text-gray-400 mt-2">"Analysis and trading suggestions for market indices"</p>
                    </div>
                    <div class="mt-4 md:mt-0 flex items-center gap-4">
                        <div class="flex items-center gap-2 text-sm bg-gray-100 dark:bg-gray-900 rounded-full px-4 py-2">
                            <Icon icon=Icon::from(LuClock) class="h-4 w-4 text-gray-500 dark:text-gray-400"/>
                            <span>{move || format_time(current_time.get())}</span>
                        </div>
                        <Button variant="outline" size="sm" class="rounded-full gap-2" on:click=toggle_pip_mode>
                            <Icon icon=Icon::from(LuPictureInPicture) class="h-4 w-4"/>
                            {move || if is_pip_mode.get() { "Disable PiP" } else { "Enable PiP" }}
                        </Button>
                        <Button variant="outline" class="rounded-full">
                            <A href="/aftermarket">"Aftermarket Analyzer" <Icon icon=Icon::from(LuArrowRight) class="ml-2 h-4 w-4"/></A>
                        </Button>
                    </div>
                </header>

                <div class="glass-card rounded-xl p-6 mb-8 animate-fade-in">
                    <div class="flex flex-col md:flex-row md:items-center gap-4">
                        <div class="relative flex-1">
                            <label for="symbol-search" class="block text-sm font-medium mb-2">"Select Symbol"</label>
                            <div class="relative">
                                <Icon icon=Icon::from(LuSearch) class="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-500 dark:text-gray-400"/>
                                <input
                                    id="symbol-search"
                                    type="text"
                                    value=search_query
                                    on:input=move |ev| set_search_query.set(event_target_value(&ev))
                                    placeholder="Search symbol..."
                                    class="form-input w-full pl-10"
                                />
                            </div>
                            <Show when=move || !search_query.get().is_empty()>
                                <div class="absolute z-10 w-full mt-1 bg-white dark:bg-gray-900 shadow-md rounded-xl max-h-60 overflow-y-auto">
                                    {move || if filtered_symbols().is_empty() {
                                        view! {
                                            <div class="px-4 py-2 text-gray-500 dark:text-gray-400">"No symbols found"</div>
                                        }.into_view()
                                    } else {
                                        filtered_symbols().into_iter().map(|symbol| {
                                            view! {
                                                <button
                                                    class="block w-full text-left px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-800"
                                                    on:click=move |_| {
                                                        set_selected_symbol.set(symbol.clone());
                                                        set_search_query.set(String::new());
                                                    }
                                                >
                                                    {symbol}
                                                </button>
                                            }
                                        }).collect::<Vec<_>>().into_view()
                                    }}
                                </div>
                            </Show>
                        </div>
                        <div class="md:w-1/3">
                            <div class="text-sm font-medium mb-2">"Selected Symbol"</div>
                            <div class="bg-gray-100 dark:bg-gray-900 rounded-xl px-4 py-3 flex items-center justify-between">
                                <span class="font-medium">{move || selected_symbol.get()}</span>
                                <button class="text-accent hover:text-accent/80" on:click=handle_analyze>"Analyze"</button>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
                    <div class="space-y-6 animate-slide-in-left">
                        <AnalysisTable title="Statistical Analysis" data=statistical_analysis_data/>
                        <AnalysisTable title="Visual Analysis" data=visual_analysis_data/>
                    </div>
                    <div class="space-y-6 animate-slide-in-right">
                        <AnalysisTable title="Indicator Analysis" data=indicator_analysis_data/>
                        <AnalysisTable title="Trading Suggestions" data=suggestion_data class="border-2 border-accent/30"/>
                    </div>
                </div>
            </div>

            <PictureInPicture
                title=move || format!("{} Analysis", selected_symbol.get())
                is_open=is_pip_mode
                on_close=move || set_is_pip_mode.set(false)
            >
                <PipContent symbol=selected_symbol.get()/>
            </PictureInPicture>
        </div>
    }
}
