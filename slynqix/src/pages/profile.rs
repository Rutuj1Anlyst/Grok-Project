// slynqix/src/pages/profile.rs
use leptos::*;
use leptos_router::*;
use leptos_icons::{Icon, LuArrowRight, LuBarChart2, LuCalendar, LuEdit, LuLogOut, LuMail, LuMapPin, LuPhone, LuShield, LuUser};
use crate::components::{Button, Card, CardContent, CardHeader, CardTitle, Separator};

#[component]
pub fn Profile() -> impl IntoView {
    let (is_admin_mode, set_is_admin_mode) = create_signal(false);

    // Simulate useEffect for localStorage (client-side only)
    #[cfg(feature = "csr")]
    {
        use wasm_bindgen::prelude::*;
        let window = web_sys::window().expect("no window");
        let storage = window.local_storage().expect("no local storage").expect("no storage");
        let admin_mode = storage.get_item("isAdminMode").expect("failed to get item").unwrap_or("false".to_string());
        set_is_admin_mode.set(admin_mode == "true");

        let closure = Closure::wrap(Box::new(move || {
            let admin_mode = storage.get_item("isAdminMode").expect("failed to get item").unwrap_or("false".to_string());
            set_is_admin_mode.set(admin_mode == "true");
        }) as Box<dyn FnMut()>);
        window.add_event_listener_with_callback("storage", closure.as_ref().unchecked_ref()).expect("failed to add listener");
        on_cleanup(move || {
            window.remove_event_listener_with_callback("storage", closure.as_ref().unchecked_ref()).expect("failed to remove listener");
            closure.forget();
        });
    }

    view! {
        <div class="page-transition pt-20 pb-6 px-4">
            <div class="max-w-6xl mx-auto">
                <header class="mb-8">
                    <h1 class="text-3xl font-semibold tracking-tight">"Profile"</h1>
                    <p class="text-gray-400 mt-2">"Manage your account information and preferences"</p>
                </header>

                <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                    <div class="col-span-1">
                        <div class="space-y-6">
                            <Card class="border border-gray-800 bg-gray-900/50 backdrop-blur-sm overflow-hidden">
                                <CardHeader class="pb-0">
                                    <div class="flex justify-between items-start">
                                        <CardTitle class="text-lg text-white">"User Information"</CardTitle>
                                        <Button variant="ghost" size="icon" class="h-8 w-8 text-gray-400 hover:text-white hover:bg-gray-800">
                                            <Icon icon=Icon::from(LuEdit) class="h-4 w-4"/>
                                        </Button>
                                    </div>
                                </CardHeader>
                                <CardContent class="pt-4">
                                    <div class="flex flex-col items-center text-center">
                                        <div class="h-20 w-20 bg-gradient-to-br from-violet-600 to-violet-800 rounded-full flex items-center justify-center text-2xl font-bold text-white mb-3">"AS"</div>
                                        <h3 class="text-lg font-medium text-white">"Aditya Singh"</h3>
                                        <p class="text-sm text-violet-400">"Premium Account"</p>
                                        <div class="w-full mt-4 space-y-3">
                                            <div class="flex items-center bg-gray-800 rounded-lg p-3">
                                                <Icon icon=Icon::from(LuUser) class="h-4 w-4 text-gray-400 mr-3"/>
                                                <div class="text-sm">
                                                    <p class="text-white">"Aditya Singh"</p>
                                                    <p class="text-gray-400 text-xs">"Full Name"</p>
                                                </div>
                                            </div>
                                            <div class="flex items-center bg-gray-800 rounded-lg p-3">
                                                <Icon icon=Icon::from(LuMail) class="h-4 w-4 text-gray-400 mr-3"/>
                                                <div class="text-sm">
                                                    <p class="text-white">"aditya.singh@gmail.com"</p>
                                                    <p class="text-gray-400 text-xs">"Email"</p>
                                                </div>
                                            </div>
                                            <div class="flex items-center bg-gray-800 rounded-lg p-3">
                                                <Icon icon=Icon::from(LuPhone) class="h-4 w-4 text-gray-400 mr-3"/>
                                                <div class="text-sm">
                                                    <p class="text-white">"+91 9876543210"</p>
                                                    <p class="text-gray-400 text-xs">"Phone"</p>
                                                </div>
                                            </div>
                                            <div class="flex items-center bg-gray-800 rounded-lg p-3">
                                                <Icon icon=Icon::from(LuMapPin) class="h-4 w-4 text-gray-400 mr-3"/>
                                                <div class="text-sm">
                                                    <p class="text-white">"Mumbai, India"</p>
                                                    <p class="text-gray-400 text-xs">"Location"</p>
                                                </div>
                                            </div>
                                            <div class="flex items-center bg-gray-800 rounded-lg p-3">
                                                <Icon icon=Icon::from(LuCalendar) class="h-4 w-4 text-gray-400 mr-3"/>
                                                <div class="text-sm">
                                                    <p class="text-white">"Member since May 2023"</p>
                                                    <p class="text-gray-400 text-xs">"Joined"</p>
                                                </div>
                                            </div>
                                            <div class="flex items-center bg-gray-800 rounded-lg p-3">
                                                <Icon icon=Icon::from(LuShield) class="h-4 w-4 text-gray-400 mr-3"/>
                                                <div class="text-sm">
                                                    <p class="text-white">{move || if is_admin_mode.get() { "Admin" } else { "User" }}</p>
                                                    <p class="text-gray-400 text-xs">"Current Access Level"</p>
                                                </div>
                                            </div>
                                        </div>
                                        <Button variant="outline" class="w-full mt-6 border-gray-700 text-gray-300 hover:bg-gray-800">
                                            <Icon icon=Icon::from(LuLogOut) class="h-4 w-4 mr-2"/>
                                            "Sign Out"
                                        </Button>
                                    </div>
                                </CardContent>
                            </Card>
                            <Show when=move || is_admin_mode.get()>
                                <Card class="border border-gray-800 bg-gray-900/50 backdrop-blur-sm overflow-hidden animate-fade-in">
                                    <CardHeader>
                                        <CardTitle class="text-lg text-white">"Admin Access"</CardTitle>
                                    </CardHeader>
                                    <CardContent>
                                        <p class="text-sm text-gray-400 mb-4">
                                            "Access administrator dashboard to manage platform settings, user accounts and advanced features."
                                        </p>
                                        <Button class="w-full bg-violet-600 hover:bg-violet-700">
                                            <A href="/admin">
                                                "Access Admin Dashboard"
                                                <Icon icon=Icon::from(LuArrowRight) class="ml-2 h-4 w-4"/>
                                            </A>
                                        </Button>
                                    </CardContent>
                                </Card>
                            </Show>
                        </div>
                    </div>

                    <div class="col-span-1 lg:col-span-2 space-y-6">
                        <Card class="border border-gray-800 bg-gray-900/50 backdrop-blur-sm overflow-hidden">
                            <CardHeader>
                                <CardTitle class="text-lg text-white">"Trading Statistics"</CardTitle>
                            </CardHeader>
                            <CardContent>
                                <div class="grid grid-cols-1 sm:grid-cols-3 gap-4 mb-6">
                                    <div class="bg-gray-800 rounded-lg p-4">
                                        <div class="flex items-center justify-between">
                                            <h4 class="text-sm font-medium text-gray-400">"Win Rate"</h4>
                                            <Icon icon=Icon::from(LuBarChart2) class="h-4 w-4 text-violet-400"/>
                                        </div>
                                        <p class="text-2xl font-semibold mt-2 text-white">"67%"</p>
                                        <p class="text-xs text-green-400 mt-1">"+5% from last month"</p>
                                    </div>
                                    <div class="bg-gray-800 rounded-lg p-4">
                                        <div class="flex items-center justify-between">
                                            <h4 class="text-sm font-medium text-gray-400">"Total Trades"</h4>
                                            <Icon icon=Icon::from(LuBarChart2) class="h-4 w-4 text-violet-400"/>
                                        </div>
                                        <p class="text-2xl font-semibold mt-2 text-white">"183"</p>
                                        <p class="text-xs text-gray-400 mt-1">"Last 3 months"</p>
                                    </div>
                                    <div class="bg-gray-800 rounded-lg p-4">
                                        <div class="flex items-center justify-between">
                                            <h4 class="text-sm font-medium text-gray-400">"Avg. Profit"</h4>
                                            <Icon icon=Icon::from(LuBarChart2) class="h-4 w-4 text-violet-400"/>
                                        </div>
                                        <p class="text-2xl font-semibold mt-2 text-white">"₹4,280"</p>
                                        <p class="text-xs text-green-400 mt-1">"+12% from last month"</p>
                                    </div>
                                </div>
                                <div class="flex justify-center h-40 items-center border border-gray-800 rounded-lg bg-gray-800/50 mb-4">
                                    <p class="text-gray-400">"Trading performance chart will appear here"</p>
                                </div>
                                <Button class="w-full mt-2 bg-violet-600 hover:bg-violet-700">
                                    "View Detailed Analytics"
                                    <Icon icon=Icon::from(LuArrowRight) class="ml-2 h-4 w-4"/>
                                </Button>
                            </CardContent>
                        </Card>

                        <Card class="border border-gray-800 bg-gray-900/50 backdrop-blur-sm overflow-hidden">
                            <CardHeader>
                                <CardTitle class="text-lg text-white">"Account Settings"</CardTitle>
                            </CardHeader>
                            <CardContent class="space-y-4">
                                <div>
                                    <h4 class="text-sm font-medium text-white mb-2">"Notification Preferences"</h4>
                                    <div class="space-y-2">
                                        <div class="flex items-center justify-between">
                                            <span class="text-sm text-gray-400">"Email Notifications"</span>
                                            <div class="w-10 h-5 bg-violet-600 rounded-full relative">
                                                <div class="absolute right-0.5 top-0.5 bg-white w-4 h-4 rounded-full"></div>
                                            </div>
                                        </div>
                                        <div class="flex items-center justify-between">
                                            <span class="text-sm text-gray-400">"Price Alerts"</span>
                                            <div class="w-10 h-5 bg-violet-600 rounded-full relative">
                                                <div class="absolute right-0.5 top-0.5 bg-white w-4 h-4 rounded-full"></div>
                                            </div>
                                        </div>
                                        <div class="flex items-center justify-between">
                                            <span class="text-sm text-gray-400">"Market Updates"</span>
                                            <div class="w-10 h-5 bg-gray-700 rounded-full relative">
                                                <div class="absolute left-0.5 top-0.5 bg-white w-4 h-4 rounded-full"></div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                <Separator class="bg-gray-800"/>
                                <div>
                                    <h4 class="text-sm font-medium text-white mb-2">"Trading Preferences"</h4>
                                    <div class="space-y-2">
                                        <div class="flex items-center justify-between">
                                            <span class="text-sm text-gray-400">"Default Market"</span>
                                            <div class="flex items-center">
                                                <span class="text-sm text-white">"Nifty 50"</span>
                                                <button class="ml-2 p-1 text-gray-400 hover:text-white">
                                                    <Icon icon=Icon::from(LuEdit) class="h-3.5 w-3.5"/>
                                                </button>
                                            </div>
                                        </div>
                                        <div class="flex items-center justify-between">
                                            <span class="text-sm text-gray-400">"Risk Level"</span>
                                            <div class="flex items-center">
                                                <span class="text-sm text-white">"Moderate"</span>
                                                <button class="ml-2 p-1 text-gray-400 hover:text-white">
                                                    <Icon icon=Icon::from(LuEdit) class="h-3.5 w-3.5"/>
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                <Separator class="bg-gray-800"/>
                                <Button class="w-full bg-violet-600 hover:bg-violet-700">"Save Preferences"</Button>
                            </CardContent>
                        </Card>
                    </div>
                </div>
            </div>
        </div>
    }
}
