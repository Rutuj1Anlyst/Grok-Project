use leptos::*;
use leptos::ev::MouseEvent;
use web_sys::window;

#[component]
pub fn ThemeToggle(class_name: Option<String>) -> impl IntoView {
    let (theme, set_theme) = create_signal("light");

    // Initial theme detection
    create_effect(move |_| {
        if let Some(window) = window() {
            if let Some(local_storage) = window.local_storage().ok().flatten() {
                if let Ok(Some(stored)) = local_storage.get_item("theme") {
                    set_theme.set(stored);
                } else if window.match_media("(prefers-color-scheme: dark)").ok().flatten()
                    .map_or(false, |media| media.matches()) {
                    set_theme.set("dark");
                }
            }
        }
    });

    // Theme application
    create_effect(move |_| {
        if let Some(window) = window() {
            if let Some(local_storage) = window.local_storage().ok().flatten() {
                let _ = local_storage.set_item("theme", theme.get());
            }
            let doc = window.document().expect("document should exist");
            if theme.get() == "dark" {
                let _ = doc.document_element().unwrap().class_list().add_1("dark");
            } else {
                let _ = doc.document_element().unwrap().class_list().remove_1("dark");
            }
        }
    });

    let toggle_theme = move |_: MouseEvent| {
        set_theme.update(|t| *t = if *t == "light" { "dark" } else { "light" });
    };

    view! {
        <button
            class=move || format!(
                "w-10 h-10 rounded-full transition-transform duration-500 ease-in-out hover:rotate-12 {}",
                class_name.unwrap_or_default()
            )
            on:click=toggle_theme
            aria-label="Toggle theme"
        >
            <Show when=move || theme.get() == "light" fallback=|| view! { <svg class="h-5 w-5 transition-all duration-500" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2a10 10 0 100 20 10 10 0 000-20zm0 18a8 8 0 110-16 8 8 0 010 16z"/></svg> }>
                <svg class="h-5 w-5 transition-all duration-500" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M21 12.79A9 9 0 1111.21 3 7 7 0 0021 12.79z" />
                </svg>
            </Show>
            <span class="sr-only">"Toggle theme"</span>
        </button>
    }
}
