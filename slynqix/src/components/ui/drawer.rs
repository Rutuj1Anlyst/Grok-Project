use leptos::*;

#[component]
pub fn Drawer(
    children: Children,
    open: ReadSignal<bool>,
    set_open: WriteSignal<bool>,
    class_name: Option<String>,
) -> impl IntoView {
    view! {
        <Show when=move || open.get()>
            <div class="fixed inset-0 z-50 bg-black/80">
                <div
                    class=move || format!(
                        "fixed inset-x-0 bottom-0 z-50 mt-24 flex h-auto flex-col rounded-t-[10px] border bg-white dark:bg-gray-900 {}",
                        class_name.unwrap_or_default()
                    )
                >
                    <div class="mx-auto mt-4 h-2 w-[100px] rounded-full bg-gray-200 dark:bg-gray-700" />
                    {children()}
                    <button on:click=move |_| set_open(false) class="absolute top-2 right-2">
                        "Close"
                    </button>
                </div>
            </div>
        </Show>
    }
}

#[component]
pub fn DrawerHeader(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!("grid gap-1.5 p-4 text-center sm:text-left {}", class_name.unwrap_or_default())>
            {children()}
        </div>
    }
}

#[component]
pub fn DrawerFooter(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!("mt-auto flex flex-col gap-2 p-4 {}", class_name.unwrap_or_default())>
            {children()}
        </div>
    }
}

#[component]
pub fn DrawerTitle(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <h2 class=move || format!("text-lg font-semibold leading-none tracking-tight {}", class_name.unwrap_or_default())>
            {children()}
        </h2>
    }
}

#[component]
pub fn DrawerDescription(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <p class=move || format!("text-sm text-gray-500 dark:text-gray-400 {}", class_name.unwrap_or_default())>
            {children()}
        </p>
    }
}
