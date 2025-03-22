use leptos::*;

pub fn toggle_variants(variant: &'static str, size: &'static str) -> String {
    format!(
        "inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-gray-950 transition-colors hover:bg-gray-200 dark:hover:bg-gray-700 hover:text-gray-500 dark:hover:text-gray-400 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 data-[state=on]:bg-gray-200 dark:data-[state=on]:bg-gray-700 data-[state=on]:text-gray-900 dark:data-[state=on]:text-white {} {}",
        match variant {
            "default" => "bg-transparent",
            "outline" => "border border-gray-300 dark:border-gray-700 bg-transparent hover:bg-gray-200 dark:hover:bg-gray-700 hover:text-gray-900 dark:hover:text-white",
            _ => "bg-transparent",
        },
        match size {
            "default" => "h-10 px-3",
            "sm" => "h-9 px-2.5",
            "lg" => "h-11 px-5",
            _ => "h-10 px-3",
        }
    )
}

#[component]
pub fn Toggle(
    value: ReadSignal<bool>,
    set_value: WriteSignal<bool>,
    variant: Option<&'static str>,
    size: Option<&'static str>,
    class_name: Option<String>,
) -> impl IntoView {
    let variant = variant.unwrap_or("default");
    let size = size.unwrap_or("default");

    view! {
        <button
            class=move || format!(
                "{} {}",
                toggle_variants(variant, size),
                class_name.unwrap_or_default()
            )
            data-state=move || if value.get() { "on" } else { "off" }
            on:click=move |_| set_value.update(|val| *val = !*val)
        >
            {move || if value.get() { "On" } else { "Off" }}
        </button>
    }
}
