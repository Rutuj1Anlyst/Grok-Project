 // slynqix/src/pages/model_trainer.rs
use leptos::*;
use leptos_icons::{Icon, LuArrowRight, LuBarChart2, LuCheck, LuDatabase, LuUpload};
use crate::components::{Button, ComingSoon}; // Assume these are defined

#[component]
pub fn ModelTrainer() -> impl IntoView {
    let is_admin = use_context::<RwSignal<bool>>().unwrap_or(create_rw_signal(true)).get();

    if !is_admin {
        return view! {
            <ComingSoon
                title="Model Trainer"
                description="Train models using study materials to generate better suggestions"
                is_admin=false
            />
        }.into_view();
    }

    view! {
        <div class="page-transition pt-20 pb-6 px-4">
            <div class="max-w-7xl mx-auto">
                <header class="mb-8">
                    <h1 class="text-3xl font-semibold tracking-tight">"Model Trainer"</h1>
                    <p class="text-gray-600 dark:text-gray-400 mt-2">
                        "Train models using study materials to generate better suggestions"
                    </p>
                </header>

                <div class="glass-card rounded-xl p-6 animate-fade-in mb-8">
                    <h2 class="text-xl font-medium mb-4">"Upload Training Data"</h2>
                    <div class="border-2 border-dashed border-gray-300 dark:border-gray-700 rounded-xl p-8 text-center">
                        <div class="flex flex-col items-center">
                            <Icon icon=Icon::from(LuUpload) class="h-10 w-10 text-gray-400 mb-4"/>
                            <h3 class="text-lg font-medium mb-2">"Drag and drop files here"</h3>
                            <p class="text-sm text-gray-600 dark:text-gray-400 mb-4">
                                "Support for CSV, JSON, and Excel files"
                            </p>
                            <Button class="bg-accent hover:bg-accent/90 rounded-full">
                                "Browse Files"
                            </Button>
                        </div>
                    </div>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
                    <div class="glass-card rounded-xl p-6 animate-fade-in">
                        <div class="p-2 rounded-full bg-accent/10 w-fit mb-4">
                            <Icon icon=Icon::from(LuDatabase) class="h-5 w-5 text-accent"/>
                        </div>
                        <h2 class="text-xl font-medium mb-3">"Available Models"</h2>
                        <p class="text-gray-600 dark:text-gray-400 mb-4">
                            "Browse and manage your trained models and their performance metrics."
                        </p>
                        <Button class="bg-accent hover:bg-accent/90 rounded-full w-full">
                            "View Models"
                        </Button>
                    </div>

                    <div class="glass-card rounded-xl p-6 animate-fade-in">
                        <div class="p-2 rounded-full bg-accent/10 w-fit mb-4">
                            <Icon icon=Icon::from(LuBarChart2) class="h-5 w-5 text-accent"/>
                        </div>
                        <h2 class="text-xl font-medium mb-3">"Performance Metrics"</h2>
                        <p class="text-gray-600 dark:text-gray-400 mb-4">
                            "View and compare performance metrics across different model versions."
                        </p>
                        <Button class="bg-accent hover:bg-accent/90 rounded-full w-full">
                            "View Metrics"
                        </Button>
                    </div>

                    <div class="glass-card rounded-xl p-6 animate-fade-in">
                        <div class="p-2 rounded-full bg-green-500/10 w-fit mb-4">
                            <Icon icon=Icon::from(LuCheck) class="h-5 w-5 text-green-500"/>
                        </div>
                        <h2 class="text-xl font-medium mb-3">"Deployment"</h2>
                        <p class="text-gray-600 dark:text-gray-400 mb-4">
                            "Deploy trained models to production environment for immediate use."
                        </p>
                        <Button class="bg-green-500 hover:bg-green-600 rounded-full w-full">
                            "Deploy Model"
                        </Button>
                    </div>
                </div>

                <div class="glass-card rounded-xl p-6 animate-fade-in">
                    <h2 class="text-xl font-medium mb-4">"Training History"</h2>
                    <div class="space-y-4">
                        <div class="p-4 rounded-xl border border-gray-200 dark:border-gray-800 hover:border-accent transition-colors">
                            <div class="flex items-center justify-between">
                                <div>
                                    <h3 class="font-medium">"MarketPredictor v2.1"</h3>
                                    <p class="text-sm text-gray-600 dark:text-gray-400">
                                        "Trained on March 19, 2025 • Accuracy: 87.5%"
                                    </p>
                                </div>
                                <Button variant="ghost" size="sm" class="flex items-center gap-1">
                                    "View Details"
                                    <Icon icon=Icon::from(LuArrowRight) class="h-4 w-4"/>
                                </Button>
                            </div>
                        </div>

                        <div class="p-4 rounded-xl border border-gray-200 dark:border-gray-800 hover:border-accent transition-colors">
                            <div class="flex items-center justify-between">
                                <div>
                                    <h3 class="font-medium">"SentimentAnalyzer v1.8"</h3>
                                    <p class="text-sm text-gray-600 dark:text-gray-400">
                                        "Trained on March 17, 2025 • Accuracy: 92.3%"
                                    </p>
                                </div>
                                <Button variant="ghost" size="sm" class="flex items-center gap-1">
                                    "View Details"
                                    <Icon icon=Icon::from(LuArrowRight) class="h-4 w-4"/>
                                </Button>
                            </div>
                        </div>

                        <div class="p-4 rounded-xl border border-gray-200 dark:border-gray-800 hover:border-accent transition-colors">
                            <div class="flex items-center justify-between">
                                <div>
                                    <h3 class="font-medium">"PatternRecognition v3.0"</h3>
                                    <p class="text-sm text-gray-600 dark:text-gray-400">
                                        "Trained on March 15, 2025 • Accuracy: 85.1%"
                                    </p>
                                </div>
                                <Button variant="ghost" size="sm" class="flex items-center gap-1">
                                    "View Details"
                                    <Icon icon=Icon::from(LuArrowRight) class="h-4 w-4"/>
                                </Button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
