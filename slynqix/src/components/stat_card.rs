use leptos::*;

#[component]
pub fn StatCard(
    title: String,
    value: String,
    icon: String, // SVG path or placeholder for now
    trend: Option<&'static str>, // "up", "down", "neutral"
    description: Option<String>,
) -> impl IntoView {
    let trend = trend.unwrap_or("neutral");

    view! {
        <div class="stat-card animate-slide-in-up" style=format!("animation-delay: {}s", rand::random::<f32>() * 0.5)>
            <div class="flex items-center justify-between mb-2">
                <h3 class="text-sm font-medium text-gray-600 dark:text-gray-300">{title}</h3>
                <div class="p-1.5 rounded-full bg-gray-100 dark:bg-gray-800">
                    <svg class="h-5 w-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d={icon} />
                    </svg>
                </div>
            </div>
            <div class="flex items-baseline gap-2">
                <div class="text-2xl font-medium tracking-tight">{value}</div>
                {move || if trend != "neutral" {
                    view! {
                        <div class=move || format!(
                            "text-xs font-medium {}",
                            if trend == "up" { "text-green-600 dark:text-green-400" } else { "text-red-600 dark:text-red-400" }
                        )>
                            {if trend == "up" { "↑" } else { "↓" }}
                        </div>
                    }
                } else {
                    None
                }}
            </div>
            {move || description.as_ref().map(|desc| {
                view! {
                    <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">{desc}</p>
                }
            })}
        </div>
    }
}
