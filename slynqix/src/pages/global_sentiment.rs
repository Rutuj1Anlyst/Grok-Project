// slynqix/src/pages/global_sentiment.rs
use leptos::*;
use crate::components::ComingSoon;

#[component]
pub fn GlobalSentiment() -> impl IntoView {
    view! {
        <ComingSoon
            title="Global Sentiment"
            description="Analyze news from global platforms to determine market sentiment"
            is_admin=true
        />
    }
}
