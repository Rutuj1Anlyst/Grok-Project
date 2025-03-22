use leptos::*;

#[component]
pub fn Tabs(
    value: RwSignal<String>,
    tabs: Vec<(String, String, Children)>, // (value, label, content)
    class_name: Option<String>,
) -> impl IntoView {
    view! {
        <div class=class_name.unwrap_or_default()>
            <div
                class="inline-flex h-10 items-center justify-center rounded-md bg-gray-200 dark:bg-gray-700 p-1 text-gray-500 dark:text-gray-400"
            >
                {tabs.iter().map(|(tab_value, label, _)| {
                    view! {
                        <button
                            class=move || format!(
                                "inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium ring-offset-gray-950 transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 {}",
                                if value.get() == *tab_value { "bg-white dark:bg-gray-900 text-gray-900 dark:text-white shadow-sm" } else { "" }
                            )
                            on:click=move |_| value.set(tab_value.clone())
                        >
                            {label.clone()}
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>
            <div class="mt-2">
                {move || tabs.iter().find(|(tab_value, _, _)| value.get() == *tab_value).map(|(_, _, content)| content()).unwrap_or_default()}
            </div>
        </div>
    }
}
