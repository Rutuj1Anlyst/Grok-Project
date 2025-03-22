use leptos::*;

#[component]
pub fn ToggleGroup(
    value: RwSignal<String>,
    options: Vec<(String, String)>, // (value, label)
    variant: Option<&'static str>,
    size: Option<&'static str>,
    class_name: Option<String>,
) -> impl IntoView {
    let variant = variant.unwrap_or("default");
    let size = size.unwrap_or("default");

    view! {
        <div class=move || format!("flex items-center justify-center gap-1 {}", class_name.unwrap_or_default())>
            {options.into_iter().map(|(opt_value, label)| {
                let is_on = move || value.get() == opt_value;
                view! {
                    <button
                        class=move || format!(
                            "inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-gray-950 transition-colors hover:bg-gray-200 dark:hover:bg-gray-700 hover:text-gray-500 dark:hover:text-gray-400 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 {} {} {}",
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
                            },
                            if is_on() { "bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white" } else { "" }
                        )
                        on:click=move |_| value.set(opt_value.clone())
                    >
                        {label}
                    </button>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
