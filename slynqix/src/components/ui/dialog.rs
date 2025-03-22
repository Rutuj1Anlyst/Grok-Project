use leptos::*;

#[component]
pub fn Dialog(
    children: Children,
    open: ReadSignal<bool>,
    set_open: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <Show when=move || open.get()>
            <div class="fixed inset-0 z-50 bg-black/80 animate-in fade-in-0">
                <div
                    class="fixed left-[50%] top-[50%] z-50 grid w-full max-w-lg translate-x-[-50%] translate-y-[-50%] gap-4 border bg-white dark:bg-gray-900 p-6 shadow-lg duration-200 animate-in fade-in-0 zoom-in-95 slide-in-from-left-1/2 slide-in-from-top-[48%] sm:rounded-lg"
                >
                    {children()}
                    <button
                        class="absolute right-4 top-4 rounded-sm opacity-70 ring-offset-gray-950 transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                        on:click=move |_| set_open(false)
                    >
                        <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M6 18L18 6M6 6l12 12" />
                        </svg>
                        <span class="sr-only">"Close"</span>
                    </button>
                </div>
            </div>
        </Show>
    }
}

#[component]
pub fn DialogContent(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!(
            "fixed left-[50%] top-[50%] z-50 grid w-full max-w-lg translate-x-[-50%] translate-y-[-50%] gap-4 border bg-white dark:bg-gray-900 p-6 shadow-lg duration-200 animate-in fade-in-0 zoom-in-95 slide-in-from-left-1/2 slide-in-from-top-[48%] sm:rounded-lg {}",
            class_name.unwrap_or_default()
        )>
            {children()}
        </div>
    }
}

#[component]
pub fn DialogHeader(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!("flex flex-col space-y-1.5 text-center sm:text-left {}", class_name.unwrap_or_default())>
            {children()}
        </div>
    }
}

#[component]
pub fn DialogTitle(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <h2 class=move || format!("text-lg font-semibold leading-none tracking-tight {}", class_name.unwrap_or_default())>
            {children()}
        </h2>
    }
}
