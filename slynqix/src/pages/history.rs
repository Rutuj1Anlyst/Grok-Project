// slynqix/src/pages/history.rs
use leptos::*;
use leptos_router::*;
use leptos_icons::{Icon, LuArrowLeft, LuDownloadCloud, LuSearch};
use crate::components::Button;

#[derive(Clone)]
struct HistoryLog {
    id: String,
    timestamp: String,
    action: String,
    details: String,
    result: String,
}

#[component]
pub fn History() -> impl IntoView {
    let history_data = vec![
        HistoryLog { id: "log-001".to_string(), timestamp: "2023-06-20 14:30:22".to_string(), action: "Analysis".to_string(), details: "Analyzed Nifty 50 with statistical methods".to_string(), result: "Success".to_string() },
        HistoryLog { id: "log-002".to_string(), timestamp: "2023-06-20 14:32:15".to_string(), action: "API Call".to_string(), details: "Fetched historical data for Nifty 50".to_string(), result: "Success".to_string() },
        HistoryLog { id: "log-003".to_string(), timestamp: "2023-06-20 14:35:47".to_string(), action: "Trading Suggestion".to_string(), details: "Generated buy suggestion for Nifty 50 at 22450".to_string(), result: "Success".to_string() },
        HistoryLog { id: "log-004".to_string(), timestamp: "2023-06-19 10:15:33".to_string(), action: "Analysis".to_string(), details: "Analyzed Banknifty with statistical methods".to_string(), result: "Success".to_string() },
        HistoryLog { id: "log-005".to_string(), timestamp: "2023-06-19 10:18:42".to_string(), action: "API Call".to_string(), details: "Fetched historical data for Banknifty".to_string(), result: "Failed".to_string() },
        HistoryLog { id: "log-006".to_string(), timestamp: "2023-06-19 11:20:05".to_string(), action: "API Call".to_string(), details: "Retry: Fetched historical data for Banknifty".to_string(), result: "Success".to_string() },
        HistoryLog { id: "log-007".to_string(), timestamp: "2023-06-19 11:25:18".to_string(), action: "Trading Suggestion".to_string(), details: "Generated sell suggestion for Banknifty at 48970".to_string(), result: "Success".to_string() },
    ];

    view! {
        <div class="page-transition pt-20 pb-6 px-4">
            <div class="max-w-7xl mx-auto">
                <header class="mb-8 flex flex-col md:flex-row md:items-center md:justify-between">
                    <div>
                        <h1 class="text-3xl font-semibold tracking-tight">"History"</h1>
                        <p class="text-gray-600 dark:text-gray-400 mt-2">"Logs of analysis, API calls, and trading suggestions"</p>
                    </div>
                    <div class="mt-4 md:mt-0 flex items-center gap-2">
                        <Button variant="outline" class="rounded-full">
                            <A href="/aftermarket">
                                <Icon icon=Icon::from(LuArrowLeft) class="mr-2 h-4 w-4"/>
                                "Back to Aftermarket"
                            </A>
                        </Button>
                    </div>
                </header>

                <div class="glass-card rounded-xl p-6 mb-8 animate-fade-in">
                    <div class="flex flex-col md:flex-row items-start md:items-center justify-between gap-4">
                        <div class="relative w-full md:w-1/3">
                            <Icon icon=Icon::from(LuSearch) class="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-500 dark:text-gray-400"/>
                            <input
                                type="text"
                                placeholder="Search logs..."
                                class="form-input w-full pl-10"
                            />
                        </div>
                        <div class="flex items-center gap-2">
                            <Button variant="outline" class="rounded-full flex items-center gap-2">
                                <Icon icon=Icon::from(LuDownloadCloud) class="h-4 w-4"/>
                                "Export Logs"
                            </Button>
                            <select class="form-input">
                                <option value="all">"All Actions"</option>
                                <option value="analysis">"Analysis"</option>
                                <option value="api">"API Calls"</option>
                                <option value="trading">"Trading Suggestions"</option>
                            </select>
                        </div>
                    </div>
                </div>

                <div class="glass-card rounded-xl p-6 animate-slide-in-up">
                    <h2 class="text-xl font-medium mb-4">"Activity Logs"</h2>
                    <div class="overflow-x-auto">
                        <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-800">
                            <thead class="bg-gray-50 dark:bg-gray-900">
                                <tr>
                                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">"ID"</th>
                                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">"Timestamp"</th>
                                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">"Action"</th>
                                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">"Details"</th>
                                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">"Result"</th>
                                </tr>
                            </thead>
                            <tbody class="bg-white dark:bg-black divide-y divide-gray-200 dark:divide-gray-800">
                                {history_data.into_iter().map(|log| {
                                    view! {
                                        <tr class="hover:bg-gray-50 dark:hover:bg-gray-900 transition-colors">
                                            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">{log.id}</td>
                                            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700 dark:text-gray-300">{log.timestamp}</td>
                                            <td class="px-6 py-4 whitespace-nowrap text-sm">
                                                <span class=format!("px-2 py-1 rounded-full text-xs font-medium {}",
                                                    if log.action == "Analysis" { "bg-blue-100 text-blue-600 dark:bg-blue-900/30 dark:text-blue-400" }
                                                    else if log.action == "API Call" { "bg-purple-100 text-purple-600 dark:bg-purple-900/30 dark:text-purple-400" }
                                                    else { "bg-green-100 text-green-600 dark:bg-green-900/30 dark:text-green-400" }
                                                )>
                                                    {log.action}
                                                </span>
                                            </td>
                                            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700 dark:text-gray-300">{log.details}</td>
                                            <td class="px-6 py-4 whitespace-nowrap text-sm">
                                                <span class=format!("px-2 py-1 rounded-full text-xs font-medium {}",
                                                    if log.result == "Success" { "bg-green-100 text-green-600 dark:bg-green-900/30 dark:text-green-400" }
                                                    else { "bg-red-100 text-red-600 dark:bg-red-900/30 dark:text-red-400" }
                                                )>
                                                    {log.result}
                                                </span>
                                            </td>
                                        </tr>
                                    }
                                }).collect::<Vec<_>>()}
                            </tbody>
                        </table>
                    </div>
                    <div class="mt-6 flex items-center justify-between">
                        <div class="text-sm text-gray-700 dark:text-gray-300">
                            "Showing " <span class="font-medium">"1"</span> " to " <span class="font-medium">"7"</span> " of " <span class="font-medium">"7"</span> " results"
                        </div>
                        <div class="flex items-center space-x-2">
                            <Button variant="outline" size="sm" class="rounded-full px-4" disabled=true>"Previous"</Button>
                            <Button variant="outline" size="sm" class="rounded-full px-4" disabled=true>"Next"</Button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
