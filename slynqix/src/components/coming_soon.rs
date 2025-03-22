use leptos::*;
use leptos_router::A;

#[component]
pub fn ComingSoon(title: String, description: String, is_admin: Option<bool>) -> impl IntoView {
    let is_admin = is_admin.unwrap_or(false);

    view! {
        <div class="page-transition pt-20 pb-6 px-4">
            <div class="max-w-7xl mx-auto">
                <header class="mb-8">
                    <h1 class="text-3xl font-semibold tracking-tight">{title}</h1>
                    <p class="text-gray-600 dark:text-gray-400 mt-2">{description}</p>
                </header>
                {move || if is_admin {
                    view! {
                        <div class="glass-card rounded-xl p-8 text-center animate-fade-in">
                            <div class="max-w-lg mx-auto">
                                <h2 class="text-2xl font-medium mb-4">"Under Development"</h2>
                                <p class="text-gray-600 dark:text-gray-400 mb-8">
                                    "This feature is currently being developed. The logic will be implemented in upcoming updates."
                                </p>
                                <A href="/dashboard" class="bg-blue-500 hover:bg-blue-500/90 rounded-full px-4 py-2 text-white inline-flex items-center">
                                    <svg class="mr-2 h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                        <path d="M19 12H5m7-7l-7 7 7 7" />
                                    </svg>
                                    "Return to Dashboard"
                                </A>
                            </div>
                        </div>
                    }
                } else {
                    view! {
                        <div class="glass-card rounded-xl p-8 text-center animate-fade-in">
                            <div class="max-w-lg mx-auto">
                                <h2 class="text-2xl font-medium mb-4">"Coming Soon..."</h2>
                                <p class="text-gray-600 dark:text-gray-400 mb-8">
                                    "We're working hard to bring you this exciting new feature. Stay tuned for updates!"
                                </p>
                                <A href="/dashboard" class="bg-blue-500 hover:bg-blue-500/90 rounded-full px-4 py-2 text-white">
                                    "Explore Slynqix"
                                </A>
                            </div>
                        </div>
                    }
                }}
            </div>
        </div>
    }
}
