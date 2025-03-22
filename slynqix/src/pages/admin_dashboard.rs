// slynqix/src/pages/admin_dashboard.rs
use leptos::*;
use leptos_icons::{Icon, LuActivity, LuAlertTriangle, LuCheckCircle, LuClock, LuRefreshCw, LuServer, LuUser, LuUsers, LuXCircle};
use crate::components::{Button, Card, CardContent, CardDescription, CardHeader, CardTitle, Switch}; // Assume these are defined

#[derive(Clone, Debug)]
struct User {
    id: String,
    name: String,
    email: String,
    status: String, // "active" | "pending" | "rejected"
    last_active: String,
    api_credentials: ApiCredentials,
}

#[derive(Clone, Debug)]
struct ApiCredentials {
    fyers: bool,
    upstox: bool,
}

#[derive(Clone, Debug)]
struct SystemLog {
    id: String,
    timestamp: String,
    log_type: String, // "info" | "warning" | "error"
    message: String,
    source: String,   // "api" | "database" | "system"
}

#[derive(Clone, Debug)]
struct ApiStatus {
    service: String,
    status: String, // "operational" | "degraded" | "down"
    last_checked: String,
    rate_limit: RateLimit,
}

#[derive(Clone, Debug)]
struct RateLimit {
    used: i32,
    total: i32,
}

