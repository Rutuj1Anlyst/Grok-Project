use leptos::*;

#[component]
pub fn AlertDialog(children: Children, open: ReadSignal<bool>) -> impl IntoView {
    view! {
        <Show when=move || open.get()>
            {children()}
        </Show>
    }
}

#[component]
pub fn AlertDialogTrigger(children: Children, on_click: Callback<()>) -> impl IntoView {
    view! {
        <button on:click=move |_| on_click.call(())>
            {children()}
        </button>
    }
}

#[component]
pub fn AlertDialogOverlay(class_name: Option<String>) -> impl IntoView {
    view! {
        <div
            class=move || format!(
                "fixed inset-0 z-50 bg-black/80 animate-in fade-in-0 {}",
                class_name.unwrap_or_default()
            )
        />
    }
}

#[component]
pub fn AlertDialogContent(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div>
            <AlertDialogOverlay />
            <div
                class=move || format!(
                    "fixed left-[50%] top-[50%] z-50 grid w-full max-w-lg translate-x-[-50%] translate-y-[-50%] gap-4 border bg-white dark:bg-gray-900 p-6 shadow-lg duration-200 animate-in fade-in-0 zoom-in-95 slide-in-from-left-1/2 slide-in-from-top-[48%] sm:rounded-lg {}",
                    class_name.unwrap_or_default()
                )
            >
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn AlertDialogHeader(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!("flex flex-col space-y-2 text-center sm:text-left {}", class_name.unwrap_or_default())>
            {children()}
        </div>
    }
}

#[component]
pub fn AlertDialogFooter(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!("flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2 {}", class_name.unwrap_or_default())>
            {children()}
        </div>
    }
}

#[component]
pub fn AlertDialogTitle(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <h2 class=move || format!("text-lg font-semibold {}", class_name.unwrap_or_default())>
            {children()}
        </h2>
    }
}

#[component]
pub fn AlertDialogDescription(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <p class=move || format!("text-sm text-gray-500 dark:text-gray-400 {}", class_name.unwrap_or_default())>
            {children()}
        </p>
    }
}

#[component]
pub fn AlertDialogAction(children: Children, class_name: Option<String>, on_click: Callback<()>) -> impl IntoView {
    view! {
        <button
            class=move || format!("px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 {}", class_name.unwrap_or_default())
            on:click=move |_| on_click.call(())
        >
            {children()}
        </button>
    }
}

#[component]
pub fn AlertDialogCancel(children: Children, class_name: Option<String>, on_click: Callback<()>) -> impl IntoView {
    view! {
        <button
            class=move || format!("px-4 py-2 border border-gray-300 dark:border-gray-700 rounded hover:bg-gray-100 dark:hover:bg-gray-900 mt-2 sm:mt-0 {}", class_name.unwrap_or_default())
            on:click=move |_| on_click.call(())
        >
            {children()}
        </button>
    }
}
