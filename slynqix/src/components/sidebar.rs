use leptos::*;
use leptos_router::{A, use_location};

#[component]
pub fn Sidebar(is_open: ReadSignal<bool>) -> impl IntoView {
    let (is_admin_mode, set_is_admin_mode) = create_signal(true); // Default to admin mode
    let (show_animation, set_show_animation) = create_signal(false);
    let location = use_location();

    // Admin links
    let admin_links = vec![
        ("Dashboard", "/dashboard", "M12 2H2v2h10V2zm0 4H2v2h10V6zm0 4H2v2h10v-2zM2 14h14v2H2v-2zm14-4V8h-2v2h2zm0 4h-2v2h2v-2z"),
        ("Console", "/console", "M21 2H3c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h7l-2 3v1h8v-1l-2-3h7c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm0 14H3V4h18v12z"),
        ("Aftermarket", "/aftermarket", "M12 4V2l-4 4 4 4V8c3.3 0 6 2.7 6 6 0 1-.3 1.9-.8 2.7l1.5 1.5C19.5 17 20 15.6 20 14c0-4.4-3.6-8-8-8zm-6 8c0-1 .3-1.9.8-2.7l-1.5-1.5C4.5 9 4 10.4 4 12c0 4.4 3.6 8 8 8v-2l4-4-4-4v2c-3.3 0-6-2.7-6-6z"),
        ("History", "/history", "M13 3c-4.4 0-8 3.6-8 8H2l3.9 3.9-3.9 3.9h3c0 4.4 3.6 8 8 8s8-3.6 8-8-3.6-8-8-8zm-1 14h-2v-5l-2-2v7h-2V7h2l2 2h2v8z"),
        ("Mindsage", "/mindsage", "M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10 10-4.5 10-10S17.5 2 12 2zm0 18c-4.4 0-8-3.6-8-8s3.6-8 8-8 8 3.6 8 8-3.6 8-8 8zm-1-14v6l5.2 3.2 1-1.7-4.2-2.5V6h-2z"),
        ("Algo Trading", "/algo-trading", "M3 17h18v2H3v-2zm0-6h18v2H3v-2zm0-6h18v2H3V5z"),
        ("Global Sentiment", "/global-sentiment", "M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10 10-4.5 10-10S17.5 2 12 2zm0 18c-4.4 0-8-3.6-8-8s3.6-8 8-8 8 3.6 8 8-3.6 8-8 8zM9 9H7v6h2V9zm8 0h-2v6h2V9z"),
        ("Model Trainer", "/model-trainer", "M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10 10-4.5 10-10S17.5 2 12 2zm4 16H8v-2h8v2zm0-4H8v-2h8v2zm0-4H8V8h8v2z"),
        ("Journal", "/journal", "M18 2H6c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM6 4h5v8l-2.5-1.5L6 12V4z"),
        ("Profile", "/profile", "M12 12c2.2 0 4-1.8 4-4s-1.8-4-4-4-4 1.8-4 4 1.8 4 4 4zm0 2c-2.7 0-8 1.3-8 4v2h16v-2c0-2.7-5.3-4-8-4z"),
        ("Admin", "/admin", "M12 2L4.5 20h15L12 2zm0 3.5L16.8 17h-9.6L12 5.5z"),
    ];

    // User links (subset of admin links)
    let user_links = vec![
        ("Dashboard", "/dashboard", "M12 2H2v2h10V2zm0 4H2v2h10V6zm0 4H2v2h10v-2zM2 14h14v2H2v-2zm14-4V8h-2v2h2zm0 4h-2v2h2v-2z"),
        ("Console", "/console", "M21 2H3c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h7l-2 3v1h8v-1l-2-3h7c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm0 14H3V4h18v12z"),
        ("Aftermarket", "/aftermarket", "M12 4V2l-4 4 4 4V8c3.3 0 6 2.7 6 6 0 1-.3 1.9-.8 2.7l1.5 1.5C19.5 17 20 15.6 20 14c0-4.4-3.6-8-8-8zm-6 8c0-1 .3-1.9.8-2.7l-1.5-1.5C4.5 9 4 10.4 4 12c0 4.4 3.6 8 8 8v-2l4-4-4-4v2c-3.3 0-6-2.7-6-6z"),
        ("History", "/history", "M13 3c-4.4 0-8 3.6-8 8H2l3.9 3.9-3.9 3.9h3c0 4.4 3.6 8 8 8s8-3.6 8-8-3.6-8-8-8zm-1 14h-2v-5l-2-2v7h-2V7h2l2 2h2v8z"),
        ("Mindsage", "/mindsage", "M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10 10-4.5 10-10S17.5 2 12 2zm0 18c-4.4 0-8-3.6-8-8s3.6-8 8-8 8 3.6 8 8-3.6 8-8 8zm-1-14v6l5.2 3.2 1-1.7-4.2-2.5V6h-2z"),
        ("Journal", "/journal", "M18 2H6c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM6 4h5v8l-2.5-1.5L6 12V4z"),
        ("Profile", "/profile", "M12 12c2.2 0 4-1.8 4-4s-1.8-4-4-4-4 1.8-4 4 1.8 4 4 4zm0 2c-2.7 0-8 1.3-8 4v2h16v-2c0-2.7-5.3-4-8-4z"),
    ];

    // Select links based on admin mode
    let sidebar_links = move || if is_admin_mode.get() { admin_links.clone() } else { user_links.clone() };

    // Animation effect
    create_effect(move |_| {
        set_show_animation(true);
        set_timeout(move || set_show_animation(false), 600);
    });

    // Placeholder for localStorage equivalent (not implemented yet)
    create_effect(move |_| {
        // In a real app, you'd use a Rust-based storage solution or backend sync
        log::info!("is_admin_mode: {}", is_admin_mode.get());
    });

    let is_active = move |path: &str| location.pathname.get() == path;

    view! {
        <aside class=move || format!(
            "fixed top-16 left-0 bottom-0 z-40 bg-gradient-to-b from-gray-900 to-gray-950 border-r border-gray-800 w-64 transition-transform duration-300 ease-in-out {}",
            if is_open.get() { "translate-x-0" } else { "-translate-x-full" }
        )>
            <div class="h-full flex flex-col py-4 overflow-y-auto">
                <nav class=move || format!(
                    "flex-1 px-2 space-y-1 {}",
                    if show_animation.get() { "animate-fade-in" } else { "" }
                )>
                    {move || sidebar_links().into_iter().enumerate().map(|(index, (name, path, icon))| {
                        view! {
                            <A
                                href=path
                                class=move || format!(
                                    "flex items-center px-3 py-2 rounded-lg text-sm font-medium transition-all duration-300 {} {}",
                                    if is_active(path) {
                                        "bg-violet-600 text-white"
                                    } else {
                                        "text-gray-400 hover:bg-gray-800 hover:text-white"
                                    },
                                    if show_animation.get() { "animate-fade-in" } else { "" }
                                )
                                style=format!("animation-delay: {}ms", index * 50)
                            >
                                <svg class="mr-3 text-violet-400 h-5 w-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <path d=icon />
                                </svg>
                                <span>{name}</span>
                                {move || if is_active(path) {
                                    view! {
                                        <svg class="ml-auto h-4 w-4 opacity-70" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                            <path d="M9 5l7 7-7 7" />
                                        </svg>
                                    }
                                } else {
                                    None
                                }}
                            </A>
                        }
                    }).collect::<Vec<_>>()}
                </nav>

                // Admin Mode Toggle
                <div class="px-3 pt-4 pb-2 border-t border-gray-800">
                    <div class="flex items-center justify-between mb-2">
                        <div class="text-xs uppercase font-semibold text-gray-500 tracking-wider">
                            "View Mode"
                        </div>
                        <button
                            class=move || format!(
                                "h-7 px-2 flex items-center gap-1 transition-all duration-300 border border-gray-700 rounded {}",
                                if is_admin_mode.get() {
                                    "bg-violet-600 text-white hover:bg-violet-700"
                                } else {
                                    "bg-gray-800 text-gray-300 hover:bg-gray-700"
                                }
                            )
                            on:click=move |_| set_is_admin_mode.update(|val| *val = !*val)
                        >
                            <svg class="h-3.5 w-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M20 6L9 17l-5-5" />
                            </svg>
                            <span class="text-xs">{move || if is_admin_mode.get() { "Switch to User" } else { "Switch to Admin" }}</span>
                        </button>
                    </div>
                </div>

                // Footer
                <div class="px-3 pt-4 pb-2 border-t border-gray-800">
                    <div class="text-xs uppercase font-semibold text-gray-500 tracking-wider mb-2">
                        "Slynqix v1.0.0"
                    </div>
                    <p class="text-xs text-gray-500">
                        "Market Analysis Platform"
                    </p>
                </div>
            </div>
        </aside>
    }
}
