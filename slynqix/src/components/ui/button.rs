use leptos::*;

#[component]
pub fn Button(
    children: Children,
    variant: Option<&'static str>, // "default", "destructive", "outline", "secondary", "ghost", "link"
    size: Option<&'static str>,   // "default", "sm", "lg", "icon"
    class_name: Option<String>,
    on_click: Option<Callback<()>>,
) -> impl IntoView {
    let variant = variant.unwrap_or("default");
    let size = size.unwrap_or("default");

    view! {
        <button
            class=move || format!(
                "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium ring-offset-gray-950 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 {} {} {}",
                match variant {
                    "destructive" => "bg-red-500 text-white hover:bg-red-500/90",
                    "outline" => "border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-900 hover:bg-gray-100 dark:hover:bg-gray-800 hover:text-gray-900 dark:hover:text-white",
                    "secondary" => "bg-gray-500 text-white hover:bg-gray-500/80",
                    "ghost" => "hover:bg-gray-100 dark:hover:bg-gray-800 hover:text-gray-900 dark:hover:text-white",
                    "link" => "text-blue-500 underline-offset-4 hover:underline",
                    _ => "bg-blue-500 text-white hover:bg-blue-500/90",
                },
                match size {
                    "sm" => "h-9 rounded-md px-3",
                    "lg" => "h-11 rounded-md px-8",
                    "icon" => "h-10 w-10",
                    _ => "h-10 px-4 py-2",
                },
                class_name.unwrap_or_default()
            )
            on:click=move |_| if let Some(on_click) = on_click { on_click.call(()) }
        >
            {children()}
        </button>
    }
}
