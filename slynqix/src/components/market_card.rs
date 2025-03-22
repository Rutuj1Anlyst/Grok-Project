use leptos::*;

#[component]
pub fn MarketCard(symbol: String, price: f64, change: f64, volume: f64) -> impl IntoView {
    let is_positive = change >= 0.0;

    view! {
        <div class="market-card animate-slide-in-up" style=format!("animation-delay: {}s", rand::random::<f32>() * 0.5)>
            <div class="flex justify-between items-start mb-4">
                <h3 class="text-lg font-medium">{symbol}</h3>
                <div class=move || format!(
                    "flex items-center text-sm px-2 py-1 rounded-full {}",
                    if is_positive {
                        "bg-green-100 dark:bg-green-900/30 text-green-600 dark:text-green-400"
                    } else {
                        "bg-red-100 dark:bg-red-900/30 text-red-600 dark:text-red-400"
                    }
                )>
                    {move || if is_positive {
                        view! { <svg class="h-3 w-3 mr-1" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 19V5m-7 7l7-7 7 7"/></svg> }
                    } else {
                        view! { <svg class="h-3 w-3 mr-1" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 5v14m7-7l-7 7-7-7"/></svg> }
                    }}
                    {format!("{:.2}%", change.abs())}
                </div>
            </div>
            <div class="mb-4">
                <div class="text-3xl font-light tracking-tight">
                    {format!("{:.2}", price)}
                </div>
                <div class="text-sm text-gray-500 dark:text-gray-400 mt-1">
                    {format!("Vol: {:.2}M", volume / 1_000_000.0)}
                </div>
            </div>
            <div class="h-16 relative overflow-hidden rounded-lg bg-gray-100 dark:bg-gray-900">
                <div class="absolute inset-0 flex items-end">
                    <div class=move || format!(
                        "h-10 w-full {}",
                        if is_positive {
                            "bg-gradient-to-r from-green-100 to-green-300 dark:from-green-900/30 dark:to-green-700/30"
                        } else {
                            "bg-gradient-to-r from-red-100 to-red-300 dark:from-red-900/30 dark:to-red-700/30"
                        }
                    )>
                        <div class="h-full w-full absolute top-0 left-0 bg-grid-white/10"></div>
                    </div>
                </div>
            </div>
        </div>
    }
}
