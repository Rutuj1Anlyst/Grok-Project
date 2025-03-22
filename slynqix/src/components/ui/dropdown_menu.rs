use leptos::*;

#[component]
pub fn DropdownMenu(
    children: Children,
    trigger: Children,
    open: ReadSignal<bool>,
    set_open: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <div>
            <button on:click=move |_| set_open.update(|val| *val = !*val)>
                {trigger()}
            </button>
            <Show when=move || open.get()>
                <div
                    class="z-50 min-w-[8rem] overflow-hidden rounded-md border bg-white dark:bg-gray-800 p-1 text-gray-900 dark:text-white shadow-md animate-in fade-in-0 zoom-in-95"
                >
                    {children()}
                </div>
            </Show>
        </div>
    }
}

#[component]
pub fn DropdownMenuItem(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div
            class=move || format!(
                "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors hover:bg-gray-100 dark:hover:bg-gray-700 hover:text-gray-900 dark:hover:text-white {}",
                class_name.unwrap_or_default()
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuSeparator(class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!("-mx-1 my-1 h-px bg-gray-200 dark:bg-gray-700 {}", class_name.unwrap_or_default()) />
    }
}

#[component]
pub fn DropdownMenuLabel(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!("px-2 py-1.5 text-sm font-semibold {}", class_name.unwrap_or_default())>
            {children()}
        </div>
    }
}
