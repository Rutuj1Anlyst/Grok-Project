use leptos::*;
use crate::components::ui::button::Button;

#[component]
pub fn Carousel(
    children: Children,
    orientation: Option<&'static str>, // "horizontal" or "vertical"
    class_name: Option<String>,
) -> impl IntoView {
    let orientation = orientation.unwrap_or("horizontal");
    let (current_index, set_current_index) = create_signal(0);
    let children_vec = children().nodes;
    let can_scroll_prev = move || current_index.get() > 0;
    let can_scroll_next = move || current_index.get() < children_vec.len() - 1;

    let scroll_prev = move |_| if can_scroll_prev() { set_current_index.update(|i| *i -= 1) };
    let scroll_next = move |_| if can_scroll_next() { set_current_index.update(|i| *i += 1) };

    view! {
        <div
            class=move || format!("relative {}", class_name.unwrap_or_default())
            role="region"
            aria-roledescription="carousel"
            on:keydown=move |ev| {
                if ev.key() == "ArrowLeft" {
                    scroll_prev(());
                    ev.prevent_default();
                } else if ev.key() == "ArrowRight" {
                    scroll_next(());
                    ev.prevent_default();
                }
            }
        >
            <div class="overflow-hidden">
                <div
                    class=move || format!(
                        "flex {}",
                        if orientation == "horizontal" { "-ml-4" } else { "-mt-4 flex-col" }
                    )
                    style=move || if orientation == "horizontal" {
                        format!("transform: translateX(-{}%);", current_index.get() * 100)
                    } else {
                        format!("transform: translateY(-{}%);", current_index.get() * 100)
                    }
                >
                    {children_vec.into_iter().enumerate().map(|(i, child)| {
                        view! {
                            <div
                                class=move || format!(
                                    "min-w-0 shrink-0 grow-0 basis-full {}",
                                    if orientation == "horizontal" { "pl-4" } else { "pt-4" }
                                )
                                role="group"
                                aria-roledescription="slide"
                                style=move || if i == current_index.get() { "display: block;" } else { "display: none;" }
                            >
                                {child}
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
            <Button
                variant="outline"
                size="icon"
                class=move || format!(
                    "absolute h-8 w-8 rounded-full {}",
                    if orientation == "horizontal" { "-left-12 top-1/2 -translate-y-1/2" } else { "-top-12 left-1/2 -translate-x-1/2 rotate-90" }
                )
                disabled=move || !can_scroll_prev()
                on_click=scroll_prev
            >
                <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M15 18l-6-6 6-6" />
                </svg>
                <span class="sr-only">"Previous slide"</span>
            </Button>
            <Button
                variant="outline"
                size="icon"
                class=move || format!(
                    "absolute h-8 w-8 rounded-full {}",
                    if orientation == "horizontal" { "-right-12 top-1/2 -translate-y-1/2" } else { "-bottom-12 left-1/2 -translate-x-1/2 rotate-90" }
                )
                disabled=move || !can_scroll_next()
                on_click=scroll_next
            >
                <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M9 18l6-6-6-6" />
                </svg>
                <span class="sr-only">"Next slide"</span>
            </Button>
        </div>
    }
}
