// slynqix/src/pages/algo_trading.rs
use leptos::*;
use crate::components::ComingSoon;

#[component]
pub fn AlgoTrading() -> impl IntoView {
    view! {
        <ComingSoon
            title="Algo Trading"
            description="Automate trades using sophisticated algorithms and voice commands"
            is_admin=true
        />
    }
}
