use leptos::*;

#[component]
pub fn Alert(
    children: Children,
    variant: Option<&'static str>, // "default" or "destructive"
    class_name: Option<String>,
) -> impl IntoView {
    let variant = variant.unwrap_or("default");

    view! {
        <div
            role="alert"
            class=move || format!(
                "relative w-full rounded-lg border p-4 [&>svg~*]:pl-7 [&>svg+div]:translate-y-[-3px] [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg]:text-foreground {} {}",
                match variant {
                    "destructive" => "border-red-500/50 text-red-600 dark:border-red-500 [&>svg]:text-red-600",
                    _ => "bg-white dark:bg-gray-900 text-gray-900 dark:text-white",
                },
                class_name.unwrap_or_default()
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn AlertTitle(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <h5 class=move || format!("mb-1 font-medium leading-none tracking-tight {}", class_name.unwrap_or_default())>
            {children()}
        </h5>
    }
}

#[component]
pub fn AlertDescription(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!("text-sm [&_p]:leading-relaxed {}", class_name.unwrap_or_default())>
            {children()}
        </div>
    }
}
