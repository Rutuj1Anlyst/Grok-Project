use leptos::*;
use web_sys::{window, MediaQueryList};

const MOBILE_BREAKPOINT: i32 = 768;

pub fn use_is_mobile() -> ReadSignal<bool> {
    let (is_mobile, set_is_mobile) = create_signal(false);

    create_effect(move |_| {
        if let Some(window) = window() {
            let update_mobile = move || {
                set_is_mobile.set(window.inner_width().unwrap_or(0) < MOBILE_BREAKPOINT);
            };

            // Initial check
            update_mobile();

            // Media query listener
            if let Ok(mql) = window.match_media(&format!("(max-width: {}px)", MOBILE_BREAKPOINT - 1)) {
                if let Some(mql) = mql {
                    let handler = Closure::wrap(Box::new(move |_: web_sys::MediaQueryListEvent| {
                        update_mobile();
                    }) as Box<dyn FnMut(_)>);
                    let _ = mql.add_listener_with_opt_callback(Some(handler.as_ref().unchecked_ref()));
                    on_cleanup(move || {
                        let _ = mql.remove_listener_with_opt_callback(Some(handler.as_ref().unchecked_ref()));
                        handler.forget(); // Prevent memory leak
                    });
                }
            }
        }
    });

    is_mobile
}
