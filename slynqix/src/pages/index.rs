// slynqix/src/pages/index.rs
use leptos::*;
use leptos_router::*;
use leptos_icons::{Icon, LuArrowRight, LuChevronDown};
use crate::components::Button;

#[component]
pub fn Index() -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col bg-gray-950 text-white">
            <header class="fixed top-0 left-0 right-0 z-50 backdrop-blur-md bg-gray-950/80 border-b border-gray-800">
                <div class="container mx-auto h-16 flex items-center justify-between px-4">
                    <div class="flex items-center">
                        <h1 class="text-xl font-semibold tracking-tighter text-white bg-clip-text">
                            <span class="text-violet-500">"S"</span>"lynqix"
                        </h1>
                    </div>
                    <div class="flex items-center space-x-4">
                        <A href="/dashboard" class="text-sm font-medium px-4 py-2 rounded-full text-gray-300 hover:text-white transition-colors">"Dashboard"</A>
                        <Button class="bg-violet-600 hover:bg-violet-700 rounded-full">
                            <A href="/dashboard">"Login"</A>
                        </Button>
                    </div>
                </div>
            </header>

            <main class="flex-1 pt-28">
                <section class="container mx-auto px-4 pt-16 md:pt-24 text-center">
                    <div class="max-w-4xl mx-auto space-y-4 animate-fade-in">
                        <h1 class="text-4xl md:text-6xl font-semibold tracking-tight leading-tight text-balance">
                            "Market Analysis," <br/>
                            <span class="text-violet-500">"Elegantly Refined"</span>
                        </h1>
                        <p class="text-xl text-gray-400 max-w-2xl mx-auto mt-6 text-balance">
                            "Slynqix offers sophisticated market analysis for Indian indices with a beautifully intuitive interface."
                        </p>
                        <div class="flex flex-col sm:flex-row gap-4 justify-center mt-10">
                            <Button size="lg" class="bg-violet-600 hover:bg-violet-700 rounded-full py-6 px-8">
                                <A href="/dashboard">
                                    "Get Started"
                                    <Icon icon=Icon::from(LuArrowRight) class="ml-2 h-5 w-5"/>
                                </A>
                            </Button>
                            <Button size="lg" variant="outline" class="border-gray-700 text-gray-300 hover:bg-gray-800 rounded-full py-6 px-8">
                                <a href="#features">
                                    "Explore Features"
                                    <Icon icon=Icon::from(LuChevronDown) class="ml-2 h-5 w-5"/>
                                </a>
                            </Button>
                        </div>
                    </div>
                    <div class="mt-20 mb-10 relative overflow-hidden rounded-3xl h-80 sm:h-96 md:h-[500px] max-w-6xl mx-auto animate-fade-in border border-gray-800 bg-gradient-to-r from-violet-950/50 to-gray-900/50" style="animation-delay: 0.3s">
                        <div class="absolute inset-0 bg-[url('data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjAwIiBoZWlnaHQ9IjIwMCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48ZGVmcz48cGF0dGVybiBpZD0iZ3JpZCIgd2lkdGg9IjIwIiBoZWlnaHQ9IjIwIiBwYXR0ZXJuVW5pdHM9InVzZXJTcGFjZU9uVXNlIj48cGF0aCBkPSJNIDIwIDAgTCAwIDAgTCAwIDIwIiBmaWxsPSJub25lIiBzdHJva2U9IiM2YjcyODAiIHN0cm9rZS1vcGFjaXR5PSIwLjEiIHN0cm9rZS13aWR0aD0iMSIvPjwvcGF0dGVybj48L2RlZnM+PHJlY3Qgd2lkdGg9IjEwMCUiIGhlaWdodD0iMTAwJSIgZmlsbD0idXJsKCNncmlkKSIvPjwvc3ZnPg==')] opacity-50"></div>
                        <div class="absolute inset-0 flex items-center justify-center text-white font-semibold text-xl">"Dashboard Preview"</div>
                    </div>
                </section>

                <section id="features" class="container mx-auto px-4 py-20">
                    <div class="text-center mb-16 animate-fade-in" style="animation-delay: 0.4s">
                        <h2 class="text-3xl font-semibold tracking-tight text-white">"Powerful Features"</h2>
                        <p class="text-gray-400 mt-4 max-w-xl mx-auto">
                            "Slynqix combines powerful analysis tools with an intuitive interface for a seamless market intelligence experience."
                        </p>
                    </div>
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                        {vec![
                            ("Console", "Comprehensive statistical and visual analysis tools for market indices."),
                            ("Aftermarket Analysis", "Deep-dive into historical data with advanced filtering and visualization."),
                            ("Mindsage", "Investment guidance considering fundamentals, technicals, and return percentages."),
                            ("Algo Trading", "Automate your trades with sophisticated algorithms and voice commands."),
                            ("Global Sentiment", "Market sentiment analysis from major financial news sources worldwide."),
                            ("Trade Journal", "Record and analyze your trades with comprehensive journaling features."),
                        ].into_iter().enumerate().map(|(index, (title, description))| {
                            view! {
                                <div
                                    class="border border-gray-800 bg-gray-900/50 backdrop-blur-md rounded-2xl p-6 transition-all duration-500 hover:shadow-md hover:shadow-violet-900/20 animate-slide-in-up hover:border-gray-700"
                                    style=format!("animation-delay: {}s", 0.1 * (index as f32 + 1.0))
                                >
                                    <div class="w-10 h-10 bg-violet-900/30 rounded-full flex items-center justify-center mb-4">
                                        <div class="w-6 h-6 bg-violet-600 rounded-full"></div>
                                    </div>
                                    <h3 class="text-xl font-medium mb-2 text-white">{title}</h3>
                                    <p class="text-gray-400">{description}</p>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </section>

                <section class="bg-gray-900 py-20 border-t border-gray-800">
                    <div class="container mx-auto px-4 text-center animate-fade-in" style="animation-delay: 0.5s">
                        <h2 class="text-3xl font-semibold tracking-tight text-white">"Ready to elevate your market analysis?"</h2>
                        <p class="text-gray-400 mt-4 max-w-xl mx-auto">
                            "Join Slynqix today and experience a new level of market intelligence."
                        </p>
                        <Button size="lg" class="mt-10 bg-violet-600 hover:bg-violet-700 rounded-full py-6 px-8">
                            <A href="/dashboard">
                                "Get Started"
                                <Icon icon=Icon::from(LuArrowRight) class="ml-2 h-5 w-5"/>
                            </A>
                        </Button>
                    </div>
                </section>
            </main>

            <footer class="bg-gray-950 border-t border-gray-800 py-8">
                <div class="container mx-auto px-4">
                    <div class="flex flex-col md:flex-row justify-between items-center">
                        <div class="mb-4 md:mb-0">
                            <h2 class="text-xl font-semibold tracking-tighter text-white">
                                <span class="text-violet-500">"S"</span>"lynqix"
                            </h2>
                            <p class="text-sm text-gray-500 mt-1">"Market Analysis Platform"</p>
                        </div>
                        <div class="text-sm text-gray-500">
                            "© " {chrono::Utc::now().year()} " Slynqix. All rights reserved."
                        </div>
                    </div>
                </div>
            </footer>
        </div>
    }
}
