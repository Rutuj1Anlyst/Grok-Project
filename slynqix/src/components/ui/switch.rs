use leptos::*;

#[component]
pub fn Switch(
    checked: ReadSignal<bool>,
    set_checked: WriteSignal<bool>,
    class_name: Option<String>,
) -> impl IntoView {
    view! {
        <button
            class=move || format!(
                "peer inline-flex h-6 w-11 shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 focus-visible:ring-offset-gray-950 disabled:cursor-not-allowed disabled:opacity-50 {} {}",
                if checked.get() { "bg-blue-500" } else { "bg-gray-300 dark:bg-gray-700" },
                class_name.unwrap_or_default()
            )
            on:click=move |_| set_checked.update(|val| *val = !*val)
        >
            <span
                class=move || format!(
                    "pointer-events-none block h-5 w-5 rounded-full bg-white dark:bg-gray-900 shadow-lg ring-0 transition-transform {}",
                    if checked.get() { "translate-x-5" } else { "translate-x-0" }
                )
            />
        </button>
    }
}
