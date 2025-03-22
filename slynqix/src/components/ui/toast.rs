use leptos::*;

#[component]
pub fn ToastProvider(children: Children) -> impl IntoView {
    view! {
        {children()}
    }
}

#[component]
pub fn ToastViewport(class_name: Option<String>) -> impl IntoView {
    view! {
        <div
            class=move || format!(
                "fixed top-0 z-[100] flex max-h-screen w-full flex-col-reverse p-4 sm:bottom-0 sm:right-0 sm:top-auto sm:flex-col md:max-w-[420px] {}",
                class_name.unwrap_or_default()
            )
        />
    }
}

#[component]
pub fn Toast(
    open: ReadSignal<bool>,
    set_open: WriteSignal<bool>,
    variant: Option<&'static str>,
    children: Children,
    class_name: Option<String>,
) -> impl IntoView {
    let variant = variant.unwrap_or("default");

    view! {
        <Show when=move || open.get()>
            <div
                class=move || format!(
                    "group pointer-events-auto relative flex w-full items-center justify-between space-x-4 overflow-hidden rounded-md border p-6 pr-8 shadow-lg transition-all animate-in slide-in-from-top-full sm:slide-in-from-bottom-full {} {}",
                    match variant {
                        "default" => "border bg-white dark:bg-gray-900 text-gray-900 dark:text-white",
                        "destructive" => "border-red-600 bg-red-600 text-white",
                        _ => "border bg-white dark:bg-gray-900 text-gray-900 dark:text-white",
                    },
                    class_name.unwrap_or_default()
                )
            >
                {children()}
                <button
                    class="absolute right-2 top-2 rounded-md p-1 text-gray-500 dark:text-gray-400 opacity-0 transition-opacity hover:text-gray-900 dark:hover:text-white focus:outline-none focus:ring-2 group-hover:opacity-100"
                    on:click=move |_| set_open(false)
                >
                    <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>
            </div>
        </Show>
    }
}

#[component]
pub fn ToastTitle(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!("text-sm font-semibold {}", class_name.unwrap_or_default())>
            {children()}
        </div>
    }
}

#[component]
pub fn ToastDescription(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!("text-sm opacity-90 {}", class_name.unwrap_or_default())>
            {children()}
        </div>
    }
}
