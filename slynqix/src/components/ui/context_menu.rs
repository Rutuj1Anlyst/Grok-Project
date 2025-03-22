use leptos::*;

#[component]
pub fn ContextMenu(children: Children, trigger: Children) -> impl IntoView {
    let (open, set_open) = create_signal(false);

    view! {
        <div on:contextmenu=move |ev| {
            ev.prevent_default();
            set_open(true);
        }>
            {trigger()}
            <Show when=move || open.get()>
                <div
                    class="z-50 min-w-[8rem] overflow-hidden rounded-md border bg-white dark:bg-gray-800 p-1 text-gray-900 dark:text-white shadow-md animate-in fade-in-80"
                    on:mouseleave=move |_| set_open(false)
                >
                    {children()}
                </div>
            </Show>
        </div>
    }
}

#[component]
pub fn ContextMenuItem(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div
            class=move || format!(
                "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none hover:bg-gray-100 dark:hover:bg-gray-700 hover:text-gray-900 dark:hover:text-white {}",
                class_name.unwrap_or_default()
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ContextMenuSeparator(class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!("-mx-1 my-1 h-px bg-gray-500 {}", class_name.unwrap_or_default()) />
    }
}
