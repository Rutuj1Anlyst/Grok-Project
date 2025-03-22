use leptos::*;

#[component]
pub fn Textarea(
    value: RwSignal<String>,
    class_name: Option<String>,
) -> impl IntoView {
    view! {
        <textarea
            class=move || format!(
                "flex min-h-[80px] w-full rounded-md border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-900 px-3 py-2 text-sm ring-offset-gray-950 placeholder:text-gray-500 dark:placeholder:text-gray-400 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 {}",
                class_name.unwrap_or_default()
            )
            value=value
            on:input=move |ev| value.set(event_target_value(&ev))
        />
    }
}
