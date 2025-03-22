use leptos::*;

#[component]
pub fn Popover(
    children: Children,
    trigger: Children,
    open: ReadSignal<bool>,
    set_open: WriteSignal<bool>,
    class_name: Option<String>,
) -> impl IntoView {
    view! {
        <div>
            <button on:click=move |_| set_open.update(|val| *val = !*val)>
                {trigger()}
            </button>
            <Show when=move || open.get()>
                <div
                    class=move || format!(
                        "z-50 w-72 rounded-md border bg-white dark:bg-gray-800 p-4 text-gray-900 dark:text-white shadow-md outline-none animate-in fade-in-0 zoom-in-95 slide-in-from-top-2 {}",
                        class_name.unwrap_or_default()
                    )
                >
                    {children()}
                </div>
            </Show>
        </div>
    }
}
