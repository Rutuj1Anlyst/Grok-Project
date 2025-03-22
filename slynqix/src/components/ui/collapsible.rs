use leptos::*;

#[component]
pub fn Collapsible(
    children: Children,
    open: ReadSignal<bool>,
    set_open: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <div>
            {children()}
        </div>
    }
}

#[component]
pub fn CollapsibleTrigger(
    children: Children,
    open: ReadSignal<bool>,
    set_open: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <button on:click=move |_| set_open.update(|val| *val = !*val)>
            {children()}
        </button>
    }
}

#[component]
pub fn CollapsibleContent(children: Children, open: ReadSignal<bool>) -> impl IntoView {
    view! {
        <Show when=move || open.get()>
            <div class="transition-all animate-[slide-down_200ms_ease-in-out]">
                {children()}
            </div>
        </Show>
    }
}
