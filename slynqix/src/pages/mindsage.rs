// slynqix/src/pages/mindsage.rs
use leptos::*;
use leptos_icons::{Icon, LuArrowRight, LuBarChart2, LuBrain, LuLineChart};
use crate::components::{Button, ComingSoon};

#[component]
pub fn Mindsage() -> impl IntoView {
    let (is_admin, _set_is_admin) = create_signal(true); // For demo purposes

    view! {
        {move || if !is_admin.get() {
            view! {
                <ComingSoon
                    title="Mindsage"
                    description="Investment guide based on stocks, commodities, and assets"
                    is_admin=false
                />
            }.into_view()
        } else {
            view! {
                <div class="page-transition pt-20 pb-6 px-4">
                    <div class="max-w-7xl mx-auto">
                        <header class="mb-8">
                            <h1 class="text-3xl font-semibold tracking-tight">"Mindsage"</h1>
                            <p class="text-gray-600 dark:text-gray-400 mt-2">"Investment guide based on stocks, commodities, and assets"</p>
                        </header>

                        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
                            <div class="glass-card rounded-xl p-6 animate-fade-in">
                                <div class="p-2 rounded-full bg-accent/10 w-fit mb-4">
                                    <Icon icon=Icon::from(LuBrain) class="h-5 w-5 text-accent"/>
                                </div>
                                <h2 class="text-xl font-medium mb-3">"Market Analysis"</h2>
                                <p class="text-gray-600 dark:text-gray-400 mb-4">"Deep analysis of market trends and potential opportunities based on historical data."</p>
                                <Button class="bg-accent hover:bg-accent/90 rounded-full w-full">"Generate Analysis"</Button>
                            </div>
                            <div class="glass-card rounded-xl p-6 animate-fade-in">
                                <div class="p-2 rounded-full bg-accent/10 w-fit mb-4">
                                    <Icon icon=Icon::from(LuLineChart) class="h-5 w-5 text-accent"/>
                                </div>
                                <h2 class="text-xl font-medium mb-3">"Prediction Models"</h2>
                                <p class="text-gray-600 dark:text-gray-400 mb-4">"Advanced AI-driven prediction models for forecasting market movements and price targets."</p>
                                <Button class="bg-accent hover:bg-accent/90 rounded-full w-full">"Run Predictions"</Button>
                            </div>
                            <div class="glass-card rounded-xl p-6 animate-fade-in">
                                <div class="p-2 rounded-full bg-accent/10 w-fit mb-4">
                                    <Icon icon=Icon::from(LuBarChart2) class="h-5 w-5 text-accent"/>
                                </div>
                                <h2 class="text-xl font-medium mb-3">"Trading Strategy"</h2>
                                <p class="text-gray-600 dark:text-gray-400 mb-4">"Personalized trading strategies based on risk profile and market conditions."</p>
                                <Button class="bg-accent hover:bg-accent/90 rounded-full w-full">"Generate Strategy"</Button>
                            </div>
                        </div>

                        <div class="glass-card rounded-xl p-6 animate-fade-in">
                            <h2 class="text-xl font-medium mb-4">"Recent Analyses"</h2>
                            <div class="space-y-4">
                                {vec![
                                    ("Market Volatility Report", "Generated on March 20, 2025"),
                                    ("Sector Performance Analysis", "Generated on March 18, 2025"),
                                    ("Technical Indicators Report", "Generated on March 15, 2025"),
                                ].into_iter().map(|(title, date)| {
                                    view! {
                                        <div class="p-4 rounded-xl border border-gray-200 dark:border-gray-800 hover:border-accent transition-colors">
                                            <div class="flex items-center justify-between">
                                                <div>
                                                    <h3 class="font-medium">{title}</h3>
                                                    <p class="text-sm text-gray-600 dark:text-gray-400">{date}</p>
                                                </div>
                                                <Button variant="ghost" size="sm" class="flex items-center gap-1">
                                                    "View Report"
                                                    <Icon icon=Icon::from(LuArrowRight) class="h-4 w-4"/>
                                                </Button>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    </div>
                </div>
            }.into_view()
        }}
    }
}
