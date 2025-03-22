use leptos::*;
use web_sys::{MouseEvent, window};

#[component]
pub fn PictureInPicture<F>(
    children: Children,
    title: String,
    is_open: ReadSignal<bool>,
    on_close: F,
    width: Option<i32>,
    height: Option<i32>,
) -> impl IntoView
where
    F: Fn() + 'static,
{
    let width = width.unwrap_or(320);
    let height = height.unwrap_or(240);
    let (position, set_position) = create_signal((window().unwrap().inner_width().unwrap().as_f64().unwrap() as i32 - width - 20, window().unwrap().inner_height().unwrap().as_f64().unwrap() as i32 - height - 20));
    let (is_dragging, set_is_dragging) = create_signal(false);
    let (drag_offset, set_drag_offset) = create_signal((0, 0));
    let container_ref = create_node_ref::<html::Div>();

    // Resize effect
    create_effect(move |_| {
        if !is_open.get() { return; }
        let win = window().unwrap();
        let max_x = win.inner_width().unwrap().as_f64().unwrap() as i32 - width;
        let max_y = win.inner_height().unwrap().as_f64().unwrap() as i32 - height;
        set_position.update(|(x, y)| {
            *x = (*x).min(max_x).max(0);
            *y = (*y).min(max_y).max(0);
        });
    });

    // Dragging effect (simplified due to WebAssembly constraints)
    let handle_mouse_down = move |ev: MouseEvent| {
        if let Some(container) = container_ref.get() {
            let rect = container.get_bounding_client_rect();
            set_drag_offset((ev.client_x() - rect.left() as i32, ev.client_y() - rect.top() as i32));
            set_is_dragging(true);
            ev.prevent_default();
        }
    };

    let handle_mouse_move = move |ev: MouseEvent| {
        if is_dragging.get() {
            let new_x = ev.client_x() - drag_offset.get().0;
            let new_y = ev.client_y() - drag_offset.get().1;
            let win = window().unwrap();
            let max_x = win.inner_width().unwrap().as_f64().unwrap() as i32 - width;
            let max_y = win.inner_height().unwrap().as_f64().unwrap() as i32 - height;
            set_position((new_x.min(max_x).max(0), new_y.min(max_y).max(0)));
            ev.prevent_default();
        }
    };

    let handle_mouse_up = move |_| {
        set_is_dragging(false);
    };

    view! {
        {move || if is_open.get() {
            view! {
                <div
                    node_ref=container_ref
                    class="fixed z-50 bg-gray-900 border border-gray-800 rounded-lg shadow-lg overflow-hidden animate-fade-in transition-all"
                    style=move || format!("left: {}px; top: {}px; width: {}px; height: {}px;", position.get().0, position.get().1, width, height)
                    on:mousemove=handle_mouse_move
                    on:mouseup=handle_mouse_up
                >
                    <div
                        class="bg-gray-800 p-2 flex items-center justify-between cursor-grab"
                        class:is-dragging=move || is_dragging.get()
                        on:mousedown=handle_mouse_down
                    >
                        <div class="text-sm font-medium text-gray-200 truncate">{title}</div>
                        <div class="flex gap-1">
                            <button
                                class="text-gray-400 hover:text-white transition-colors p-1 rounded hover:bg-gray-700"
                                on:click=move |_| on_close()
                            >
                                <svg class="h-3.5 w-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <path d="M6 18L18 6M6 6l12 12" />
                                </svg>
                            </button>
                        </div>
                    </div>
                    <div class="h-[calc(100%-36px)] overflow-auto px-2 py-2">
                        {children()}
                    </div>
                </div>
            }
        } else {
            None
        }}
    }
}
