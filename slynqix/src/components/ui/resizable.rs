use leptos::*;

#[component]
pub fn ResizablePanelGroup(
    children: Children,
    direction: &'static str, // "horizontal" or "vertical"
    class_name: Option<String>,
) -> impl IntoView {
    view! {
        <div
            class=move || format!(
                "flex h-full w-full {} {}",
                if direction == "vertical" { "flex-col" } else { "" },
                class_name.unwrap_or_default()
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ResizablePanel(children: Children, size: RwSignal<f32>, class_name: Option<String>) -> impl IntoView {
    view! {
        <div
            class=class_name.unwrap_or_default()
            style=move || format!("flex: 0 0 {}%", size.get())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ResizableHandle(
    with_handle: Option<bool>,
    direction: &'static str, // "horizontal" or "vertical"
    on_resize: Callback<f32>,
    class_name: Option<String>,
) -> impl IntoView {
    let with_handle = with_handle.unwrap_or(false);
    let (dragging, set_dragging) = create_signal(false);

    view! {
        <div
            class=move || format!(
                "relative flex items-center justify-center bg-gray-500 {} {} {}",
                if direction == "vertical" { "h-px w-full" } else { "w-px h-full" },
                if dragging.get() { "focus-visible:ring-1 focus-visible:ring-blue-500 focus-visible:ring-offset-1" } else { "" },
                class_name.unwrap_or_default()
            )
            on:mousedown=move |_| set_dragging(true)
            on:mouseup=move |_| set_dragging(false)
            on:mousemove=move |ev| {
                if dragging.get() {
                    let delta = if direction == "vertical" { ev.movement_y() } else { ev.movement_x() };
                    on_resize.call(delta as f32);
                }
            }
        >
            {move || if with_handle {
                view! {
                    <div class="z-10 flex h-4 w-3 items-center justify-center rounded-sm border bg-gray-500">
                        <svg class=move || format!("h-2.5 w-2.5 {}", if direction == "vertical" { "rotate-90" } else { "" }) viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M12 5v14M5 12h14" />
                        </svg>
                    </div>
                }.into()
            } else { None }}
        </div>
    }
}
