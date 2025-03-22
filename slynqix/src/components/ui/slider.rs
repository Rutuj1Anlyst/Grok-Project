use leptos::*;

#[component]
pub fn Slider(
    value: RwSignal<f32>,
    min: Option<f32>,
    max: Option<f32>,
    class_name: Option<String>,
) -> impl IntoView {
    let min = min.unwrap_or(0.0);
    let max = max.unwrap_or(100.0);

    view! {
        <input
            type="range"
            min=min
            max=max
            value=value
            on:input=move |ev| value.set(event_target_value(&ev).parse::<f32>().unwrap_or(min))
            class=move || format!(
                "relative flex w-full touch-none select-none items-center h-2 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden appearance-none {}",
                class_name.unwrap_or_default()
            )
            style=move || format!(
                "background: linear-gradient(to right, #3b82f6 {}%, #e5e7eb {}%)",
                (value.get() - min) / (max - min) * 100.0,
                (value.get() - min) / (max - min) * 100.0
            )
        />
    }
}
