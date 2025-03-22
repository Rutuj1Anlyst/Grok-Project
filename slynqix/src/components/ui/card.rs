use leptos::*;

#[component]
pub fn Card(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div
            class=move || format!(
                "rounded-lg border bg-white dark:bg-gray-900 text-gray-900 dark:text-white shadow-sm {}",
                class_name.unwrap_or_default()
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CardHeader(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!("flex flex-col space-y-1.5 p-6 {}", class_name.unwrap_or_default())>
            {children()}
        </div>
    }
}

#[component]
pub fn CardTitle(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <h3 class=move || format!("text-2xl font-semibold leading-none tracking-tight {}", class_name.unwrap_or_default())>
            {children()}
        </h3>
    }
}

#[component]
pub fn CardDescription(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <p class=move || format!("text-sm text-gray-500 dark:text-gray-400 {}", class_name.unwrap_or_default())>
            {children()}
        </p>
    }
}

#[component]
pub fn CardContent(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!("p-6 pt-0 {}", class_name.unwrap_or_default())>
            {children()}
        </div>
    }
}

#[component]
pub fn CardFooter(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!("flex items-center p-6 pt-0 {}", class_name.unwrap_or_default())>
            {children()}
        </div>
    }
}
