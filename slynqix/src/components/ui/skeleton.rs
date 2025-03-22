use leptos::*;

#[component]
pub fn Skeleton(class_name: Option<String>) -> impl IntoView {
    view! {
        <div
            class=move || format!(
                "animate-pulse rounded-md bg-gray-200 dark:bg-gray-700 {}",
                class_name.unwrap_or_default()
            )
        />
    }
}
