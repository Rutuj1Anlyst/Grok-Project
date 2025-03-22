use leptos::*;

#[component]
pub fn Checkbox(
    checked: ReadSignal<bool>,
    set_checked: WriteSignal<bool>,
    class_name: Option<String>,
) -> impl IntoView {
    view! {
        <input
            type="checkbox"
            checked=checked
            on:change=move |ev| set_checked(event_target_checked(&ev))
            class=move || format!(
                "peer h-4 w-4 shrink-0 rounded-sm border border-blue-500 ring-offset-gray-950 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 {} {}",
                if checked.get() { "bg-blue-500 text-white" } else { "" },
                class_name.unwrap_or_default()
            )
        />
        <Show when=move || checked.get()>
            <svg class="h-4 w-4 text-current" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M5 13l4 4L19 7" />
            </svg>
        </Show>
    }
}
