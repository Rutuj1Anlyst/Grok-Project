use leptos::*;

#[component]
pub fn Progress(value: ReadSignal<f32>, class_name: Option<String>) -> impl IntoView {
    view! {
        <div
            class=move || format!(
                "relative h-4 w-full overflow-hidden rounded-full bg-gray-200 dark:bg-gray-700 {}",
                class_name.unwrap_or_default()
            )
        >
            <div
                class="h-full w-full flex-1 bg-blue-500 transition-all"
                style=move || format!("transform: translateX(-{}%)", 100.0 - value.get())
            />
        </div>
    }
}
