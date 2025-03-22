use leptos::*;

#[component]
pub fn Input(
    value: RwSignal<String>,
    input_type: Option<&'static str>,
    class_name: Option<String>,
) -> impl IntoView {
    view! {
        <input
            type=input_type.unwrap_or("text")
            value=value
            on:input=move |ev| value.set(event_target_value(&ev))
            class=move || format!(
                "flex h-10 w-full rounded-md border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-900 px-3 py-2 text-base ring-offset-gray-950 file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-gray-900 dark:file:text-white placeholder:text-gray-500 dark:placeholder:text-gray-400 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 md:text-sm {}",
                class_name.unwrap_or_default()
            )
        />
    }
}
