use leptos::*;
use leptos_router::{A, use_location};

#[component]
pub fn Header(toggle_sidebar: Callback<()>) -> impl IntoView {
    let (show_profile, set_show_profile) = create_signal(false);
    let (show_notifications, set_show_notifications) = create_signal(false);
    let location = use_location();

    let nav_links = vec![
        ("Home", "/dashboard"),
        ("Console", "/console"),
        ("Mindsage", "/mindsage"),
        ("Algo", "/algo-trading"),
        ("Sentiment", "/global-sentiment"),
        ("Journal", "/journal"),
    ];

    let notifications = vec![
        ("New trading signal", "Buy signal detected for NIFTY at 22,500", "2 min ago", false, "success"),
        ("API limit warning", "Fyers API reaching 80% rate limit", "20 min ago", false, "warning"),
        ("System update", "Platform updated to version 1.1.2", "1 hour ago", true, "info"),
        ("Model training complete", "Custom model training completed with 94% accuracy", "Yesterday", true, "success"),
    ];

    let unread_count = notifications.iter().filter(|(_, _, _, read, _)| !read).count();

    let is_active = move |path: &str| location.pathname.get() == path;

    view! {
        <header class="fixed top-0 left-0 right-0 z-50 backdrop-blur-md bg-white/80 dark:bg-black/80 border-b border-gray-200 dark:border-gray-800">
            <div class="container mx-auto px-4 h-16 flex items-center justify-between">
                <div class="flex items-center gap-4">
                    <button
                        class="rounded-full hover:bg-gray-100 dark:hover:bg-gray-900 p-2"
                        on:click=move |_| toggle_sidebar.call(())
                    >
                        <svg class="h-5 w-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <line x1="3" y1="6" x2="21" y2="6" />
                            <line x1="3" y1="12" x2="21" y2="12" />
                            <line x1="3" y1="18" x2="21" y2="18" />
                        </svg>
                        <span class="sr-only">"Toggle sidebar"</span>
                    </button>
                    <A href="/" class="flex items-center">
                        <h1 class="text-xl font-semibold tracking-tighter transition-transform hover:scale-105">"Slynqix"</h1>
                    </A>
                </div>

                <nav class="hidden md:flex items-center space-x-1">
                    {nav_links.into_iter().map(|(name, path)| {
                        view! {
                            <A
                                href=path
                                class=move || format!(
                                    "nav-link px-3 py-2 rounded {}",
                                    if is_active(path) { "bg-gray-200 dark:bg-gray-700" } else { "hover:bg-gray-100 dark:hover:bg-gray-900" }
                                )
                            >
                                {name}
                            </A>
                        }
                    }).collect::<Vec<_>>()}
                </nav>

                <div class="flex items-center gap-2">
                    // Notification Bell and Dropdown
                    <div class="relative">
                        <button
                            class="rounded-full hover:bg-gray-100 dark:hover:bg-gray-900 p-2 relative"
                            on:click=move |_| {
                                set_show_notifications.update(|val| *val = !*val);
                                if show_profile.get() { set_show_profile(false); }
                            }
                        >
                            <svg class="h-5 w-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M12 22c1.1 0 2-.9 2-2h-4c0 1.1.9 2 2 2zm6-6v-5c0-3.1-1.6-5.6-4.5-6.3V4c0-.8-.7-1.5-1.5-1.5s-1.5.7-1.5 1.5v.7C7.6 5.4 6 7.9 6 11v5l-2 2v1h16v-1l-2-2z" />
                            </svg>
                            {move || if unread_count > 0 {
                                view! { <span class="absolute top-0 right-0 w-2 h-2 bg-red-500 rounded-full animate-pulse"></span> }
                            } else {
                                None
                            }}
                            <span class="sr-only">"Notifications"</span>
                        </button>

                        {move || if show_notifications.get() {
                            view! {
                                <div class="absolute right-0 mt-2 w-80 rounded-xl shadow-md py-1 z-50 bg-white/80 dark:bg-black/80 backdrop-blur-md border border-gray-200 dark:border-gray-800 animate-fade-in overflow-hidden">
                                    <div class="flex items-center justify-between px-4 py-2 border-b border-gray-200 dark:border-gray-800">
                                        <div class="font-medium">"Notifications"</div>
                                        <button
                                            class="rounded-full h-6 w-6 p-1"
                                            on:click=move |_| set_show_notifications(false)
                                        >
                                            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                <path d="M6 18L18 6M6 6l12 12" />
                                            </svg>
                                        </button>
                                    </div>
                                    <div class="max-h-96 overflow-y-auto">
                                        {if notifications.is_empty() {
                                            view! { <div class="py-8 text-center text-gray-500 dark:text-gray-400">"No notifications"</div> }
                                        } else {
                                            notifications.into_iter().map(|(title, message, time, read, type_)| {
                                                view! {
                                                    <div class=format!(
                                                        "p-3 border-b border-gray-100 dark:border-gray-800 hover:bg-gray-50 dark:hover:bg-gray-900/50 transition-colors {}",
                                                        if !read { "bg-accent/5" } else { "" }
                                                    )>
                                                        <div class="flex items-start">
                                                            <div class=format!(
                                                                "mt-0.5 w-2 h-2 rounded-full flex-shrink-0 mr-2 {}",
                                                                match type_ {
                                                                    "success" => "bg-green-500",
                                                                    "warning" => "bg-yellow-500",
                                                                    "info" => "bg-blue-500",
                                                                    "error" => "bg-red-500",
                                                                    _ => "bg-gray-500",
                                                                }
                                                            )></div>
                                                            <div class="flex-1">
                                                                <div class="flex justify-between">
                                                                    <span class="font-medium">{title}</span>
                                                                    <span class="text-xs text-gray-500">{time}</span>
                                                                </div>
                                                                <p class="text-sm text-gray-600 dark:text-gray-300 mt-1">{message}</p>
                                                            </div>
                                                        </div>
                                                    </div>
                                                }
                                            }).collect::<Vec<_>>()
                                        }}
                                    </div>
                                    <div class="p-2 border-t border-gray-200 dark:border-gray-800">
                                        <button class="w-full justify-center text-xs px-4 py-2 border border-gray-300 dark:border-gray-700 rounded hover:bg-gray-100 dark:hover:bg-gray-900 flex items-center">
                                            <svg class="mr-1 h-3 w-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                <path d="M5 13l4 4L19 7" />
                                            </svg>
                                            "Mark all as read"
                                        </button>
                                    </div>
                                </div>
                            }
                        } else {
                            None
                        }}
                    </div>

                    // Theme Toggle (placeholder, assumes ThemeToggle component exists)
                    <ThemeToggle />

                    // Profile Dropdown
                    <div class="relative">
                        <button
                            class="rounded-full hover:bg-gray-100 dark:hover:bg-gray-900 p-2"
                            on:click=move |_| {
                                set_show_profile.update(|val| *val = !*val);
                                if show_notifications.get() { set_show_notifications(false); }
                            }
                        >
                            <svg class="h-5 w-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M12 12c2.2 0 4-1.8 4-4s-1.8-4-4-4-4 1.8-4 4 1.8 4 4 4zm0 2c-2.7 0-8 1.3-8 4v2h16v-2c0-2.7-5.3-4-8-4z" />
                            </svg>
                            <span class="sr-only">"Profile"</span>
                        </button>

                        {move || if show_profile.get() {
                            view! {
                                <div class="absolute right-0 mt-2 w-48 rounded-xl shadow-md py-1 z-50 bg-white/80 dark:bg-black/80 backdrop-blur-md border border-gray-200 dark:border-gray-800 animate-fade-in">
                                    <button
                                        class="absolute right-2 top-2 rounded-full h-6 w-6 p-1"
                                        on:click=move |_| set_show_profile(false)
                                    >
                                        <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                            <path d="M6 18L18 6M6 6l12 12" />
                                        </svg>
                                    </button>
                                    <div class="px-4 py-6">
                                        <div class="font-medium">"Admin User"</div>
                                        <div class="text-sm text-gray-500 dark:text-gray-400">"admin@slynqix.com"</div>
                                    </div>
                                    <div class="border-t border-gray-200 dark:border-gray-800">
                                        <A
                                            href="/profile"
                                            class="block px-4 py-2 text-sm hover:bg-gray-100 dark:hover:bg-gray-900"
                                            on:click=move |_| set_show_profile(false)
                                        >
                                            "Profile"
                                        </A>
                                        <button class="block w-full text-left px-4 py-2 text-sm hover:bg-gray-100 dark:hover:bg-gray-900">
                                            "Logout"
                                        </button>
                                    </div>
                                </div>
                            }
                        } else {
                            None
                        }}
                    </div>
                </div>
            </div>
        </header>
    }
}
