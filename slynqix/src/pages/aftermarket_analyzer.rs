// slynqix/src/pages/aftermarket_analyzer.rs
use leptos::*;
use leptos_router::*;
use leptos_icons::{Icon, LuArrowRight, LuCalendar, LuFilter};
use crate::components::Button;

#[derive(Clone)]
struct MarketData {
    symbol: String,
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: i32,
}

#[component]
pub fn AftermarketAnalyzer() -> impl IntoView {
    let sample_data = vec![
        MarketData { symbol: "Nifty 50".to_string(), date: "2023-06-20".to_string(), open: 22345.67, high: 22567.89, low: 22234.56, close: 22456.78, volume: 1234567 },
        MarketData { symbol: "Nifty 50".to_string(), date: "2023-06-19".to_string(), open: 22123.45, high: 22345.67, low: 22098.76, close: 22345.67, volume: 1178943 },
        MarketData { symbol: "Nifty 50".to_string(), date: "2023-06-16".to_string(), open: 22056.78, high: 22178.90, low: 21987.65, close: 22123.45, volume: 1087654 },
        MarketData { symbol: "Banknifty".to_string(), date: "2023-06-20".to_string(), open: 48765.43, high: 49234.56, low: 48654.32, close: 48967.21, volume: 978654 },
        MarketData { symbol: "Banknifty".to_string(), date: "2023-06-19".to_string(), open: 48543.21, high: 48876.54, low: 48432.10, close: 48765.43, volume: 954321 },
    ];

    let symbols = vec!["Nifty 50", "Banknifty", "Finnifty", "Sensex", "Midcap"];
    let (selected_symbol, set_selected_symbol) = create_signal("Nifty 50".to_string());
    let (selected_date, set_selected_date) = create_signal(chrono::Utc::now().format("%Y-%m-%d").to_string());
    let (filtered_data, set_filtered_data) = create_signal(sample_data.clone());

    let handle_analyze = move |_| {
        let new_filtered_data = sample_data.iter()
            .filter(|item| item.symbol == selected_symbol.get())
            .cloned()
            .collect::<Vec<_>>();
        set_filtered_data.set(new_filtered_data);
    };

    view! {
        <div class="page-transition pt-20 pb-6 px-4">
            <div class="max-w-7xl mx-auto">
                <header class="mb-8 flex flex-col md:flex-row md:items-center md:justify-between">
                    <div>
                        <h1 class="text-3xl font-semibold tracking-tight">"Aftermarket Analyzer"</h1>
                        <p class="text-gray-600 dark:text-gray-400 mt-2">"Analyze historical OHLCV data for market indices"</p>
                    </div>
                    <div class="mt-4 md:mt-0">
                        <Button variant="outline" class="rounded-full">
                            <A href="/history">"History" <Icon icon=Icon::from(LuArrowRight) class="ml-2 h-4 w-4"/></A>
                        </Button>
                    </div>
                </header>

                <div class="glass-card rounded-xl p-6 mb-8 animate-fade-in">
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                        <div>
                            <label for="symbol-select" class="block text-sm font-medium mb-2">"Select Symbol"</label>
                            <select
                                id="symbol-select"
                                value=selected_symbol
                                on:change=move |ev| set_selected_symbol.set(event_target_value(&ev))
                                class="form-input w-full"
                            >
                                {symbols.into_iter().map(|symbol| {
                                    view! { <option value=symbol>{symbol}</option> }
                                }).collect::<Vec<_>>()}
                            </select>
                        </div>
                        <div>
                            <label for="date-picker" class="block text-sm font-medium mb-2">"Select Date"</label>
                            <div class="relative">
                                <Icon icon=Icon::from(LuCalendar) class="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-500 dark:text-gray-400"/>
                                <input
                                    id="date-picker"
                                    type="date"
                                    value=selected_date
                                    on:change=move |ev| set_selected_date.set(event_target_value(&ev))
                                    class="form-input w-full pl-10"
                                />
                            </div>
                        </div>
                        <div class="flex items-end">
                            <Button
                                on:click=handle_analyze
                                class="w-full bg-accent hover:bg-accent/90 rounded-full flex items-center justify-center gap-2"
                            >
                                <Icon icon=Icon::from(LuFilter) class="h-4 w-4"/>
                                "Analyze"
                            </Button>
                        </div>
                    </div>
                </div>

                <div class="glass-card rounded-xl p-6 animate-slide-in-up">
                    <h2 class="text-xl font-medium mb-4">"Historical Data"</h2>
                    <div class="overflow-x-auto">
                        <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-800">
                            <thead class="bg-gray-50 dark:bg-gray-900">
                                <tr>
                                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">"Symbol"</th>
                                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">"Date"</th>
                                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">"Open"</th>
                                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">"High"</th>
                                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">"Low"</th>
                                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">"Close"</th>
                                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">"Volume"</th>
                                </tr>
                            </thead>
                            <tbody class="bg-white dark:bg-black divide-y divide-gray-200 dark:divide-gray-800">
                                {move || if filtered_data.get().is_empty() {
                                    view! {
                                        <tr>
                                            <td colspan=7 class="px-6 py-4 text-center text-sm text-gray-500 dark:text-gray-400">
                                                "No data available for the selected filter."
                                            </td>
                                        </tr>
                                    }.into_view()
                                } else {
                                    filtered_data.get().into_iter().map(|item| {
                                        view! {
                                            <tr class="hover:bg-gray-50 dark:hover:bg-gray-900 transition-colors">
                                                <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">{item.symbol.clone()}</td>
                                                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700 dark:text-gray-300">{item.date.clone()}</td>
                                                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700 dark:text-gray-300">{format!("{:.2}", item.open)}</td>
                                                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700 dark:text-gray-300">{format!("{:.2}", item.high)}</td>
                                                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700 dark:text-gray-300">{format!("{:.2}", item.low)}</td>
                                                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700 dark:text-gray-300">{format!("{:.2}", item.close)}</td>
                                                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700 dark:text-gray-300">{item.volume.to_string()}</td>
                                            </tr>
                                        }
                                    }).collect::<Vec<_>>().into_view()
                                }}
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </div>
    }
}
