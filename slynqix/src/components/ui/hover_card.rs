use leptos::*;

#[component]
pub fn HoverCard(
    children: Children,
    trigger: Children,
    class_name: Option<String>,
) -> impl IntoView {
    let (is_open, set_is_open) = create_signal(false);

    view! {
        <div
            on:mouseover=move |_| set_is_open(true)
            on:mouseout=move |_| set_is_open(false)
        >
            {trigger()}
            <Show when=move || is_open.get()>
                <div
                    class=move || format!(
                        "z-50 w-64 rounded-md border bg-white dark:bg-gray-800 p-4 text-gray-900 dark:text-white shadow-md outline-none animate-in fade-in-0 zoom-in-95 slide-in-from-top-2 {}",
                        class_name.unwrap_or_default()
                    )
                >
                    {children()}
                </div>
            </Show>
        </div>
    }
}
