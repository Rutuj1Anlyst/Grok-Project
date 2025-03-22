use leptos::*;

#[component]
pub fn NavigationMenu(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div
            class=move || format!(
                "relative z-10 flex max-w-max flex-1 items-center justify-center {}",
                class_name.unwrap_or_default()
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn NavigationMenuList(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <ul
            class=move || format!(
                "group flex flex-1 list-none items-center justify-center space-x-1 {}",
                class_name.unwrap_or_default()
            )
        >
            {children()}
        </ul>
    }
}

#[component]
pub fn NavigationMenuItem(children: Children, trigger: Children, class_name: Option<String>) -> impl IntoView {
    let (open, set_open) = create_signal(false);

    view! {
        <li class="relative">
            <button
                class=move || format!(
                    "group inline-flex h-10 w-max items-center justify-center rounded-md bg-white dark:bg-gray-900 px-4 py-2 text-sm font-medium transition-colors hover:bg-gray-100 dark:hover:bg-gray-700 hover:text-gray-900 dark:hover:text-white focus:bg-gray-100 dark:focus:bg-gray-700 focus:text-gray-900 dark:focus:text-white focus:outline-none disabled:pointer-events-none disabled:opacity-50 {} {}",
                    if open.get() { "bg-gray-100/50 dark:bg-gray-700/50" } else { "" },
                    class_name.unwrap_or_default()
                )
                on:click=move |_| set_open.update(|val| *val = !*val)
            >
                {trigger()}
                <svg
                    class=move || format!(
                        "relative top-[1px] ml-1 h-3 w-3 transition duration-200 {}",
                        if open.get() { "rotate-180" } else { "" }
                    )
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                >
                    <path d="M6 9l6 6 6-6" />
                </svg>
            </button>
            <Show when=move || open.get()>
                <div
                    class="absolute left-0 top-full mt-1.5 w-full overflow-hidden rounded-md border bg-white dark:bg-gray-800 text-gray-900 dark:text-white shadow-lg animate-in fade-in-0 zoom-in-90"
                >
                    {children()}
                </div>
            </Show>
        </li>
    }
}
