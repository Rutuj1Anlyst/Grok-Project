use leptos::*;

#[component]
pub fn Sheet(
    children: Children,
    open: ReadSignal<bool>,
    set_open: WriteSignal<bool>,
    side: Option<&'static str>,
    class_name: Option<String>,
) -> impl IntoView {
    let side = side.unwrap_or("right");

    view! {
        <Show when=move || open.get()>
            <div class="fixed inset-0 z-50 bg-black/80 animate-in fade-in-0">
                <div
                    class=move || format!(
                        "fixed z-50 gap-4 bg-white dark:bg-gray-900 p-6 shadow-lg transition ease-in-out animate-in {} {} {}",
                        match side {
                            "top" => "inset-x-0 top-0 border-b slide-in-from-top",
                            "bottom" => "inset-x-0 bottom-0 border-t slide-in-from-bottom",
                            "left" => "inset-y-0 left-0 h-full w-3/4 border-r slide-in-from-left sm:max-w-sm",
                            "right" => "inset-y-0 right-0 h-full w-3/4 border-l slide-in-from-right sm:max-w-sm",
                            _ => "inset-y-0 right-0 h-full w-3/4 border-l slide-in-from-right sm:max-w-sm",
                        },
                        class_name.unwrap_or_default()
                    )
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
pub fn SheetHeader(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!("flex flex-col space-y-2 text-center sm:text-left {}", class_name.unwrap_or_default())>
            {children()}
        </div>
    }
}

#[component]
pub fn SheetFooter(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!("flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2 {}", class_name.unwrap_or_default())>
            {children()}
        </div>
    }
}

#[component]
pub fn SheetTitle(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <h2 class=move || format!("text-lg font-semibold text-gray-900 dark:text-white {}", class_name.unwrap_or_default())>
            {children()}
        </h2>
    }
}

#[component]
pub fn SheetDescription(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <p class=move || format!("text-sm text-gray-500 dark:text-gray-400 {}", class_name.unwrap_or_default())>
            {children()}
        </p>
    }
}
