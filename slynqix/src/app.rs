// slynqix/src/app.rs
use leptos::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;
use crate::pages::admin_dashboard::AdminDashboard;
use crate::pages::aftermarket_analyzer::AftermarketAnalyzer;
use crate::pages::algo_trading::AlgoTrading;
use crate::pages::console::Console;
use crate::pages::dashboard::Dashboard;
use crate::pages::global_sentiment::GlobalSentiment;
use crate::pages::history::History;
use crate::pages::index::Index;
use crate::pages::journal::Journal;
use crate::pages::mindsage::Mindsage;
use crate::pages::model_trainer::ModelTrainer;
use crate::pages::not_found::NotFound;
use crate::pages::profile::Profile;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Sidebar state
    let (sidebar_open, set_sidebar_open) = create_signal(cx, false);

    // Admin state from local storage
    let (is_admin, set_is_admin) = create_signal(cx, {
        let window = web_sys::window().expect("no window");
        let storage = window.local_storage().expect("no local storage").expect("storage unavailable");
        storage.get_item("isAdminMode").unwrap_or(Some("false".to_string())).unwrap() == "true"
    });
    provide_context(cx, is_admin);

    view! { cx,
        <div class="min-h-screen bg-gray-950 text-white">
            <Router>
                <AppContent sidebar_open set_sidebar_open />
            </Router>
        </div>
    }
}

#[component]
fn AppContent(
    cx: Scope,
    sidebar_open: ReadSignal<bool>,
    set_sidebar_open: WriteSignal<bool>,
) -> impl IntoView {
    let location = use_location(cx);
    let is_landing_page = move || location.pathname.get() == "/";

    view! { cx,
        <Show
            when=move || !is_landing_page()
            fallback=move |cx| view! { cx,
                <Routes>
                    <Route path="" view=Index />
                </Routes>
            }
        >
            <Header toggle_sidebar=move || set_sidebar_open.set(!sidebar_open.get()) />
            <Sidebar is_open=sidebar_open set_is_open=set_sidebar_open />
            <main class=move || format!(
                "transition-all duration-300 ease-in-out {}",
                if sidebar_open.get() { "ml-64" } else { "ml-0" }
            )>
                <Routes>
                    <Route path="/dashboard" view=Dashboard />
                    <Route path="/console" view=Console />
                    <Route path="/aftermarket" view=AftermarketAnalyzer />
                    <Route path="/history" view=History />
                    <Route path="/mindsage" view=Mindsage />
                    <Route path="/algo-trading" view=AlgoTrading />
                    <Route path="/global-sentiment" view=GlobalSentiment />
                    <Route path="/model-trainer" view=ModelTrainer />
                    <Route path="/journal" view=Journal />
                    <Route path="/profile" view=Profile />
                    <Route path="/admin" view=AdminDashboard />
                    <Route path="/*any" view=NotFound />
                </Routes>
            </main>
        </Show>
    }
}

#[component]
fn Header(cx: Scope, toggle_sidebar: impl Fn() + 'static) -> impl IntoView {
    let navigate = use_navigate(cx);
    view! { cx,
        <header class="fixed top-0 left-0 right-0 z-50 backdrop-blur-md bg-gray-950/80 border-b border-gray-800">
            <div class="container mx-auto h-16 flex items-center justify-between px-4">
                <div class="flex items-center">
                    <h1 class="text-xl font-semibold tracking-tighter text-white">
                        <span class="text-violet-500">"S"</span>"lynqix"
                    </h1>
                </div>
                <div class="flex items-center space-x-4">
                    <button
                        class="text-gray-300 hover:text-white md:hidden"
                        on:click=move |_| toggle_sidebar()
                    >
                        "☰"
                    </button>
                    <div class="hidden md:flex space-x-4">
                        <A href="/dashboard" class="text-sm font-medium text-gray-300 hover:text-white">"Dashboard"</A>
                        <A href="/profile" class="text-sm font-medium text-gray-300 hover:text-white">"Profile"</A>
                        <button
                            class="text-sm font-medium text-gray-300 hover:text-white"
                            on:click=move |_| {
                                let window = web_sys::window().unwrap();
                                let storage = window.local_storage().unwrap().unwrap();
                                storage.set_item("isAdminMode", "false").unwrap();
                                navigate("/login", Default::default());
                            }
                        >
                            "Logout"
                        </button>
                    </div>
                </div>
            </div>
        </header>
    }
}

