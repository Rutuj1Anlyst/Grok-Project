use leptos::*;

#[component]
pub fn Avatar(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!(
            "relative flex h-10 w-10 shrink-0 overflow-hidden rounded-full {}",
            class_name.unwrap_or_default()
        )>
            {children()}
        </div>
    }
}

#[component]
pub fn AvatarImage(src: String, alt: String, class_name: Option<String>) -> impl IntoView {
    view! {
        <img
            src=src
            alt=alt
            class=move || format!("aspect-square h-full w-full {}", class_name.unwrap_or_default())
        />
    }
}

#[component]
pub fn AvatarFallback(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!(
            "flex h-full w-full items-center justify-center rounded-full bg-gray-200 dark:bg-gray-700 {}",
            class_name.unwrap_or_default()
        )>
            {children()}
        </div>
    }
}
