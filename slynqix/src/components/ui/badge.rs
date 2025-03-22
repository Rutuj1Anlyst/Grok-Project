use leptos::*;

#[component]
pub fn Badge(
    children: Children,
    variant: Option<&'static str>, // "default", "secondary", "destructive", "outline"
    class_name: Option<String>,
) -> impl IntoView {
    let variant = variant.unwrap_or("default");

    view! {
        <div
            class=move || format!(
                "inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 {} {}",
                match variant {
                    "secondary" => "border-transparent bg-gray-500 text-white hover:bg-gray-500/80",
                    "destructive" => "border-transparent bg-red-500 text-white hover:bg-red-500/80",
                    "outline" => "text-gray-900 dark:text-white",
                    _ => "border-transparent bg-blue-500 text-white hover:bg-blue-500/80",
                },
                class_name.unwrap_or_default()
            )
        >
            {children()}
        </div>
    }
}