#[component]
fn Sidebar(
    cx: Scope,
    is_open: ReadSignal<bool>,
    set_is_open: WriteSignal<bool>,
) -> impl IntoView {
    let location = use_location(cx);
    let current_path = move || location.pathname.get();

    view! { cx,
        <aside class=move || format!(
            "fixed top-16 bottom-0 w-64 bg-gray-900 border-r border-gray-800 transition-transform duration-300 ease-in-out {}",
            if is_open.get() { "translate-x-0" } else { "-translate-x-full md:translate-x-0" }
        )>
            <nav class="p-4">
                <ul class="space-y-2">
                    <li>
                        <A href="/dashboard" class=move || format!(
                            "block p-2 rounded {}",
                            if current_path() == "/dashboard" { "bg-violet-600 text-white" } else { "hover:bg-gray-800" }
                        )>"Dashboard"</A>
                    </li>
                    <li>
                        <A href="/console" class=move || format!(
                            "block p-2 rounded {}",
                            if current_path() == "/console" { "bg-violet-600 text-white" } else { "hover:bg-gray-800" }
                        )>"Console"</A>
                    </li>
                    <li>
                        <A href="/aftermarket" class=move || format!(
                            "block p-2 rounded {}",
                            if current_path() == "/aftermarket" { "bg-violet-600 text-white" } else { "hover:bg-gray-800" }
                        )>"Aftermarket"</A>
                    </li>
                    <li>
                        <A href="/history" class=move || format!(
                            "block p-2 rounded {}",
                            if current_path() == "/history" { "bg-violet-600 text-white" } else { "hover:bg-gray-800" }
                        )>"History"</A>
                    </li>
                    <li>
                        <A href="/mindsage" class=move || format!(
                            "block p-2 rounded {}",
                            if current_path() == "/mindsage" { "bg-violet-600 text-white" } else { "hover:bg-gray-800" }
                        )>"Mindsage"</A>
                    </li>
                    <li>
                        <A href="/algo-trading" class=move || format!(
                            "block p-2 rounded {}",
                            if current_path() == "/algo-trading" { "bg-violet-600 text-white" } else { "hover:bg-gray-800" }
                        )>"Algo Trading"</A>
                    </li>
                    <li>
                        <A href="/global-sentiment" class=move || format!(
                            "block p-2 rounded {}",
                            if current_path() == "/global-sentiment" { "bg-violet-600 text-white" } else { "hover:bg-gray-800" }
                        )>"Global Sentiment"</A>
                    </li>
                    <li>
                        <A href="/model-trainer" class=move || format!(
                            "block p-2 rounded {}",
                            if current_path() == "/model-trainer" { "bg-violet-600 text-white" } else { "hover:bg-gray-800" }
                        )>"Model Trainer"</A>
                    </li>
                    <li>
                        <A href="/journal" class=move || format!(
                            "block p-2 rounded {}",
                            if current_path() == "/journal" { "bg-violet-600 text-white" } else { "hover:bg-gray-800" }
                        )>"Journal"</A>
                    </li>
                    <li>
                        <A href="/profile" class=move || format!(
                            "block p-2 rounded {}",
                            if current_path() == "/profile" { "bg-violet-600 text-white" } else { "hover:bg-gray-800" }
                        )>"Profile"</A>
                    </li>
                    <li>
                        <A href="/admin" class=move || format!(
                            "block p-2 rounded {}",
                            if current_path() == "/admin" { "bg-violet-600 text-white" } else { "hover:bg-gray-800" }
                        )>"Admin"</A>
                    </li>
                </ul>
            </nav>
            <button
                class="absolute top-4 right-4 text-gray-300 hover:text-white md:hidden"
                on:click=move |_| set_is_open.set(false)
            >
                "X"
            </button>
        </aside>
    }
}