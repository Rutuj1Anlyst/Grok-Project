use leptos::*;
use crate::components::ui::dialog::{Dialog, DialogContent};

#[component]
pub fn Command(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div
            class=move || format!(
                "flex h-full w-full flex-col overflow-hidden rounded-md bg-white dark:bg-gray-800 text-gray-900 dark:text-white {}",
                class_name.unwrap_or_default()
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CommandDialog(children: Children, open: ReadSignal<bool>, set_open: WriteSignal<bool>) -> impl IntoView {
    view! {
        <Dialog open=open set_open=set_open>
            <DialogContent class_name="overflow-hidden p-0 shadow-lg">
                <Command class_name="[&_[cmdk-group-heading]]:px-2 [&_[cmdk-group-heading]]:font-medium [&_[cmdk-group-heading]]:text-gray-500 [&_[cmdk-group]:not([hidden])_~[cmdk-group]]:pt-0 [&_[cmdk-group]]:px-2 [&_[cmdk-input-wrapper]_svg]:h-5 [&_[cmdk-input-wrapper]_svg]:w-5 [&_[cmdk-input]]:h-12 [&_[cmdk-item]]:px-2 [&_[cmdk-item]]:py-3 [&_[cmdk-item]_svg]:h-5 [&_[cmdk-item]_svg]:w-5">
                    {children()}
                </Command>
            </DialogContent>
        </Dialog>
    }
}

#[component]
pub fn CommandInput(value: RwSignal<String>, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class="flex items-center border-b px-3">
            <svg class="mr-2 h-4 w-4 shrink-0 opacity-50" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>
            <input
                type="text"
                value=value
                on:input=move |ev| value.set(event_target_value(&ev))
                class=move || format!(
                    "flex h-11 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder:text-gray-500 dark:placeholder:text-gray-400 disabled:cursor-not-allowed disabled:opacity-50 {}",
                    class_name.unwrap_or_default()
                )
                placeholder="Search..."
            />
        </div>
    }
}

#[component]
pub fn CommandList(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!("max-h-[300px] overflow-y-auto overflow-x-hidden {}", class_name.unwrap_or_default())>
            {children()}
        </div>
    }
}

#[component]
pub fn CommandEmpty(children: Children) -> impl IntoView {
    view! {
        <div class="py-6 text-center text-sm">
            {children()}
        </div>
    }
}

#[component]
pub fn CommandGroup(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!(
            "overflow-hidden p-1 text-gray-900 dark:text-white [&_[cmdk-group-heading]]:px-2 [&_[cmdk-group-heading]]:py-1.5 [&_[cmdk-group-heading]]:text-xs [&_[cmdk-group-heading]]:font-medium [&_[cmdk-group-heading]]:text-gray-500 {}",
            class_name.unwrap_or_default()
        )>
            {children()}
        </div>
    }
}

#[component]
pub fn CommandItem(children: Children, class_name: Option<String>, on_select: Option<Callback<()>>) -> impl IntoView {
    let (selected, set_selected) = create_signal(false);

    view! {
        <div
            class=move || format!(
                "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none {} {}",
                if selected.get() { "bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-white" } else { "" },
                class_name.unwrap_or_default()
            )
            on:click=move |_| {
                set_selected(true);
                if let Some(on_select) = on_select { on_select.call(()) }
            }
        >
            {children()}
        </div>
    }
}
