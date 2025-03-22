use leptos::*;

#[component]
pub fn Accordion(children: Children) -> impl IntoView {
    view! {
        <div>
            {children()}
        </div>
    }
}

#[component]
pub fn AccordionItem(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!("border-b {}", class_name.unwrap_or_default())>
            {children()}
        </div>
    }
}

#[component]
pub fn AccordionTrigger(
    children: Children,
    class_name: Option<String>,
    #[prop(into)] value: String,
    #[prop(into)] on_toggle: Callback<String>,
) -> impl IntoView {
    let (is_open, set_is_open) = create_signal(false);

    view! {
        <div class="flex">
            <button
                class=move || format!(
                    "flex flex-1 items-center justify-between py-4 font-medium transition-all hover:underline {} {}",
                    if is_open.get() { "[&>svg]:rotate-180" } else { "" },
                    class_name.unwrap_or_default()
                )
                on:click=move |_| {
                    set_is_open.update(|val| *val = !*val);
                    on_toggle(value.clone());
                }
            >
                {children()}
                <svg
                    class="h-4 w-4 shrink-0 transition-transform duration-200"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                >
                    <path d="M6 9l6 6 6-6" />
                </svg>
            </button>
        </div>
    }
}

#[component]
pub fn AccordionContent(children: Children, class_name: Option<String>) -> impl IntoView {
    let (is_open, _) = use_context::<(ReadSignal<bool>, WriteSignal<bool>)>()
        .unwrap_or_else(|| create_signal(false)); // Fallback if no context

    view! {
        <Show when=move || is_open.get()>
            <div class="overflow-hidden text-sm transition-all animate-accordion-down">
                <div class=move || format!("pb-4 pt-0 {}", class_name.unwrap_or_default())>
                    {children()}
                </div>
            </div>
        </Show>
    }
}