#[component]
pub fn AdminDashboard() -> impl IntoView {
    let (users, set_users) = create_signal(vec![
        User {
            id: "1".to_string(),
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            status: "active".to_string(),
            last_active: "2023-06-15T10:30:00Z".to_string(),
            api_credentials: ApiCredentials { fyers: true, upstox: false },
        },
        User {
            id: "2".to_string(),
            name: "Jane Smith".to_string(),
            email: "jane@example.com".to_string(),
            status: "pending".to_string(),
            last_active: "2023-06-14T14:20:00Z".to_string(),
            api_credentials: ApiCredentials { fyers: false, upstox: false },
        },
        User {
            id: "3".to_string(),
            name: "Mike Johnson".to_string(),
            email: "mike@example.com".to_string(),
            status: "active".to_string(),
            last_active: "2023-06-15T09:15:00Z".to_string(),
            api_credentials: ApiCredentials { fyers: true, upstox: true },
        },
        User {
            id: "4".to_string(),
            name: "Lisa Brown".to_string(),
            email: "lisa@example.com".to_string(),
            status: "rejected".to_string(),
            last_active: "2023-06-10T16:45:00Z".to_string(),
            api_credentials: ApiCredentials { fyers: false, upstox: false },
        },
    ]);

    let (logs, _set_logs) = create_signal(vec![
        SystemLog { id: "1".to_string(), timestamp: "2023-06-15T10:30:00Z".to_string(), log_type: "info".to_string(), message: "User 'John Doe' logged in successfully".to_string(), source: "system".to_string() },
        SystemLog { id: "2".to_string(), timestamp: "2023-06-15T10:15:00Z".to_string(), log_type: "warning".to_string(), message: "Fyers API rate limit at 80%".to_string(), source: "api".to_string() },
        SystemLog { id: "3".to_string(), timestamp: "2023-06-15T09:45:00Z".to_string(), log_type: "error".to_string(), message: "Database connection timeout".to_string(), source: "database".to_string() },
        SystemLog { id: "4".to_string(), timestamp: "2023-06-15T09:30:00Z".to_string(), log_type: "info".to_string(), message: "System backup completed successfully".to_string(), source: "system".to_string() },
        SystemLog { id: "5".to_string(), timestamp: "2023-06-15T09:00:00Z".to_string(), log_type: "warning".to_string(), message: "High memory usage detected".to_string(), source: "system".to_string() },
    ]);

    let (api_status, _set_api_status) = create_signal(vec![
        ApiStatus { service: "Fyers API".to_string(), status: "operational".to_string(), last_checked: "2023-06-15T10:30:00Z".to_string(), rate_limit: RateLimit { used: 812, total: 1000 } },
        ApiStatus { service: "Upstox API".to_string(), status: "degraded".to_string(), last_checked: "2023-06-15T10:30:00Z".to_string(), rate_limit: RateLimit { used: 750, total: 1000 } },
        ApiStatus { service: "Database".to_string(), status: "operational".to_string(), last_checked: "2023-06-15T10:30:00Z".to_string(), rate_limit: RateLimit { used: 0, total: 0 } },
        ApiStatus { service: "Mindsage Model".to_string(), status: "down".to_string(), last_checked: "2023-06-15T10:30:00Z".to_string(), rate_limit: RateLimit { used: 0, total: 0 } },
    ]);

    let (log_filter, set_log_filter) = create_signal("all".to_string());

    let filtered_logs = move || {
        if log_filter.get() == "all" {
            logs.get()
        } else {
            logs.get().into_iter().filter(|log| log.log_type == log_filter.get() || log.source == log_filter.get()).collect::<Vec<_>>()
        }
    };

    let format_date = |date_string: String| {
        use chrono::{DateTime, Utc};
        let date = DateTime::parse_from_rfc3339(&date_string).unwrap_or(Utc::now());
        date.format("%Y-%m-%d %H:%M:%S").to_string() // Simplified; adjust formatting as needed
    };

    view! {
        <div class="page-transition pt-20 pb-6 px-4">
            <div class="max-w-7xl mx-auto">
                <header class="mb-8">
                    <h1 class="text-3xl font-semibold tracking-tight">"Admin Dashboard"</h1>
                    <p class="text-gray-600 dark:text-gray-400 mt-2">"Manage users, monitor system performance, and view logs"</p>
                </header>

                <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
                    <Card>
                        <CardHeader class="pb-2">
                            <CardTitle class="text-lg font-medium flex items-center">
                                <Icon icon=Icon::from(LuUsers) class="mr-2 h-4 w-4"/>
                                "User Statistics"
                            </CardTitle>
                        </CardHeader>
                        <CardContent>
                            <div class="grid grid-cols-3 gap-4 text-center">
                                <div>
                                    <div class="text-2xl font-bold">{move || users.get().iter().filter(|u| u.status == "active").count()}</div>
                                    <div class="text-xs text-gray-500">"Active"</div>
                                </div>
                                <div>
                                    <div class="text-2xl font-bold">{move || users.get().iter().filter(|u| u.status == "pending").count()}</div>
                                    <div class="text-xs text-gray-500">"Pending"</div>
                                </div>
                                <div>
                                    <div class="text-2xl font-bold">{move || users.get().iter().filter(|u| u.status == "rejected").count()}</div>
                                    <div class="text-xs text-gray-500">"Rejected"</div>
                                </div>
                            </div>
                            <div class="mt-4 pt-4 border-t">
                                <div class="text-sm font-medium">"API Credentials"</div>
                                <div class="grid grid-cols-2 gap-4 mt-2 text-center">
                                    <div>
                                        <div class="text-lg font-bold">{move || users.get().iter().filter(|u| u.api_credentials.fyers).count()}</div>
                                        <div class="text-xs text-gray-500">"Fyers"</div>
                                    </div>
                                    <div>
                                        <div class="text-lg font-bold">{move || users.get().iter().filter(|u| u.api_credentials.upstox).count()}</div>
                                        <div class="text-xs text-gray-500">"Upstox"</div>
                                    </div>
                                </div>
                            </div>
                        </CardContent>
                    </Card>

                    <Card>
                        <CardHeader class="pb-2">
                            <CardTitle class="text-lg font-medium flex items-center">
                                <Icon icon=Icon::from(LuActivity) class="mr-2 h-4 w-4"/>
                                "System Activity"
                            </CardTitle>
                        </CardHeader>
                        <CardContent>
                            <div class="space-y-4">
                                <div>
                                    <div class="text-sm font-medium mb-1">"API Requests (24h)"</div>
                                    <div class="w-full bg-gray-200 dark:bg-gray-800 rounded-full h-2.5">
                                        <div class="bg-blue-600 h-2.5 rounded-full" style="width: 70%"></div>
                                    </div>
                                    <div class="flex justify-between text-xs mt-1">
                                        <span>"0"</span>
                                        <span>"7,000 / 10,000"</span>
                                    </div>
                                </div>
                                <div>
                                    <div class="text-sm font-medium mb-1">"Database Usage"</div>
                                    <div class="w-full bg-gray-200 dark:bg-gray-800 rounded-full h-2.5">
                                        <div class="bg-green-600 h-2.5 rounded-full" style="width: 30%"></div>
                                    </div>
                                    <div class="flex justify-between text-xs mt-1">
                                        <span>"0"</span>
                                        <span>"30% / 100%"</span>
                                    </div>
                                </div>
                                <div>
                                    <div class="text-sm font-medium mb-1">"Memory Usage"</div>
                                    <div class="w-full bg-gray-200 dark:bg-gray-800 rounded-full h-2.5">
                                        <div class="bg-yellow-600 h-2.5 rounded-full" style="width: 65%"></div>
                                    </div>
                                    <div class="flex justify-between text-xs mt-1">
                                        <span>"0"</span>
                                        <span>"65% / 100%"</span>
                                    </div>
                                </div>
                            </div>
                        </CardContent>
                    </Card>

                    <Card>
                        <CardHeader class="pb-2">
                            <CardTitle class="text-lg font-medium flex items-center">
                                <Icon icon=Icon::from(LuAlertTriangle) class="mr-2 h-4 w-4"/>
                                "System Alerts"
                            </CardTitle>
                        </CardHeader>
                        <CardContent>
                            <div class="space-y-4">
                                <div class="flex items-center p-2 bg-yellow-50 dark:bg-yellow-900/20 text-yellow-800 dark:text-yellow-200 rounded-lg">
                                    <Icon icon=Icon::from(LuAlertTriangle) class="h-4 w-4 mr-2 flex-shrink-0"/>
                                    <div class="text-sm">"Fyers API approaching rate limit (81%)"</div>
                                </div>
                                <div class="flex items-center p-2 bg-red-50 dark:bg-red-900/20 text-red-800 dark:text-red-200 rounded-lg">
                                    <Icon icon=Icon::from(LuAlertTriangle) class="h-4 w-4 mr-2 flex-shrink-0"/>
                                    <div class="text-sm">"Mindsage Model service is down"</div>
                                </div>
                                <div class="flex items-center p-2 bg-yellow-50 dark:bg-yellow-900/20 text-yellow-800 dark:text-yellow-200 rounded-lg">
                                    <Icon icon=Icon::from(LuAlertTriangle) class="h-4 w-4 mr-2 flex-shrink-0"/>
                                    <div class="text-sm">"Upstox API experiencing degraded performance"</div>
                                </div>
                            </div>
                            <Button variant="outline" size="sm" class="w-full mt-4">
                                <Icon icon=Icon::from(LuRefreshCw) class="h-3 w-3 mr-2"/>
                                "Refresh Status"
                            </Button>
                        </CardContent>
                    </Card>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
                    <Card class="lg:col-span-2">
                        <CardHeader>
                            <CardTitle>"User Management"</CardTitle>
                            <CardDescription>"Approve or reject user access requests and manage existing users"</CardDescription>
                        </CardHeader>
                        <CardContent>
                            <div class="overflow-x-auto">
                                <table class="w-full border-collapse">
                                    <thead>
                                        <tr class="border-b border-gray-200 dark:border-gray-800">
                                            <th class="text-left py-3 px-4 text-sm font-medium">"User"</th>
                                            <th class="text-left py-3 px-4 text-sm font-medium">"Status"</th>
                                            <th class="text-left py-3 px-4 text-sm font-medium">"API Credentials"</th>
                                            <th class="text-left py-3 px-4 text-sm font-medium">"Last Active"</th>
                                            <th class="text-right py-3 px-4 text-sm font-medium">"Actions"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {move || users.get().into_iter().map(|user| {
                                            view! {
                                                <tr class="border-b border-gray-200 dark:border-gray-800">
                                                    <td class="py-3 px-4">
                                                        <div class="flex items-center">
                                                            <div class="flex items-center justify-center h-8 w-8 rounded-full bg-gray-100 dark:bg-gray-800 mr-3">
                                                                <Icon icon=Icon::from(LuUser) class="h-4 w-4 text-gray-500"/>
                                                            </div>
                                                            <div>
                                                                <div class="font-medium">{user.name}</div>
                                                                <div class="text-sm text-gray-500">{user.email}</div>
                                                            </div>
                                                        </div>
                                                    </td>
                                                    <td class="py-3 px-4">
                                                        <div class=format!("inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {}",
                                                            if user.status == "active" { "bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400" }
                                                            else if user.status == "pending" { "bg-yellow-100 text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-400" }
                                                            else { "bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400" }
                                                        )>
                                                            {user.status.chars().next().unwrap().to_uppercase().to_string() + &user.status[1..]}
                                                        </div>
                                                    </td>
                                                    <td class="py-3 px-4">
                                                        <div class="flex space-x-2">
                                                            <div class=format!("inline-flex items-center px-2 py-0.5 rounded text-xs font-medium {}",
                                                                if user.api_credentials.fyers { "bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-400" }
                                                                else { "bg-gray-100 text-gray-800 dark:bg-gray-900/30 dark:text-gray-400" }
                                                            )>
                                                                "Fyers"
                                                            </div>
                                                            <div class=format!("inline-flex items-center px-2 py-0.5 rounded text-xs font-medium {}",
                                                                if user.api_credentials.upstox { "bg-purple-100 text-purple-800 dark:bg-purple-900/30 dark:text-purple-400" }
                                                                else { "bg-gray-100 text-gray-800 dark:bg-gray-900/30 dark:text-gray-400" }
                                                            )>
                                                                "Upstox"
                                                            </div>
                                                        </div>
                                                    </td>
                                                    <td class="py-3 px-4 text-sm text-gray-500">
                                                        {format_date(user.last_active.clone())}
                                                    </td>
                                                    <td class="py-3 px-4 text-right">
                                                        {if user.status == "pending" {
                                                            view! {
                                                                <div class="flex justify-end gap-2">
                                                                    <Button size="sm" variant="outline" class="h-8 px-2" on:click=move |_| set_users.update(|us| {
                                                                        if let Some(u) = us.iter_mut().find(|u| u.id == user.id) { u.status = "active".to_string() }
                                                                    })>
                                                                        <Icon icon=Icon::from(LuCheckCircle) class="h-4 w-4 text-green-500"/>
                                                                    </Button>
                                                                    <Button size="sm" variant="outline" class="h-8 px-2" on:click=move |_| set_users.update(|us| {
                                                                        if let Some(u) = us.iter_mut().find(|u| u.id == user.id) { u.status = "rejected".to_string() }
                                                                    })>
                                                                        <Icon icon=Icon::from(LuXCircle) class="h-4 w-4 text-red-500"/>
                                                                    </Button>
                                                                </div>
                                                            }
                                                        } else {
                                                            view! {
                                                                <Button size="sm" variant="outline" class="h-8" on:click=move |_| logging::log!("View user details: {:?}", user)>
                                                                    "View Details"
                                                                </Button>
                                                            }
                                                        }}
                                                    </td>
                                                </tr>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </tbody>
                                </table>
                            </div>
                        </CardContent>
                    </Card>

                    <Card>
                        <CardHeader class="flex flex-row items-center justify-between">
                            <div>
                                <CardTitle>"API Status"</CardTitle>
                                <CardDescription>"Monitor external API services and rate limits"</CardDescription>
                            </div>
                            <Button variant="outline" size="sm" class="h-8">
                                <Icon icon=Icon::from(LuRefreshCw) class="h-3.5 w-3.5 mr-2"/>
                                "Refresh"
                            </Button>
                        </CardHeader>
                        <CardContent>
                            <div class="space-y-4">
                                {move || api_status.get().into_iter().map(|api| {
                                    view! {
                                        <div class="flex items-center justify-between">
                                            <div class="flex items-center gap-4">
                                                <div class=format!("h-3 w-3 rounded-full {}",
                                                    if api.status == "operational" { "bg-green-500" }
                                                    else if api.status == "degraded" { "bg-yellow-500" }
                                                    else { "bg-red-500" }
                                                )></div>
                                                <div>
                                                    <div class="font-medium">{api.service.clone()}</div>
                                                    <div class="text-xs text-gray-500">
                                                        {api.status.chars().next().unwrap().to_uppercase().to_string() + &api.status[1..]} " • Last checked: " {format_date(api.last_checked.clone())}
                                                    </div>
                                                </div>
                                            </div>
                                            {if api.rate_limit.total > 0 {
                                                view! {
                                                    <div class="text-right">
                                                        <div class="text-sm font-medium">{api.rate_limit.used} " / " {api.rate_limit.total}</div>
                                                        <div class="text-xs text-gray-500">
                                                            {((api.rate_limit.used as f32 / api.rate_limit.total as f32) * 100.0).round() as i32} "% used"
                                                        </div>
                                                    </div>
                                                }
                                            } else {
                                                view! { <div></div> }
                                            }}
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                            <div class="mt-6 pt-6 border-t border-gray-200 dark:border-gray-800">
                                <div class="flex items-center justify-between mb-4">
                                    <div class="text-sm font-medium">"API Kill Switches"</div>
                                    <div class="text-xs text-gray-500">"Use with caution"</div>
                                </div>
                                <div class="space-y-3">
                                    {move || api_status.get().into_iter()
                                        .filter(|api| api.service != "Database")
                                        .enumerate()
                                        .map(|(i, api)| {
                                            view! {
                                                <div class="flex items-center justify-between">
                                                    <div class="font-medium text-sm">{api.service.clone()}</div>
                                                    <Switch id=format!("api-switch-{}", i)/>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                </div>
                            </div>
                        </CardContent>
                    </Card>

                    <Card>
                        <CardHeader class="flex flex-row items-center justify-between">
                            <div>
                                <CardTitle>"System Logs"</CardTitle>
                                <CardDescription>"View activity and error logs from the system"</CardDescription>
                            </div>
                            // ToggleGroup not directly implemented; using buttons as a simple alternative
                            <div class="grid grid-cols-3 h-8 gap-1">
                                <Button size="sm" variant=move || if log_filter.get() == "all" { "default" } else { "outline" } on:click=move |_| set_log_filter.set("all".to_string())>"All"</Button>
                                <Button size="sm" variant=move || if log_filter.get() == "warning" { "default" } else { "outline" } on:click=move |_| set_log_filter.set("warning".to_string())>"Warnings"</Button>
                                <Button size="sm" variant=move || if log_filter.get() == "error" { "default" } else { "outline" } on:click=move |_| set_log_filter.set("error".to_string())>"Errors"</Button>
                            </div>
                        </CardHeader>
                        <CardContent>
                            <div class="space-y-3 max-h-96 overflow-y-auto pr-2">
                                {move || filtered_logs().into_iter().map(|log| {
                                    view! {
                                        <div class=format!("p-3 text-sm rounded-lg {}",
                                            if log.log_type == "info" { "bg-blue-50 dark:bg-blue-900/20 text-blue-800 dark:text-blue-300" }
                                            else if log.log_type == "warning" { "bg-yellow-50 dark:bg-yellow-900/20 text-yellow-800 dark:text-yellow-300" }
                                            else { "bg-red-50 dark:bg-red-900/20 text-red-800 dark:text-red-300" }
                                        )>
                                            <div class="flex items-start gap-3">
                                                <div class=format!("mt-0.5 h-4 w-4 flex-shrink-0 {}",
                                                    if log.log_type == "info" { "text-blue-500" }
                                                    else if log.log_type == "warning" { "text-yellow-500" }
                                                    else { "text-red-500" }
                                                )>
                                                    {if log.log_type == "info" {
                                                        view! { <Icon icon=Icon::from(LuServer) class="h-4 w-4"/> }
                                                    } else {
                                                        view! { <Icon icon=Icon::from(LuAlertTriangle) class="h-4 w-4"/> }
                                                    }}
                                                </div>
                                                <div class="flex-1">
                                                    <div class="font-medium">{log.message.clone()}</div>
                                                    <div class="text-xs opacity-70 mt-1 flex items-center gap-2">
                                                        <span>{format_date(log.timestamp.clone())}</span>
                                                        <span class="w-1 h-1 rounded-full bg-current opacity-50"></span>
                                                        <span class="capitalize">{log.source.clone()}</span>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </CardContent>
                    </Card>
                </div>
            </div>
        </div>
    }
}
