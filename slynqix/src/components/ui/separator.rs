use leptos::*;

#[component]
pub fn Separator(
    orientation: Option<&'static str>,
    class_name: Option<String>,
) -> impl IntoView {
    let orientation = orientation.unwrap_or("horizontal");

    view! {
        <div
            class=move || format!(
                "shrink-0 bg-gray-300 dark:bg-gray-700 {} {}",
                if orientation == "horizontal" { "h-[1px] w-full" } else { "h-full w-[1px]" },
                class_name.unwrap_or_default()
            )
        />
    }
}
