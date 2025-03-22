use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="container mx-auto p-4">
            <h1 class="text-3xl font-bold text-primary">"Welcome to Slynqix"</h1>
            <p>"A Rust/Leptos project."</p>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logging");
    mount_to_body(|| view! { <App/> })
}
