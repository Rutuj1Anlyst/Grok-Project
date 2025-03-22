// slynqix/src/pages/journal.rs
use leptos::*;
use leptos_icons::{Icon, LuCalendar};
use crate::components::JournalForm;

#[derive(Clone)]
struct JournalEntry {
    id: String,
    date: String,
    symbol: String,
    quantity: i32,
    buy_price: f64,
    sell_price: f64,
    action: String,
    fees: f64,
    pnl: f64,
    notes: String,
}

#[component]
pub fn Journal() -> impl IntoView {
    let sample_entries = vec![
        JournalEntry {
            id: "entry-001".to_string(),
            date: "2023-06-20".to_string(),
            symbol: "Nifty 50".to_string(),
            quantity: 10,
            buy_price: 22345.67,
            sell_price: 22456.78,
            action: "Buy".to_string(),
            fees: 50.0,
            pnl: 1061.1,
            notes: "Bought based on bullish pattern and strong support level.".to_string(),
        },
        JournalEntry {
            id: "entry-002".to_string(),
            date: "2023-06-20".to_string(),
            symbol: "Banknifty".to_string(),
            quantity: 5,
            buy_price: 48765.43,
            sell_price: 48967.21,
            action: "Buy".to_string(),
            fees: 75.0,
            pnl: 933.9,
            notes: "Followed the momentum after positive GDP data.".to_string(),
        },
        JournalEntry {
            id: "entry-003".to_string(),
            date: "2023-06-19".to_string(),
            symbol: "Nifty 50".to_string(),
            quantity: 15,
            buy_price: 22234.56,
            sell_price: 22123.45,
            action: "Sell".to_string(),
            fees: 65.0,
            pnl: -1731.65,
            notes: "Stop loss hit due to unexpected market reversal.".to_string(),
        },
    ];

    let (selected_date, set_selected_date) = create_signal(chrono::Utc::now().format("%Y-%m-%d").to_string());
    let (entries, _set_entries) = create_signal(sample_entries);

    let filtered_entries = move || {
        entries.get().into_iter()
            .filter(|entry| entry.date == selected_date.get())
            .collect::<Vec<_>>()
    };

    let total_pnl = move || filtered_entries().iter().map(|entry| entry.pnl).sum::<f64>();

    view! {
        <div class="page-transition pt-20 pb-6 px-4">
            <div class="max-w-7xl mx-auto">
                <header class="mb-8">
                    <h1 class="text-3xl font-semibold tracking-tight">"Journal"</h1>
                    <p class="text-gray-600 dark:text-gray-400 mt-2">"Record and analyze your trading activities"</p>
                </header>

                <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                    <div class="lg:col-span-1">
                        <div class="glass-card rounded-xl p-6 animate-fade-in">
                            <h2 class="text-xl font-medium mb-4">"Select Date"</h2>
                            <div class="relative mb-6">
                                <Icon icon=Icon::from(LuCalendar) class="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-500 dark:text-gray-400"/>
                                <input
                                    type="date"
                                    value=selected_date
                                    on:change=move |ev| set_selected_date.set(event_target_value(&ev))
                                    class="form-input w-full pl-10"
                                />
                            </div>
                            <div class="p-4 bg-gray-50 dark:bg-gray-900 rounded-xl">
                                <h3 class="text-sm font-medium mb-2">"Summary"</h3>
                                <div class="flex justify-between items-center mb-2">
                                    <span class="text-sm text-gray-600 dark:text-gray-400">"Entries:"</span>
                                    <span class="font-medium">{move || filtered_entries().len()}</span>
                                </div>
                                <div class="flex justify-between items-center">
                                    <span class="text-sm text-gray-600 dark:text-gray-400">"Total PnL:"</span>
                                    <span class=move || format!("font-medium {}", if total_pnl() > 0.0 { "text-green-600 dark:text-green-400" } else if total_pnl() < 0.0 { "text-red-600 dark:text-red-400" } else { "" })>
                                        {move || format!("{:.2}", total_pnl())}
                                    </span>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="lg:col-span-2">
                        <JournalForm/>
                        <div class="glass-card rounded-xl p-6 mt-6 animate-slide-in-up">
                            <h2 class="text-xl font-medium mb-4">"Journal Entries for " {selected_date}</h2>
                            <div class="overflow-x-auto">
                                <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-800">
                                    <thead class="bg-gray-50 dark:bg-gray-900">
                                        <tr>
                                            <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">"Symbol"</th>
                                            <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">"Qty"</th>
                                            <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">"Buy"</th>
                                            <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">"Sell"</th>
                                            <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">"Action"</th>
                                            <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">"Fees"</th>
                                            <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">"PnL"</th>
                                        </tr>
                                    </thead>
                                    <tbody class="bg-white dark:bg-black divide-y divide-gray-200 dark:divide-gray-800">
                                        {move || if filtered_entries().is_empty() {
                                            view! {
                                                <tr>
                                                    <td colspan=7 class="px-4 py-4 text-center text-sm text-gray-500 dark:text-gray-400">
                                                        "No entries found for this date."
                                                    </td>
                                                </tr>
                                            }.into_view()
                                        } else {
                                            filtered_entries().into_iter().map(|entry| {
                                                view! {
                                                    <tr class="hover:bg-gray-50 dark:hover:bg-gray-900 transition-colors">
                                                        <td class="px-4 py-3 whitespace-nowrap text-sm font-medium">{entry.symbol.clone()}</td>
                                                        <td class="px-4 py-3 whitespace-nowrap text-sm text-gray-700 dark:text-gray-300">{entry.quantity}</td>
                                                        <td class="px-4 py-3 whitespace-nowrap text-sm text-gray-700 dark:text-gray-300">{format!("{:.2}", entry.buy_price)}</td>
                                                        <td class="px-4 py-3 whitespace-nowrap text-sm text-gray-700 dark:text-gray-300">{format!("{:.2}", entry.sell_price)}</td>
                                                        <td class="px-4 py-3 whitespace-nowrap text-sm">
                                                            <span class=format!("px-2 py-1 rounded-full text-xs font-medium {}",
                                                                if entry.action == "Buy" { "bg-green-100 text-green-600 dark:bg-green-900/30 dark:text-green-400" }
                                                                else { "bg-red-100 text-red-600 dark:bg-red-900/30 dark:text-red-400" }
                                                            )>
                                                                {entry.action.clone()}
                                                            </span>
                                                        </td>
                                                        <td class="px-4 py-3 whitespace-nowrap text-sm text-gray-700 dark:text-gray-300">{format!("{:.2}", entry.fees)}</td>
                                                        <td class=format!("px-4 py-3 whitespace-nowrap text-sm font-medium {}",
                                                            if entry.pnl > 0.0 { "text-green-600 dark:text-green-400" } else { "text-red-600 dark:text-red-400" }
                                                        )>
                                                            {format!("{:.2}", entry.pnl)}
                                                        </td>
                                                    </tr>
                                                }
                                            }).collect::<Vec<_>>().into_view()
                                        }}
                                    </tbody>
                                </table>
                            </div>
                            {move || if !filtered_entries().is_empty() {
                                view! {
                                    <div class="mt-6">
                                        <h3 class="text-md font-medium mb-2">"Notes"</h3>
                                        {filtered_entries().into_iter().map(|entry| {
                                            view! {
                                                <div class="mb-3 p-3 bg-gray-50 dark:bg-gray-900 rounded-lg">
                                                    <div class="flex items-center gap-2 mb-1">
                                                        <span class="font-medium">{entry.symbol.clone()}</span>
                                                        <span class=format!("text-xs px-2 py-0.5 rounded-full {}",
                                                            if entry.pnl > 0.0 { "bg-green-100 text-green-600 dark:bg-green-900/30 dark:text-green-400" }
                                                            else { "bg-red-100 text-red-600 dark:bg-red-900/30 dark:text-red-400" }
                                                        )>
                                                            {format!("{:.2}", entry.pnl)}
                                                        </span>
                                                    </div>
                                                    <p class="text-sm text-gray-700 dark:text-gray-300">{entry.notes.clone()}</p>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }.into_view()
                            } else {
                                ().into_view()
                            }}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
