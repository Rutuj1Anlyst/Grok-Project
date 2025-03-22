use leptos::*;

#[component]
pub fn Tooltip(
    trigger: Children,
    content: Children,
    class_name: Option<String>,
) -> impl IntoView {
    let (open, set_open) = create_signal(false);

    view! {
        <div
            on:mouseover=move |_| set_open(true)
            on:mouseout=move |_| set_open(false)
        >
            {trigger()}
            <Show when=move || open.get()>
                <div
                    class=move || format!(
                        "z-50 overflow-hidden rounded-md border bg-white dark:bg-gray-800 px-3 py-1.5 text-sm text-gray-900 dark:text-white shadow-md animate-in fade-in-0 zoom-in-95 slide-in-from-bottom-2 {}",
                        class_name.unwrap_or_default()
                    )
                >
                    {content()}
                </div>
            </Show>
        </div>
    }
}
