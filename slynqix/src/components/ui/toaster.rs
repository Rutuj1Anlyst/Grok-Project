use leptos::*;
use std::collections::VecDeque;

#[component]
pub fn Toaster(
    toasts: RwSignal<VecDeque<(String, String)>>, // (id, message)
    class_name: Option<String>,
) -> impl IntoView {
    view! {
        <div
            class=move || format!(
                "fixed bottom-4 right-4 z-50 flex flex-col gap-2 {}",
                class_name.unwrap_or_default()
            )
        >
            {move || toasts.get().iter().map(|(id, message)| {
                view! {
                    <div
                        class="bg-white dark:bg-gray-900 text-gray-900 dark:text-white border border-gray-300 dark:border-gray-700 shadow-lg p-4 rounded-md animate-in fade-in-0"
                    >
                        {message}
                        <button
                            class="ml-2 text-gray-500 dark:text-gray-400"
                            on:click=move |_| toasts.update(|t| { t.retain(|(t_id, _)| t_id != id) })
                        >
                            "×"
                        </button>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

pub fn toast(toasts: RwSignal<VecDeque<(String, String)>>, message: String) {
    let id = uuid::Uuid::new_v4().to_string();
    toasts.update(|t| t.push_back((id, message)));
}
// In src/components/ui/toaster.rs
use crate::hooks::use_toast::ToasterToast;

#[component]
pub fn Toaster(toasts: ReadSignal<VecDeque<ToasterToast>>, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!("fixed top-0 z-[100] flex max-h-screen w-full flex-col-reverse p-4 sm:bottom-0 sm:right-0 sm:top-auto sm:flex-col md:max-w-[420px] {}", class_name.unwrap_or_default())>
            {move || toasts.get().iter().map(|toast| {
                view! {
                    <Toast open=toast.open set_open=toast.set_open variant=Some("default")>
                        <div class="grid gap-1">
                            {move || toast.title.clone().map(|t| view! { <ToastTitle>{t}</ToastTitle> })}
                            {move || toast.description.clone().map(|d| view! { <ToastDescription>{d}</ToastDescription> })}
                        </div>
                    </Toast>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}