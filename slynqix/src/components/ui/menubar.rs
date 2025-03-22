use leptos::*;

#[component]
pub fn Menubar(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div
            class=move || format!(
                "flex h-10 items-center space-x-1 rounded-md border bg-white dark:bg-gray-900 p-1 {}",
                class_name.unwrap_or_default()
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn MenubarMenu(children: Children, trigger: Children, class_name: Option<String>) -> impl IntoView {
    let (open, set_open) = create_signal(false);

    view! {
        <div class="relative">
            <button
                class=move || format!(
                    "flex cursor-default select-none items-center rounded-sm px-3 py-1.5 text-sm font-medium outline-none {} {}",
                    if open.get() { "bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-white" } else { "" },
                    class_name.unwrap_or_default()
                )
                on:click=move |_| set_open.update(|val| *val = !*val)
            >
                {trigger()}
            </button>
            <Show when=move || open.get()>
                <div
                    class="z-50 min-w-[12rem] overflow-hidden rounded-md border bg-white dark:bg-gray-800 p-1 text-gray-900 dark:text-white shadow-md animate-in fade-in-0 zoom-in-95 slide-in-from-top-2"
                >
                    {children()}
                </div>
            </Show>
        </div>
    }
}

#[component]
pub fn MenubarItem(children: Children, class_name: Option<String>) -> impl IntoView {
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
pub fn MenubarSeparator(class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!("-mx-1 my-1 h-px bg-gray-200 dark:bg-gray-700 {}", class_name.unwrap_or_default()) />
    }
}
