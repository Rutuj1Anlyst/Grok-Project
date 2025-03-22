// slynqix/src/pages/not_found.rs
use leptos::*;
use leptos_router::*;

#[component]
pub fn NotFound() -> impl IntoView {
    let location = use_location();

    create_effect(move |_| {
        logging::error!(
            "404 Error: User attempted to access non-existent route: {}",
            location.pathname.get()
        );
    });

    view! {
        <div class="min-h-screen flex items-center justify-center bg-gray-100">
            <div class="text-center">
                <h1 class="text-4xl font-bold mb-4">"404"</h1>
                <p class="text-xl text-gray-600 mb-4">"Oops! Page not found"</p>
                <A href="/" class="text-blue-500 hover:text-blue-700 underline">
                    "Return to Home"
                </A>
            </div>
        </div>
    }
}
