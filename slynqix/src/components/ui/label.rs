use leptos::*;

#[component]
pub fn Label(
    children: Children,
    html_for: String,
    class_name: Option<String>,
) -> impl IntoView {
    view! {
        <label
            for=html_for
            class=move || format!(
                "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 {}",
                class_name.unwrap_or_default()
            )
        >
            {children()}
        </label>
    }
}
