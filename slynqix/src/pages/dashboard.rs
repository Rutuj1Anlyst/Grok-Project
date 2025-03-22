// slynqix/src/pages/dashboard.rs
use leptos::*;
use leptos_router::*;
use leptos_icons::{Icon, LuActivity, LuArrowRight, LuBarChart2, LuBookOpen, LuBrain, LuClock, LuTrendingUp};
use crate::components::{Button, MarketCard, StatCard};

#[derive(Clone)]
struct MarketData {
    symbol: String,
    price: f64,
    change: f64,
    volume: i32,
}

#[component]
pub fn Dashboard() -> impl IntoView {
    let dummy_market_data = vec![
        MarketData { symbol: "Nifty 50".to_string(), price: 22456.78, change: 0.87, volume: 1235678 },
        MarketData { symbol: "Banknifty".to_string(), price: 48967.21, change: -0.32, volume: 987654 },
        MarketData { symbol: "Finnifty".to_string(), price: 21345.67, change: 1.23, volume: 876543 },
        MarketData { symbol: "Sensex".to_string(), price: 74321.45, change: 0.54, volume: 2345678 },
        MarketData { symbol: "Midcap".to_string(), price: 38765.43, change: -0.76, volume: 765432 },
        MarketData { symbol: "Bitcoin".to_string(), price: 60123.45, change: 2.15, volume: 4567890 },
        MarketData { symbol: "Gold".to_string(), price: 2371.82, change: 0.42, volume: 567890 },
    ];

    view! {
        <div class="page-transition pt-20 pb-6 px-4">
            <div class="max-w-7xl mx-auto">
                <header class="mb-8">
                    <h1 class="text-3xl font-semibold tracking-tight">"Dashboard"</h1>
                    <p class="text-gray-600 dark:text-gray-400 mt-2">"Welcome to Slynqix, your all-in-one market analysis platform"</p>
                </header>

                <section class="mb-10">
                    <h2 class="text-xl font-medium mb-4">"Market Overview"</h2>
                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-5 gap-4">
                        {dummy_market_data.into_iter().map(|market| {
                            view! {
                                <MarketCard
                                    symbol=market.symbol
                                    price=market.price
                                    change=market.change
                                    volume=market.volume
                                />
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </section>

                <section class="mb-10">
                    <h2 class="text-xl font-medium mb-4">"Market Statistics"</h2>
                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                        <StatCard
                            title="Market Volatility"
                            value="Medium"
                            icon=view! { <Icon icon=Icon::from(LuActivity) class="h-4 w-4 text-accent"/> }
                            description="Based on VIX and price movements"
                        />
                        <StatCard
                            title="Trend Direction"
                            value="Bullish"
                            trend="up"
                            icon=view! { <Icon icon=Icon::from(LuTrendingUp) class="h-4 w-4 text-green-500"/> }
                            description="Short-term market movement"
                        />
                        <StatCard
                            title="Average Volume"
                            value="1.2M"
                            icon=view! { <Icon icon=Icon::from(LuTrendingUp) class="h-4 w-4 text-accent"/> }
                            description="Across Nifty50 stocks"
                        />
                        <StatCard
                            title="Trading Hours"
                            value="3:05:32"
                            icon=view! { <Icon icon=Icon::from(LuClock) class="h-4 w-4 text-accent"/> }
                            description="Time remaining in session"
                        />
                    </div>
                </section>

                <section>
                    <h2 class="text-xl font-medium mb-4">"Quick Access"</h2>
                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
                        {vec![
                            ("Market Console", "Analyze market conditions and get trading suggestions based on statistical analysis.", Icon::from(LuBarChart2), "/console"),
                            ("Mindsage", "Get investment guidance considering fundamentals, technicals, and expected returns.", Icon::from(LuBrain), "/mindsage"),
                            ("Trade Journal", "Record and analyze your trades with comprehensive journaling features.", Icon::from(LuBookOpen), "/journal"),
                        ].into_iter().enumerate().map(|(index, (title, description, icon, link))| {
                            view! {
                                <div
                                    class="glass-card rounded-2xl p-6 animate-slide-in-up flex flex-col h-full"
                                    style=format!("animation-delay: {}s", 0.1 * (index as f32 + 1.0))
                                >
                                    <div class="p-2 rounded-full bg-accent/10 w-fit mb-4">
                                        <Icon icon=icon class="h-6 w-6 text-accent"/>
                                    </div>
                                    <h3 class="text-xl font-medium mb-2">{title}</h3>
                                    <p class="text-gray-600 dark:text-gray-400 mb-4 flex-grow">{description}</p>
                                    <Button variant="ghost" class="justify-start pl-0 hover:pl-2 transition-all w-fit">
                                        <A href=link>
                                            "Access now"
                                            <Icon icon=Icon::from(LuArrowRight) class="ml-2 h-4 w-4"/>
                                        </A>
                                    </Button>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </section>
            </div>
        </div>
    }
}
